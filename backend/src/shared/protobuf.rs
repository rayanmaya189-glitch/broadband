use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{FromRequest, Request},
    response::Response,
};
use bytes::{Bytes, BytesMut};
use prost::Message;

use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;

// Protobuf content type constant
pub const PROTOBUF_CONTENT_TYPE: &str = "application/protobuf";

// Protobuf extractor for request bodies
pub struct Proto<T: Message + Default> {
    pub inner: T,
}

impl<T: Message + Default> Proto<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

#[async_trait]
impl<T: Message + Default + Send + 'static> FromRequest<AppState> for Proto<T> {
    type Rejection = AppError;

    async fn from_request(req: Request, _state: &AppState) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();

        // Check content type
        let content_type = parts
            .headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.contains(PROTOBUF_CONTENT_TYPE) {
            return Err(AppError::Validation(format!(
                "Expected content-type: {}, got: {}",
                PROTOBUF_CONTENT_TYPE, content_type
            )));
        }

        // Read body bytes
        let body_bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to read request body: {}", e)))?;

        // Decode protobuf
        let message = T::decode(&body_bytes[..])
            .map_err(|e| AppError::Validation(format!("Invalid protobuf: {}", e)))?;

        Ok(Self { inner: message })
    }
}

// Helper function to create protobuf response
pub fn proto_response<T: Message>(data: T) -> Response<Body> {
    let mut buf = BytesMut::with_capacity(data.encoded_len());
    data.encode(&mut buf).unwrap();

    Response::builder()
        .header("content-type", PROTOBUF_CONTENT_TYPE)
        .body(Body::from(buf.freeze()))
        .unwrap()
}

// Helper function to create protobuf response from bytes
pub fn proto_response_from_bytes(data: Bytes) -> Response<Body> {
    Response::builder()
        .header("content-type", PROTOBUF_CONTENT_TYPE)
        .body(Body::from(data))
        .unwrap()
}

// Include generated protobuf code
pub mod proto {
    #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

    // Common types (Response, ResponseStatus, PaginationRequest, etc.)
    include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.rs"));

    // Module-specific types
    pub mod accounting { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.accounting.rs")); }
    pub mod admin { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.admin.rs")); }
    pub mod audit { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.audit.rs")); }
    pub mod bandwidth { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.bandwidth.rs")); }
    pub mod billing { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.billing.rs")); }
    pub mod branches { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.branches.rs")); }
    pub mod compliance { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.compliance.rs")); }
    pub mod coverage { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.coverage.rs")); }
    pub mod customer { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.customer.rs")); }
    pub mod device { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.device.rs")); }
    pub mod discovery { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.discovery.rs")); }
    pub mod document { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.document.rs")); }
    pub mod gateway { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.gateway.rs")); }
    pub mod identity { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.identity.rs")); }
    pub mod installation { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.installation.rs")); }
    pub mod inventory { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.inventory.rs")); }
    pub mod lead { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.lead.rs")); }
    pub mod monitoring { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.monitoring.rs")); }
    pub mod network { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.network.rs")); }
    pub mod notification { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.notification.rs")); }
    pub mod payment { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.payment.rs")); }
    pub mod plans { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.plans.rs")); }
    pub mod referral { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.referral.rs")); }
    pub mod scheduler { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.scheduler.rs")); }
    pub mod security { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.security.rs")); }
    pub mod subscription { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.subscription.rs")); }
    pub mod ticket { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.ticket.rs")); }
    pub mod workflow { include!(concat!(env!("OUT_DIR"), "/aeroxe.v1.workflow.rs")); }
}
