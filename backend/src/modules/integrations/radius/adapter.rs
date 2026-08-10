//! RADIUS Adapter for PPPoE Authentication
//!
//! Implements RADIUS client for:
//! - PPPoE authentication (Access-Request)
//! - Accounting (Accounting-Request)
//! - CoA (Change of Authorization) for dynamic bandwidth changes
//!
//! Protocol: RFC 2865 (Authentication), RFC 2866 (Accounting), RFC 5176 (CoA)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::{debug, info, warn};

use crate::shared::errors::AppError;

// ============================================================================
// Configuration
// ============================================================================

/// RADIUS server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusConfig {
    pub server: String,
    pub port: u16,
    pub accounting_port: u16,
    pub coa_port: u16,
    pub secret: String,
    pub timeout_seconds: u32,
    pub max_retries: u32,
}

impl Default for RadiusConfig {
    fn default() -> Self {
        Self {
            server: std::env::var("RADIUS_SERVER").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("RADIUS_PORT")
                .unwrap_or_else(|_| "1812".to_string())
                .parse()
                .unwrap_or(1812),
            accounting_port: std::env::var("RADIUS_ACCOUNTING_PORT")
                .unwrap_or_else(|_| "1813".to_string())
                .parse()
                .unwrap_or(1813),
            coa_port: std::env::var("RADIUS_COA_PORT")
                .unwrap_or_else(|_| "3799".to_string())
                .parse()
                .unwrap_or(3799),
            secret: std::env::var("RADIUS_SECRET").unwrap_or_default(),
            timeout_seconds: std::env::var("RADIUS_TIMEOUT")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            max_retries: std::env::var("RADIUS_MAX_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .unwrap_or(3),
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

/// RADIUS packet types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadiusPacketType {
    AccessRequest = 1,
    AccessAccept = 2,
    AccessReject = 3,
    AccountingRequest = 4,
    AccountingResponse = 5,
    CoARequest = 37,
    CoAACK = 38,
    CoANAK = 39,
}

impl RadiusPacketType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::AccessRequest),
            2 => Some(Self::AccessAccept),
            3 => Some(Self::AccessReject),
            4 => Some(Self::AccountingRequest),
            5 => Some(Self::AccountingResponse),
            37 => Some(Self::CoARequest),
            38 => Some(Self::CoAACK),
            39 => Some(Self::CoANAK),
            _ => None,
        }
    }
}

/// RADIUS attribute types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RadiusAttribute {
    UserName(String),
    UserPassword(String),
    NasIpAddress(String),
    NasPort(u32),
    ServiceType(u32),
    FramedProtocol(u32),
    FramedIpAddress(String),
    FramedIpNetmask(String),
    FramedRoute(String),
    FilterId(String),
    AcctSessionId(String),
    AcctStatusType(u32),
    AcctInputOctets(u64),
    AcctOutputOctets(u64),
    AcctSessionTime(u32),
    AcctTerminateCause(u32),
    MessageAuthenticator(Vec<u8>),
    VendorSpecific(u32, Vec<u8>),
    Other(u8, Vec<u8>),
}

/// RADIUS request builder
#[derive(Debug, Clone)]
pub struct RadiusRequest {
    pub packet_type: RadiusPacketType,
    pub identifier: u8,
    pub attributes: Vec<RadiusAttribute>,
}

/// RADIUS response
#[derive(Debug, Clone)]
pub struct RadiusResponse {
    pub packet_type: RadiusPacketType,
    pub identifier: u8,
    pub attributes: Vec<RadiusAttribute>,
    pub response_auth_valid: bool,
}

/// PPPoE authentication request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PppoeAuthRequest {
    pub username: String,
    pub password: String,
    pub nas_ip: String,
    pub nas_port: u32,
    pub calling_station_id: String, // MAC address
    pub framed_ip: Option<String>,
}

/// PPPoE authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PppoeAuthResponse {
    pub accepted: bool,
    pub framed_ip: Option<String>,
    pub framed_route: Option<String>,
    pub filter_id: Option<String>,
    pub session_id: Option<String>,
    pub error_message: Option<String>,
}

