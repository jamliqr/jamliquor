# 05 — Differentiation & Competitive Landscape

**Assessment date:** 2026-08-13
**Framework sections:** 5, 9, 10 (Gates G5, G6)

---

## 5.1 Differentiator scoring

Scores reflect *current, evidenced* state — not aspiration. (Potential noted in parentheses.)

| Differentiator | Score /5 | Notes |
|---|---:|---|
| Minimal resource consumption | 1 (potential 4) | Zero measurements exist; the claim is unproven |
| ARM support | 1 (3) | rust-toolchain lists aarch64 targets; nothing tested on hardware |
| RISC-V support | 1 (4) | Named boards in README; no riscv target even in rust-toolchain.toml |
| Embedded/edge deployment | 1 (4) | The one genuinely unoccupied niche in the ecosystem |
| Fast startup | 0 | Nothing to start |
| Low memory footprint | 1 (3) | Untested |
| Easy self-hosting | 1 | No node exists to host |
| Security isolation | 1 | No PVM/sandbox |
| AI workload integration | 0 (1) | Zero code; see 06 |
| Research value | 2 (4) | The edge-benchmark question ("what does JAM actually cost on a 64 MB SBC?") is publishable and nobody has answered it |
| Developer experience | 2 | Good docs habit; tiny codebase is approachable |
| Independent implementation diversity | 2 | Cleanroom history is real and continuous; but Rust diversity is saturated |
| Educational value | 3 | A ~2k-LOC readable repo with honest audit docs is a decent JAM learning artifact |

## 5.2 Competitive landscape (2026)

| Project | Language | Conformance | PVM | Edge focus | Activity |
|---|---|---|---|---|---|
| **JamLiquor** | Rust | ~0% (4 JSON files parse) | none | claimed, unmeasured | 1 author, bursty |
| polkajam (Parity) | Rust | 100% public suite; fastest (recompiler) | custom JIT | no | high |
| spacejam | Rust | 100% public suite (2025) | yes | no | high |
| vinwolf | Rust | 100% public suite (2025) | yes | no | high |
| turbojam | C++ | high | yes | no | high |
| jamzig | Zig | high | yes | no | high |
| strawberry / jamzilla / new-jamneration | Go | M1-track | yes | no | high |
| typeberry / tsjam / pbnjam | TS | M1-track | yes | no | medium |
| pyjamaz / tessera | Python | partial | yes | no | medium |

(Ecosystem counts: ~43 teams, ~15 M1 submissions by Jan 2026. Sources in 02_JAM_2026_BASELINE.md.)

### The critical question

> Is JamLiquor differentiated enough that another contributor would choose to work on it?

**As "a Rust JAM client": no.** Three Rust clients already pass the public suite; one is Parity's own and the performance leader. No rational contributor picks the 4th-place Rust client at 0% conformance.

**As "the JAM-on-constrained-hardware project": possibly.** Nobody owns that story. But the story attracts contributors only once there is one honest, reproducible benchmark on real hardware — which requires the minimal conformant core first.

## 5.3 Gate G5 — Ecosystem differentiation

**Verdict: FAIL as currently positioned; CONDITIONAL PASS under the pivoted thesis** (edge/light-role JAM runtime + published measurements). The condition is evidence, not branding.

## 5.4 Contributor & ecosystem potential (framework §10)

| Factor | Score /5 |
|---|---:|
| Clear onboarding | 3 (README quick-start works) |
| Documentation | 4 |
| Modular architecture | 2 (single flat crate) |
| Good first issues | 1 (issue tracker not curated) |
| Reproducible development | 3 (submodules pinned, CI exists; bench broken) |
| Test infrastructure | 2 |
| Contributor friendliness | 2 |
| Research visibility | 1 |
| Academic collaboration potential | 2 (edge-benchmark angle could change this) |

## 5.5 Gate G6 — Survives low founder attention

**Verdict: FAIL.** 61/61 commits by one author; a 7-month total stall (2025-09 → 2026-04) demonstrated exactly what happens when founder attention drops: everything stops and the spec moves on. The architecture decision (07/DECISION) must therefore optimize for *small, finishable, self-contained milestones* — work that stays true when abandoned for a month — rather than a long-lived integration branch.
