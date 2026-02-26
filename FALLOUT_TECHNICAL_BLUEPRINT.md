# Fallout-Grade Technical Blueprint

## 1. Mission Profile and Non-Negotiables
- Primary objective: survive long-term offline operation on ARM-class hardware.
- Recovery objective: deterministic rebuild from blank device without internet.
- Longevity objective: preserve operability and data readability for 200+ years.
- Reliability objective: degrade safely, fail loudly, and recover predictably.

## 2. System Boundaries and Minimal Core
- Define a small immutable core:
  - Data model and storage engine contract.
  - Recovery commands and backup format.
  - Essential read/write workflows.
- Keep optional features as detachable modules with strict interfaces.
- Do not allow optional features to modify core durability behavior.

## 3. Hardware and OS Assumptions
- Target baseline: low-power ARM board (e.g., Orange Pi), local storage, no network.
- Specify minimum kernel, filesystem, memory, and storage requirements.
- Prefer components available from multiple vendors to reduce replacement risk.
- Record all hardware assumptions in machine-readable and human-readable forms.

## 4. Reproducible and Offline Build Chain
- Pin Rust toolchain and target triples.
- Vendor all dependencies into the repository.
- Require lockfile-based deterministic builds.
- Provide a documented offline build path:
  - No network access.
  - No dynamic downloads.
  - Byte-for-byte reproducibility target where practical.

## 5. Dependency Governance and Supply Chain Freezing
- Maintain an allowlist of runtime dependencies.
- Track purpose of each dependency and replacement strategy.
- Keep archived source snapshots for crates, toolchain, and build tools.
- Define policy for adding new dependencies:
  - Justification.
  - Attack surface impact.
  - Long-term maintenance burden.

## 6. Storage Durability and Data Integrity
- Configure storage for crash durability (journal mode, sync policy, timeouts).
- Use explicit checksums for all critical persisted artifacts.
- Add periodic integrity scrubbing and verification tasks.
- Detect corruption early and provide actionable recovery status.

## 7. Schema Evolution and Long-Term Data Compatibility
- Version schemas explicitly and keep forward migration history.
- Treat exported archives as stable public format, separate from internal DB layout.
- Include schema version, encoding, and hash metadata in every archive.
- Maintain migration validation tests for every historical version.

## 8. Backup, Restore, and Bare-Metal Recovery
- Ship first-class commands for:
  - Full backup.
  - Verify backup.
  - Restore from backup.
  - Dry-run restore.
- Require one-command cold-start recovery from empty disk.
- Test restoration into a clean environment regularly.
- Keep recovery runbook concise, printable, and offline-friendly.

## 9. Failure Model and Resilience Testing
- Define failure classes:
  - Power loss during write.
  - Partial disk failure.
  - Corrupted database pages.
  - Clock anomalies.
  - Interrupted migrations.
- Build automated tests and simulations for each class.
- Treat untested failure classes as unresolved technical debt.

## 10. Time, Identity, and Ordering Guarantees
- Avoid hard reliance on wall-clock time for canonical ordering.
- Use monotonic sequence identifiers for authoritative ordering.
- Keep timestamps for human display and diagnostics, not core correctness.
- Document behavior under clock drift, reset, and leap-related anomalies.

## 11. Runtime Operations, Observability, and Diagnostics
- Provide local-only operational visibility:
  - Structured logs to files.
  - Rotating retention.
  - Startup self-check report.
- Include health and consistency checks executable without network.
- Standardize error codes and remediation guidance for operators.

## 12. Security and Cryptographic Agility (Offline Context)
- Keep threat model realistic for physically accessible offline devices.
- Protect archive integrity with versioned cryptographic primitives.
- Plan for algorithm rotation without invalidating historical data.
- Separate confidentiality goals from integrity goals in design docs.

## 13. Maintainability and Knowledge Preservation
- Maintain Architecture Decision Records (ADRs) for key choices.
- Keep critical docs in plain text formats with low tooling dependency.
- Write concise runbooks:
  - Install.
  - Operate.
  - Backup/restore.
  - Incident response.
- Include glossary and system map for future maintainers.

## 14. Portability and Future Hardware Migration
- Isolate hardware-specific behavior behind narrow interfaces.
- Maintain at least one emulator/simulator execution path.
- Define migration checklist for moving data and binaries to new hardware.
- Avoid undocumented assumptions tied to one board model.

## 15. Governance, Validation Cadence, and Drift Control
- Define periodic audits:
  - Recovery drill cadence.
  - Integrity scan cadence.
  - Dependency review cadence.
- Record audit artifacts in-repo for historical traceability.
- Reject undocumented operational changes.
- Treat drift from this blueprint as change requests requiring review.

## 16. Immediate Implementation Roadmap (Code Next)
- Phase 1:
  - Remove network runtime dependencies.
  - Enforce durability PRAGMAs.
  - Add backup/restore/verify commands.
- Phase 2:
  - Add migration replay and corruption tests.
  - Add stable archive format and manifest checksums.
- Phase 3:
  - Add health/self-check subsystem.
  - Add structured log retention and diagnostics bundle export.

