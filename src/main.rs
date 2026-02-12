use axum::{
    Router,
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    fs,
    net::SocketAddr,
    path::{Path as FsPath, PathBuf},
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone)]
struct AppState {
    store: Arc<Mutex<NotesStore>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Note {
    id: String,
    title: String,
    body: String,
    created_at_unix: u64,
}

#[derive(Serialize, Deserialize)]
struct NotesStore {
    notes: Vec<Note>,
    #[serde(skip)]
    file_path: PathBuf,
}

#[derive(Deserialize)]
struct NoteForm {
    title: String,
    body: String,
}

impl NotesStore {
    fn load_or_new(data_dir: &FsPath) -> Self {
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

    fn list_desc(&self) -> Vec<Note> {
        let mut notes = self.notes.clone();
        notes.sort_by_key(|note| Reverse(note.created_at_unix));
        notes
    }

    fn create_note(&mut self, title: String, body: String) {
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

    fn delete_note(&mut self, id: &str) {
        self.notes.retain(|note| note.id != id);
        self.save();
    }

    fn get_note(&self, id: &str) -> Option<Note> {
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

#[tokio::main]
async fn main() {
    let store = NotesStore::load_or_new(FsPath::new("notes_data"));
    let state = AppState {
        store: Arc::new(Mutex::new(store)),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/notes", post(create_note))
        .route("/notes/:id", get(show_note).delete(delete_note))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to bind on {addr}: {err}");
        });

    println!("Notes app: http://{addr}");
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|err| panic!("Server error: {err}"));
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let store = state.store.lock().expect("store mutex poisoned");
    let notes = store.list_desc();
    let list_html = render_notes_list(&notes);
    let page = format!(
        r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Offline Notes</title>
  <script src="https://unpkg.com/htmx.org@1.9.12"></script>
  <style>
    :root {{
      --bg: #f7f7f0;
      --card: #ffffff;
      --text: #1f241f;
      --muted: #657068;
      --line: #d9ddcf;
      --accent: #2b6f5a;
      --danger: #9f2d2d;
    }}
    * {{ box-sizing: border-box; }}
    body {{
      margin: 0;
      font-family: "Iosevka", "SF Mono", "Consolas", monospace;
      color: var(--text);
      background: radial-gradient(circle at 1px 1px, #e6e8dc 1px, transparent 0);
      background-size: 24px 24px;
      min-height: 100vh;
    }}
    .wrap {{
      max-width: 980px;
      margin: 0 auto;
      padding: 24px;
      display: grid;
      gap: 16px;
      grid-template-columns: 1fr;
    }}
    @media (min-width: 900px) {{
      .wrap {{
        grid-template-columns: 1fr 1fr;
      }}
    }}
    .card {{
      background: var(--card);
      border: 1px solid var(--line);
      border-radius: 12px;
      padding: 16px;
      box-shadow: 0 8px 20px rgba(0,0,0,0.04);
    }}
    h1, h2 {{
      margin-top: 0;
      margin-bottom: 12px;
      letter-spacing: 0.03em;
    }}
    input, textarea, button {{
      width: 100%;
      font: inherit;
      border-radius: 8px;
      border: 1px solid var(--line);
      padding: 10px 12px;
      background: #fff;
    }}
    textarea {{ min-height: 120px; resize: vertical; }}
    button {{
      cursor: pointer;
      background: var(--accent);
      border-color: var(--accent);
      color: white;
      font-weight: 700;
    }}
    ul {{ list-style: none; margin: 0; padding: 0; display: grid; gap: 10px; }}
    li {{
      border: 1px solid var(--line);
      border-radius: 8px;
      padding: 10px;
      background: #fff;
      display: grid;
      gap: 8px;
    }}
    .note-actions {{
      display: flex;
      gap: 8px;
    }}
    .note-actions button {{
      width: auto;
      flex: 1;
    }}
    .note-actions button.delete {{
      background: var(--danger);
      border-color: var(--danger);
    }}
    .muted {{
      color: var(--muted);
      font-size: 0.9rem;
    }}
    .preview {{
      white-space: pre-wrap;
      border-top: 1px dashed var(--line);
      padding-top: 10px;
      margin-top: 10px;
    }}
  </style>
</head>
<body>
  <main class="wrap">
    <section class="card">
      <h1>Offline Notes</h1>
      <p class="muted">No DB. Notes are saved to <code>notes_data/notes.json</code>.</p>
      <form hx-post="/notes" hx-target="#notes-list" hx-swap="outerHTML">
        <input type="text" name="title" placeholder="Title" required />
        <div style="height: 8px;"></div>
        <textarea name="body" placeholder="Write your note..." required></textarea>
        <div style="height: 8px;"></div>
        <button type="submit">Save note</button>
      </form>
    </section>

    <section class="card">
      <h2>Notes</h2>
      {list_html}
    </section>

    <section class="card" id="note-view" style="grid-column: 1 / -1;">
      <p class="muted">Open a note from list.</p>
    </section>
  </main>
</body>
</html>"##
    );

    Html(page)
}

async fn create_note(
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

async fn show_note(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let store = state.store.lock().expect("store mutex poisoned");
    match store.get_note(&id) {
        Some(note) => {
            let html = format!(
                r##"<section class="card" id="note-view" style="grid-column: 1 / -1;">
  <h2>{}</h2>
  <p class="muted">id: {}</p>
  <div class="preview">{}</div>
</section>"##,
                escape_html(&note.title),
                escape_html(&note.id),
                escape_html(&note.body)
            );
            (StatusCode::OK, Html(html)).into_response()
        }
        None => (StatusCode::NOT_FOUND, Html("<section class=\"card\" id=\"note-view\" style=\"grid-column: 1 / -1;\"><p class=\"muted\">Note not found.</p></section>".to_string())).into_response(),
    }
}

async fn delete_note(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let mut store = state.store.lock().expect("store mutex poisoned");
    store.delete_note(&id);
    let notes = store.list_desc();
    Html(render_notes_list(&notes)).into_response()
}

fn render_notes_list(notes: &[Note]) -> String {
    if notes.is_empty() {
        return "<div id=\"notes-list\"><p class=\"muted\">No notes yet.</p></div>".to_string();
    }

    let mut html = String::from("<div id=\"notes-list\"><ul>");
    for note in notes {
        let item = format!(
            r##"<li>
  <strong>{}</strong>
  <span class="muted">created: {}</span>
  <div class="note-actions">
    <button hx-get="/notes/{}" hx-target="#note-view" hx-swap="outerHTML">Open</button>
    <button class="delete" hx-delete="/notes/{}" hx-target="#notes-list" hx-swap="outerHTML" hx-confirm="Delete this note?">Delete</button>
  </div>
</li>"##,
            escape_html(&note.title),
            note.created_at_unix,
            note.id,
            note.id
        );
        html.push_str(&item);
    }
    html.push_str("</ul></div>");
    html
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
