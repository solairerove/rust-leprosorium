use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    models::NoteForm,
    state::AppState,
    views::{
        render_index_page, render_note_edit_form, render_note_not_found, render_note_view,
        render_notes_list, render_notes_list_oob,
    },
};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let notes = state.store.list_desc().await;
    Html(render_index_page(&notes))
}

pub async fn create_note(
    State(state): State<AppState>,
    Form(form): Form<NoteForm>,
) -> impl IntoResponse {
    let title = form.title.trim();
    let body = form.body.trim();

    if title.is_empty() || body.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Html("<div id=\"notes-list\">Title and body are required.</div>".to_string()),
        )
            .into_response();
    }

    state
        .store
        .create_note(title.to_owned(), body.to_owned())
        .await;
    let notes = state.store.list_desc().await;
    Html(render_notes_list(&notes)).into_response()
}

pub async fn show_note(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.store.get_note(&id).await {
        Some(note) => (StatusCode::OK, Html(render_note_view(&note))).into_response(),
        None => (StatusCode::NOT_FOUND, Html(render_note_not_found())).into_response(),
    }
}

pub async fn edit_note(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.store.get_note(&id).await {
        Some(note) => (StatusCode::OK, Html(render_note_edit_form(&note))).into_response(),
        None => (StatusCode::NOT_FOUND, Html(render_note_not_found())).into_response(),
    }
}

pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Form(form): Form<NoteForm>,
) -> impl IntoResponse {
    let title = form.title.trim();
    let body = form.body.trim();

    if title.is_empty() || body.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Html(
                "<section class=\"card\" id=\"note-view\" style=\"grid-column: 1 / -1;\"><p class=\"muted\">Title and body are required.</p></section>".to_string(),
            ),
        )
            .into_response();
    }

    match state
        .store
        .update_note(&id, title.to_owned(), body.to_owned())
        .await
    {
        Some(note) => {
            let notes = state.store.list_desc().await;
            let html = format!(
                "{}{}",
                render_notes_list_oob(&notes),
                render_note_view(&note)
            );
            (StatusCode::OK, Html(html)).into_response()
        }
        None => (StatusCode::NOT_FOUND, Html(render_note_not_found())).into_response(),
    }
}

pub async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    state.store.delete_note(&id).await;
    let notes = state.store.list_desc().await;
    Html(render_notes_list(&notes)).into_response()
}
