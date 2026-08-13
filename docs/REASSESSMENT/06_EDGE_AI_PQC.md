# 06 — Edge Feasibility, AI Thesis, PQC Reassessment

**Assessment date:** 2026-08-13
**Framework sections:** 6, 7, 8 (Gates G3, G4)

---

## 6.1 Edge hardware feasibility (Gate G3)

### Tier analysis

| Tier | Hardware class | Full validator? | Realistic JAM roles |
|---|---|---|---|
| A — Raspberry Pi-class (ARM64, 1–4 GB) | Pi 4/5, Zero 2 W (512 MB is Tier B/C boundary) | Marginal-to-plausible | STF verifier, light client, service worker, non-validating observer |
| B — Constrained SBC (256–512 MB, ARM/RISC-V) | VisionFive 2 class | No (erasure coding + PVM + availability buffers) | Block-import checker, light client, edge compute agent |
| C — Ultra-constrained (≤128 MB) | Milk-V Duo (64 MB), Ox64 | **No — treat as settled** | At most: header/Safrole verification, ticket checking, telemetry. The framework itself warns against assuming Tier C validators; concur. |

### Benchmark matrix — status

**Every cell is empty.** No binary-size, RAM, startup, or execution measurement has ever been recorded on any target. The edge thesis is exactly one year old and has produced zero numbers. The only honest statement: a debug build of the current importer compiles to a small binary because it does almost nothing.

### Gate G3 verdict

**UNPROVEN — cannot PASS or FAIL on evidence.** The framework requires "at least one meaningful JAM role operating on constrained hardware with objectively competitive resource requirements." Producing that evidence is precisely MVP-2/MVP-3 of the recommended program (framework §14) and *is the pivot's central experiment*. G3 becomes the project's primary open research question with a hard deadline (see DECISION.md stop conditions).

## 6.2 AI thesis (Gate G4)

Evaluating the three models from framework §7:

| Criterion /5 | A: JAM as AI execution layer | B: JAM as coordination layer | C: Edge inference marketplace |
|---|---:|---:|---:|
| Technical feasibility | 1 (PVM is gas-metered RV64 — hostile to tensor workloads) | 2 | 1 |
| JAM-native fit | 2 | 3 | 2 |
| Competitive differentiation | 1 | 2 | 1 |
| Economic viability | 1 | 1 | 1 |
| Research value | 2 | 3 | 2 |
| Security/verifiability | 1 (verifiable inference is an unsolved research field) | 2 | 1 |

All three models require extensive non-JAM infrastructure (model distribution, quantized runtimes, result attestation). Per the framework's rule, that classifies AI as an **application layer**, not a JamLiquor feature.

**Gate G4 verdict: FAIL.** Remove AI from the project's identity. Delete the pillar from README/Manifesto; keep at most one paragraph in a `docs/research-notes/` file as a future-work idea. This also retires "Adaptive" from the tagline.

## 6.3 PQC reassessment

- **JAM's actual primitives:** Bandersnatch ring VRF (tickets/seals), Ed25519 (judgments/guarantees), Blake2b, erasure coding. All fixed by the spec; a client using Dilithium signatures is by definition non-conformant and cannot interoperate.
- **Binary/RAM/latency cost:** Dilithium3 keys/signatures are KBs (vs 32/64 bytes); on 64–512 MB targets this directly attacks the project's one real differentiator (smallness).
- **Actual JAM use case:** none today. PQ migration will be a spec-level decision, not a client-level add-on.

**Verdict per framework §8 gate: FAIL for core inclusion.** Move to `experimental/pqc/` in spirit: no code exists, so this is a documentation change — strip PQC from the tagline and pillars; keep a research note. "Quantum-Resistant" leaves the identity.

## 6.4 Net effect on project identity

Of the original tagline — *"Lightweight. Adaptive. Quantum-Resistant. JAM-native."* — the reassessment retains **"Lightweight"** and **"JAM-native"** (as goals to be earned via G2/G3 evidence) and retires **"Adaptive"** (G4 FAIL) and **"Quantum-Resistant"** (§8 FAIL).
