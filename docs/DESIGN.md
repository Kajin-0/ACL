# Axiom Design

## Major choices
1. **Compiler/VM strategy:** Bytecode-ready front end + tree-walking reference runtime first (Option B). Rationale: fastest path to deterministic semantics and effect/capability enforcement, with explicit IR boundary for future bytecode/JIT.
2. **Memory management:** Rust ownership + ARC (`Arc`) for runtime shared objects to guarantee memory safety and predictable behavior.
3. **Concurrency:** Structured concurrency model in the language design; current reference runtime executes deterministically in a single-threaded loop with ordered event logging.
4. **Effects in IR:** Function and statement nodes carry effect tags (`Pure`, `Tool` now; extensible enum for `Net/Fs/Time/Random`).
5. **Capability tokens:** Runtime-only opaque token with monotonic unique IDs and kind tags; constructors are trusted-runtime only.

## Architecture
- `compiler/`: AST, parser, typechecker with effect inference.
- `runtime/`: evaluator, capability checks, tool bridge, replay/event log.
- `tools/axiom`: CLI and developer tooling front door (`run/fmt/lint/test/bench/pkg`).
- `lsp/`: protocol skeleton.

## Non-goals in v0.1
- Full optimizing compiler/JIT.
- Full LSP transport/protocol implementation.
- Cryptographic package signatures (checksum lockfile scaffolding only).

## Milestones
- **M1:** Lexer+parser+AST with golden tests.
- **M2:** Typechecker with compile-fail tests.
- **M3:** Pure interpreter execution.
- **M4:** Async runtime + structured task API.
- **M5:** Tool call ABI + capability enforcement.
- **M6:** Deterministic replay hash stability.
- **M7:** CLI tooling + package lock scaffolding.
- **M8:** LSP core features + benchmark harness.


## v0.1 delivered in this branch
- Typed tool declaration parsing and static callsite cap/type checks.
- Capability manifest rendering from typechecked programs.

- Introduced structured compile/runtime error enums to replace raw string-only core API errors.
