# JamLiquor Revival TODO
**Date:** 2026-04-01  
**Last active dev:** ~September 2025  
**Gap:** ~7 months

---

## 0. Ground Truth: What Actually Happened

The JAM ecosystem did not stand still.

| Event | When | Impact on JamLiquor |
|---|---|---|
| Gray Paper v0.6.6 → v0.6.7 | June 2025 | Breaking: new Service Account elements, storage key read/write representation changes, affects codec and state structures |
| Gray Paper v0.7.0 | June 2025 | Breaking: PVM execution pseudocode added, Aggregating Scheduler defined, Proof-of-Provisioning (PoP) process specified |
| Gray Paper v0.8 (near-final) | Late 2025 | Near-freeze: final pre-audit draft. This is the target version. |
| M1 Conformance Suite completed | Late 2025 | The official test vectors at `w3f/jamtestvectors` are now the acceptance criteria. Multiple teams (PolkaJAM/Rust, go-jam-duna/Go, Jampy/Python) are passing M1 block import vectors. You are not in this list. |
| M1 now requires PVM | 2025 | Block import test vectors exercise host calls and PVM boot. PVM is not optional for M1. |
| JAM mainnet target | 2026 | Window is open but competitive. ~15+ active teams. |

**Honest state of the repo right now:**
- Task 1 (Foundation): ✅ Done
- Task 2 (Importer): 🟡 50% done — codec/parsing/CoreTime done, but validator judgments, state transitions, and integration all pending
- Tasks 3–10: ❌ Not started
- Gray Paper alignment: **Unknown** — the existing codec/data structures were written against an earlier spec version. Must verify against v0.8 before any new work.

---

## Phase 0: Audit Before Anything Else
**Estimated time: 1–2 weeks**  
**Do not write new code until this is complete.**

### 0.1 Determine current Gray Paper version alignment

```bash
# Add test vectors as submodules (pins to a reproducible commit, not a moving branch)
git submodule add https://github.com/w3f/jamtestvectors.git tests/vectors/official
git submodule add https://github.com/davxy/jam-test-vectors.git tests/vectors/unofficial
git commit -m "chore: add jam test vector submodules"

# Run existing JamLiquor codec against official vectors
cargo test --all-targets 2>&1 | tee audit_results.txt
```

To update submodules when the Gray Paper version bumps:
```bash
git submodule update --remote tests/vectors/official
git submodule update --remote tests/vectors/unofficial
git commit -m "chore: update test vector submodules"
```

Fresh clone:
```bash
git clone --recurse-submodules https://github.com/jamliqr/jamliquor.git
# or after cloning without submodules:
git submodule update --init --recursive
```

- [ ] Map every data structure in `src/` against Gray Paper v0.8 section by section
- [ ] Document which GP version each struct was written against (check git log dates vs GP release dates)
- [ ] Flag all structs that touch: Service Accounts, storage keys, block headers, extrinsics

### 0.2 Run M1 conformance vectors

