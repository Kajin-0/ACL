# Runtime

- Deterministic mode: single-thread executor, seeded RNG, synthetic time source.
- Replay log entries: `Print`, `ToolCall` (with provenance metadata), `Random`, `Time`.
- Event log digest: stable FNV-1a 64-bit hash over canonical line-based replay event encoding (v0.1 reference implementation).
- Structured concurrency API planned around nursery scopes and cancellation trees.
