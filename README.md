# rust-leprosorium

Starter offline notes service on Rust:
- backend: `axum`
- frontend: server-side HTML + `htmx`
- storage: local file `notes_data/notes.json` (no database)
- rust notes: `README_RUST_NOTES.md`

## Run

```shell
cargo run
```

Open `http://127.0.0.1:3000`.

## What is implemented

- Create note
- Notes list
- Open note details
- Delete note
- Persistent file storage without DB

## Why this stack

- `axum` and `tokio` are stable and production-proven.
- `htmx` keeps frontend simple, no SPA complexity.
- JSON file storage is enough for first MVP and easy to inspect manually.

## Next steps

- Edit note
- Tag support
- Full-text search
- Export to `.md`
