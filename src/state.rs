use std::sync::{Arc, Mutex};

use crate::store::NotesStore;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<Mutex<NotesStore>>,
}

impl AppState {
    pub fn new(store: NotesStore) -> Self {
        Self {
            store: Arc::new(Mutex::new(store)),
        }
    }
}
