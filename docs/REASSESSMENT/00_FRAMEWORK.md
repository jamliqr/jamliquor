# JamLiquor 2026 Reassessment Framework

**Project:** JamLiquor  
**Repository:** https://github.com/jamliqr/jamliquor  
**Assessment date:** 2026-08-13  
**Purpose:** Decide whether JamLiquor should be **KEEP, KILL, REWRITE, or PIVOT** before investing significant development time.

---

## 0. Executive Decision

### Primary question

> **Does JamLiquor have a technically credible and strategically differentiated reason to exist in the 2026 JAM ecosystem, given its original goal of a lightweight Rust implementation suitable for constrained/edge hardware?**

### Allowed outcomes

| Decision | Meaning |
|---|---|
| **KEEP** | Continue the current architecture with targeted modernization |
| **REWRITE** | Preserve the project thesis but substantially replace outdated implementation |
| **PIVOT** | Preserve useful IP but change the primary product/research thesis |
| **KILL** | Archive the project because the opportunity is no longer compelling |
| **HOLD** | Do not develop yet; maintain a research/watch status until a defined trigger occurs |

### Hard rule

Do **not** resume feature development until Sections 1–8 have produced a written decision.

---

# 1. Reconstruct the Original Thesis

Before looking at code, document what JamLiquor was originally trying to achieve.

## 1.1 Original thesis

Fill in:

- What problem was JamLiquor intended to solve?
- Who was the intended user?
- Why was a new implementation necessary?
- Why Rust?
- Why edge hardware?
- Why a clean-room implementation?
- What did "lightweight" mean quantitatively?
- What role were AI inference and PQC supposed to play?
- What assumptions were true when the project started?
- Which assumptions are still true in 2026?

### Deliverable

Create:

`docs/REASSESSMENT/01_ORIGINAL_THESIS.md`

---

# 2. 2026 Ecosystem Reality Check

JAM has evolved substantially from its early specification.

The current Polkadot documentation describes JAM as a prospective successor to the relay chain and as a distributed computational model based around services, work packages, refine/accumulate processing, and permissionless execution.

Therefore, **do not assess JamLiquor against an old understanding of JAM.**

Use the current authoritative JAM specification and ecosystem material as the baseline.

### Questions

- What is the current JAM protocol specification?
- Which specification revision should JamLiquor target?
- What are the current protocol components?
- What are the current conformance requirements?
- What implementations already exist?
- What test vectors/conformance infrastructure exists?
- What hardware assumptions do current implementations make?
- What parts of the protocol have changed since JamLiquor was started?
- Is there still room for an independent implementation?

### Deliverable

`docs/REASSESSMENT/02_JAM_2026_BASELINE.md`

### Gate

**G0 — Protocol relevance**

PASS only if:

> JamLiquor can identify a meaningful technical contribution that remains relevant to the current JAM architecture.

If FAIL → **KILL or PIVOT.**

---

# 3. Repository Forensics

Treat the existing repository as archaeological evidence.

Do not assume the old architecture is still correct.

## 3.1 Inventory

Measure:

- Lines of Rust
- Number of crates/modules
- Dependencies
- Unsafe Rust
- TODO/FIXME count
- Test count
- Integration tests
- CI status
- Build reproducibility
- Documentation coverage
- Last meaningful commit
- Current compiler/toolchain compatibility
- Current dependency compatibility

## 3.2 Architecture mapping

Create a map:

```text
JAM specification
       |
       +-- State
       +-- Block
       +-- Safrole / tickets
       +-- Work packages
       +-- Refine
       +-- Accumulate
       +-- PVM
       +-- Networking
       +-- Storage
       +-- Services
       +-- Cryptography
       |
       v
JamLiquor
       |
       +-- implemented
       +-- partial
       +-- stub
       +-- obsolete
       +-- missing
```

## 3.3 Code health score

Score each category 0–5:

| Category | Score |
|---|---:|
| Buildability | /5 |
| Rust modernization | /5 |
| Architecture clarity | /5 |
| Test coverage | /5 |
| Specification alignment | /5 |
| Documentation | /5 |
| Maintainability | /5 |

### Gate

**G1 — Salvageability**

PASS if:

- average ≥ 3.0/5, and
- no critical subsystem is fundamentally incompatible with the current JAM design.

Otherwise:

**REWRITE or PIVOT.**

