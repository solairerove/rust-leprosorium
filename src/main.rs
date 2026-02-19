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
    let store = match NotesStore::load_or_new(Path::new("notes_data")).await {
        Ok(store) => store,
        Err(err) => {
            eprintln!("Failed to initialize store: {err}");
            return;
        }
    };
    let state = AppState::new(store);
    let app = app::build_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Failed to bind on {addr}: {err}");
            return;
        }
    };

    println!("Notes app: http://{addr}");
    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Server error: {err}");
    }
}
