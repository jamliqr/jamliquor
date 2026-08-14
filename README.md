# JamLiquor

#### A cleanroom JAM protocol research core, targeting binary-vector conformance and constrained-hardware benchmarks

_Lightweight. JAM-native._ — both goals to be earned via CI evidence, not assumed.

> **Status (2026-08):** JamLiquor completed a full [2026 reassessment](./docs/REASSESSMENT/README.md) against the current JAM ecosystem. Outcome: **PIVOT**. The original five-pillar vision (Lightweight, Adaptive/AI, Quantum-Resistant, Decentralized, Simple) is retired to two claims that must be proven in CI: **Lightweight** and **JAM-native**. See [`spec.md`](./spec.md) for the binding product contract and [`DECISION.md`](./DECISION.md) for the full rationale.

## 📚 Table of Contents

1. [What is JamLiquor?](#-what-is-jamliquor)
2. [Core Principles](#-core-principles)
3. [Requirements & Installation](#-requirements--installation)
4. [Getting Started](#-getting-started)
5. [Workflows](#-workflows)
6. [Configuration](#-configuration)
7. [Testing & Validation](#-testing--validation)
8. [Troubleshooting](#-troubleshooting)
9. [Project Status](#-project-status)
10. [Edge Targets](#-edge-targets)
11. [Cleanroom Policy](#-cleanroom-policy)
12. [Key References](#-key-references)
13. [Contribute](#-contribute)
14. [License](#-license)

---

## 📌 What is JamLiquor?

**JamLiquor** is a cleanroom **JAM protocol research implementation**, engineered from scratch in Rust. The current, evidence-gated goal is:

- ✅ **Prove deterministic agreement** with the official `w3f/jamtestvectors` binary codec + `stf/safrole` vectors (MVP-1, due 2026-11-15)
- ✅ **Measure**, not assume, JAM's resource cost on ARM64/RISC-V single-board computers (MVP-2/3, due 2027-02-15)
- ❌ It is **not** a validator, a full client, a JAM Prize entrant, or a production node. Do not use it as one.

JamLiquor is not a fork and reuses no non-auditable JAM source code — see [Cleanroom Policy](#-cleanroom-policy).

---

## 🧭 Core Principles

Two claims survive the 2026 reassessment. Both are **hypotheses under test**, not settled facts — see [`spec.md`](./spec.md) for the gates that must pass before either is asserted as true.

### 1. Lightweight (unproven — gated by MVP-2/3)

The central open question: what does JAM actually cost on constrained hardware, and which JAM roles fit on a ≤512 MB SBC? No client in the ecosystem has published this. No numbers exist yet in this repo either — see [Project Status](#-project-status).

### 2. JAM-native (unproven — gated by MVP-1)

A conformant subset (binary codec + Safrole state transition) that provably agrees with the official test vectors, not a JSON-fixture importer that merely contains JAM-related code.

### Retired

AI inference ("Adaptive") and post-quantum cryptography ("Quantum-Resistant") were evaluated and dropped from the project identity — both are application-layer ideas with no JAM-native technical justification today. One research note each is kept for the record: [`docs/src/research-notes.md`](./docs/src/research-notes.md).

---

## ⚙ Requirements & Installation

### Prerequisites

- Rust toolchain `1.75` or newer (`rustup` recommended)
- `cargo` and `git`
- Linux or macOS host (Windows support via WSL2)
- Optional: cross-compilation toolchains for RISC-V and ARM (see [`docs/src/building.md`](./docs/src/building.md))

### Install

```bash
git clone https://github.com/jamliqr/jamliquor.git
cd jamliquor
rustup default stable
cargo build --release
```

### Verify the Toolchain

```bash
# Run unit and property tests
cargo test

# Check formatting and lints
cargo fmt -- --check
cargo clippy --all-targets --all-features
```

---

## 🚀 Getting Started

### Quick Demo (legacy importer)

This runs the **legacy JSON-fixture importer** — pre-pivot code, kept only until `jam-core` (the binary-codec conformance core described in [`spec.md`](./spec.md)) replaces it. It does not exercise binary codec or STF conformance vectors.

```bash
cargo run --release
# Prints parsed block metadata and state summary
```

This loads `tests/vectors/codec/data/block.json` and outputs:

- Header details (slot, author index, entropy source)
- Ticket counters from `state::State`
- State transition status (structural checks only — not GP-conformant STF)

### Explore as a Library

```rust
use jamliquor::Importer;

fn main() -> anyhow::Result<()> {
    let mut importer = Importer::new();
    let block = importer.import_block("tests/vectors/codec/data/block.json")?;
    println!("Imported slot {}", block.header.slot);
    Ok(())
}
```

See `src/main.rs` and additional guides in [`docs/src/README.md`](./docs/src/README.md).

---

## 🔁 Workflows

### Development Loop

1. Review the roadmap in [`Plans.md`](./Plans.md) and [`docs/src/MILESTONE.md`](./docs/src/MILESTONE.md).
2. Start the watch-based loop:
   ```bash
   cargo watch -x "test --lib" -x "fmt"
   ```
3. Legacy importer work lives in `src/importer.rs` / `src/state.rs`. New conformance work lands under the `jam-core` workspace crate (`codec`, `crypto`, `safrole` modules) once scaffolded — see Plans.md Phase 2.
4. Update fixtures in `tests/vectors/` to cover new scenarios.
5. Run `cargo test --all-targets` before opening a PR.

---

## ⚙ Configuration

- **Logging**: Set `RUST_LOG=<level>` when running (`env_logger` backend).
- **Input paths**: Pass alternate block files to `Importer::import_block` or adjust fixtures in `tests/vectors/`.

Cleanroom practices and configuration expectations are documented in [`docs/src/CLEANROOM.md`](./docs/src/CLEANROOM.md).

---

## 🧪 Testing & Validation

- **Unit & integration tests**: `cargo test --all-targets`
- **Property tests**: See `tests/` for `proptest`-based suites.
- **Coverage**: GitHub Actions publishes coverage; locally run `cargo llvm-cov --all-features` (requires `cargo-llvm-cov`).
- **Cross-compilation checks**: Follow [`docs/src/building.md`](./docs/src/building.md) and extend CI when adding new targets.
- **Conformance vectors**: tracked in [`docs/src/CONFORMATION.md`](./docs/src/CONFORMATION.md); the vector pin (v0.7.1, `1dc503a`) is frozen for the MVP-1 window per [`spec.md`](./spec.md).

---

## 🛠 Troubleshooting

### Build Issues

- Update toolchain: `rustup update`
- Clear build artifacts: `cargo clean && cargo build`
- Confirm target toolchains for cross builds (see [Requirements & Installation](#-requirements--installation)).

### Runtime Issues

- Enable verbose logging: `RUST_LOG=debug cargo run`
- Verify fixture paths and JSON structure in `tests/vectors/`

For unresolved issues, [open an issue](https://github.com/jamliqr/jamliquor/issues) with logs and reproduction steps.

---

## 🚀 Project Status

Status reflects what is **currently green in CI**, not aspiration — see [`spec.md`](./spec.md) §8 (claims policy).

| Component | Description | Status |
| --- | --- | --- |
| `src/` (legacy importer) | JSON-fixture parser, structural checks only, pre-pivot | 🟡 Kept until superseded, then removed (Plans.md Phase 2) |
| `jam-core/codec` | Binary JAM codec vs. official vectors | ⚪ Not started (Plans.md Phase 3) |
| `jam-core/crypto` | Ed25519 + Bandersnatch ring VRF verification | ⚪ Not started (Plans.md Phase 4) |
| `jam-core/safrole` | Safrole STF vs. `stf/safrole` vectors | ⚪ Not started (Plans.md Phase 4) |
| Edge/SBC benchmarks | Peak RAM, binary size, vectors/sec on ARM64 + RISC-V | ⚪ Not started (Plans.md Phase 5); zero measurements exist today |

`jam-core/trie` (`spec.md` §6's fourth module) is deliberately unscheduled — it sequences after Safrole and has no Plans.md phase yet; it is out of scope for MVP-1.

No PVM, authorer, networking, PQC, or AI work is planned inside this repo's current scope — see `spec.md` §2 for the full non-goals list.

For task-level detail, see [`Plans.md`](./Plans.md). For historical context, see [`docs/REASSESSMENT/`](./docs/REASSESSMENT/README.md) and the archived pre-pivot plan at [`docs/archive/TODO-2026-04.md`](./docs/archive/TODO-2026-04.md).

---

## 🔧 Edge Targets

Named as **candidate** targets for the MVP-2/3 benchmark milestone — not yet verified. No binary-size, RAM, or startup measurement has been recorded on any of these as of 2026-08.

- Milk-V Duo (RISC-V, 64MB RAM) — Tier C, at most header/ticket verification per `spec.md`
- Pine64 Ox64 (RISC-V) — Tier C
- Raspberry Pi Zero 2 W (ARM) — Tier A/B boundary
- StarFive VisionFive 2 (RISC-V, quad-core) — Tier B

---

## 🧪 Cleanroom Policy

JamLiquor is built:

- Without reuse of non-auditable JAM source code
- With public commit history and architecture provenance

See: [`docs/src/CLEANROOM.md`](./docs/src/CLEANROOM.md)

---

## 📚 Key References

- [JamLiquor product contract (spec.md)](./spec.md)
- [2026 Reassessment & Decision](./docs/REASSESSMENT/README.md)
- [JAM Protocol Graypaper](https://graypaper.com)
- [w3f/jamtestvectors (conformance authority)](https://github.com/w3f/jamtestvectors)

---

## 🤝 Contribute

We welcome contributors interested in:

- JAM binary codec and Safrole state-transition conformance
- Embedded Rust and RISC-V/ARM cross-compilation

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) and the curated good-first-issues (from Plans.md Phase 2 onward) to get started.

---

## 🛡 License

Licensed under **GPL-3.0**. Cleanroom declarations and IP policy documented in [Cleanroom Policy](./docs/src/CLEANROOM.md).
