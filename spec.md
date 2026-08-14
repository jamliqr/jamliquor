# JamLiquor — Product Contract (spec.md)

**Status:** Authoritative product contract, adopted 2026-08-14
**Derives from:** [DECISION.md](./DECISION.md) (2026-08-13 reassessment, outcome: PIVOT) and [docs/REASSESSMENT/](./docs/REASSESSMENT/README.md)
**Precedence:** spec.md > sub-specs > Plans.md. When this file and any other doc disagree, this file wins; fix the other doc.

---

## 1. What JamLiquor is

> A **minimal, independently implemented JAM core (codec + STF subset) that provably agrees with the official test vectors and is benchmarked on ARM64/RISC-V single-board computers.** A research / light-role runtime — not a validator, not a full client, not a JAM Prize entrant.

**Tagline:** *Lightweight. JAM-native.* — both words are goals to be **earned via evidence** (Gates G2/G3), not marketing. "Adaptive" and "Quantum-Resistant" are retired from the identity (Gates G4 and §8, FAIL).

**The central research question** (this is the project's reason to exist):

> What does JAM actually cost on constrained hardware — and which JAM roles fit on a ≤512 MB SBC?

Nobody in the ~43-team ecosystem has published an answer. Producing one honest, reproducible benchmark is the contribution.

## 2. What JamLiquor is NOT (binding non-goals)

Per DECISION.md, we deliberately will **not** build:

- A token, marketplace, or frontend
- Networking (QUIC), validator or block-authoring roles
- A custom PVM
- AI inference modules (application-layer idea; one research note only)
- PQC modules (spec-incompatible; one research note only)
- Any README claim not backed by a green CI job
- The phrase "production JAM node," anywhere

## 3. Conformance target and freeze policy

- **Authority:** `w3f/jamtestvectors` as pinned by the `tests/vectors` submodule — **v0.7.1, commit `1dc503a`**.
- **Freeze rule:** the pin is **frozen for the duration of MVP-1**. Gray Paper head movement (including JAM 1.0 ratification) does not move the target mid-milestone. Re-pinning is a deliberate act: it happens only *between* milestones, via a commit that updates the submodule and this section together.
- **Conformance-first rule (restated):** until the narrow conformance target is green, no edge tuning, no benchmarks, no new README claims.

## 4. Milestones and gates

### MVP-1 — narrow conformance (due **2026-11-15**)

Decode/encode round-trip of the official **binary codec vectors** (tiny + full) **plus** green **`stf/safrole`** vectors, running in CI. This is the gate to continue at all.

### MVP-2/3 — edge evidence (due **2027-02-15**)

The same binary cross-compiled and benchmarked on **one ARM64 SBC and one RISC-V SBC**: peak RAM, binary size, vectors/sec, startup time. This is the first and only test of Gate G3.

**G3 scope clarification** (adopted 2026-08-14): the framework's G3 wording requires a JAM "role" on constrained hardware. MVP-2/3 benchmarks a *workload* (codec + safrole STF over official vectors), which is a strict subset of a light-client role. G3 is therefore assessed as: **PASS if the measured workload runs within Tier-B budgets (≤512 MB peak RAM) with headroom that makes a header/seal-verifying light role plausible**, and the writeup must say exactly that — workload evidence, not role evidence. Claiming a deployable "role" requires a follow-on milestone, defined only if G3 passes.

### Time budget

**Mode B: 3–5 hours/week, hard-capped.** Mode C (10+ h/wk) is prohibited unless MVP-3 produces a measured, competitive result **and** an external contributor has landed a nontrivial PR.

## 5. Stop conditions (pre-committed)

KILL the pivot and archive the repo if **any** of:

1. Codec + safrole vectors not green by **2026-11-15**
2. No SBC benchmark exists by **2027-02-15**
3. Benchmarks show no meaningful resource advantage for any role vs. running an existing client (G3 FAIL)
4. Two consecutive months pass with zero commits

Condition (4) must be **machine-checked**, not self-monitored: a scheduled CI job that opens a tracking issue on inactivity (Plans.md task). Archiving counts as a *successful outcome* of the reassessment.

## 6. Architecture (Option B — clean rewrite of the core)

```
jam-core (new; no_std-friendly where practical)
├── codec    → w3f codec vectors (binary; tiny + full)
├── crypto   → ed25519 + Blake2b via audited crates; Bandersnatch ring VRF via ark-vrf/w3f crates
├── safrole  → stf/safrole vectors
└── trie     → trie vectors (after safrole)
```

- The existing `src/` importer is **legacy**: kept only until `jam-core` replaces its useful behavior, then removed. `coretime.rs` (spec-orphaned) is deleted immediately.
- Milestones must be **small, finishable, and self-contained** — work that stays true when paused for a month (G6 mitigation).
- Salvage from the old repo: docs corpus, CI scaffolding, vector submodule wiring, Phase 0 audit method, domain knowledge in the schema structs. Not the architecture.

## 7. Contributor path (G6)

G6 failed (bus factor 1, demonstrated 7-month stall). The named mechanism for attracting a second contributor is the **benchmark writeup**: MVP-3's numbers are published as a short technical report (repo + suitable venue), and the repo maintains a small set of curated good-first-issues from MVP-1 onward. No other community-building work is in scope.

## 8. Claims policy

Every capability claim in README or docs must map to a green CI job or a committed measurement artifact. Aspirations are labeled as hypotheses under test. The reassessment corpus (`docs/REASSESSMENT/`, `DECISION.md`) is immutable history — never edited to fit later narratives.