---

# 4. JAM Conformance Assessment

This is the most important technical gate.

Do **not** call JamLiquor a JAM implementation merely because it contains JAM-related code.

## 4.1 Capability matrix

Create:

| JAM subsystem | Status | Evidence | Gap | Effort |
|---|---|---|---|---|
| Serialization | | | | |
| State transition | | | | |
| Block processing | | | | |
| Cryptography | | | | |
| Ticket/Safrole logic | | | | |
| Work packages | | | | |
| Refine | | | | |
| Accumulate | | | | |
| PVM | | | | |
| Service model | | | | |
| Networking | | | | |
| Availability/data handling | | | | |
| Storage | | | | |
| Conformance vectors | | | | |

Status values:

- `GREEN` = current and tested
- `YELLOW` = partial/outdated
- `RED` = missing/broken
- `N/A` = intentionally out of scope

## 4.2 Conformance-first rule

The first technical objective is:

> **Pass protocol/state-transition conformance tests before adding novel features.**

No AI layer.
No edge optimization.
No PQC.
No productization.

Until the protocol core is credible.

### Gate

**G2 — JAM Conformance**

PASS only when JamLiquor can demonstrate deterministic agreement with authoritative test vectors for the supported scope.

If FAIL after a predefined effort budget → **REWRITE/KILL.**

---

# 5. Differentiation Analysis

A technically correct JAM client is not automatically valuable.

Ask:

> **Why should anyone care about JamLiquor instead of using another implementation?**

Score each possible differentiator 0–5.

| Differentiator | Score |
|---|---:|
| Minimal resource consumption | /5 |
| ARM support | /5 |
| RISC-V support | /5 |
| Embedded/edge deployment | /5 |
| Fast startup | /5 |
| Low memory footprint | /5 |
| Easy self-hosting | /5 |
| Security isolation | /5 |
| AI workload integration | /5 |
| Research value | /5 |
| Developer experience | /5 |
| Independent implementation diversity | /5 |
| Educational value | /5 |

### Required hypothesis

The strongest candidate thesis is:

> **JamLiquor is a resource-constrained, portable JAM implementation optimized for edge/embedded hardware.**

This must be validated rather than assumed.

---

# 6. Edge Hardware Feasibility

This was one of the original ideas and should be tested quantitatively.

## 6.1 Target tiers

### Tier A — Raspberry Pi-class

Example:

- ARM64
- 1–4 GB RAM
- Linux
- SSD/SD storage

### Tier B — constrained SBC

Example:

- 256–512 MB RAM
- ARM/RISC-V
- limited storage
- lower CPU

### Tier C — ultra-constrained

Example:

- ≤128 MB RAM
- ≤1 GHz class CPU
- limited persistent storage

Do not assume Tier C is realistic for a full validator.

It may instead be appropriate for:

- light clients
- observers
- service workers
- edge compute agents
- specialized execution nodes

## 6.2 Benchmark matrix

Measure:

| Metric | Target |
|---|---:|
| Binary size | |
| Peak RAM | |
| Idle RAM | |
| CPU utilization | |
| Startup time | |
| State-sync time | |
| Disk usage | |
| Work-package execution time | |
| Energy consumption | |
| Network bandwidth | |

### Gate

**G3 — Edge viability**

PASS if at least one meaningful JAM role can operate on constrained hardware with objectively competitive resource requirements.

---

# 7. AI Thesis Reassessment

The original project included AI inference.

Do not automatically retain this.

Ask:

> **Does JAM actually create a meaningful advantage for decentralized AI inference?**

Evaluate three models.

### Model A — JAM as AI execution layer

```text
AI workload
    |
    v
JAM service
    |
    v
distributed execution
```

### Model B — JAM as coordination layer

```text
AI model
   |
Edge nodes
   |
JAM
   |
task allocation / verification / settlement
```

### Model C — JAM + edge inference marketplace

```text
Model owner
     |
     v
JAM services
     |
     +---- Edge node A
     +---- Edge node B
     +---- Edge node C
     |
     v
Verified inference result
```

Score:

| Criterion | /5 |
|---|---:|
| Technical feasibility | |
| JAM-native fit | |
| Competitive differentiation | |
| Economic viability | |
| Research value | |
| Security/verifiability | |

### Rule

