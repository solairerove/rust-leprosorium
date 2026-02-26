# Tech Debt Backlog

## 5) Add DB Index for List Query
- Current: Notes are ordered by `created_at_unix` without explicit index.
- Target: Improve scalability for `ORDER BY created_at_unix DESC`.
- Actions:
  - Add migration with:
    - `CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at_unix DESC);`

## 6) Graceful Shutdown + Structured Logging
- Current: Server runs without shutdown signal handling and minimal logging.
- Target: Better ops behavior and observability.
- Actions:
  - Add `tokio::signal` based graceful shutdown.
  - Add `tracing` and `tracing-subscriber`.
  - Replace ad-hoc prints with structured logs.

## 7) Template System for Maintainability (Optional)
- Current: HTML is built with string formatting in `src/views.rs`.
- Target: Safer, composable templates as UI grows.
- Actions:
  - Evaluate `askama` or `maud`.
  - Migrate high-churn views first.

