# Tech Debt Backlog

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

