# Axiom

Axiom is an agent-native language and runtime focused on deterministic execution, capability security, typed tools, and replayability.

## Build
```bash
cargo build --workspace
```

## Test
```bash
cargo test --workspace
```

## Run examples
```bash
cargo run -p axiom -- run examples/pure_pipeline.ax
cargo run -p axiom -- run examples/tool_call.ax --replay-out replay.log
cargo run -p axiom -- replay-check replay.log
cargo run -p axiom -- manifest examples/tool_call.ax
```

## Tooling commands
```bash
cargo run -p axiom -- fmt examples/pure_pipeline.ax
cargo run -p axiom -- lint examples/tool_call.ax
cargo run -p axiom -- pkg --lock
cargo run -p axiom -- bench
```
