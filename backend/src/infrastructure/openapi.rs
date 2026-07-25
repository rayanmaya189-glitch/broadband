//! OpenAPI documentation setup using utoipa.
//! Delegates to the docs module for the canonical ApiDoc definition.
//! Provides OpenAPI JSON spec at /api-docs/openapi.json.

/// Create the OpenAPI JSON spec router (only in non-production).
pub fn swagger_routes() -> axum::Router<crate::shared::app_state::SharedState> {
    use axum::routing::get;

    let is_prod = std::env::var("APP_ENV")
        .unwrap_or_default()
        .to_lowercase()
        == "production";

    if is_prod {
        return axum::Router::new();
    }

    axum::Router::new().route(
        "/api-docs/openapi.json",
        get(|| async {
            use utoipa::OpenApi;
            let spec = crate::docs::ApiDoc::openapi()
                .to_pretty_json()
                .unwrap_or_default();
            axum::Json(serde_json::from_str::<serde_json::Value>(&spec).unwrap_or_default())
        }),
    )
}
