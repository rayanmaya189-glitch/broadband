use axum::http::{header, Method};
use tower_http::cors::{Any, CorsLayer};

use crate::common::config::config::Config;

/// Build a CORS layer from the application config.
pub fn build_cors() -> CorsLayer {
    let config = Config::get();

    if config.cors_origins.is_empty() {
        return CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
            .allow_headers([
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT,
            ]);
    }

    let origins: Vec<header::HeaderValue> = config
        .cors_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ])
}
