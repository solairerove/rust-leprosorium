use crate::{
    models::Note,
    util::{escape_html, markdown_to_safe_html},
};

pub fn render_index_page(notes: &[Note]) -> String {
    let list_html = render_notes_list(notes);
    format!(
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
    )
}

pub fn render_notes_list(notes: &[Note]) -> String {
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
    <button hx-get="/notes/{}/edit" hx-target="#note-view" hx-swap="outerHTML">Edit</button>
    <button class="delete" hx-delete="/notes/{}" hx-target="#notes-list" hx-swap="outerHTML" hx-confirm="Delete this note?">Delete</button>
  </div>
</li>"##,
            escape_html(&note.title),
            note.created_at_unix,
            note.id,
            note.id,
            note.id
        );
        html.push_str(&item);
    }
    html.push_str("</ul></div>");
    html
}

pub fn render_note_view(note: &Note) -> String {
    let rendered_body = markdown_to_safe_html(&note.body);
    format!(
        r##"<section class="card" id="note-view" style="grid-column: 1 / -1;">
  <h2>{}</h2>
  <p class="muted">id: {}</p>
  <div class="preview">{}</div>
</section>"##,
        escape_html(&note.title),
        escape_html(&note.id),
        rendered_body
    )
}

pub fn render_note_not_found() -> String {
    "<section class=\"card\" id=\"note-view\" style=\"grid-column: 1 / -1;\"><p class=\"muted\">Note not found.</p></section>".to_string()
}

pub fn render_note_edit_form(note: &Note) -> String {
    format!(
        r##"<section class="card" id="note-view" style="grid-column: 1 / -1;">
  <h2>Edit note</h2>
  <p class="muted">id: {}</p>
  <form hx-put="/notes/{}" hx-target="#note-view" hx-swap="outerHTML">
    <input type="text" name="title" value="{}" required />
    <div style="height: 8px;"></div>
    <textarea name="body" required>{}</textarea>
    <div style="height: 8px;"></div>
    <button type="submit">Update note</button>
  </form>
</section>"##,
        escape_html(&note.id),
        escape_html(&note.id),
        escape_html(&note.title),
        escape_html(&note.body)
    )
}

pub fn render_notes_list_oob(notes: &[Note]) -> String {
    render_notes_list(notes).replace(
        "<div id=\"notes-list\">",
        "<div id=\"notes-list\" hx-swap-oob=\"outerHTML\">",
    )
}
