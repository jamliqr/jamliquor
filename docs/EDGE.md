# 📦 EDGE.md — Lightweight by Design

## 🔧 Principle: Lightweight by Design

> If your blockchain needs a server farm, it isn’t decentralized.

JamLiquor is designed for minimal hardware, including:
- ⬇️ Devices with **<128MB RAM**
- 🧠 Low-power CPUs like **RISC-V (e.g. Milk-V Duo, Pine64 Ox64)**
- ⚙️ Compile-time performance profiling and optimization

This raises an important question:

> How can low-resource devices run a blockchain at all?

This document answers that by outlining how **JAM Protocol + PolkaVM + collective edge computing** solves the challenge.

---

## ⚠️ The Concern: Blockchains Need Power
Traditional blockchains demand:
- 🔐 Heavy cryptographic operations (e.g. ECDSA, hashing)
- 🧾 Massive state and history storage (e.g. 500GB+)
- 🌐 High-bandwidth syncs

Lightweight devices (e.g., 64MB RAM, 1GHz CPU) lack:
- Enough memory for full state
- CPU for global consensus
- Storage for the full chain

So how can they run JamLiquor?

---

## ✅ The JAM + PolkaVM Solution

### 1. **Efficient Execution via PolkaVM**
- RISC-V based, 64-bit architecture
- ≤128KB per instance
- JIT-compiled smart contract execution
- Native compatibility with embedded edge CPUs

### 2. **Sharding & Role Specialization**
- JAM splits execution roles: importers, authorers, validators
- A device only needs to fulfill one role (not all at once)
- Importers handle state validation; Authorers handle block production

Low-resource nodes can:
- ✅ Validate local state (importer)
- ✅ Execute contracts for specific domains (e.g., IoT region)
- ❌ Skip global history or full-chain validation

### 3. **Minimal Storage & Pruned State**
- Nodes only store **the latest state and headers**
- JAM supports deterministic re-execution
- Offload historical data to archive or mid-tier nodes

Example:
- Milk-V Duo stores 1MB snapshot
- Contract + state = 2–5MB
- No gigabyte chain bloat

### 4. **Network Throughput via Collective Bandwidth**
- Devices sync only relevant data via gossip or light protocols
- Fetch proofs or state deltas on-demand
- Use compressed messages (~KB/sec)

> 10,000 devices syncing 1KB/sec = 10MB/sec aggregate bandwidth

### 5. **Swarm-Based Computation & Redundancy**
- 1 device = 1GHz
- 1,000 devices = 1,000GHz (distributed)
- Total network throughput scales with adoption

> Redundancy = resilience; if one fails, others take over

---

## 🌆 Smart City Example

Use Case: 10,000 Milk-V Duo devices ($5 each)

Each:
- Monitors air quality (sensor input)
- Executes simple contract (`if pollution > X, alert`)
- Sends alert as a JAM transaction

Network:
- Authorers compile blocks
- Importers verify execution
- Mid-tier relayers sync light clients to full state

✅ No datacenter.
✅ No single point of failure.
✅ True decentralization.

---

## ⚖️ Limitations & How We Address Them

| Concern                         | Mitigation                                                                 |
|--------------------------------|----------------------------------------------------------------------------|
| ❗ Limited RAM/CPU              | Sharded roles, minimal state, JIT PolkaVM                                |
| ❗ No full chain history        | Pruned state, archive nodes, light clients                               |
| ❗ Sync latency on weak links   | Gossip + local caching + compressed updates                              |
| ❗ Software/tooling maturity    | Rust ecosystem, JAM spec alignment, PolkaVM roadmap                      |

---

## ✅ Conclusion

JamLiquor makes **edge blockchain execution viable** by combining:
- ✳️ PolkaVM’s hardware-native efficiency
- ✳️ JAM’s modular role architecture
- ✳️ Distributed swarm computation
- ✳️ Pruned, minimal state design

It doesn’t fight device limitations—it **embraces** them, then turns them into a strength.

**The edge isn’t a compromise. It’s the future.**