If AI requires extensive non-JAM infrastructure to become useful, classify it as an **application layer**, not a core JamLiquor feature.

---

# 8. PQC Reassessment

Post-quantum cryptography should be treated as an experimental extension, not a reason to distort the core implementation.

Evaluate:

- Which cryptographic primitives are currently used by JAM?
- Which can realistically be replaced or augmented?
- Does PQC increase binary size?
- Does it increase RAM requirements?
- Does it increase latency?
- Does it create interoperability problems?
- Is there an actual JAM use case?

### Gate

PQC remains only if:

> It produces measurable security/research value without compromising JAM interoperability.

Otherwise move it to:

`experimental/pqc/`

---

# 9. Competitive Landscape

Build a 2026 comparison.

Include:

- major JAM implementations
- official/reference implementations
- alternative Rust implementations
- research implementations
- lightweight clients
- embedded/edge-oriented projects

Compare:

| Project | Language | Conformance | Networking | PVM | Edge focus | License | Activity |
|---|---|---|---|---|---|---|---|
| JamLiquor | Rust | | | | | | |
| Implementation B | | | | | | | |
| Implementation C | | | | | | | |

### Critical question

> Is JamLiquor differentiated enough that another contributor would choose to work on it?

---

# 10. Contributor & Ecosystem Potential

A project that depends entirely on you is incompatible with your current career constraints.

Score:

| Factor | /5 |
|---|---:|
| Clear onboarding | |
| Good documentation | |
| Modular architecture | |
| Good first issues | |
| Reproducible development | |
| Test infrastructure | |
| Contributor friendliness | |
| Research visibility | |
| Potential academic collaboration | |

### Personal constraint

Assume:

> **You have limited weekly development time because medicine remains your primary professional focus.**

Therefore the architecture must be capable of surviving periods where you contribute little or nothing.

---

# 11. Time-Budget Reality Check

Evaluate three operating models.

## Mode A — Research hobby

**1–2 hours/week**

Goal:

- maintain repository
- read JAM developments
- occasional experiments

## Mode B — Serious side project

**3–5 hours/week**

Goal:

- conformance
- edge prototype
- documentation
- periodic public releases

## Mode C — Startup/research initiative

**10+ hours/week**

Goal:

- complete implementation
- ecosystem participation
- hardware deployments
- external contributors
- commercialization/research funding

### Rule

Do not choose Mode C unless there is evidence from Gates G0–G5 that justifies it.

---

# 12. Architecture Decision

After Sections 1–11, choose one architecture.

## Option A — Incremental modernization

```text
Existing JamLiquor
       |
       +-- update specification
       +-- repair implementation
       +-- conformance
       +-- optimize
       +-- edge
```

Use only if the existing architecture remains fundamentally sound.

## Option B — Clean rewrite

```text
JAM specification
       |
       v
minimal Rust core
       |
       +-- PVM
       +-- state transition
       +-- conformance
       |
       v
networking
       |
       v
edge optimization
```

Use if the original code is useful conceptually but structurally obsolete.

## Option C — Split project

```text
JamLiquor
   |
   +-- jam-core
   |
   +-- jam-node
   |
   +-- jam-edge
   |
   +-- jam-ai
   |
   +-- jam-pqc
```

This is the preferred architecture **if** the technical assessment supports it.

The core protocol must remain independent of experimental features.

---

# 13. Hard Go/No-Go Gates

| Gate | Question | PASS condition |
|---|---|---|
| G0 | Is JAM still strategically relevant? | Clear opportunity |
| G1 | Is old code salvageable? | ≥3/5 health |
| G2 | Can we achieve conformance? | Test-vector agreement |
| G3 | Is edge differentiation real? | Measured advantage |
| G4 | Is AI technically justified? | JAM-native value |
| G5 | Is there ecosystem differentiation? | Clear niche |
| G6 | Can it survive low founder attention? | Contributor-ready |
| G7 | Is the time investment rational? | Positive expected value |

### Final decision

#### KEEP

If G0, G2, G3 and G5 pass.

#### REWRITE

If G0/G3/G5 pass but G1 fails.

#### PIVOT

If G0 passes but the original architecture/use case fails while a new thesis scores strongly.

#### HOLD

If technical potential is high but the ecosystem is not sufficiently mature.

#### KILL

If:

- JAM relevance is weak,
- conformance is impractical,
- differentiation is weak,
- and no credible pivot exists.

