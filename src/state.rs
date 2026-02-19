use crate::store::NotesStore;

#[derive(Clone)]
pub struct AppState {
    pub store: NotesStore,
}

impl AppState {
    pub fn new(store: NotesStore) -> Self {
        Self { store }
    }
}
