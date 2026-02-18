use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub created_at_unix: u64,
}

#[derive(Deserialize)]
pub struct NoteForm {
    pub title: String,
    pub body: String,
}