/// Accounting request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingRequest {
    pub username: String,
    pub session_id: String,
    pub status_type: AccountingStatusType,
    pub nas_ip: String,
    pub nas_port: u32,
    pub input_octets: Option<u64>,
    pub output_octets: Option<u64>,
    pub session_time: Option<u32>,
    pub terminate_cause: Option<u32>,
}

/// Accounting status types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AccountingStatusType {
    Start = 1,
    Stop = 2,
    InterimUpdate = 3,
}

/// CoA request for dynamic bandwidth changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoARequest {
    pub username: String,
    pub nas_ip: String,
    pub nas_port: u32,
    pub filter_id: Option<String>,
    pub framed_ip: Option<String>,
}

/// CoA response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoAResponse {
    pub accepted: bool,
    pub error_message: Option<String>,
}

// ============================================================================
// RADIUS Protocol Implementation
// ============================================================================

/// RADIUS protocol constants
const RADIUS_HEADER_SIZE: usize = 20;
const RADIUS_MAX_PACKET_SIZE: usize = 4096;

/// Build a RADIUS packet from request
fn build_radius_packet(request: &RadiusRequest, secret: &str) -> Vec<u8> {
    let mut packet = Vec::with_capacity(RADIUS_MAX_PACKET_SIZE);

    // Header
    packet.push(request.packet_type as u8);
    packet.push(request.identifier);

    // Length placeholder (2 bytes)
    let length_pos = packet.len();
    packet.push(0);
    packet.push(0);

    // Authenticator (16 bytes) - will be filled later
    let authenticator_pos = packet.len();
    packet.extend_from_slice(&[0u8; 16]);

    // Attributes
    for attr in &request.attributes {
        match attr {
            RadiusAttribute::UserName(name) => {
                packet.push(1); // Type
                let name_bytes = name.as_bytes();
                packet.push((2 + name_bytes.len()) as u8);
                packet.extend_from_slice(name_bytes);
            }
            RadiusAttribute::UserPassword(password) => {
                packet.push(2); // Type
                let padded_len = ((password.len() / 16) + 1) * 16;
                let attr_len = 2 + padded_len;
                packet.push(attr_len as u8);

                // RFC 2865 password encoding: chain XOR blocks with MD5(prev_ciphertext + secret)
                let password_bytes = password.as_bytes();

                // Initial vector: the authenticator (16 bytes)
                let mut prev_ciphertext = [0u8; 16];
                prev_ciphertext.copy_from_slice(&packet[authenticator_pos..authenticator_pos + 16]);

                for chunk_start in (0..padded_len).step_by(16) {
                    // MD5(previous ciphertext block + secret)
                    let mut hash_input = Vec::with_capacity(32);
                    hash_input.extend_from_slice(&prev_ciphertext);
                    hash_input.extend_from_slice(secret.as_bytes());
                    let hash = md5_hash(&hash_input);

                    // XOR this 16-byte block of the password with the hash
                    for i in 0..16 {
                        let p = if chunk_start + i < password_bytes.len() {
                            password_bytes[chunk_start + i]
                        } else {
                            0
                        };
                        let ciphertext = p ^ hash[i];
                        prev_ciphertext[i] = ciphertext;
                        packet.push(ciphertext);
                    }
                }
            }
            RadiusAttribute::NasIpAddress(ip) => {
                packet.push(4); // Type
                packet.push(6); // Length
                let addr: std::net::Ipv4Addr =
                    ip.parse().unwrap_or(std::net::Ipv4Addr::UNSPECIFIED);
                packet.extend_from_slice(&addr.octets());
            }
            RadiusAttribute::NasPort(port) => {
                packet.push(5); // Type
                packet.push(6); // Length
                packet.extend_from_slice(&port.to_be_bytes());
            }
            RadiusAttribute::ServiceType(st) => {
                packet.push(6); // Type
                packet.push(6); // Length
                packet.extend_from_slice(&st.to_be_bytes());
            }
            RadiusAttribute::FramedProtocol(proto) => {
                packet.push(7); // Type
                packet.push(6); // Length
                packet.extend_from_slice(&proto.to_be_bytes());
            }
            RadiusAttribute::FramedIpAddress(ip) => {
                packet.push(8); // Type
                packet.push(6); // Length
                let addr: std::net::Ipv4Addr =
                    ip.parse().unwrap_or(std::net::Ipv4Addr::UNSPECIFIED);
                packet.extend_from_slice(&addr.octets());
            }
            RadiusAttribute::FilterId(filter) => {
                packet.push(11); // Type
                let filter_bytes = filter.as_bytes();
                packet.push((2 + filter_bytes.len()) as u8);
                packet.extend_from_slice(filter_bytes);
            }
            RadiusAttribute::AcctSessionId(session_id) => {
                packet.push(44); // Type
                let sid_bytes = session_id.as_bytes();
                packet.push((2 + sid_bytes.len()) as u8);
                packet.extend_from_slice(sid_bytes);
            }
            RadiusAttribute::AcctStatusType(status) => {
                packet.push(40); // Type
                packet.push(6); // Length
                packet.extend_from_slice(&status.to_be_bytes());
            }
            RadiusAttribute::AcctInputOctets(octets) => {
                packet.push(47); // Type
                packet.push(10); // Length
                packet.extend_from_slice(&octets.to_be_bytes());
            }
            RadiusAttribute::AcctOutputOctets(octets) => {
                packet.push(48); // Type
                packet.push(10); // Length
                packet.extend_from_slice(&octets.to_be_bytes());
            }
            RadiusAttribute::AcctSessionTime(time) => {
                packet.push(41); // Type
                packet.push(6); // Length
                packet.extend_from_slice(&time.to_be_bytes());
            }
            _ => {} // Skip unsupported attributes for now
        }
    }

    // Update length
    let length = packet.len() as u16;
    packet[length_pos] = (length >> 8) as u8;
    packet[length_pos + 1] = length as u8;

    // Generate authenticator
    match request.packet_type {
        RadiusPacketType::AccessRequest => {
            // Random authenticator for Access-Request
            use rand::Rng;
            let mut rng = rand::thread_rng();
            for i in 0..16 {
                packet[authenticator_pos + i] = rng.gen();
            }
        }
        RadiusPacketType::AccountingRequest => {
            // RFC 2866: MD5(Code + Id + Length + Request-Authenticator + Attributes + Secret)
            let mut data = Vec::new();
            data.extend_from_slice(&packet[..authenticator_pos]);
            data.extend_from_slice(&packet[authenticator_pos..authenticator_pos + 16]);
            data.extend_from_slice(&packet[authenticator_pos + 16..]);
            data.extend_from_slice(secret.as_bytes());
            let hash = md5_hash(&data);
            packet[authenticator_pos..authenticator_pos + 16].copy_from_slice(&hash);
        }
        _ => {}
    }

    packet
}

