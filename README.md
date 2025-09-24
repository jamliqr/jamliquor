# JamLiquor

#### **A Cleanroom JAM Client with Edge, AI, and PQC Extensions**

_Lightweight. Adaptive. Quantum-Resistant. JAM-native._

![License](https://img.shields.io/badge/license-GPL_3.0-blue.svg) ![Language](https://img.shields.io/badge/language-Rust-orange.svg) [![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/jamliqr/e21c2f06f08f5795f50860fcbbbeb1f3/raw/jamliquor-coverage.json)](https://github.com/jamliqr/jamliquor/actions) ![Status](https://img.shields.io/badge/status-Experimental-lightgrey) [![Docs](https://img.shields.io/badge/docs-online-blueviolet)](https://yourusername.github.io/jamliquor/) [![Build Status](https://github.com/jamliqr/jamliquor/actions/workflows/coverage.yml/badge.svg)](https://github.com/jamliqr/jamliquor/actions)

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
15. [Join the Movement](#-join-the-movement)

---

## 📌 What is JamLiquor?

**JamLiquor** is a purpose-driven, cleanroom **JAM Protocol implementation**, engineered from scratch in Rust. It is designed to:

- ✅ **Conform to JAM's execution model** and participate in the JAM ecosystem
- ✅ **Run on edge hardware** like Milk-V Duo and Raspberry Pi Zero 2 W
- ✅ **Support modular innovation**, including AI inference and PQ cryptography

JamLiquor is not a fork. It's not a derivative. It is a **JAM-native node with radical modularity**.

---

## 🧭 Core Principles

### 1. **Lightweight by Default**

Runs on ≤128MB RAM and ≤1GHz CPUs. JAM should live at the edge.

### 2. **Real Decentralization**

No validator elitism. JamLiquor is designed for community-run nodes.

### 3. **Adaptability with AI**

AI inference modules allow the node to adapt in real time—on-chain and off-chain.

### 4. **Quantum-Resistance First**

Includes opt-in modules for Dilithium, Kyber, and XMSS.

### 5. **Simplicity is Strength**

Minimalist, auditable, and modular. No unnecessary cryptographic gymnastics.

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

### Quick Demo (Importer)

Run the importer prototype against the bundled JAM block fixture:

```bash
cargo run --release
# Prints parsed block metadata and state summary
```

This loads `tests/vectors/codec/data/block.json` and outputs:

- Header details (slot, author index, entropy source)
- Ticket counters from `state::State`
- State transition status

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

See the minimalist entry point in `src/main.rs` and additional guides in [`docs/src/README.md`](./docs/src/README.md).

---

## 🔁 Workflows

### Node Operation (Preview)

1. Provision a supported edge target (see [Edge Targets](#-edge-targets)).
2. Install cross targets:
   ```bash
   rustup target add riscv64gc-unknown-linux-gnu
   rustup target add armv7-unknown-linux-gnueabihf
   ```
3. Cross-compile the importer:
   ```bash
   cargo build --release --target riscv64gc-unknown-linux-gnu
   ```
4. Deploy the binary plus fixture data under `tests/vectors/` to the device.
5. Run the importer offline while networking and authoring modules mature.

### Development Loop

1. Review roadmap items in [`docs/src/MILESTONE.md`](./docs/src/MILESTONE.md).
2. Start the watch-based loop:
   ```bash
   cargo watch -x "test --lib" -x "fmt"
   ```
3. Implement features in `src/importer.rs`, `src/state.rs`, or new modules under `extensions/`.
4. Update fixtures in `tests/vectors/` to cover new scenarios.
5. Run `cargo test --all-targets` and `cargo bench` (optional) before opening a PR.

Additional workflows for AI, PQC, and lightweight profiles live in `docs/src/` (e.g., [`AI.md`](./docs/src/AI.md), [`PQC.md`](./docs/src/PQC.md), [`EDGE.md`](./docs/src/EDGE.md)).

---

## ⚙ Configuration

- **Logging**: Set `RUST_LOG=<level>` when running (`env_logger` backend).
- **Input paths**: Pass alternate block files to `Importer::import_block` or adjust fixtures in `tests/vectors/`.
- **Environment profiles**: Capture board-specific notes in [`docs/src/EDGE.md`](./docs/src/EDGE.md). A consolidated `config.toml` will land alongside future networking modules.

Cleanroom practices and configuration expectations are documented in [`docs/src/CLEANROOM.md`](./docs/src/CLEANROOM.md).

---

## 🧪 Testing & Validation

- **Unit & integration tests**: `cargo test --all-targets`
- **Property tests**: See `tests/` for `proptest`-based suites.
- **Benchmarks**: `cargo bench --bench core_benchmarks`
- **Coverage**: GitHub Actions publishes coverage; locally run `cargo llvm-cov --all-features` (requires `cargo-llvm-cov`).
- **Cross-compilation checks**: Follow [`docs/src/building.md`](./docs/src/building.md) and extend CI when adding new targets.

---

## 🛠 Troubleshooting

### Build Issues

- Update toolchain: `rustup update`
- Clear build artifacts: `cargo clean && cargo build`
- Confirm target toolchains for cross builds (see [Requirements & Installation](#-requirements--installation)).

### Runtime Issues

- Enable verbose logging: `RUST_LOG=debug cargo run`
- Verify fixture paths and JSON structure in `tests/vectors/`
- Ensure at least 128 MB RAM on edge devices.

### Networking & Extensions

- Networking modules remain experimental—track progress in [`docs/src/NETWORK.md`](./docs/src/NETWORK.md).
- PQC and AI extensions live under `extensions/`; verify feature availability before enabling in production prototypes.

For unresolved issues, [open an issue](https://github.com/jamliqr/jamliquor/issues) with logs, hardware profile, and reproduction steps.

---

## 🚀 Project Status

| Component          | Description                           | Status          |
| ------------------ | ------------------------------------- | --------------- |
| `importer/`        | JAM-compliant block/state validation  | 🟡 In Progress  |
| `authorer/`        | SAFROLE-based block production engine | ⚪ Planned      |
| `vm/`              | PolkaVM (RISC-V) execution backend    | ⚪ Planned      |
| `extensions/pqc/`  | PQ-safe signatures (Dilithium)        | ⚪ Experimental |
| `extensions/ai/`   | TinyML-based inference at edge        | ⚪ Experimental |
| `extensions/lite/` | 128MB RAM profile w/ minimal overhead | ⚪ Experimental |

For detailed roadmap, milestones, and technical specifications, see our [Milestone Documentation](./docs/src/MILESTONE.md).

---

## 🔧 Edge Targets

- Milk-V Duo (RISC-V, 64MB RAM)
- Pine64 Ox64 (RISC-V)
- Raspberry Pi Zero 2 W (ARM)
- StarFive VisionFive 2 (RISC-V, quad-core)

---

## 🧪 Cleanroom Policy

JamLiquor is built:

- Without reuse of non-auditable JAM source code
- With public commit history and architecture provenance
- To be compatible with JAM Prize auditing

See: [`docs/cleanroom.md`](./docs/cleanroom.md)

---

## 📚 Key References

- [JamLiquor Manifesto (v0.2.0)](./MANIFESTO.md)
- [JAM Protocol Graypaper](https://github.com/gavofyork/graypaper)
- [PolkaVM](https://github.com/paritytech/polkavm)
- [JAM Prize Specification](https://hackmd.io/@polkadot/jamprize)

---

## 🤝 Contribute

We welcome contributors with passion for:

- JAM internals and protocol fidelity
- Embedded Rust, RISC-V, and minimal blockchain clients
- Cryptography (PQC, VRFs) and inference (TinyML, Rust ML)

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) to get started.

---

## 🛡 License

Licensed under **GPL-3.0**. Cleanroom declarations and IP policy documented in [Cleanroom Policy](./docs/CLEANROOM.md)

---

## 🚀 Join the Movement

JamLiquor is a JAM client with purpose.
Compliant by design. Modular by philosophy. Edge-ready by default.

**We don't fork the future. We build it.**

🧪 Disclaimer: A Dream Under Construction

JamLiquor is a bold claim. It reads like an engineering utopia. Many may see it as naïve or speculative.

And they're not wrong.

This work is:

🌱 Experimental

🧪 Untested at scale

🧩 Composed of speculative modules

But:

All real systems begin as dreams.

JamLiquor is:

🚧 A sandbox of ideas and engineering

🔁 Built by iteration: brake, fail, fix, retry

🙋 In need of true believers who can build with us

> **⚠ Warning**
>
> We don't promise it will work tomorrow. But we are building it every day to make it work.
>
> - **If it works**: the credit belongs to _everyone who dared_ to try.
> - **If it fails**: we'll have _learned more_ than most systems ever attempt.
>
> This is not just software. It's a belief in what's possible.

🤝 **Welcome to JamLiquor. Let's tinker toward something impossible.**