---

# 14. Recommended 2026 MVP

Do **not** start with a full JAM node.

The first MVP should answer one question:

> **Can JamLiquor become a genuinely lightweight, conformant JAM implementation?**

### MVP-1

```text
Rust
 |
 +-- canonical serialization
 +-- state transition
 +-- cryptography
 +-- PVM
 +-- conformance vectors
 |
 v
Deterministic JAM execution
```

### MVP-2

Run the same implementation on:

```text
x86_64
ARM64
RISC-V
```

### MVP-3

Benchmark:

```text
Desktop
    vs
Raspberry Pi
    vs
constrained SBC
```

Only after these work should edge AI become a serious development track.

---

# 15. Proposed 90-Day Reassessment Program

## Phase 1 — Days 1–14: Archaeology

Deliver:

- repository inventory
- architecture diagram
- dependency audit
- original thesis
- current build status

**Gate:** G1

---

## Phase 2 — Days 15–30: JAM Mapping

Deliver:

- current JAM specification map
- protocol/component matrix
- implementation gap analysis
- competing implementations

**Gate:** G0 + preliminary G2

---

## Phase 3 — Days 31–60: Minimal Conformance Prototype

Goal:

> Make one narrow, deterministic part of JamLiquor agree with authoritative JAM test vectors.

Deliver:

- test harness
- reproducible fixtures
- CI
- benchmark baseline

**Gate:** G2

---

## Phase 4 — Days 61–75: Edge Experiment

Run the smallest meaningful implementation on:

- ARM64 SBC
- optionally RISC-V

Measure:

- RAM
- CPU
- startup
- storage
- execution performance

**Gate:** G3

---

## Phase 5 — Days 76–90: Strategic Decision

Evaluate:

- JAM conformance
- edge differentiation
- AI opportunity
- PQC opportunity
- ecosystem competition
- contributor potential
- personal time constraints

Produce:

`DECISION.md`

with exactly one:

```text
KEEP
REWRITE
PIVOT
HOLD
KILL
```

---

# 16. Success Metrics

Do not measure success by GitHub stars initially.

### Technical

- % of supported conformance vectors passing
- deterministic state-transition agreement
- RAM usage
- CPU usage
- binary size
- startup time
- synchronization performance

### Ecosystem

- external contributors
- issues opened by others
- PRs from others
- implementation interoperability
- JAM community feedback

### Research

- benchmark publication
- technical report
- conference/workshop interest
- academic collaboration

### Strategic

- unique technical niche
- defensible architecture
- reasonable maintenance burden
- optional commercial pathway

---

# 17. What NOT to Do

Until the reassessment is complete:

- Do not build a token.
- Do not build a marketplace.
- Do not add unnecessary cryptography.
- Do not optimize code that may be obsolete.
- Do not build a frontend.
- Do not market JamLiquor as a production JAM node.
- Do not prematurely build AI infrastructure.
- Do not spend months rewriting everything before conformance is understood.
- Do not commit to becoming a full-time blockchain developer.

---

# 18. Strategic Thesis to Test

The most promising hypothesis to investigate is:

> **JamLiquor can become a small, portable, independently implemented JAM client/runtime optimized for constrained edge hardware, providing an experimental foundation for decentralized edge computation and eventually verifiable AI workloads.**

This is a **hypothesis, not the project's new mission statement**.

It must survive G0–G7.

---

# 19. Final Decision Template

At the end of the reassessment, complete:

## Decision

**[KEEP / REWRITE / PIVOT / HOLD / KILL]**

### One-sentence rationale

> ...

### Strongest evidence

1. ...
2. ...
3. ...

### Biggest risk

> ...

### Required next milestone

> ...

### Maximum time investment

> ...

### Stop condition

> ...

### What we deliberately will NOT build

> ...

---

# 20. Guiding Principle

The goal is **not to revive an old project because it is yours.**

The goal is to determine whether the project has become **more valuable because the JAM ecosystem has matured**, and whether that value can be captured without requiring you to abandon your medical career.

The correct outcome may be:

> **"This is worth 3 hours/week for the next 12 months."**

It may also be:

> **"The original architecture is obsolete; keep the idea and rewrite it."**

Or:

> **"JAM is interesting, but JamLiquor is not differentiated enough. Archive it."**

All three are successful outcomes of the reassessment.
