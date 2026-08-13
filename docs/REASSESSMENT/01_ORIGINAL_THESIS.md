# 01 — Original Thesis Reconstruction

**Assessment date:** 2026-08-13
**Framework section:** 1
**Evidence base:** README.md, docs/src/MANIFESTO.md, docs/src/MILESTONE.md, TODO.md, git history (2025-04-16 → 2026-04-02, 61 commits, 1 contributor)

---

## 1.1 What JamLiquor set out to be

From the README and Manifesto, the original thesis (April 2025) was:

> A cleanroom JAM Protocol implementation, engineered from scratch in Rust, that (a) conforms to JAM's execution model, (b) runs on edge hardware like Milk-V Duo (64 MB RAM) and Raspberry Pi Zero 2 W, and (c) supports modular innovation including AI inference and post-quantum cryptography.

Answers to the framework's questions:

| Question | Reconstructed answer |
|---|---|
| Problem to solve | JAM node software was assumed to be datacenter-class; nobody was building for constrained/community hardware. "Real decentralization" via community-run edge nodes. |
| Intended user | Hobbyist/community node operators on SBCs; secondarily the JAM Prize audit process (cleanroom provenance is called out explicitly). |
| Why a new implementation | JAM Implementer's Prize rewards independent cleanroom clients; forks are ineligible. Implementation diversity is a protocol goal. |
| Why Rust | Memory safety without GC, first-class cross-compilation to RISC-V/ARM, the dominant systems language of the Polkadot ecosystem. |
| Why edge hardware | Differentiation ("no validator elitism") and the belief that JAM's 6-second slots could fit in ≤128 MB RAM / ≤1 GHz CPU. |
| Why cleanroom | JAM Prize eligibility requires demonstrable independent authorship. |
| "Lightweight" quantitatively | ≤128 MB RAM, ≤1 GHz CPU (README "Core Principles"); later refined in TODO.md to "fit in ~48 MB on a 64 MB Milk-V Duo". |
| Role of AI | "Adaptability with AI" — TinyML inference modules letting the node adapt in real time. Never specified concretely; TODO.md (2026-04) itself demoted it to "most speculative component, lowest priority". |
| Role of PQC | Opt-in Dilithium/Kyber/XMSS modules — "Quantum-Resistance First". Never implemented. |

## 1.2 Founding assumptions, then vs. now

| Assumption (2025) | Still true in 2026? |
|---|---|
| The JAM spec is early; an implementer starting now can reach conformance alongside the field | **No.** ~43 teams competed; ~15 submitted M1 by Jan 2026; multiple Rust clients hit 100% conformance on the public suite in 2025. The catch-up window for the prize track has effectively closed. |
| Implementation diversity is valued and rewarded | **Partly.** Diversity is still a protocol goal, but the prize pipeline is congested (M1 reviews delayed; some teams paused awaiting payment). New entrants face a mature bar: PVM, full STF, networking. |
| Nobody targets constrained/edge hardware | **Still true.** No known client publishes 64–128 MB RAM footprints or RISC-V SBC benchmarks as a headline goal. The niche is unoccupied — but also unvalidated. |
| A full validator can run in ≤128 MB | **Doubtful.** M1 already requires a functional PVM plus Bandersnatch ring-VRF crypto and erasure coding; no measurement in this repo supports the 128 MB claim. Lighter roles (light client, observer, service worker) remain plausible. |
| AI inference is a natural node feature | **No.** One year of development produced zero AI code; the project's own TODO reclassified it as speculative. It is an application-layer idea, not a client feature. |
| PQC is a differentiator | **Weak.** JAM's protocol crypto (Bandersnatch, Ed25519) is fixed by the spec; non-conformant signature schemes cannot interoperate. PQC only makes sense as an off-spec experiment. |

## 1.3 What the thesis quietly became

The repo's own trajectory tells the story: the Manifesto promises a five-pillar vision (lightweight, decentralized, AI, PQC, simplicity), but 100% of the shipped code is a JSON block importer with partial SAFROLE structures. The April 2026 revival plan (TODO.md) already narrowed the thesis to "pass M1, then differentiate on edge hardware" — implicitly conceding that AI and PQC were marketing, not engineering.

**Conclusion:** The original thesis has one surviving, testable kernel — *a small, portable JAM implementation for constrained hardware* — wrapped in two layers (AI, PQC) that the project itself has already abandoned in practice, and one strategic goal (JAM Prize) that the ecosystem has outrun.