/// Simple MD5 hash (for RADIUS authenticator)
fn md5_hash(data: &[u8]) -> [u8; 16] {
    let digest = md5::compute(data);
    digest.0
}

/// Parse RADIUS response packet with authenticator validation
fn parse_radius_packet(
    data: &[u8],
    request_authenticator: &[u8; 16],
    shared_secret: &str,
) -> Result<RadiusResponse, AppError> {
    if data.len() < RADIUS_HEADER_SIZE {
        return Err(AppError::External("RADIUS packet too short".to_string()));
    }

    let packet_type = RadiusPacketType::from_u8(data[0])
        .ok_or_else(|| AppError::External(format!("Invalid RADIUS packet type: {}", data[0])))?;

    let identifier = data[1];
    let _length = ((data[2] as u16) << 8) | (data[3] as u16);

    // Extract response authenticator (bytes 4..20)
    let response_auth = &data[4..20];

    // Validate response authenticator: MD5(code + id + length + req_auth + attributes + secret)
    let mut auth_input = Vec::with_capacity(data.len() + shared_secret.len());
    auth_input.extend_from_slice(&data[0..4]); // code + id + length
    auth_input.extend_from_slice(request_authenticator);
    auth_input.extend_from_slice(&data[20..]); // attributes
    auth_input.extend_from_slice(shared_secret.as_bytes());
    let expected_auth = md5_hash(&auth_input);
    let auth_valid = expected_auth == response_auth;

    if !auth_valid {
        warn!(
            identifier = identifier,
            "RADIUS response authenticator mismatch — possible spoofing"
        );
    }

    let mut attributes = Vec::new();
    let mut pos = RADIUS_HEADER_SIZE;

    while pos + 2 <= data.len() {
        let attr_type = data[pos];
        let attr_len = data[pos + 1] as usize;

        if attr_len < 2 || pos + attr_len > data.len() {
            break;
        }

        let attr_data = &data[pos + 2..pos + attr_len];

        match attr_type {
            1 => {
                // User-Name
                if let Ok(name) = std::str::from_utf8(attr_data) {
                    attributes.push(RadiusAttribute::UserName(name.to_string()));
                }
            }
            18 => {
                // Reply-Message
                if let Ok(msg) = std::str::from_utf8(attr_data) {
                    attributes.push(RadiusAttribute::Other(attr_type, attr_data.to_vec()));
                    debug!(message = %msg, "RADIUS reply message");
                }
            }
            8 => {
                // Framed-IP-Address
                if attr_data.len() == 4 {
                    let ip = std::net::Ipv4Addr::new(
                        attr_data[0],
                        attr_data[1],
                        attr_data[2],
                        attr_data[3],
                    );
                    attributes.push(RadiusAttribute::FramedIpAddress(ip.to_string()));
                }
            }
            11 => {
                // Filter-Id
                if let Ok(filter) = std::str::from_utf8(attr_data) {
                    attributes.push(RadiusAttribute::FilterId(filter.to_string()));
                }
            }
            _ => {
                attributes.push(RadiusAttribute::Other(attr_type, attr_data.to_vec()));
            }
        }

        pos += attr_len;
    }

    Ok(RadiusResponse {
        packet_type,
        identifier,
        attributes,
        response_auth_valid: auth_valid,
    })
}

