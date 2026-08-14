# 📜 Project Manifesto

[← Back to Index](./index.md)

# The JamLiquor Manifesto: A Cleanroom JAM Research Core

Version 0.3.0 | 2026-08-14 (post-pivot)

Drafted by: [Kresna Sucandra, MD](https://github.com/SHA888)

Supersedes: [Manifesto v0.1.0, March 2025](../archive/MANIFESTO-2025.md) (preserved verbatim for cleanroom provenance)

---

## Why this manifesto changed

JamLiquor's original manifesto committed to five principles: Lightweight, Decentralized, Adaptive (AI), Quantum-Resistant, Simple. In August 2026, a full [reassessment against the current JAM ecosystem](../REASSESSMENT/README.md) measured that commitment against evidence — 1,517 lines of Rust, ~0.3% of official test vectors exercised, zero edge-hardware benchmarks, zero AI code after a year of development. The outcome was **PIVOT**: keep the kernel of the original idea, drop what evidence didn't support.

This document states what remains, and on what terms.

## The two claims that remain

### 1. Lightweight — unproven, gated by measurement

> A blockchain client that cannot run on a $10 board is worth investigating, not assuming works.

Nobody in the ~43-team JAM ecosystem has published constrained-hardware benchmarks. That silence is either an opportunity or a reason nobody bothered — JamLiquor's job is to find out, on real ARM64 and RISC-V single-board computers, and publish the numbers whether they're good or not. See [`spec.md`](../../spec.md) §4 (MVP-2/3) for the gate.

### 2. JAM-native — unproven, gated by conformance

> Containing JAM-related code is not the same as being a JAM implementation.

JamLiquor's own 2026 audit found it was a JSON-fixture importer, not a conformant client. "JAM-native" is earned only when a defined subset (binary codec + Safrole state transition) provably agrees with the official `w3f/jamtestvectors`. See [`spec.md`](../../spec.md) §3–4 (MVP-1) for the gate.

## What was retired, and why

- **Adaptive (AI inference)** — evaluated against three concrete models in the 2026 reassessment; all require infrastructure outside JAM's scope. Application-layer idea, not a client feature. One paragraph kept at [`docs/src/research-notes.md`](./research-notes.md).
- **Quantum-Resistant (PQC)** — JAM's protocol cryptography is fixed by the Gray Paper; non-conformant signatures cannot interoperate, and PQ primitives are far larger than the classical ones they'd replace. One paragraph kept at [`docs/src/research-notes.md`](./research-notes.md).
- **Full validator / JAM Prize entrant** — the prize-track catch-up window is closed (~43 teams, ~15 M1 submissions, multiple Rust clients at 100% public-suite conformance by the time this pivot happened). Competing there now has no realistic path at 3–5 hours/week.

## What JamLiquor is not

Not a fork. Not a production node. Not a validator. Not a marketing claim ahead of a green CI job. See [`spec.md`](../../spec.md) §2 for the full binding non-goals list.

## Terms of engagement

- Every capability claim in this repo's docs must map to a green CI job or a committed measurement artifact ([`spec.md`](../../spec.md) §8).
- Time budget is capped at Mode B (3–5 h/week); Mode C requires evidence, not ambition ([`spec.md`](../../spec.md) §4).
- Pre-committed stop conditions exist and will be honored — archiving this project on schedule if the evidence doesn't support continuing is a **successful outcome**, not a failure ([`spec.md`](../../spec.md) §5).

---

**Navigation**
Previous: [Quick Start Guide](./README.md) | Next: [Cleanroom Policy](./CLEANROOM.md)

**Related Topics**
- [Product Contract (spec.md)](../../spec.md)
- [2026 Reassessment & Decision](../REASSESSMENT/README.md)
- [Research Notes (AI, PQC)](./research-notes.md)
