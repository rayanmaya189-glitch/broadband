//! Email SMTP delivery service with OTP support.
//!
//! Uses the `lettre` crate for SMTP transport. Config stored in
//! `notification_channels.config` as:
//! ```json
//! {
//!   "smtp_host": "smtp.gmail.com",
//!   "smtp_port": 587,
//!   "smtp_username": "noreply@aeroxe.com",
//!   "smtp_password": "app-password-here",
//!   "from_name": "AeroXe Broadband",
//!   "from_email": "noreply@aeroxe.com",
//!   "use_tls": true
//! }
//! ```

use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::{info, warn};

use crate::common::errors::app_error::AppError;

/// SMTP-specific configuration extracted from the channel config JSON.
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_name: String,
    pub from_email: String,
    pub use_tls: bool,
}

impl SmtpConfig {
    /// Parse from the channel config JSON stored in the database.
    pub fn from_json(config: &serde_json::Value) -> Result<Self, AppError> {
        let host = config
            .get("smtp_host")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Validation("SMTP config missing 'smtp_host'".into()))?
            .to_string();

        let port = config
            .get("smtp_port")
            .and_then(|v| v.as_u64())
            .unwrap_or(587) as u16;

        let username = config
            .get("smtp_username")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Validation("SMTP config missing 'smtp_username'".into()))?
            .to_string();

        let password = config
            .get("smtp_password")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Validation("SMTP config missing 'smtp_password'".into()))?
            .to_string();

        let from_name = config
            .get("from_name")
            .and_then(|v| v.as_str())
            .unwrap_or("AeroXe Broadband")
            .to_string();

        let from_email = config
            .get("from_email")
            .and_then(|v| v.as_str())
            .unwrap_or(&username)
            .to_string();

        let use_tls = config
            .get("use_tls")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        if host.is_empty() {
            return Err(AppError::Validation("SMTP host is empty".into()));
        }
        if username.is_empty() {
            return Err(AppError::Validation("SMTP username is empty".into()));
        }

        Ok(Self {
            host,
            port,
            username,
            password,
            from_name,
            from_email,
            use_tls,
        })
    }

    /// Build the SMTP transport from this config.
    fn build_transport(&self) -> Result<SmtpTransport, AppError> {
        let credentials = Credentials::new(self.username.clone(), self.password.clone());

        let transport = if self.use_tls {
            SmtpTransport::relay(&self.host)
                .map_err(|e| AppError::External(format!("SMTP relay error: {e}")))?
                .port(self.port)
                .credentials(credentials)
                .build()
        } else {
            SmtpTransport::builder_dangerous(&self.host)
                .port(self.port)
                .credentials(credentials)
                .build()
        };

        Ok(transport)
    }
}

/// Result of sending an email.
#[derive(Debug)]
pub struct SendResult {
    pub success: bool,
    pub error: Option<String>,
}

/// Helper to parse an email address string into a `lettre::message::Mailbox`.
///
/// Accepts both `"user@example.com"` and `"Name <user@example.com>"` formats.
fn parse_mailbox(addr: &str) -> Result<Mailbox, AppError> {
    addr.parse().map_err(|e| {
        AppError::Validation(format!("Invalid email address '{addr}': {e}"))
    })
}

/// Send a plain text email.
pub async fn send_email(
    config: &SmtpConfig,
    to_email: &str,
    to_name: Option<&str>,
    subject: &str,
    body: &str,
) -> Result<SendResult, AppError> {
    let to_addr = match to_name {
        Some(name) => format!("{} <{}>", name, to_email),
        None => to_email.to_string(),
    };

    let from_addr = format!("{} <{}>", config.from_name, config.from_email);

    let email = Message::builder()
        .from(parse_mailbox(&from_addr)?)
        .to(parse_mailbox(&to_addr)?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .map_err(|e| AppError::Validation(format!("Failed to build email: {e}")))?;

    let transport = config.build_transport()?;

    match transport.send(&email) {
        Ok(_) => {
            info!(to = to_email, "Email sent successfully");
            Ok(SendResult {
                success: true,
                error: None,
            })
        }
        Err(e) => {
            warn!(to = to_email, error = %e, "Email send failed");
            Ok(SendResult {
                success: false,
                error: Some(format!("SMTP error: {e}")),
            })
        }
    }
}

/// Send an HTML email with optional plain text fallback.
pub async fn send_html_email(
    config: &SmtpConfig,
    to_email: &str,
    to_name: Option<&str>,
    subject: &str,
    html_body: &str,
    text_body: Option<&str>,
) -> Result<SendResult, AppError> {
    let to_addr = match to_name {
        Some(name) => format!("{} <{}>", name, to_email),
        None => to_email.to_string(),
    };

    let from_addr = format!("{} <{}>", config.from_name, config.from_email);

    let email = match text_body {
        Some(text) => {
            let text_part = SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(text.to_string());
            let html_part = SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html_body.to_string());
            Message::builder()
                .from(parse_mailbox(&from_addr)?)
                .to(parse_mailbox(&to_addr)?)
                .subject(subject)
                .multipart(
                    MultiPart::alternative()
                        .singlepart(text_part)
                        .singlepart(html_part),
                )
                .map_err(|e| {
                    AppError::Validation(format!("Failed to build multipart email: {e}"))
                })?
        }
        None => {
            let html_part = SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(html_body.to_string());
            Message::builder()
                .from(parse_mailbox(&from_addr)?)
                .to(parse_mailbox(&to_addr)?)
                .subject(subject)
                .singlepart(html_part)
                .map_err(|e| AppError::Validation(format!("Failed to build HTML email: {e}")))?
        }
    };

    let transport = config.build_transport()?;

    match transport.send(&email) {
        Ok(_) => {
            info!(to = to_email, "HTML email sent successfully");
            Ok(SendResult {
                success: true,
                error: None,
            })
        }
        Err(e) => {
            warn!(to = to_email, error = %e, "HTML email send failed");
            Ok(SendResult {
                success: false,
                error: Some(format!("SMTP error: {e}")),
            })
        }
    }
}

