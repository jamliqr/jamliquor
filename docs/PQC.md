# 🔐 PQC.md — Quantum Resistance is Non-Negotiable

## 🛡️ Principle: Quantum Resistance is Non-Negotiable

> The cryptography of today won’t survive the quantum world of tomorrow.

JamLiquor integrates post-quantum cryptography (PQC) by design—not as a patch, but as a core modular extension that keeps the network resilient into the 2030s and beyond.

This document outlines how PQC works in JamLiquor, and how it can be adopted without sacrificing JAM compatibility.

---

## ⚠️ The Threat of Quantum Computing
Algorithms like **Shor’s** and **Grover’s** could soon break:
- RSA, ECDSA
- sr25519 (used in most Substrate-based chains)
- BLS and SNARK-based crypto

Quantum-resilient systems need to:
- Use lattice-based or hash-based cryptography
- Prepare for migration paths today—not after compromise

---

## ✅ JamLiquor's PQC Strategy
JamLiquor uses modular PQC extensions:

### 1. **Dilithium (CRYSTALS)**
- Lattice-based digital signature
- NIST-standardized (2022, 2024)
- 2–4KB signature size (vs. 64B for ECDSA)
- Suitable for JAM validators, work reports, judgments

### 2. **Kyber** (optional key exchange)
- Module for encrypted messaging or shared secrets
- JAM doesn’t require this, but future parachains might

### 3. **XMSS (Hash-Based)**
- Stateless variant for resource-constrained validators
- Very fast signing, longer verification
- Used in certain bootstrapping tasks

---

## 🔄 JAM Compatibility
- PQC support lives in `jamliquor-pqc/`
- Signature schemes are *interchangeable* via runtime contexts
- Work Reports, validator judgments, and transaction signers can toggle PQ or classical modes
- On-chain verifiers are optimized for batch validation

> JAM’s modular signature layers make it ideal for cryptographic flexibility.

---

## 🧪 Module: `jamliquor-pqc`
- Rust-first (via `pqcrypto`, `orion`, `dilithium`, `kyber`)
- Selectable via Cargo feature flags (e.g. `--features pq-dilithium`)
- Signature abstraction implements JAM-style `Signer`/`Verifier` traits

### Storage Impact
- Public key: 1–2KB
- Signature: ~2.4–4KB

Performance:
- Sign: ~1–2ms
- Verify: ~2–5ms

---

## 🔐 Use Cases

### 🔒 Validator Judgments
- Validators sign reports using Dilithium
- Proofs are verifiable by importers

### 🔏 Quantum-Safe Work Reports
- Authorers can sign blocks or extrinsics with PQ keys

### 🗃 Future Key Migration
- Dual-signature support enables hybrid rollout
- Classical + PQ keys for seamless transition

---

## 🛠 Deployment Tips
- Use secure RNG hardware for PQ key generation
- Limit bandwidth overhead by batching PQ signatures
- Compress keys/signatures for gossip propagation

---

## ✅ Summary
Quantum attacks are not theoretical—they’re **inevitable**.

JamLiquor doesn’t wait for catastrophe. It makes PQC **native**, **modular**, and **future-safe**.

**Security delayed is security denied.**

