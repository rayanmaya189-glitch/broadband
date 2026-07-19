//! OpenAPI documentation setup using utoipa.
//! Delegates to the docs module for the canonical ApiDoc definition.
//! Provides OpenAPI JSON spec at /api-docs/openapi.json.

/// Create the OpenAPI JSON spec router.
/// Swagger UI is served from routes::health_routes() at /swagger-ui.
pub fn swagger_routes() -> axum::Router<crate::shared::app_state::SharedState> {
    use axum::routing::get;

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
