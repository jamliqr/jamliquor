# JamLiquor 2026 Reassessment — Decision

**Date:** 2026-08-13
**Framework:** JamLiquor 2026 Reassessment Framework (Sections 1–20)
**Supporting evidence:** `docs/REASSESSMENT/01…06` — all gate verdicts derive from measurements and cited sources there.

---

## Gate results (framework §13)

| Gate | Question | Verdict |
|---|---|---|
| G0 | Is JAM still strategically relevant? | **PASS (narrow)** — only for a constrained-hardware/light-role thesis; the prize-track full-client opportunity has closed (~43 teams, ~15 M1 submissions, 3 Rust clients at 100% public conformance) |
| G1 | Is old code salvageable? | **FAIL** — health 2.71/5; spec alignment 1/5; JSON-shaped schema is structurally mismatched with binary-first conformance |
| G2 | Can we achieve conformance? | **FAIL today** — ~0.3% of official vectors touched, no PVM, no real crypto; a narrow scope (codec + safrole STF) is reachable in ~3–4 months at Mode B |
| G3 | Is edge differentiation real? | **UNPROVEN** — zero measurements in 12 months; becomes the pivot's central experiment |
| G4 | Is AI technically justified? | **FAIL** — application layer at best; removed from project identity |
| G5 | Is there ecosystem differentiation? | **FAIL as-is; CONDITIONAL PASS** under the edge/light-role thesis with published benchmarks |
| G6 | Can it survive low founder attention? | **FAIL** — bus factor 1, demonstrated 7-month stall |
| G7 | Is the time investment rational? | **PASS only at Mode A–B (≤5 h/wk)** with hard stop conditions; Mode C is explicitly unjustified by G0–G5 |

Framework decision table: G0 passes while the original architecture and use case fail (G1, G2, G4, old G5) and a new thesis scores strongly → **PIVOT**.

---

## Decision

# **PIVOT**

### One-sentence rationale

> The original thesis — a prize-track cleanroom full JAM client with AI and PQC pillars — is dead on the 2026 evidence, but its one unoccupied kernel (a genuinely small, portable, conformance-honest JAM runtime for constrained hardware) is worth a strictly time-boxed research pivot.

### The pivoted thesis (from framework §18, now adopted as hypothesis under test)

> JamLiquor becomes a **minimal, independently implemented JAM core (codec + STF subset) that provably agrees with official test vectors and is benchmarked on ARM64/RISC-V single-board computers** — a research/light-role runtime, not a validator, with AI and PQC removed from its identity.

### Strongest evidence

1. **The gap is unbridgeable on the old path:** competitors reached 100% public-suite conformance with ~full-time teams during 2025; JamLiquor holds 1,517 LOC, no PVM, no real cryptography, and parses 4 of ~1,350 official vector files — at 3–5 h/week the full-client path is multi-year against an already-ratifying spec.
2. **The niche is real and empty:** no known client targets ≤512 MB devices or publishes SBC benchmarks; "what does JAM actually cost on constrained hardware?" is an unanswered, publishable question aligned with JAM's own decentralization narrative.
3. **The repo's own history concurs:** the April 2026 revival already demoted AI to "most speculative, lowest priority," and the Phase 0 audit documented the v0.6.7→v0.8 breakage honestly — this decision formalizes what the evidence already showed.

### Biggest risk

> The same one that already materialized once: a bus-factor-1 project stalls for months, the spec ratifies and moves, and the rewrite becomes another half-finished importer. Mitigation is structural — small self-contained milestones that stay true when paused — plus the stop conditions below.

### Architecture (framework §12)

**Option B — clean rewrite of a minimal core**, executed inside this repo:

```
jam-core (new, no_std-friendly where possible)
  ├── codec        → pass w3f codec vectors (binary, tiny + full)
  ├── crypto       → ed25519 + Blake2b via audited crates; ring-VRF via ark-vrf/w3f crates
  ├── safrole STF  → pass stf/safrole vectors
  └── trie         → pass trie vectors (state roots)
```

Existing `src/` is retained as reference until each module is superseded, then deleted. `coretime.rs` (spec-orphaned) is deleted first. Option C (split crates for edge/ai/pqc) is rejected: it multiplies surface area for a one-person team, and two of its crates (ai, pqc) no longer exist as concepts.

### Required next milestone (gate to continue at all)

> **MVP-1 (framework §14), narrowed:** decode/encode round-trip of the official **binary codec vectors** plus green **stf/safrole** vectors, in CI, by **2026-11-15** (~13 weeks at Mode B). No other code until this is green — no edge tuning, no benchmarks, no README claims.

Then **MVP-2/3:** the same binary cross-compiled and benchmarked on one ARM64 SBC and one RISC-V SBC (peak RAM, binary size, vectors/sec, startup) — this is the first and only test of Gate G3, due **2027-02-15**.

### Maximum time investment

> **Mode B: 3–5 hours/week, hard-capped.** Medicine remains the primary profession. Mode C is prohibited unless MVP-3 produces a measured, competitive result *and* an external contributor has landed a nontrivial PR (G6 evidence).

### Stop condition (pre-committed)

> **KILL the pivot and archive the repo if any of:** (1) safrole+codec vectors are not green by 2026-11-15; (2) no SBC benchmark exists by 2027-02-15; (3) the benchmark shows no meaningful resource advantage or role vs. running an existing client (G3 FAIL); (4) two consecutive months pass with zero commits. Archiving then counts as a *successful outcome* of this reassessment, per framework §20.

### What we deliberately will NOT build

> A token, a marketplace, a frontend, networking, a validator/authorer, a custom PVM, AI inference, PQC modules, or any README claim not backed by a green CI job. The words "Adaptive" and "Quantum-Resistant" leave the tagline; "production JAM node" is never used.

---

*Framework hard rule honored: no feature development resumed before this written decision. The next commit after this one should delete `coretime.rs` and open the `jam-core/codec` milestone.*
