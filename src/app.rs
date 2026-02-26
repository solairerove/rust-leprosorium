use axum::{
    Router,
    routing::{get, post},
};
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::{Span, error, warn};

use crate::{
    handlers::{create_note, delete_note, edit_note, index, show_note, update_note},
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/notes", post(create_note))
        .route("/notes/{id}/edit", get(edit_note))
        .route(
            "/notes/{id}",
            get(show_note).put(update_note).delete(delete_note),
        )
        .layer(TraceLayer::new_for_http().on_response(
            |response: &axum::response::Response, latency: Duration, _span: &Span| {
                let status = response.status();
                if status.is_server_error() {
                    error!(%status, latency_ms = latency.as_millis(), "request failed");
                } else if status.is_client_error() {
                    warn!(%status, latency_ms = latency.as_millis(), "client request error");
                }
            },
        ))
        .with_state(state)
}
