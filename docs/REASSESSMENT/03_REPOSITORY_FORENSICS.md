# 03 — Repository Forensics

**Assessment date:** 2026-08-13
**Framework section:** 3 (Gate G1)
**Method:** direct measurement on a fresh clone at commit `b5b3d1b`, toolchain Rust 1.97.1 stable.

---

## 3.1 Inventory (measured)

| Metric | Value |
|---|---|
| Lines of Rust (src/) | 1,517 (schema.rs 641, importer.rs 352, coretime.rs 309, main.rs 114, state.rs 82, lib.rs 19) |
| Lines of Rust (tests + benches) | ~786 |
| Crates | 1 (single flat crate, no workspace) |
| Dependencies | 11 runtime + 4 dev. Notable: `proptest` is wrongly in `[dependencies]` (ships in release builds); no crypto beyond `blake2b_simd`; **no `polkavm`**, no ed25519/bandersnatch, no QUIC, no trie, no erasure coding |
| Unsafe Rust | 0 occurrences |
| TODO/FIXME in src | 2 |
| Test functions | 40 (`#[test]`/`#[rstest]`) |
| Test results (2026-08-13) | **1 failing** (`test_ticket_count_mismatch` — stale test contradicting the Phase 1.0 fix), 1 ignored, remainder pass. **Criterion benchmark panics** (`core_benchmarks` unwraps a failed re-import) |
| Build | ✅ Clean on stable 1.97.1, 14 s debug build, 2 warnings (unused variables in the stubbed `validate_extrinsic_hash`) |
| CI | 3 workflows (ci, coverage, docs); coverage badge wired to a gist |
| Documentation | 2,514 lines across 19 files in docs/src + README (310) + TODO (424) — **documentation exceeds source code ~2:1** |
| Contributors | 1 (author only) |
| Last meaningful commit | 2026-04-02 (revival burst); prior activity gap 2025-09 → 2026-04 |
| Test vectors | Submodules pinned to w3f v0.7.1 (Jan 2026) — but see §3.2: barely exercised |

## 3.2 Architecture map (spec subsystem → JamLiquor)

| JAM subsystem | JamLiquor status | Where |
|---|---|---|
| Serialization (JAM codec, binary) | **missing** — JSON-only parsing via serde | schema.rs |
| State | **stub** — slot counter + ticket counters, no service accounts, no trie | state.rs |
| Safrole / tickets | **partial/outdated** — envelope structs + length-based "signature validation", no ring VRF | schema.rs, importer.rs |
| Work packages / Refine | **missing** | — |
| Accumulate | **missing** | — |
| PVM | **missing** | — |
| Networking (QUIC) | **missing** | — |
| Storage / trie | **missing** | — |
| Services | **missing** | — |
| Cryptography | **stub** — Blake2b hashing only; ed25519/bandersnatch verification not implemented | schema.rs |
| Disputes | **missing** (schema field exists, `Null` in fixtures) | schema.rs |
| Availability / erasure | **missing** | — |
| CoreTime ledger | **obsolete/non-spec** — 309 LOC modeling a "CoreTime" accounting concept that does not correspond to a GP v0.7 STF subsystem | coretime.rs |

Roughly: **~20% of the shipped code (coretime.rs) models a non-spec concept; the rest is a JSON importer with structural checks.**

## 3.3 Code health score

| Category | Score /5 | Justification |
|---|---:|---|
| Buildability | 4 | Builds clean on current stable; 1 stale failing test, broken bench |
| Rust modernization | 3 | Edition 2021, deps compile in 2026; several outdated pins (env_logger 0.10, thiserror 1.x); proptest misplaced in runtime deps |
| Architecture clarity | 2 | Flat single crate; no codec/STF/crypto separation; coretime.rs is spec-orphaned; extension dirs (`extensions/`) referenced in docs don't exist |
| Test coverage | 2 | 40 tests but overwhelmingly JSON-parse and structural checks; 4 of ~1,350 official vector files touched; no binary codec, no STF, no negative-path vector tests |
| Specification alignment | 1 | Written against GP v0.6.7/early-0.7.0; own audit (AUDIT_MAPPING.md) confirms breaking changes across EpochMark, Preimage, Extrinsic; no PVM |
| Documentation | 4 | Extensive, well-organized, honest (Phase 0 audit is genuinely good); penalized one point for aspiration/reality gap (README markets AI/PQC/edge that don't exist in code) |
| Maintainability | 3 | Small, safe (0 unsafe), readable; but one bus-factor-1 author and misleading abstractions |
| **Average** | **2.71 / 5** | |

## 3.4 Gate G1 — Salvageability

> PASS if average ≥ 3.0/5 **and** no critical subsystem is fundamentally incompatible with the current JAM design.

**Verdict: FAIL** (2.71 < 3.0; specification alignment scores 1/5; the schema layer is JSON-shaped rather than codec-shaped, which is a structural mismatch with how conformance is actually tested — binary encoding first).

**Consequence per framework:** REWRITE or PIVOT. The salvageable assets are: the docs corpus, the CI scaffolding, the vector submodule wiring, the Phase 0 audit method, and the *domain knowledge encoded in the schema structs* — not the architecture itself.
