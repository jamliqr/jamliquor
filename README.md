# JamLiquor: A Cleanroom JAM Client with Edge, AI, and PQC Extensions

### *Lightweight. Adaptive. Quantum-Resistant. JAM-native.*

![License](https://img.shields.io/badge/license-GPL_3.0-blue.svg) ![Language](https://img.shields.io/badge/language-Rust-orange.svg) ![Status](https://img.shields.io/badge/status-Experimental-lightgrey)

---

## 📌 What is JamLiquor?
**JamLiquor** is a purpose-driven, cleanroom **JAM Protocol implementation**, engineered from scratch in Rust. It is designed to:

- ✅ **Conform to JAM’s execution model** and participate in the JAM ecosystem
- ✅ **Run on edge hardware** like Milk-V Duo and Raspberry Pi Zero 2 W
- ✅ **Support modular innovation**, including AI inference and PQ cryptography

JamLiquor is not a fork. It’s not a derivative. It is a **JAM-native node with radical modularity**.

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

## 🚀 Goals
| Component | Description | Status |
|-----------|-------------|--------|
| `importer/` | JAM-compliant block/state validation | ⚪ Planned |
| `authorer/` | SAFROLE-based block production engine | ⚪ Planned |
| `vm/` | PolkaVM (RISC-V) execution backend | ⚪ Planned |
| `extensions/pqc/` | PQ-safe signatures (Dilithium) | ⚪ Experimental |
| `extensions/ai/` | TinyML-based inference at edge | ⚪ Experimental |
| `extensions/lite/` | 128MB RAM profile w/ minimal overhead | ⚪ Experimental |

---

## 📦 Milestone Roadmap (JAM Prize Alignment)

### Milestone 1: **Importer**  
- [ ] Block verification and state transitions (JAM spec compliant)  
- [ ] CoreTime & validator judgment ruleset

### Milestone 2: **Authorer**  
- [ ] Implement SAFROLE + Ring VRF  
- [ ] Full authoring pipeline with consensus randomness

### Milestone 3: **PolkaVM Integration**  
- [ ] PolkaVM (RISC-V) with 64-bit JIT support  
- [ ] JAM-compatible syscall interface

### Milestone 4: **Modular Extensions**  
- [ ] Publish `jamliquor-pqc`, `jamliquor-ai`, `jamliquor-lite`  
- [ ] Optional hooks for JAM runtime

### Milestone 5: **Performance Readiness**  
- [ ] JAM block throughput on 256MB RAM  
- [ ] 6s block time + 18s finality on edge hardware

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

**We don’t fork the future. We build it.**

