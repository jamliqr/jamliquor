# JamLiquor

#### **A Cleanroom JAM Client with Edge, AI, and PQC Extensions**

_Lightweight. Adaptive. Quantum-Resistant. JAM-native._

![License](https://img.shields.io/badge/license-GPL_3.0-blue.svg) ![Language](https://img.shields.io/badge/language-Rust-orange.svg) [![Coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/jamliqr/e21c2f06f08f5795f50860fcbbbeb1f3/raw/jamliquor-coverage.json)](https://github.com/jamliqr/jamliquor/actions) ![Status](https://img.shields.io/badge/status-Experimental-lightgrey) [![Docs](https://img.shields.io/badge/docs-online-blueviolet)](https://yourusername.github.io/jamliquor/) [![Build Status](https://github.com/yourusername/jamliquor/actions/workflows/coverage.yml/badge.svg)](https://github.com/yourusername/jamliquor/actions)

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