The M1 STF vectors cover these subsystems (from `w3f/jamtestvectors` issue #21):

- [ ] Codec (block/block header serialization)
- [ ] SAFROLE (Section 6 — block production, chain growth)
- [ ] Recent History (Section 7)
- [ ] Authorization (Section 8)
- [ ] Disputes / Verdicts / Judgements (Section 10)
- [ ] Reporting and Assurance (Section 11)
- [ ] Accumulation (Section 12)
- [ ] Preimages

Run each category against the official JSON test vectors. Record pass/fail per subsystem. This is your real backlog, not the tasks.json.

### 0.3 PVM gap assessment

M1 now requires PVM to be functional (host calls exercised, PVM boots). Assess:

- [ ] Is there any PVM code in the repo? (README says "Planned")
- [ ] What version of `polkavm` crate is currently in `Cargo.toml`?
- [ ] Does the current `polkavm` crate version align with the RISC-V spec in GP v0.8?

```bash
cargo tree | grep polkavm
```

---

## Phase 1: Gray Paper v0.8 Re-alignment
**Estimated time: 3–5 weeks**  
**Prerequisite: Phase 0 complete**

This is the most unglamorous but most critical phase. Every hour spent here saves two hours in Phase 2.

### 1.1 Codec re-sync

The GP v0.6.7 introduced representation changes to Service Account elements and storage key serialization. The GP v0.7 introduced PVM invocation serialization.

- [ ] Audit `src/importer.rs` and all data structs against GP v0.8 Appendix sections (encoding)
- [ ] Update `block.rs` / header structs to match current spec
- [ ] Verify SCALE-like encoding matches GP v0.8 exactly (JAM uses its own codec, not vanilla SCALE)
- [ ] Pass all codec test vectors from `w3f/jamtestvectors/codec/`

**Reference:** `jamtestvectors/codec/` directory contains `tiny` and `full` chain configurations. Both must pass.

### 1.2 SAFROLE re-sync (Section 6, GP v0.8)

SAFROLE has been refined significantly. The Ring VRF input construction and ticket verification have had multiple corrections since v0.3.

- [ ] Verify `jam_seal` (not `jam_ticket_seal`) is used in Ring VRF context construction
- [ ] Verify Blake2b-256 (not Blake2b-512) is used for test vector hashing
- [ ] Verify Fisher-Yates shuffle implementation matches GP spec exactly
- [ ] Verify fallback key selection logic (modulo behavior on 600-element array)
- [ ] Pass all SAFROLE test vectors from `w3f/jamtestvectors/safrole/`

### 1.3 State structure re-sync

- [ ] Update `state::State` to match GP v0.8 state schema
- [ ] Verify ticket counter representation
- [ ] Verify validator set representation and rotation
- [ ] Verify recent blocks history structure (Section 7)

---

## Phase 2: Complete the Importer (M1 Core)
**Estimated time: 4–6 weeks**  
**Prerequisite: Phase 1 complete**

This completes tasks 2.4, 2.5, and 2.6 from the original plan — but now grounded against v0.8.

### 2.1 Validator judgment verification (Task 2.4)

Section 10 of the Gray Paper — Disputes, Verdicts, and Judgements.

- [ ] Implement Ed25519 validator signature verification
- [ ] Implement quorum threshold check (2/3 + 1 of validator set)
- [ ] Implement verdict aggregation (valid/invalid/wonky)
- [ ] Implement culprit and fault extrinsic handling
- [ ] Pass Section 10 test vectors from `w3f/jamtestvectors/disputes/`

### 2.2 Reporting and Assurance (Section 11)

- [ ] Implement work report validation
- [ ] Implement guarantor signature verification (3-of-N on a core)
- [ ] Implement assurance aggregation (availability attestation)
- [ ] Implement work report queueing (pending → ready pipeline)
- [ ] Pass Section 11 test vectors from `w3f/jamtestvectors/reports/`

### 2.3 Accumulation (Section 12)

- [ ] Implement Accumulate entry point dispatch
- [ ] Implement service storage read/write
- [ ] Implement fund transfer with memo
- [ ] Implement service creation and code upgrade
- [ ] Implement preimage availability requests
- [ ] Pass Section 12 test vectors from `w3f/jamtestvectors/accumulate/`

### 2.4 Preimage handling

- [ ] Implement preimage lookup (hash → data) for Refine function
- [ ] Implement preimage expunge after `preimage_expunge_period` slots
- [ ] Pass preimage test vectors

### 2.5 Full state transition integration (Task 2.5 + 2.6)

- [ ] Wire all subsystems into a single `apply_block(state, block) -> Result<State>` function
- [ ] Implement atomic rollback on any subsystem failure
- [ ] Implement Merkle state root computation (verify against GP v0.8 Appendix)
- [ ] Benchmark: block import must complete within the 6s slot window on target hardware
- [ ] Memory usage must stay ≤128MB for lite profile

---

## Phase 3: Minimal PVM Integration (M1 Requirement)
**Estimated time: 3–4 weeks**  
**Prerequisite: Phase 2 complete**

M1 now requires PVM to boot and exercise host calls. This is not optional. Use the upstream `polkavm` crate — do not write a PVM from scratch at this stage. A custom PVM is required for M4/M5 (the prize paths), but for M1 the spec allows third-party PVM.

### 3.1 Integrate polkavm crate

```toml
[dependencies]
polkavm = "latest-stable"
```

- [ ] Confirm polkavm crate version is aligned with GP v0.8 PVM spec (RISC-V RV64IM + JAM-specific extensions)
- [ ] Implement `VmBackend` wrapper that satisfies JAM's execution interface
- [ ] Implement memory layout: 4GB virtual address space, guard pages, program memory, stack, heap
- [ ] Implement execution entry points: `refine`, `accumulate`, `on_transfer`

### 3.2 Host calls (syscall interface)

GP v0.7 added detailed pseudocode for PVM host calls. These are the interface between the VM and the chain state.

- [ ] `lookup` — preimage lookup by hash
- [ ] `read` / `write` — service storage access
- [ ] `bless` / `assign` — core assignment (accumulate only)
- [ ] `checkpoint` — gas metering checkpoint
- [ ] `info` — service metadata query
- [ ] `empower` — privileged operations
- [ ] `expunge` — preimage expunge request
- [ ] `forget` — preimage forget request
- [ ] `yield` — result submission from refine
- [ ] `panic` — explicit fault

### 3.3 Gas metering

- [ ] Implement instruction-level gas counter
- [ ] Implement gas limit enforcement (block gas: `max_block_gas`, refine gas: `max_refine_gas`)
- [ ] Verify against tiny chain params: `max_block_gas: 20_000_000`, `max_refine_gas: 1_000_000_000`
- [ ] Verify against full chain params: `max_block_gas: 3_500_000_000`, `max_refine_gas: 5_000_000_000`

### 3.4 Pass PVM test vectors

- [ ] Pass all instruction test vectors from `w3f/jamtestvectors/pvm/`
- [ ] Pass modulo signed 64-bit edge cases (known footgun in Rust: `-x % y` is negative, GP requires positive result)

---

## Phase 4: M1 Submission Preparation
**Estimated time: 1–2 weeks**  
**Prerequisite: Phases 1–3 complete**

### 4.1 Conformance tool integration

The M1 conformance tool works via simple I/O (pipes or networking). It delivers a stream of blocks from genesis and reads back expected state root hashes.

- [ ] Implement conformance protocol I/O interface (stdin/stdout pipe mode)
- [ ] Expose `--conformance-mode` CLI flag
- [ ] Verify state root output format matches tool expectations
- [ ] Run full M1 conformance tool locally before submission

### 4.2 Documentation for submission

Per JAM Prize rules, cleanroom provenance must be demonstrable.

- [ ] Ensure all commits since day 1 are public and timestamped (or blockchain-timestamped if private)
- [ ] Document any external library usage (cryptographic primitives, polkavm — these are allowed)
- [ ] Document any architectural discussions with other teams (disclose even high-level ones)
- [ ] Update `docs/src/CLEANROOM.md` with current status

### 4.3 Register on graypaper.com/clients

- [ ] Submit JamLiquor for listing at graypaper.com/clients
- [ ] Request access to JAM Conformance Dashboard

---

## Phase 5: Authorer (M2)
**Estimated time: 6–10 weeks**  
**Prerequisite: M1 submission accepted**

M2 = "Fully conformant and can produce blocks (including networking, off-chain)."

### 5.1 SAFROLE block production

- [ ] Implement Ring VRF key generation (Bandersnatch keys)
- [ ] Implement ticket generation and submission (two epochs ahead)
- [ ] Implement fallback authoring (when ticket ring is empty)
- [ ] Implement SAFROLE epoch transition logic
- [ ] Implement slot claim logic (determine if we are the author for slot N)

### 5.2 Block assembly

- [ ] Implement extrinsic selection and ordering
- [ ] Implement block header construction (parent hash, state root, extrinsic hash, slot, author index, entropy)
- [ ] Implement work package collection for in-core execution
- [ ] Implement guarantee aggregation

### 5.3 Networking (QUIC)

JAM uses QUIC for networking. No gossip — point-to-point with grid-diffusion for availability.

- [ ] Integrate `quinn` crate (QUIC implementation in Rust)
- [ ] Implement validator discovery (from state, not DHT)
- [ ] Implement block announcement to validators
- [ ] Implement work package distribution (grid diffusion)
- [ ] Implement assurance collection

---

## Phase 6: Custom PVM (M3/M4 Path)
**Estimated time: 8–16 weeks**  
**Prerequisite: M2 complete**

This is the path to the full prize. A custom PVM is required for M4 (100,000 DOT). It must achieve Kusama-level performance (M3) and Polkadot-level performance (M4).

- [ ] Implement RISC-V RV64IM decoder from scratch
- [ ] Implement interpreter (correctness baseline)
- [ ] Implement JIT compiler backend (x86_64 target for dev machines, aarch64 for ARM edge targets)
- [ ] Implement sandboxing (memory isolation, no syscall passthrough)
- [ ] Benchmark against polkavm reference on JAM Toaster
- [ ] Target: Kusama-level performance = M3 (50,000 DOT)
- [ ] Target: Polkadot-level performance = M4 (100,000 DOT)

---

## Phase 7: Differentiators (Post-M1, Parallel Track)
**These are what make JamLiquor distinct. Work on these in parallel once M1 is submitted.**

### 7.1 Edge hardware CI (the actual moat)

No other JAM team is targeting Milk-V Duo (64MB RAM). This is real differentiation if it works.

- [ ] Set up cross-compilation CI for `riscv64gc-unknown-linux-gnu`
- [ ] Set up cross-compilation CI for `armv7-unknown-linux-gnueabihf`
- [ ] Add Milk-V Duo hardware-in-the-loop test (or QEMU emulation as proxy)
- [ ] Benchmark block import time on Milk-V Duo hardware
- [ ] Benchmark memory usage on Milk-V Duo (budget: 64MB total, node must fit in ~48MB leaving OS headroom)
- [ ] Document verified edge hardware support in README with real numbers

### 7.2 PQC extension (`jamliquor-pqc`)

Use existing, audited Rust crates. Do not implement these from scratch.

```toml
[dependencies]
pqcrypto-dilithium = "latest"  # Dilithium3
pqcrypto-kyber = "latest"      # Kyber768
```

- [ ] Wrap Dilithium3 for validator signing (opt-in replacement for Ed25519)
- [ ] Wrap Kyber768 for key encapsulation (peer-to-peer encrypted channels)
- [ ] XMSS: evaluate `xmss-rs` crate maturity before committing
- [ ] Feature-flag all PQC behind `--features pqc`
- [ ] Document performance overhead on edge hardware

### 7.3 Lite profile (`jamliquor-lite`)

- [ ] Define memory budget per component (state DB, PVM sandbox, networking buffers)
- [ ] Implement configurable cache size limits
- [ ] Implement deferred state root computation (trade latency for memory)
- [ ] Verify full block import works within 48MB on Milk-V Duo

### 7.4 TinyML extension (`jamliquor-ai`) — lowest priority

This is the most speculative component. Do not build until M1 is done.

- [ ] Define what "AI-adaptive" means concretely (specific decisions the model makes)
- [ ] Evaluate `tract` crate (ONNX runtime in pure Rust, no C deps) for edge inference
- [ ] Start with a single, small use case: network condition classification
- [ ] Gate behind `--features ai`

---

## Immediate Next Actions (This Week)

In order, no skipping:

1. `git pull` — check if any commits happened since September 2025
2. `cargo update` — update all dependencies to latest stable
3. `cargo build` — does it even compile today?
4. `cargo test --all-targets` — baseline pass rate
5. Clone `w3f/jamtestvectors` and run codec vectors against current impl
6. Record which subsystems pass and which fail
7. Open a `REVIVAL.md` in the repo root documenting the gap analysis

**Do not start Phase 1 until step 6 is done.**

---

## Framework Check

Against the build-with-AI framework:

| Question | Answer |
|---|---|
| One giant company update away? | **Yes, partially.** Every Gray Paper revision can invalidate implementation work. The GP is not stable until post-audit. Hedge: pass M1 now while spec is near-frozen. |
| Building a process or just using a model? | Building a process. Correct. |
| Gets smarter with every user? | No. This is infrastructure. Network resilience improves with node count. Rephrase the moat: edge-node diversity is the defensible position, not data accumulation. |
| Users lose something irreplaceable if shut down? | Not yet. Nothing at M1. At M2+ with an active validator set, yes. |
| Building a community or just an audience? | The manifesto is audience-facing. The JAM Prize process is community-facing (conformance chat, fellowship). Pick one and be honest about it. |
| Value in output or everything around it? | The output (a JAM node) is not differentiated. Everything around it — edge-first, PQC, auditable cleanroom — is the value. Only true if it actually works on edge hardware with measured results. |
| Building what the platform will do for free? | The base JAM node, eventually. The edge-hardware story is what the platform cannot do. Protect that. |

---

## References

- Official test vectors: https://github.com/w3f/jamtestvectors
- Unofficial extended vectors: https://github.com/davxy/jam-test-vectors
- Gray Paper v0.8: https://graypaper.com
- JAM Prize rules: https://jam.web3.foundation
- JAM client list: https://graypaper.com/clients
- JAM conformance docs: https://docs.jamcha.in/testing/conformance
- PolkaVM: https://github.com/paritytech/polkavm
- QUIC in Rust (quinn): https://github.com/quinn-rs/quinn
- pqcrypto crates: https://github.com/rustpq/pqcrypto
