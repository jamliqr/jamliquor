# 🚀 MILESTONE.md — Roadmap

[← Back to Index](./index.md)

## 📋 Overview

This roadmap reflects the [2026 reassessment PIVOT decision](../REASSESSMENT/README.md). It replaces the pre-pivot Authorer / PolkaVM / Modular Extensions roadmap, which is archived at [`docs/archive/TODO-2026-04.md`](../archive/TODO-2026-04.md) for history. Task-level tracking lives in [`Plans.md`](../../Plans.md); this document states only the milestone gates and dates from [`spec.md`](../../spec.md) §4.

There are exactly two milestones in current scope. Nothing past MVP-2/3 is planned until MVP-2/3 lands — see the stop conditions below.

## 🎯 MVP-1 — Narrow Conformance

**Due: 2026-11-15**

Decode/encode round-trip of the official **binary codec vectors** (tiny + full configurations) plus green **`stf/safrole`** vectors, running as a required CI check. Vector authority: `w3f/jamtestvectors`, frozen at **v0.7.1, commit `1dc503a`** for the duration of this milestone (`spec.md` §3).

No other code changes until this is green: no edge tuning, no benchmarks, no new README claims (`spec.md`'s conformance-first rule).

### Scope

- JAM binary codec (variable-length integers, sequences, discriminators) per Gray Paper v0.7.1
- Header + extrinsic structs, byte-exact round-trip
- Ed25519 verification, Blake2b-256, Bandersnatch ring VRF (verification only, via audited crates — no hand-rolled cryptography)
- Safrole state transition: tickets, epoch marks, fallback selection, Fisher-Yates shuffle

### Out of scope for MVP-1

Work packages, Refine, Accumulate, PVM, service model, networking, availability/erasure coding, storage trie. See `spec.md` §6 for the architecture this targets.

## 🎯 MVP-2/3 — Edge Evidence (Gate G3)

**Due: 2027-02-15**

The same conformant core, cross-compiled and benchmarked on **one ARM64 SBC and one RISC-V SBC**: peak RAM, binary size, vectors/sec, startup time.

This is the project's central research experiment and the only test of Gate G3 — see `spec.md` §4 for the exact PASS wording (workload evidence within Tier-B budgets, not a claim of a deployable role).

### Deliverables

- Cross-compilation targets in CI (`aarch64-unknown-linux-gnu`, `riscv64gc-unknown-linux-gnu`)
- Reproducible benchmark harness with machine-readable output
- Measurements on real hardware, committed to the repo
- A published technical report (the named contributor-attraction mechanism — `spec.md` §7)

## 🛑 Stop conditions

Per `spec.md` §5, the pivot is killed and the repo archived if any of:

1. Codec + safrole vectors not green by 2026-11-15
2. No SBC benchmark exists by 2027-02-15
3. Benchmarks show no meaningful resource advantage for any JAM role vs. an existing client
4. Two consecutive months pass with zero commits (CI-monitored, `Plans.md` Phase 2)

Archiving on schedule is a documented successful outcome of the 2026 reassessment, not a failure.

## ⏱ Time budget

**Mode B: 3–5 hours/week, hard-capped.** Medicine remains the primary profession. Mode C (10+ h/week) is prohibited unless MVP-3 produces a measured, competitive result **and** an external contributor has landed a nontrivial PR.

---

**Navigation**
Previous: [Cleanroom Policy](./CLEANROOM.md)

**Related Topics**
- [Product Contract (spec.md)](../../spec.md)
- [Task Ledger (Plans.md)](../../Plans.md)
- [2026 Reassessment & Decision](../REASSESSMENT/README.md)
