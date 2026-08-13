# 02 — JAM 2026 Ecosystem Baseline

**Assessment date:** 2026-08-13
**Framework section:** 2 (Gate G0)
**Evidence base:** w3f/jamtestvectors submodule pinned in this repo (v0.7.1, commit `1dc503a`, Jan 2026), davxy/jam-test-vectors submodule, public reporting on the JAM Implementer's Prize and testnet (see Sources).

---

## 2.1 Where the protocol stands

- **Specification:** The Gray Paper progressed through v0.6.7 (June 2025, service-account/storage-key breaking changes), v0.7.0 (June 2025, full PVM execution pseudocode), v0.7.1 (July 2025), and a near-final pre-audit draft in early 2026, heading toward ratification of JAM 1.0 by the Polkadot Fellowship.
- **Target revision for any new work:** the version pinned by the official test vectors (currently v0.7.1 in `tests/vectors`, with 0.7.2 deviations noted in its changelog). Chasing the moving spec head is a losing game for a part-time project; chasing the vector pin is tractable.
- **Testnet:** a public JAM testnet launched January 2026. Mainnet requires a DOT governance referendum, expected H2 2026.

## 2.2 Protocol components (current architecture)

Serialization (JAM codec, not vanilla SCALE) · State & state Merkle trie · Safrole/tickets (Bandersnatch ring VRF) · Disputes/verdicts · Work packages · Refine · Accumulate · PVM (RISC-V RV64, gas-metered) · Availability/erasure coding · Networking (QUIC, grid diffusion) · Services model.

## 2.3 Conformance infrastructure

- **Official vectors:** `w3f/jamtestvectors` — codec, erasure, shuffle, trie, STF (safrole, disputes, history, authorizations, reports, assurances, preimages, accumulate, statistics), and full block-import traces. ~1,350 JSON vectors plus binary equivalents in the pin used by this repo.
- **Extended vectors & fuzzing:** `davxy/jam-test-vectors` and `davxy/jam-conformance` (fuzz reports against real clients).
- **Dashboards:** a public conformance/performance dashboard tracks client implementations.
- **M1 bar:** deterministic agreement on block-import from genesis, *including a working PVM with host calls*. PVM is not optional for M1.

## 2.4 Competitive field

- ~**43 teams** were competing across five language sets (Oct 2025); ~**15 M1 submissions** were with the Web3 Foundation by Jan 2026, with review/payment delays publicly reported by March 2026.
- Publicly listed clients include: **polkajam (Rust, Parity — fastest)**, **spacejam (Rust)**, **vinwolf (Rust)**, turbojam (C++), jamzig (Zig), strawberry (Go), jamzilla (Go), new-jamneration (Go), javajam (Java), typeberry/tsjam/pbnjam (TS), pyjamaz/tessera (Python), boka (Swift), and more. Multiple Rust clients reported 100% public-suite conformance during 2025.
- **Rust is the most crowded category.** "Another Rust JAM client" is, by itself, worth approximately nothing in 2026.

## 2.5 Hardware assumptions of existing clients

Existing implementations target workstation/server-class hardware; performance work (JIT/recompilers, JAM Toaster benchmarks) is oriented at validator-grade throughput. No known client publishes constrained-hardware (≤128 MB RAM, SBC-class) figures as a goal. Light-client and observer roles are comparatively under-explored across the ecosystem.

## 2.6 What changed since JamLiquor started

| Change | Impact on JamLiquor |
|---|---|
| GP v0.6.7 → v0.7.1+ breaking changes (service accounts, storage keys, variable-length item reordering, PVM serialization) | All schema structs written against v0.6.7 are suspect; the repo's own Phase 0 audit confirms this. |
| M1 requires PVM + host calls | JamLiquor has zero PVM code — the single largest gap. |
| Conformance suite became the acceptance criterion | JSON parsing of sample files no longer counts as progress; binary codec + STF vector agreement is the bar. |
| 15 teams already submitted M1 | The prize-track catch-up premise is gone. |
| Testnet live, mainnet vote pending | The ecosystem no longer needs *more full clients*; it arguably needs tooling, light clients, and diversity in deployment profiles. |

## 2.7 Gate G0 — Protocol relevance

> PASS only if JamLiquor can identify a meaningful technical contribution that remains relevant to the current JAM architecture.

**Verdict: PASS — narrowly, and only for a reframed contribution.**

- As a *competitive full client / prize entrant*: **FAIL.** That opportunity has closed.
- As a *resource-constrained, portable JAM implementation* (light roles: STF verification, block-import checking, observer/light-client experiments on ARM/RISC-V SBCs): **PASS.** The niche is real, unoccupied, aligned with JAM's decentralization narrative, and testable with the official vector suite. It survives only as a hypothesis to be validated (Gate G3), not as an assumption.

**Consequence per framework:** G0 passes for a *pivoted* thesis, not the original one. KILL is not mandated; PIVOT is on the table.

## Sources

- [Inside the JAM Implementers Prize — 43 Teams, 15 Submissions](https://polkadot.cloud/blog/en/polkadot-jam-implementers-prize)
- [JAM Conformance Performance dashboard](https://paritytech.github.io/jam-conformance-dashboard/)
- [graypaper.com/clients — JAM client list](https://graypaper.com/clients/)
- [w3f/jamtestvectors](https://github.com/w3f/jamtestvectors) · [davxy/jam-conformance](https://github.com/davxy/jam-conformance)
- [JAM Chain: Polkadot's Paradigm Shift (BlockEden, Oct 2025)](https://blockeden.xyz/blog/2025/10/28/jam-chain-polkadot-s-paradigm-shift-toward-the-decentralized-global-computer/)
- [Polkadot's JAM: Redefining Blockchain Architecture with RISC-V (BlockEden, Jan 2026)](https://blockeden.xyz/blog/2026/01/16/polkadot-jam-architecture-blockchain-virtual-machine-paradigm-shift/)
- [JAM Timeline Update Inquiry — Polkadot Forum](https://forum.polkadot.network/t/jam-timeline-update-inquiry/18300)