// ============================================================================
// Adapter Trait
// ============================================================================

/// Trait for RADIUS client operations
#[async_trait]
pub trait RadiusClient: Send + Sync {
    /// Authenticate a PPPoE user
    async fn authenticate(&self, request: &PppoeAuthRequest)
        -> Result<PppoeAuthResponse, AppError>;

    /// Send accounting start
    async fn accounting_start(&self, request: &AccountingRequest) -> Result<(), AppError>;

    /// Send accounting stop
    async fn accounting_stop(&self, request: &AccountingRequest) -> Result<(), AppError>;

    /// Send interim update
    async fn accounting_interim(&self, request: &AccountingRequest) -> Result<(), AppError>;

    /// Send CoA request to change bandwidth
    async fn change_authorization(&self, request: &CoARequest) -> Result<CoAResponse, AppError>;
}

// ============================================================================
// UDP RADIUS Client
// ============================================================================

/// RADIUS client using UDP
pub struct RadiusAdapter {
    config: RadiusConfig,
}

impl RadiusAdapter {
    /// Create a new RADIUS adapter
    pub fn new(config: RadiusConfig) -> Self {
        Self { config }
    }

    /// Create adapter from environment variables
    pub fn from_env() -> Self {
        Self::new(RadiusConfig::default())
    }

    /// Send a RADIUS packet and receive response with retry logic
    async fn send_and_receive(&self, packet: &[u8], port: u16) -> Result<Vec<u8>, AppError> {
        let addr: SocketAddr = format!("{}:{}", self.config.server, port)
            .parse()
            .map_err(|e| AppError::External(format!("Invalid RADIUS server address: {}", e)))?;

        let max_retries = self.config.max_retries.max(1);
        let mut last_error = None;

        for attempt in 1..=max_retries {
            let socket = UdpSocket::bind("0.0.0.0:0")
                .await
                .map_err(|e| AppError::External(format!("Failed to bind UDP socket: {}", e)))?;

            if let Err(e) = socket.send_to(packet, addr).await {
                last_error = Some(format!("Send failed on attempt {}: {}", attempt, e));
                continue;
            }

            let mut buf = vec![0u8; RADIUS_MAX_PACKET_SIZE];
            match tokio::time::timeout(
                std::time::Duration::from_secs(self.config.timeout_seconds as u64),
                socket.recv_from(&mut buf),
            )
            .await
            {
                Ok(Ok((len, _))) => {
                    buf.truncate(len);
                    return Ok(buf);
                }
                Ok(Err(e)) => {
                    last_error = Some(format!("Receive failed on attempt {}: {}", attempt, e));
                }
                Err(_) => {
                    last_error = Some(format!(
                        "Request timed out on attempt {}/{}",
                        attempt, max_retries
                    ));
                }
            }

            if attempt < max_retries {
                debug!(
                    attempt = attempt,
                    max_retries = max_retries,
                    "RADIUS request failed, retrying..."
                );
            }
        }

        Err(AppError::External(format!(
            "RADIUS request failed after {} retries: {}",
            max_retries,
            last_error.unwrap_or_else(|| "unknown error".to_string())
        )))
    }
}