/// Send a formatted OTP email with both plain text and HTML versions.
///
/// The OTP is prominently displayed in a styled HTML card for better UX.
pub async fn send_otp(
    config: &SmtpConfig,
    to_email: &str,
    to_name: Option<&str>,
    otp_code: &str,
    expiry_minutes: u32,
    app_name: Option<&str>,
) -> Result<SendResult, AppError> {
    let app = app_name.unwrap_or("AeroXe");
    let subject = format!("{} — Your Verification Code", app);

    let text_body = format!(
        "Your {} verification code is:\n\n{}\n\nThis code expires in {} minutes. Do not share it with anyone.",
        app, otp_code, expiry_minutes
    );

    let html_body = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"></head>
<body style="margin:0;padding:0;background:#f4f4f7;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;">
  <div style="max-width:480px;margin:40px auto;background:#ffffff;border-radius:8px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,0.08);">
    <div style="background:#1a73e8;padding:24px;text-align:center;">
      <h1 style="color:#ffffff;margin:0;font-size:20px;">{app}</h1>
    </div>
    <div style="padding:32px 24px;text-align:center;">
      <h2 style="color:#333;margin:0 0 16px;font-size:18px;">Verification Code</h2>
      <p style="color:#666;margin:0 0 24px;font-size:14px;">Use the code below to verify your identity.</p>
      <div style="background:#f8f9fa;border:2px dashed #1a73e8;border-radius:8px;padding:20px;margin:0 0 24px;">
        <span style="font-size:32px;font-weight:bold;color:#1a73e8;letter-spacing:8px;">{otp_code}</span>
      </div>
      <p style="color:#999;margin:0;font-size:12px;">This code expires in {expiry_minutes} minutes.<br>Do not share it with anyone.</p>
    </div>
    <div style="background:#f4f4f7;padding:16px;text-align:center;">
      <p style="color:#999;margin:0;font-size:11px;">This is an automated message from {app} Broadband.</p>
    </div>
  </div>
</body>
</html>"#,
        app = app,
        otp_code = otp_code,
        expiry_minutes = expiry_minutes,
    );

    send_html_email(
        config,
        to_email,
        to_name,
        &subject,
        &html_body,
        Some(&text_body),
    )
    .await
}

/// Send a notification email with HTML formatting.
///
/// Used for invoice delivery, account updates, and other transactional emails.
pub async fn send_notification(
    config: &SmtpConfig,
    to_email: &str,
    to_name: Option<&str>,
    subject: &str,
    body: &str,
) -> Result<SendResult, AppError> {
    let html_body = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"></head>
<body style="margin:0;padding:0;background:#f4f4f7;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif;">
  <div style="max-width:600px;margin:40px auto;background:#ffffff;border-radius:8px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,0.08);">
    <div style="background:#1a73e8;padding:24px;">
      <h1 style="color:#ffffff;margin:0;font-size:18px;">{}</h1>
    </div>
    <div style="padding:32px 24px;">
      <div style="color:#333;font-size:14px;line-height:1.6;">{}</div>
    </div>
    <div style="background:#f4f4f7;padding:16px;text-align:center;">
      <p style="color:#999;margin:0;font-size:11px;">AeroXe Broadband — Your Trusted Internet Partner</p>
    </div>
  </div>
</body>
</html>"#,
        escape_html(subject),
        body,
    );

    send_html_email(
        config,
        to_email,
        to_name,
        subject,
        &html_body,
        Some(body),
    )
    .await
}

/// Escape HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_json() {
        let json = serde_json::json!({
            "smtp_host": "smtp.gmail.com",
            "smtp_port": 587,
            "smtp_username": "test@gmail.com",
            "smtp_password": "app-password",
            "from_name": "AeroXe",
            "from_email": "noreply@aeroxe.com",
            "use_tls": true
        });
        let config = SmtpConfig::from_json(&json).unwrap();
        assert_eq!(config.host, "smtp.gmail.com");
        assert_eq!(config.port, 587);
        assert_eq!(config.username, "test@gmail.com");
        assert_eq!(config.from_name, "AeroXe");
        assert!(config.use_tls);
    }

    #[test]
    fn test_config_defaults() {
        let json = serde_json::json!({
            "smtp_host": "smtp.example.com",
            "smtp_username": "user@example.com",
            "smtp_password": "pass"
        });
        let config = SmtpConfig::from_json(&json).unwrap();
        assert_eq!(config.port, 587);
        assert_eq!(config.from_name, "AeroXe Broadband");
        assert!(config.use_tls);
    }

    #[test]
    fn test_parse_mailbox_simple() {
        let mb = parse_mailbox("user@example.com").unwrap();
        assert_eq!(mb.email.to_string(), "user@example.com");
    }

    #[test]
    fn test_parse_mailbox_with_name() {
        let mb = parse_mailbox("John Doe <john@example.com>").unwrap();
        assert_eq!(mb.email.to_string(), "john@example.com");
        assert_eq!(mb.name.unwrap(), "John Doe");
    }

    #[test]
    fn test_parse_mailbox_invalid() {
        assert!(parse_mailbox("not-an-email").is_err());
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("a < b & c"), "a &lt; b &amp; c");
    }
}
