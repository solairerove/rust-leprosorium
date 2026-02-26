use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use thiserror::Error;

use crate::views::render_note_not_found;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error")]
    Db(#[from] sqlx::Error),
    #[error("bad request")]
    BadRequest(String),
    #[error("not found")]
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Db(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("<p class=\"muted\">Internal server error.</p>".to_string()),
            )
                .into_response(),
            AppError::BadRequest(html) => (StatusCode::BAD_REQUEST, Html(html)).into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, Html(render_note_not_found())).into_response(),
        }
    }
}