#[async_trait]
impl RadiusClient for RadiusAdapter {
    async fn authenticate(
        &self,
        request: &PppoeAuthRequest,
    ) -> Result<PppoeAuthResponse, AppError> {
        use rand::Rng;
        let identifier: u8 = rand::thread_rng().gen();

        let radius_request = RadiusRequest {
            packet_type: RadiusPacketType::AccessRequest,
            identifier,
            attributes: vec![
                RadiusAttribute::UserName(request.username.clone()),
                RadiusAttribute::UserPassword(request.password.clone()),
                RadiusAttribute::NasIpAddress(request.nas_ip.clone()),
                RadiusAttribute::NasPort(request.nas_port),
            ],
        };

        let packet = build_radius_packet(&radius_request, &self.config.secret);
        let request_auth: [u8; 16] = packet[4..20].try_into().unwrap_or([0u8; 16]);
        let response_data = self.send_and_receive(&packet, self.config.port).await?;
        let response = parse_radius_packet(&response_data, &request_auth, &self.config.secret)?;

        if !response.response_auth_valid {
            warn!(
                username = %request.username,
                "RADIUS response authenticator validation failed — possible spoofing"
            );
        }

        let accepted = response.packet_type == RadiusPacketType::AccessAccept;

        let framed_ip = response.attributes.iter().find_map(|attr| {
            if let RadiusAttribute::FramedIpAddress(ip) = attr {
                Some(ip.clone())
            } else {
                None
            }
        });

        let filter_id = response.attributes.iter().find_map(|attr| {
            if let RadiusAttribute::FilterId(filter) = attr {
                Some(filter.clone())
            } else {
                None
            }
        });

        info!(
            username = %request.username,
            accepted = accepted,
            framed_ip = ?framed_ip,
            "RADIUS authentication result"
        );

        Ok(PppoeAuthResponse {
            accepted,
            framed_ip,
            framed_route: None,
            filter_id,
            session_id: None,
            error_message: if !accepted {
                Some("Authentication failed".to_string())
            } else {
                None
            },
        })
    }

    async fn accounting_start(&self, request: &AccountingRequest) -> Result<(), AppError> {
        use rand::Rng;
        let identifier: u8 = rand::thread_rng().gen();

        let radius_request = RadiusRequest {
            packet_type: RadiusPacketType::AccountingRequest,
            identifier,
            attributes: vec![
                RadiusAttribute::UserName(request.username.clone()),
                RadiusAttribute::AcctSessionId(request.session_id.clone()),
                RadiusAttribute::AcctStatusType(AccountingStatusType::Start as u32),
                RadiusAttribute::NasIpAddress(request.nas_ip.clone()),
                RadiusAttribute::NasPort(request.nas_port),
            ],
        };

        let packet = build_radius_packet(&radius_request, &self.config.secret);
        let request_auth: [u8; 16] = packet[4..20].try_into().unwrap_or([0u8; 16]);
        let response_data = self
            .send_and_receive(&packet, self.config.accounting_port)
            .await?;
        let response = parse_radius_packet(&response_data, &request_auth, &self.config.secret)?;

        if response.packet_type != RadiusPacketType::AccountingResponse {
            return Err(AppError::External(
                "RADIUS accounting start rejected".to_string(),
            ));
        }

        info!(
            username = %request.username,
            session_id = %request.session_id,
            "RADIUS accounting started"
        );
        Ok(())
    }

