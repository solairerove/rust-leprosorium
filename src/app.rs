use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{create_note, delete_note, index, show_note},
    state::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/notes", post(create_note))
        .route("/notes/:id", get(show_note).delete(delete_note))
        .with_state(state)
}
