# rust-leprosorium

Starter offline notes service on Rust:
- backend: `axum`
- frontend: server-side HTML + `htmx`
- storage: local file `notes_data/notes.json` (no database)
- rust notes: `README_RUST_NOTES.md`

## Development Commands

```shell
# format code
cargo fmt

# quick compile check
cargo check

# run app locally
cargo run

# run tests
cargo test

# lints (recommended)
cargo clippy --all-targets --all-features -- -D warnings

# production build
cargo build --release
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
