# Tech Debt Backlog

## 7) Template System for Maintainability (Optional)
- Current: HTML is built with string formatting in `src/views.rs`.
- Target: Safer, composable templates as UI grows.
- Actions:
  - Evaluate `askama` or `maud`.
  - Migrate high-churn views first.
