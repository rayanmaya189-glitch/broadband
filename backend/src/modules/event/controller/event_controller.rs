use axum::extract::{Query, State};
use crate::app::SharedState;
use crate::common::errors::app_error::AppError;
use crate::modules::event::request::event_request::*;
use crate::modules::event::response::event_response::*;
use crate::modules::event::service::event_service::EventService;

pub async fn list_events(State(state): State<SharedState>, Query(q): Query<EventQuery>) -> Result<axum::Json<EventListResponse>, AppError> { let svc = EventService::new(&state.db); Ok(axum::Json(svc.list(q).await?)) }
