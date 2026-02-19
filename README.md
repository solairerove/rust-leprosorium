# rust-leprosorium

Starter offline notes service on Rust:
- backend: `axum`
- frontend: server-side HTML + `htmx`
- storage: local SQLite database `notes_data/notes.db`
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
- Edit/update note
- Delete note
- Persistent local SQLite storage

## Why this stack

- `axum` and `tokio` are stable and production-proven.
- `htmx` keeps frontend simple, no SPA complexity.
- SQLite gives offline durability and better base for search/tags later.
- Markdown note body is rendered safely (sanitized HTML).

## Next steps

- Tag support
- Full-text search
- Export to `.md`
