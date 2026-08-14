# JamLiquor Plans.md

Created: 2026-08-14
Contract: [spec.md](./spec.md) (product contract) · [DECISION.md](./DECISION.md) (PIVOT decision)
Supersedes: `TODO.md` (2026-04 revival plan — Phases 2–7 retired by the PIVOT decision; archived under `docs/archive/`)

Markers: `cc:todo` → `cc:wip` → `cc:done` (`blocked` requires a stated reason).

---

## Phase 1: Reassessment propagation (docs only — no code)

Goal: every public-facing doc reflects the pivoted identity. No doc may claim what CI cannot prove.

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 1.1 | Create root `spec.md` product contract encoding the pivoted thesis, vector-pin freeze policy, G3 scope clarification, stop conditions [tdd:skip:docs-only] | `spec.md` exists at repo root and covers identity, non-goals, freeze rule, MVP dates, stop conditions | - | cc:done |
| 1.2 | Rewrite `README.md` to the pivoted identity: tagline "Lightweight. JAM-native.", remove AI/PQC/prize/edge-hardware claims not backed by CI, honest status table (legacy importer + jam-core plan), fix broken links (`MANIFESTO.md`, `docs/cleanroom.md`) [tdd:skip:docs-only] | README contains no retired-pillar claims; all relative links resolve; status table matches spec.md §6 | 1.1 | cc:done |
| 1.3 | Consolidate AI + PQC into a single research note `docs/src/research-notes.md` (one paragraph each, per DECISION); remove `AI.md` and `PQC.md` from nav [tdd:skip:docs-only] | `AI.md`/`PQC.md` deleted or archived; research note ≤1 page; SUMMARY.md/index.md no longer list them as pillars | 1.1 | cc:done |
| 1.4 | Rework `MANIFESTO.md`: archive the five-pillar original to `docs/archive/MANIFESTO-2025.md` unchanged; replace live doc with the pivoted two-value statement referencing spec.md [tdd:skip:docs-only] | Original preserved verbatim in archive; live manifesto claims only Lightweight + JAM-native as hypotheses | 1.1 | cc:done |
| 1.5 | Replace `MILESTONE.md` content with the MVP-1/2/3 roadmap and dates from spec.md §4; drop Authorer/PolkaVM/Extensions milestones [tdd:skip:docs-only] | MILESTONE.md lists exactly MVP-1, MVP-2/3, and stop conditions; no retired milestones remain | 1.1 | cc:done |
| 1.6 | Supersede `TODO.md`: move to `docs/archive/TODO-2026-04.md` with a header banner pointing to Plans.md; leave a stub `TODO.md` pointer or remove per repo convention [tdd:skip:docs-only] | Root no longer presents the 2026-04 plan as current; archive copy intact; Plans.md referenced | 1.1 | cc:done |
| 1.7 | Alignment pass over remaining docs (`SUMMARY.md`, `index.md`, `ARCHITECTURE.md`, `EDGE.md`, `NETWORK.md`, `DECENTRALIZATION.md`, `SIMPLICITY.md`, `CONFORMATION.md`, `building.md`): update nav, add status banners to aspirational content, remove five-pillar framing [tdd:skip:docs-only] | mdBook nav builds without dead links; no doc presents AI/PQC/networking as current scope | 1.2, 1.3, 1.4, 1.5 | cc:done |
| 1.8 | Commit Phase 1 as a single reviewed docs commit | `git log` shows one docs commit; CI green; no `src/` changes included | 1.2–1.7 | cc:wip |

## Phase 2: Legacy hygiene + jam-core scaffold

