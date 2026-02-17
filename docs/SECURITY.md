# Axiom Security

- Capability-based isolation; no ambient IO.
- Tool outputs treated as untrusted bytes/JSON until schema validation.
- Secrets modeled as opaque wrappers; default observability sinks redact.
- Policy engine restricts tool allowlists, spending budgets, and approval gates.
- Supply chain: lockfile checksums; optional signature verification.
- Replay logs support forensic provenance and post-incident diffing.
