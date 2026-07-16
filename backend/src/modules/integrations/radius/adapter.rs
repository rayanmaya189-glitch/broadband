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
use tracing::{debug, info};

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
            server: std::env::var("RADIUS_SERVER")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
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
    for _ in 0..16 {
        packet.push(0);
    }

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

                // XOR password with MD5(authenticator + secret)
                let mut data = Vec::new();
                data.extend_from_slice(&packet[authenticator_pos..authenticator_pos + 16]);
                data.extend_from_slice(secret.as_bytes());
                let hash = md5_hash(&data);

                let password_bytes = password.as_bytes();
                for i in 0..padded_len {
                    let p = if i < password_bytes.len() {
                        password_bytes[i]
                    } else {
                        0
                    };
                    let h = if i < 16 { hash[i] } else { hash[i % 16] };
                    packet.push(p ^ h);
                }
            }
            RadiusAttribute::NasIpAddress(ip) => {
                packet.push(4); // Type
                packet.push(6); // Length
                let addr: std::net::Ipv4Addr = ip.parse().unwrap_or(std::net::Ipv4Addr::UNSPECIFIED);
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
                let addr: std::net::Ipv4Addr = ip.parse().unwrap_or(std::net::Ipv4Addr::UNSPECIFIED);
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
            // MD5(Code + Identifier + Length + Authenticator + Attributes + Secret)
            let mut data = Vec::new();
            data.extend_from_slice(&packet[..authenticator_pos]);
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

/// Parse RADIUS response packet
fn parse_radius_packet(data: &[u8]) -> Result<RadiusResponse, AppError> {
    if data.len() < RADIUS_HEADER_SIZE {
        return Err(AppError::External("RADIUS packet too short".to_string()));
    }

    let packet_type = RadiusPacketType::from_u8(data[0])
        .ok_or_else(|| AppError::External(format!("Invalid RADIUS packet type: {}", data[0])))?;

    let identifier = data[1];
    let _length = ((data[2] as u16) << 8) | (data[3] as u16);

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
                        attr_data[0], attr_data[1], attr_data[2], attr_data[3],
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
    })
}

// ============================================================================
// Adapter Trait
// ============================================================================

/// Trait for RADIUS client operations
#[async_trait]
pub trait RadiusClient: Send + Sync {
    /// Authenticate a PPPoE user
    async fn authenticate(
        &self,
        request: &PppoeAuthRequest,
    ) -> Result<PppoeAuthResponse, AppError>;

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

    /// Send a RADIUS packet and receive response
    async fn send_and_receive(
        &self,
        packet: &[u8],
        port: u16,
    ) -> Result<Vec<u8>, AppError> {
        let addr: SocketAddr = format!("{}:{}", self.config.server, port)
            .parse()
            .map_err(|e| AppError::External(format!("Invalid RADIUS server address: {}", e)))?;

        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .map_err(|e| AppError::External(format!("Failed to bind UDP socket: {}", e)))?;

        socket
            .send_to(packet, addr)
            .await
            .map_err(|e| AppError::External(format!("Failed to send RADIUS packet: {}", e)))?;

        let mut buf = vec![0u8; RADIUS_MAX_PACKET_SIZE];
        let (len, _) = tokio::time::timeout(
            std::time::Duration::from_secs(self.config.timeout_seconds as u64),
            socket.recv_from(&mut buf),
        )
        .await
        .map_err(|_| AppError::External("RADIUS request timed out".to_string()))?
        .map_err(|e| AppError::External(format!("Failed to receive RADIUS response: {}", e)))?;

        buf.truncate(len);
        Ok(buf)
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
                RadiusAttribute::ServiceType(2), // Framed
                RadiusAttribute::FramedProtocol(1), // PPP
            ],
        };

        let packet = build_radius_packet(&radius_request, &self.config.secret);
        let response_data = self.send_and_receive(&packet, self.config.port).await?;
        let response = parse_radius_packet(&response_data)?;

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
        let response_data = self.send_and_receive(&packet, self.config.accounting_port).await?;
        let response = parse_radius_packet(&response_data)?;

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
        let response_data = self.send_and_receive(&packet, self.config.accounting_port).await?;
        let response = parse_radius_packet(&response_data)?;

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
        let response_data = self.send_and_receive(&packet, self.config.accounting_port).await?;
        let response = parse_radius_packet(&response_data)?;

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
        let response_data = self.send_and_receive(&packet, self.config.coa_port).await?;
        let response = parse_radius_packet(&response_data)?;

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
