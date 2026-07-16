//! Real SSH Client for Huawei OLT using ssh2 (libssh2 bindings)
//!
//! Provides actual SSH connections to Huawei OLT devices with:
//! - Password authentication
//! - Enable mode escalation
//! - Command execution with output capture
//! - Connection per request pattern (creates new connection for each command)

use std::io::Read;
use std::net::TcpStream;

use ssh2::Session;
use tracing::{debug, info};

use crate::shared::errors::AppError;

// ============================================================================
// SSH Helper Functions
// ============================================================================

/// Connect to an SSH server and authenticate
fn ssh_connect(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    timeout_secs: u64,
) -> Result<Session, String> {
    let addr = format!("{}:{}", host, port);

    debug!(host = %host, port = port, "Connecting to Huawei OLT via SSH");

    // Establish TCP connection
    let tcp = TcpStream::connect(&addr)
        .map_err(|e| format!("TCP connection failed to {}: {}", addr, e))?;

    tcp.set_read_timeout(Some(std::time::Duration::from_secs(timeout_secs)))
        .map_err(|e| format!("Failed to set read timeout: {}", e))?;

    tcp.set_write_timeout(Some(std::time::Duration::from_secs(timeout_secs)))
        .map_err(|e| format!("Failed to set write timeout: {}", e))?;

    // Create SSH session
    let mut session = Session::new()
        .map_err(|e| format!("Failed to create SSH session: {}", e))?;

    session.set_tcp_stream(tcp);
    session.set_timeout((timeout_secs * 1000) as u32);

    // Perform SSH handshake
    session.handshake()
        .map_err(|e| format!("SSH handshake failed: {}", e))?;

    // Authenticate with password
    session.userauth_password(username, password)
        .map_err(|e| format!("SSH authentication failed: {}", e))?;

    if !session.authenticated() {
        return Err("SSH authentication rejected".to_string());
    }

    info!(host = %host, "Successfully connected to Huawei OLT");

    Ok(session)
}

/// Execute a command on an SSH session and return the output
fn ssh_execute_command(session: &Session, command: &str) -> Result<String, String> {
    debug!(command = %command, "Executing SSH command");

    let mut channel = session.channel_session()
        .map_err(|e| format!("Failed to open channel: {}", e))?;

    channel.exec(command)
        .map_err(|e| format!("Failed to exec command: {}", e))?;

    let mut output = String::new();
    channel.read_to_string(&mut output)
        .map_err(|e| format!("Failed to read output: {}", e))?;

    let _ = channel.wait_close();

    debug!(output_len = output.len(), "SSH command execution completed");

    Ok(output)
}

// ============================================================================
// Public API - All functions are async via spawn_blocking
// ============================================================================

/// Execute an SSH command on a Huawei OLT device
///
/// This function creates a new SSH connection, executes the command, and disconnects.
/// This is suitable for periodic polling operations where connection reuse is not critical.
pub async fn execute_olt_command(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    command: &str,
    timeout_secs: u64,
) -> Result<String, AppError> {
    let host = host.to_string();
    let username = username.to_string();
    let password = password.to_string();
    let command = command.to_string();

    let result = tokio::task::spawn_blocking(move || {
        // Connect
        let session = ssh_connect(&host, port, &username, &password, timeout_secs)
            .map_err(AppError::External)?;

        // Execute command
        let output = ssh_execute_command(&session, &command)
            .map_err(AppError::External)?;

        // Session will be dropped and cleaned up automatically

        Ok::<String, AppError>(output)
    })
    .await
    .map_err(|e| AppError::External(format!("SSH task failed: {}", e)))?;

    result
}

/// Execute an SSH command with enable mode on a Huawei OLT device
///
/// This function creates a new SSH connection, enters enable mode, executes the command, and disconnects.
pub async fn execute_olt_command_with_enable(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    enable_password: &str,
    command: &str,
    timeout_secs: u64,
) -> Result<String, AppError> {
    let host = host.to_string();
    let username = username.to_string();
    let password = password.to_string();
    let enable_password = enable_password.to_string();
    let command = command.to_string();

    let result = tokio::task::spawn_blocking(move || {
        // Connect
        let session = ssh_connect(&host, port, &username, &password, timeout_secs)
            .map_err(AppError::External)?;

        // Enter enable mode
        debug!("Entering enable mode");
        let enable_output = ssh_execute_command(&session, "enable")
            .map_err(AppError::External)?;

        if enable_output.contains("Password:") || enable_output.contains("password:") {
            let _ = ssh_execute_command(&session, &enable_password)
                .map_err(AppError::External)?;
        }
        info!("Entered enable mode");

        // Execute the actual command
        let output = ssh_execute_command(&session, &command)
            .map_err(AppError::External)?;

        // Session will be dropped and cleaned up automatically

        Ok::<String, AppError>(output)
    })
    .await
    .map_err(|e| AppError::External(format!("SSH task failed: {}", e)))?;

    result
}
