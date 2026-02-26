use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NoteForm {
    pub title: String,
    pub body: String,
}
