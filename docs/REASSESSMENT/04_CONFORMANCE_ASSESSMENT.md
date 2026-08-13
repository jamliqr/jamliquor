# 04 — JAM Conformance Assessment

**Assessment date:** 2026-08-13
**Framework section:** 4 (Gate G2)

---

## 4.1 Capability matrix

Status: GREEN = current and tested · YELLOW = partial/outdated · RED = missing/broken · N/A = out of scope

| JAM subsystem | Status | Evidence | Gap | Effort (est.) |
|---|---|---|---|---|
| Serialization | **RED** | JSON-only via serde; official suite is binary-first (`.bin` files, 1,349 in pin) | Entire JAM codec (discriminators, variable-length ordering per v0.7.0) | 3–5 wks |
| State transition | **RED** | `State::apply_block` checks slot monotonicity + counters only | Full STF for every subsystem | 3–6 mo |
| Block processing | **YELLOW** | `test_importer_with_official_vectors` passes on 1 JSON block; extrinsic-hash validation stubbed out | Real validation pipeline, binary decode, state-root check | 4–8 wks |
| Cryptography | **RED** | Blake2b hashing only; "signature validation" = length check (64 vs 784 bytes) | ed25519 verify, Bandersnatch ring VRF, erasure coding | 4–8 wks (using ark-vrf/w3f crates) |
| Ticket/Safrole logic | **YELLOW** | Structs + counters exist; v0.8-era semantics patched in Phase 1.0 | Ring VRF verify, epoch transitions, fallback selection, `stf/safrole` vectors | 4–6 wks |
| Work packages | **RED** | None | All | — |
| Refine | **RED** | None | All (requires PVM) | — |
| Accumulate | **RED** | None | All (requires PVM) | — |
| PVM | **RED** | Zero code, no `polkavm` dependency | Interpreter or crate integration + host calls + gas | 6–12 wks |
| Service model | **RED** | None (service accounts absent from schema) | All | — |
| Networking | **N/A** (deliberately deferred) | None | QUIC/quinn, discovery, diffusion | — |
| Availability/data | **RED** | None | Erasure coding, assurances | — |
| Storage | **RED** | In-memory structs only; no Merkle trie | State trie + root computation (`trie/` vectors) | 2–4 wks |
| Conformance vectors | **RED** | 4 JSON files of ~1,350 exercised (~0.3%); traces/STF/erasure/shuffle/trie untouched | Everything | — |

## 4.2 Honest summary

By the framework's own rule — *"Do not call JamLiquor a JAM implementation merely because it contains JAM-related code"* — **JamLiquor is not currently a JAM implementation.** It is a JSON block-fixture importer with partial Safrole data structures and a spec-orphaned CoreTime module.

Total distance to M1-grade conformance is realistically **9–18 months of full-time-equivalent work** (teams that passed had 1–4 people working ~a year). At the stated sustainable budget of 3–5 h/week, M1-grade conformance is a **multi-year** proposition against a spec that will have been ratified, audited, and shipped by then.

## 4.3 Gate G2 — Conformance

> PASS only when JamLiquor demonstrates deterministic agreement with authoritative test vectors for the supported scope.

**Verdict: FAIL today — but the gate is re-scopable.** The framework allows a "supported scope". A deliberately narrow scope — **binary codec + one STF subsystem (safrole) against `w3f/jamtestvectors`** — is achievable in ~3–4 months at Mode B hours, and would be the first moment JamLiquor can truthfully claim any conformance at all.

**Consequence per framework:** With the predefined effort budget exceeded for full-client conformance, the options are REWRITE (scoped) or KILL. Combined with G1's failure, this points at **rewrite-the-core-under-a-pivoted-thesis**, not incremental repair of the current importer.

## 4.4 Conformance-first rule (restated as policy)

Per framework §4.2, until the narrow conformance target is green: **no AI layer, no edge optimization work, no PQC, no productization, no README claims.** The first PR that matters is the one that makes `cargo test conformance` decode official binary vectors.
