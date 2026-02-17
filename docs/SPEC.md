# Axiom Language Specification (v0.1)

## 1. Overview
Axiom is an agent-native, deterministic-by-default language for autonomous systems that observe, decide, act via tools, and learn with memory under explicit policy and capability constraints.

## 2. Lexical Structure
- UTF-8 source files, `.ax` extension.
- Comments: `// line`, `/* block */`, doc comments `///`.
- Strings: `"..."`; raw strings `r"..."`.
- Numeric literals: decimal ints/floats; optional unit suffix (reserved in v0.1).

## 3. Grammar (EBNF sketch)
```ebnf
program      = { statement } ;
statement    = tool_decl | let_stmt | print_stmt | tool_call_stmt | expr_stmt ;
let_stmt      = "let" ident "=" expr ";" ;
print_stmt    = "print" expr ";" ;
tool_decl     = "tool" ident "input" "{" [typed_fields] "}" "output" "{" [typed_fields] "}" "cap" ident ";" ;
tool_call_stmt= "call" ident "{" [ field_list ] "}" "using" ident "timeout" integer ";" ;
field_list    = field { "," field } ;
field         = ident ":" expr ;
expr          = binary_expr | literal | ident ;
binary_expr   = expr ("+"|"-"|"*"|"/") expr ;
literal       = integer | boolean | string ;
```

## 4. Modules and Visibility
- `mod`, `use`, and `pub` reserved; module system fully specified in v0.2.

## 5. Type System
- Primitive: `Int`, `Float`, `Bool`, `String`, `Bytes`.
- Structural: `Array[T]`, `Map[K,V]`, `Set[T]`, tuples.
- ADTs: `struct`, `enum`.
- `Option[T]` and `Result[T,E]` are first-class.
- Non-nullable by default.
- Inference is local and never changes effect obligations.

## 6. Effects
- Default effect is `!pure`.
- Current tracked effects: `tool`, `net`, `fs`, `time`, `random`.
- Syntax example:
  - `fn score(x: Int) -> Int !pure`
  - `fn fetch(...) -> Result[Doc, E] !tool[ToolCap]`
- Pure code cannot perform IO/tool calls/randomness.

## 7. Capabilities
- No ambient authority.
- Privileged operations require explicit capability values.
- Capabilities are unforgeable runtime tokens.
- Narrowing derives weaker capabilities only.
- Compiler emits capability manifest mapping callsites to required caps (`axiom manifest <file>`).

## 8. Concurrency
- Structured concurrency with `task::scope` / nurseries.
- `async/await` with typed cancellation.
- Deadlines/timeouts/budgets are required for tool calls.
- Deterministic scheduler mode defines stable task interleaving and channel ordering.

## 9. Memory Model
- Runtime uses memory-safe host implementation.
- Reference model: ownership + ARC runtime objects, no unsafe shared mutation without synchronization.

## 10. Error Model
- No exceptions for control flow.
- `Result[T,E]` + `?` for propagation.
- Reference implementation now uses structured compile/runtime error enums (phase toward fully surfaced language-level typed errors).
- Typed errors and retry policies for tool operations.

## 11. Tool Call Semantics
- Tools have typed input/output schemas via explicit `tool` declarations.
- Tool outputs are untrusted and require validation before trust elevation.
- Each result carries provenance metadata `{tool, timestamp, hash, policy_tags}`.

## 12. Determinism and Replay
- Record mode logs scheduler decisions, tool IO, time, randomness.
- Replay mode rehydrates event stream and verifies event hash equality via `axiom replay-check` in v0.1.
- Audit bundle includes source hash, manifest, replay log, and provenance chain.

## 13. Standard Library Surface
- `tool`, `agent`, `task`, `net`, `fs`, `time`, `crypto`, `random`, `data`, `obs`.
- `net/fs/time/random` require capabilities.

## 14. Security Model
- Threats: malicious tool outputs, prompt injection payloads, hostile packages, secret exfiltration.
- Default deny for external effects.
- `Secret[T]` redacts from logs by default.
- Optional package signatures + mandatory checksums.