    async fn accounting_stop(&self, request: &AccountingRequest) -> Result<(), AppError> {
        use rand::Rng;
        let identifier: u8 = rand::thread_rng().gen();

        let mut attributes = vec![
            RadiusAttribute::UserName(request.username.clone()),
            RadiusAttribute::AcctSessionId(request.session_id.clone()),
            RadiusAttribute::AcctStatusType(AccountingStatusType::Stop as u32),
            RadiusAttribute::NasIpAddress(request.nas_ip.clone()),
            RadiusAttribute::NasPort(request.nas_port),
        ];

        if let Some(input) = request.input_octets {
            attributes.push(RadiusAttribute::AcctInputOctets(input));
        }
        if let Some(output) = request.output_octets {
            attributes.push(RadiusAttribute::AcctOutputOctets(output));
        }
        if let Some(time) = request.session_time {
            attributes.push(RadiusAttribute::AcctSessionTime(time));
        }
        if let Some(cause) = request.terminate_cause {
            attributes.push(RadiusAttribute::AcctTerminateCause(cause));
        }

        let radius_request = RadiusRequest {
            packet_type: RadiusPacketType::AccountingRequest,
            identifier,
            attributes,
        };

        let packet = build_radius_packet(&radius_request, &self.config.secret);
        let request_auth: [u8; 16] = packet[4..20].try_into().unwrap_or([0u8; 16]);
        let response_data = self
            .send_and_receive(&packet, self.config.accounting_port)
            .await?;
        let response = parse_radius_packet(&response_data, &request_auth, &self.config.secret)?;

        if response.packet_type != RadiusPacketType::AccountingResponse {
            return Err(AppError::External(
                "RADIUS accounting stop rejected".to_string(),
            ));
        }

        info!(
            username = %request.username,
            session_id = %request.session_id,
            "RADIUS accounting stopped"
        );
        Ok(())
    }

    async fn accounting_interim(&self, request: &AccountingRequest) -> Result<(), AppError> {
        use rand::Rng;
        let identifier: u8 = rand::thread_rng().gen();

        let mut attributes = vec![
            RadiusAttribute::UserName(request.username.clone()),
            RadiusAttribute::AcctSessionId(request.session_id.clone()),
            RadiusAttribute::AcctStatusType(AccountingStatusType::InterimUpdate as u32),
            RadiusAttribute::NasIpAddress(request.nas_ip.clone()),
            RadiusAttribute::NasPort(request.nas_port),
        ];

        if let Some(input) = request.input_octets {
            attributes.push(RadiusAttribute::AcctInputOctets(input));
        }
        if let Some(output) = request.output_octets {
            attributes.push(RadiusAttribute::AcctOutputOctets(output));
        }
        if let Some(time) = request.session_time {
            attributes.push(RadiusAttribute::AcctSessionTime(time));
        }

        let radius_request = RadiusRequest {
            packet_type: RadiusPacketType::AccountingRequest,
            identifier,
            attributes,
        };

        let packet = build_radius_packet(&radius_request, &self.config.secret);
        let request_auth: [u8; 16] = packet[4..20].try_into().unwrap_or([0u8; 16]);
        let response_data = self
            .send_and_receive(&packet, self.config.accounting_port)
            .await?;
        let response = parse_radius_packet(&response_data, &request_auth, &self.config.secret)?;

        if response.packet_type != RadiusPacketType::AccountingResponse {
            return Err(AppError::External(
                "RADIUS accounting interim rejected".to_string(),
            ));
        }

        debug!(
            username = %request.username,
            session_id = %request.session_id,
            "RADIUS accounting interim update sent"
        );
        Ok(())
    }

