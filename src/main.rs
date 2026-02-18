mod app;
mod handlers;
mod models;
mod state;
mod store;
mod util;
mod views;

use state::AppState;
use std::{net::SocketAddr, path::Path};
use store::NotesStore;

#[tokio::main]
async fn main() {
    let store = NotesStore::load_or_new(Path::new("notes_data"));
    let state = AppState::new(store);
    let app = app::build_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("Failed to bind on {addr}: {err}"));

    println!("Notes app: http://{addr}");
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|err| panic!("Server error: {err}"));
}
