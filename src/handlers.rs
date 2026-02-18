use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{
    models::NoteForm,
    state::AppState,
    views::{render_index_page, render_note_not_found, render_note_view, render_notes_list},
};

pub async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let store = state.store.lock().expect("store mutex poisoned");
    let notes = store.list_desc();
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

    let mut store = state.store.lock().expect("store mutex poisoned");
    store.create_note(title.to_owned(), body.to_owned());
    let notes = store.list_desc();
    Html(render_notes_list(&notes)).into_response()
}

pub async fn show_note(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let store = state.store.lock().expect("store mutex poisoned");
    match store.get_note(&id) {
        Some(note) => (StatusCode::OK, Html(render_note_view(&note))).into_response(),
        None => (StatusCode::NOT_FOUND, Html(render_note_not_found())).into_response(),
    }
}

pub async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut store = state.store.lock().expect("store mutex poisoned");
    store.delete_note(&id);
    let notes = store.list_desc();
    Html(render_notes_list(&notes)).into_response()
}