    async fn change_authorization(&self, request: &CoARequest) -> Result<CoAResponse, AppError> {
        use rand::Rng;
        let identifier: u8 = rand::thread_rng().gen();

        let mut attributes = vec![
            RadiusAttribute::UserName(request.username.clone()),
            RadiusAttribute::NasIpAddress(request.nas_ip.clone()),
            RadiusAttribute::NasPort(request.nas_port),
        ];

        if let Some(ref filter) = request.filter_id {
            attributes.push(RadiusAttribute::FilterId(filter.clone()));
        }
        if let Some(ref ip) = request.framed_ip {
            attributes.push(RadiusAttribute::FramedIpAddress(ip.clone()));
        }

        let radius_request = RadiusRequest {
            packet_type: RadiusPacketType::CoARequest,
            identifier,
            attributes,
        };

        let packet = build_radius_packet(&radius_request, &self.config.secret);
        let request_auth: [u8; 16] = packet[4..20].try_into().unwrap_or([0u8; 16]);
        let response_data = self.send_and_receive(&packet, self.config.coa_port).await?;
        let response = parse_radius_packet(&response_data, &request_auth, &self.config.secret)?;

        let accepted = response.packet_type == RadiusPacketType::CoAACK;

        info!(
            username = %request.username,
            accepted = accepted,
            "RADIUS CoA result"
        );

        Ok(CoAResponse {
            accepted,
            error_message: if !accepted {
                Some("CoA rejected".to_string())
            } else {
                None
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_type_from_u8() {
        assert_eq!(
            RadiusPacketType::from_u8(1),
            Some(RadiusPacketType::AccessRequest)
        );
        assert_eq!(
            RadiusPacketType::from_u8(2),
            Some(RadiusPacketType::AccessAccept)
        );
        assert_eq!(
            RadiusPacketType::from_u8(3),
            Some(RadiusPacketType::AccessReject)
        );
        assert_eq!(
            RadiusPacketType::from_u8(4),
            Some(RadiusPacketType::AccountingRequest)
        );
        assert_eq!(
            RadiusPacketType::from_u8(5),
            Some(RadiusPacketType::AccountingResponse)
        );
        assert_eq!(
            RadiusPacketType::from_u8(37),
            Some(RadiusPacketType::CoARequest)
        );
        assert_eq!(
            RadiusPacketType::from_u8(38),
            Some(RadiusPacketType::CoAACK)
        );
        assert_eq!(
            RadiusPacketType::from_u8(39),
            Some(RadiusPacketType::CoANAK)
        );
        assert_eq!(RadiusPacketType::from_u8(200), None);
    }

    #[test]
    fn test_build_access_request_packet() {
        let request = RadiusRequest {
            packet_type: RadiusPacketType::AccessRequest,
            identifier: 0x42,
            attributes: vec![
                RadiusAttribute::UserName("pppoe-user".to_string()),
                RadiusAttribute::UserPassword("secret-pass".to_string()),
            ],
        };

        let packet = build_radius_packet(&request, "shared-secret");

        assert_eq!(packet[0], 1, "Access-Request code");
        assert_eq!(packet[1], 0x42, "identifier");
        let length = ((packet[2] as u16) << 8) | packet[3] as u16;
        assert_eq!(length as usize, packet.len(), "packet length");

        // Random request authenticator must be present (non-zero)
        assert_ne!(&packet[4..20], &[0u8; 16]);

        // User-Name attribute (type 1)
        assert_eq!(packet[20], 1, "User-Name attribute type");
        assert_eq!(packet[21] as usize, 2 + "pppoe-user".len());
        assert_eq!(&packet[22..22 + "pppoe-user".len()], b"pppoe-user");

        // User-Password attribute (type 2), padded to 16 bytes for RFC 2865
        let pw_start = 20 + 2 + "pppoe-user".len();
        assert_eq!(packet[pw_start], 2, "User-Password attribute type");
        assert_eq!(packet[pw_start + 1], 18, "2 + 16 padded bytes");
        assert_ne!(
            &packet[pw_start + 2..pw_start + 18],
            b"secret-pass" as &[u8]
        );
    }

    #[test]
    fn test_build_accounting_request_authenticator() {
        let request = RadiusRequest {
            packet_type: RadiusPacketType::AccountingRequest,
            identifier: 0x07,
            attributes: vec![
                RadiusAttribute::UserName("user1".to_string()),
                RadiusAttribute::AcctSessionId("session-1".to_string()),
                RadiusAttribute::AcctStatusType(AccountingStatusType::Start as u32),
            ],
        };
        let secret = "s3cret-key";

        let packet = build_radius_packet(&request, secret);

        // RFC 2866: Request-Authenticator = MD5(Code + Id + Length + Auth + Attributes + Secret)
        let mut with_zero_auth = packet.clone();
        with_zero_auth[4..20].fill(0);
        let mut expected_input = Vec::new();
        expected_input.extend_from_slice(&with_zero_auth[..4]);
        expected_input.extend_from_slice(&with_zero_auth[4..20]);
        expected_input.extend_from_slice(&with_zero_auth[20..]);
        expected_input.extend_from_slice(secret.as_bytes());
        let expected = md5::compute(&expected_input);
        assert_eq!(&packet[4..20], &expected.0[..]);
    }

    #[test]
    fn test_parse_response_valid_authenticator() {
        let request_auth = [0x11u8; 16];
        let secret = "s3cret";

        let mut packet: Vec<u8> = vec![
            2,    // Access-Accept
            0x12, // identifier
            0, 0, // length placeholder
        ];
        packet.extend_from_slice(&[0u8; 16]); // authenticator placeholder

        // Framed-IP-Address attribute
        packet.extend_from_slice(&[8, 6, 192, 168, 1, 50]);
        // Filter-Id attribute
        packet.extend_from_slice(&[11, 10]); // 2 + 8
        packet.extend_from_slice(b"speed-10");
        // Reply-Message attribute
        packet.extend_from_slice(&[18, 10]); // 2 + 8
        packet.extend_from_slice(b"Welcome!");

        let length = packet.len() as u16;
        packet[2] = (length >> 8) as u8;
        packet[3] = length as u8;

        let mut auth_input = Vec::new();
        auth_input.extend_from_slice(&packet[0..4]);
        auth_input.extend_from_slice(&request_auth);
        auth_input.extend_from_slice(&packet[20..]);
        auth_input.extend_from_slice(secret.as_bytes());
        let hash = md5::compute(&auth_input);
        packet[4..20].copy_from_slice(&hash.0);

        let response = parse_radius_packet(&packet, &request_auth, secret).unwrap();
        assert_eq!(response.packet_type, RadiusPacketType::AccessAccept);
        assert_eq!(response.identifier, 0x12);
        assert!(response.response_auth_valid);
        assert!(response
            .attributes
            .iter()
            .any(|a| matches!(a, RadiusAttribute::FramedIpAddress(ip) if ip == "192.168.1.50")));
        assert!(response
            .attributes
            .iter()
            .any(|a| matches!(a, RadiusAttribute::FilterId(f) if f == "speed-10")));
    }

    #[test]
    fn test_parse_response_invalid_authenticator_flag() {
        let request_auth = [0x22u8; 16];
        let secret = "s3cret";

        let mut packet: Vec<u8> = vec![2, 0x01, 0, 0];
        packet.extend_from_slice(&[0u8; 16]);
        packet.extend_from_slice(&[8, 6, 10, 0, 0, 1]);

        let length = packet.len() as u16;
        packet[2] = (length >> 8) as u8;
        packet[3] = length as u8;

        let mut auth_input = Vec::new();
        auth_input.extend_from_slice(&packet[0..4]);
        auth_input.extend_from_slice(&request_auth);
        auth_input.extend_from_slice(&packet[20..]);
        auth_input.extend_from_slice(secret.as_bytes());
        let hash = md5::compute(&auth_input);
        packet[4..20].copy_from_slice(&hash.0);

        // Tamper with a single attribute byte so the authenticator no longer matches
        packet[25] ^= 0xff;

        let response = parse_radius_packet(&packet, &request_auth, secret).unwrap();
        assert_eq!(response.packet_type, RadiusPacketType::AccessAccept);
        assert!(!response.response_auth_valid);
    }

    #[test]
    fn test_parse_response_too_short() {
        let result = parse_radius_packet(&[0u8; 10], &[0u8; 16], "secret");
        assert!(matches!(result, Err(AppError::External(_))));
    }

    #[test]
    fn test_parse_response_invalid_type() {
        let mut packet = vec![0u8; 20];
        packet[0] = 99;
        let result = parse_radius_packet(&packet, &[0u8; 16], "secret");
        assert!(matches!(result, Err(AppError::External(_))));
    }
}
