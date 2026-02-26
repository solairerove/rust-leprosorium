use askama::Template;

use crate::{
    models::Note,
    util::{format_utc_display, markdown_to_safe_html},
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    notes: &'a [NoteListItem],
}

#[derive(Template)]
#[template(path = "notes_list.html")]
struct NotesListTemplate<'a> {
    notes: &'a [NoteListItem],
}

#[derive(Template)]
#[template(path = "notes_list_oob.html")]
struct NotesListOobTemplate<'a> {
    notes: &'a [NoteListItem],
}

#[derive(Template)]
#[template(path = "note_view.html")]
struct NoteViewTemplate<'a> {
    note: NoteViewItem<'a>,
}

#[derive(Template)]
#[template(path = "note_not_found.html")]
struct NoteNotFoundTemplate;

#[derive(Template)]
#[template(path = "note_edit_form.html")]
struct NoteEditFormTemplate<'a> {
    note: NoteEditItem<'a>,
}

struct NoteListItem {
    id: String,
    title: String,
    created_display: String,
}

struct NoteViewItem<'a> {
    id: &'a str,
    title: &'a str,
    rendered_body_html: String,
}

struct NoteEditItem<'a> {
    id: &'a str,
    title: &'a str,
    body: &'a str,
}

fn render_template<T: Template>(template: &T) -> String {
    template
        .render()
        .unwrap_or_else(|err| format!("<div>Template render error: {err}</div>"))
}

fn map_notes(notes: &[Note]) -> Vec<NoteListItem> {
    notes
        .iter()
        .map(|note| NoteListItem {
            id: note.id.clone(),
            title: note.title.clone(),
            created_display: format_utc_display(&note.created_at),
        })
        .collect()
}

pub fn render_index_page(notes: &[Note]) -> String {
    let notes = map_notes(notes);
    render_template(&IndexTemplate { notes: &notes })
}

pub fn render_notes_list(notes: &[Note]) -> String {
    let notes = map_notes(notes);
    render_template(&NotesListTemplate { notes: &notes })
}

pub fn render_note_view(note: &Note) -> String {
    render_template(&NoteViewTemplate {
        note: NoteViewItem {
            id: &note.id,
            title: &note.title,
            rendered_body_html: markdown_to_safe_html(&note.body),
        },
    })
}

pub fn render_note_not_found() -> String {
    render_template(&NoteNotFoundTemplate)
}

pub fn render_note_edit_form(note: &Note) -> String {
    render_template(&NoteEditFormTemplate {
        note: NoteEditItem {
            id: &note.id,
            title: &note.title,
            body: &note.body,
        },
    })
}

pub fn render_notes_list_oob(notes: &[Note]) -> String {
    let notes = map_notes(notes);
    render_template(&NotesListOobTemplate { notes: &notes })
}
