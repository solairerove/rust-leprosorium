use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    error::AppError,
    models::NoteForm,
    state::AppState,
    views::{
        render_index_page, render_note_edit_form, render_note_view, render_notes_list,
        render_notes_list_oob,
    },
};

pub async fn index(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let notes = state.store.list_desc().await?;
    Ok(Html(render_index_page(&notes)).into_response())
}

pub async fn create_note(
    State(state): State<AppState>,
    Form(form): Form<NoteForm>,
) -> Result<impl IntoResponse, AppError> {
    let title = form.title.trim();
    let body = form.body.trim();

    if title.is_empty() || body.is_empty() {
        return Err(AppError::BadRequest(
            "<div id=\"notes-list\">Title and body are required.</div>".to_string(),
        ));
    }

    state
        .store
        .create_note(title.to_owned(), body.to_owned())
        .await?;

    let notes = state.store.list_desc().await?;
    Ok(Html(render_notes_list(&notes)).into_response())
}

pub async fn show_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match state.store.get_note(&id).await? {
        Some(note) => Ok((StatusCode::OK, Html(render_note_view(&note))).into_response()),
        None => Err(AppError::NotFound),
    }
}

pub async fn edit_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    match state.store.get_note(&id).await? {
        Some(note) => Ok((StatusCode::OK, Html(render_note_edit_form(&note))).into_response()),
        None => Err(AppError::NotFound),
    }
}

pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Form(form): Form<NoteForm>,
) -> Result<impl IntoResponse, AppError> {
    let title = form.title.trim();
    let body = form.body.trim();

    if title.is_empty() || body.is_empty() {
        return Err(AppError::BadRequest(
            "<section class=\"card\" id=\"note-view\"><p class=\"muted\">Title and body are required.</p></section>".to_string(),
        ));
    }

    match state
        .store
        .update_note(&id, title.to_owned(), body.to_owned())
        .await
    {
        Ok(Some(note)) => {
            let notes = state.store.list_desc().await?;
            let html = format!(
                "{}{}",
                render_notes_list_oob(&notes),
                render_note_view(&note)
            );
            Ok((StatusCode::OK, Html(html)).into_response())
        }
        Ok(None) => Err(AppError::NotFound),
        Err(err) => Err(err.into()),
    }
}

pub async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    state.store.delete_note(&id).await?;

    let notes = state.store.list_desc().await?;
    Ok(Html(render_notes_list(&notes)).into_response())
}
