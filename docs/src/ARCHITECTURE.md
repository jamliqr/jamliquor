# 🧩 ARCHITECTURE.md — Simplicity Is Strength

> **Status:** Pre-pivot design vision, kept for context. The five-layer sketch below (`importer/`, `authorer/`, `vm/`, `extensions/`, `net/`) is aspirational and not the current build plan. `extensions/pqc` and `extensions/ai` are retired — see [Research Notes](./research-notes.md). The current, evidence-gated plan is `jam-core`'s four modules (`codec`, `crypto`, `safrole`, `trie`) — see [`spec.md`](../../spec.md) §6.

## 🔧 Principle: Simplicity Is Strength

> Complexity is fragility. Simplicity is power.

JamLiquor is intentionally minimalist—not because we lack ambition, but because **trustless systems should be understandable**, extensible, and verifiable by anyone.

This document explains how JamLiquor’s architecture is designed to be simple, modular, and human-readable—without sacrificing performance or functionality.

---

## 📐 Why Simplicity Matters in Blockchain

Modern blockchain stacks suffer from:
- 🌀 Recursive rollups
- 🧩 Layered bridges
- 🧠 Unverifiable ZK complexity
- 🛠 Inaccessible tooling for most developers

> If your blockchain needs a PhD to audit, it's not secure.

JamLiquor chooses another path:
- **1 runtime** (JAM-compatible)
- **1 execution model** (PolkaVM)
- **1 state tree** (unified JAM model)
- **Modular extensions** (not embedded complexity)

---

## 🧱 Architectural Overview
JamLiquor is structured in five primary layers:

### 1. `importer/`
- JAM state transitions
- Block and Work Report validation
- Simple interface: `validate(block, state)`

### 2. `authorer/`
- SAFROLE block production
- Random leader election (via Ring VRF)
- Rate-limited authorship window

### 3. `vm/`
- PolkaVM runtime (RISC-V JIT)
- JAM syscall surface
- Metered, deterministic, sandboxed

### 4. `extensions/`
- `pqc/`: post-quantum signature support
- `ai/`: inference modules (TinyML)
- `lite/`: profiles for edge RAM limits

### 5. `net/`
- Gossip + sync
- Peer discovery
- No privileged RPC—every node is equal

---

## 🪞 Code Design Philosophy
JamLiquor follows:
- 🧠 **Rust-first ergonomics** (clear, type-safe interfaces)
- 🔍 **Modular crates**, no god-objects
- 📦 **Small units of code**, each ~<500 LOC
- 🧪 **Testable in isolation**, not just integrated

Every line is:
- Explicit
- Auditable
- Justified by JAM specs or clean extensions

---

## 🧪 Developer Experience (DX)
### For newcomers:
- `cargo build --release`
- `cargo run --bin importer`
- Minimal configuration, real outputs

### For contributors:
- Readable interfaces and folder layout
- Self-documenting logic
- IDE-friendly

### For auditors:
- Clean commit trail
- Architecture diagrams
- No external binaries or unsafe code by default

---

## 🚫 What We Avoid
- ❌ Recursive rollups
- ❌ Multi-layer fraud proofs
- ❌ Hidden gas accounting
- ❌ Hardcoded validators
- ❌ WASM-only obscurity

> Complexity is technical debt in disguise.

---

## ✅ Summary
JamLiquor is architected for simplicity:
- ✳️ Easy to audit
- ✳️ Easy to run
- ✳️ Easy to modify

It is not jut a lab experiment. It's aiming to be a working node for humans.

**Simplicity scales. Simplicity survives.**
