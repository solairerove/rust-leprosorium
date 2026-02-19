use std::{
    cmp::Reverse,
    fs,
    path::{Path, PathBuf},
};

use crate::{models::Note, util::now_unix};

pub struct NotesStore {
    notes: Vec<Note>,
    file_path: PathBuf,
}

impl NotesStore {
    pub fn load_or_new(data_dir: &Path) -> Self {
        let file_path = data_dir.join("notes.json");

        if let Err(err) = fs::create_dir_all(data_dir) {
            eprintln!("Failed to create data directory: {err}");
        }

        if let Ok(content) = fs::read_to_string(&file_path) {
            match serde_json::from_str::<Vec<Note>>(&content) {
                Ok(notes) => {
                    return Self { notes, file_path };
                }
                Err(err) => {
                    eprintln!("Failed to parse notes file, starting empty store: {err}");
                }
            }
        }

        Self {
            notes: Vec::new(),
            file_path,
        }
    }

    pub fn list_desc(&self) -> Vec<Note> {
        let mut notes = self.notes.clone();
        notes.sort_by_key(|note| Reverse(note.created_at_unix));
        notes
    }

    pub fn create_note(&mut self, title: String, body: String) {
        let now = now_unix();
        let note = Note {
            id: format!("{now}"),
            title,
            body,
            created_at_unix: now,
        };
        self.notes.push(note);
        self.save();
    }

    pub fn delete_note(&mut self, id: &str) {
        self.notes.retain(|note| note.id != id);
        self.save();
    }

    pub fn update_note(&mut self, id: &str, title: String, body: String) -> Option<Note> {
        let index = self.notes.iter().position(|note| note.id == id)?;
        self.notes[index].title = title;
        self.notes[index].body = body;
        let updated_note = self.notes[index].clone();
        self.save();
        Some(updated_note)
    }

    pub fn get_note(&self, id: &str) -> Option<Note> {
        self.notes.iter().find(|note| note.id == id).cloned()
    }

    fn save(&self) {
        match serde_json::to_string_pretty(&self.notes) {
            Ok(content) => {
                if let Err(err) = fs::write(&self.file_path, content) {
                    eprintln!("Failed to save notes: {err}");
                }
            }
            Err(err) => eprintln!("Failed to serialize notes: {err}"),
        }
    }
}