Goal: repo structure matches spec.md §6 before any conformance code is written.

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 2.1 | Delete `src/coretime.rs` and its tests/benches; remove the stale failing `test_ticket_count_mismatch`; fix broken `core_benchmarks` bench or delete it | `cargo test --all-targets` fully green; no coretime references remain | Phase 1 | cc:todo |
| 2.2 | Cargo hygiene: move `proptest` to `[dev-dependencies]`; update stale pins (`env_logger`, `thiserror`); fix 2 build warnings | `cargo build --release` zero warnings; proptest absent from release dep tree (`cargo tree`) | 2.1 | cc:todo |
| 2.3 | Scaffold workspace: `jam-core` crate (modules `codec`, `crypto`, `safrole` — empty but compiling), legacy importer kept as-is; CI runs both | Workspace builds; CI matrix includes jam-core; lint/fmt gates apply to new crate | 2.1 | cc:todo |
| 2.4 | Stop-condition automation: scheduled GitHub Actions job (monthly) that opens/updates a tracking issue if no commit landed in 60 days | Workflow merged; dry-run verified via `workflow_dispatch` | 2.3 | cc:todo |
| 2.5 | Curate 3–5 good-first-issues from Phase 3 codec primitives (G6/contributor path, spec.md §7) | Issues exist, labeled, each with pointers to vectors + DoD | 2.3 | cc:todo |

## Phase 3: MVP-1a — binary codec conformance

Goal: decode/encode round-trip of official binary codec vectors, tiny + full, in CI. Vector pin frozen at v0.7.1 `1dc503a` (spec.md §3).

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 3.1 | JAM codec primitives: variable-length integers, sequences, discriminators, option/fixed-length encodings per GP v0.7.1 appendix [tdd:required] | Property tests + first official codec vectors pass for primitives | Phase 2 | cc:todo |
| 3.2 | Header + extrinsic structs (v0.7.1 shapes) with binary decode/encode round-trip against `codec/tiny` vectors [tdd:required] | All tiny codec vectors round-trip byte-exact in CI | 3.1 | cc:todo |
| 3.3 | Full-config codec vectors green; `cargo test conformance` job in CI as a required check [tdd:required] | 100% of pinned `codec/` vectors (tiny + full, .bin) pass in CI | 3.2 | cc:todo |

## Phase 4: MVP-1b — safrole STF conformance (gate: 2026-11-15)

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 4.1 | Crypto foundation: ed25519 verify (audited crate), Blake2b-256, Bandersnatch ring VRF via ark-vrf/w3f crates — verification only [tdd:required] | Crypto unit vectors pass; no hand-rolled primitives (`cargo deny`/review check) | Phase 3 | cc:todo |
| 4.2 | Safrole state + transition: tickets, epoch marks, fallback selection, Fisher-Yates shuffle per GP v0.7.1 §6 [tdd:required] | `stf/safrole` tiny vectors pass | 4.1 | cc:todo |
| 4.3 | Full `stf/safrole` vector suite green in CI — **MVP-1 complete** | 100% of pinned `stf/safrole` vectors pass in required CI check by 2026-11-15 | 4.2 | cc:todo |

## Phase 5: MVP-2/3 — edge benchmarks (gate G3: 2027-02-15)

Only unlocked by MVP-1 green (conformance-first rule, spec.md §3).

| Task | Description | DoD | Depends | Status |
|------|-------------|-----|---------|--------|
| 5.1 | Cross-compilation: `aarch64-unknown-linux-gnu` + `riscv64gc-unknown-linux-gnu` release builds in CI (add riscv64 to rust-toolchain.toml) | Both targets build in CI; binary size recorded as artifact | Phase 4 | cc:todo |
| 5.2 | Benchmark harness: vectors/sec, peak RSS, startup time, binary size — reproducible script + machine-readable output [tdd:required] | Harness runs on x86_64 in CI producing committed baseline JSON | 5.1 | cc:todo |
| 5.3 | Hardware runs: one ARM64 SBC + one RISC-V SBC; publish measurements + G3 assessment per spec.md §4 wording (workload evidence, Tier-B budget) | Numbers committed for both boards by 2027-02-15; G3 verdict written against spec.md criteria | 5.2 | cc:todo |
| 5.4 | Benchmark writeup (technical report) — the contributor-attraction mechanism (spec.md §7) | Report in repo; shared to at least one JAM community venue | 5.3 | cc:todo |
