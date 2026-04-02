# JamLiquor Code to Gray Paper v0.8 Mapping

**Audit Date:** 2026-04-02  
**Target Spec:** Gray Paper v0.8 (near-final pre-audit draft)  
**Codebase Last Active:** ~September 2025  

## Overview

This document maps every JamLiquor data structure and function against the corresponding Gray Paper v0.8 sections to identify alignment gaps and breaking changes.

## File-by-File Analysis

### src/schema.rs

| Struct/Function | GP Section | Status | Notes |
|----------------|------------|--------|-------|
| `BlockchainError` | Various | ⚠️ **Needs Review** | Error types may need updates for v0.8 validation rules |
| `ValidationResult` | §3.1 Validation | ✅ **Likely OK** | Basic success/failure enum, should be compatible |
| `State` | §4 State Management | ⚠️ **Needs Review** | Missing Service Account integration |
| `TicketState` | §5.2 Tickets | ⚠️ **Needs Review** | May not reflect v0.8 ticket format changes |
| `OpaqueHash` | §2.1 Cryptography | ✅ **OK** | Standard 32-byte hash, compatible |
| `EpochMark` | §5.1 Epochs | ⚠️ **Breaking Change** | Missing Service Account elements from v0.8 |
| `Header` | §3.2 Block Header | ⚠️ **Breaking Change** | `entropy_source` field may be outdated format |
| `TicketEnvelope` | §5.2 Tickets | ⚠️ **Needs Review** | Signature validation may need updates |
| `TicketBody` | §5.2 Tickets | ⚠️ **Needs Review** | Field alignment with v0.8 needs verification |
| `Preimage` | §5.3 Preimages | ⚠️ **Needs Review** | Storage key representation may have changed |
| `Extrinsic` | §3.3 Extrinsics | ⚠️ **Breaking Change** | Missing PVM integration fields |
| `Block` | §3 Blocks | ✅ **OK** | Container structure, depends on inner structs |

**Key Issues in schema.rs:**
1. **EpochMark**: Missing Service Account fields added in v0.8
2. **Header**: `entropy_source` format may have changed
3. **Extrinsic**: No PVM-related fields for M1 conformance
4. **Preimage**: Storage key read/write representation changes in v0.8

### src/state.rs

| Struct/Function | GP Section | Status | Notes |
|----------------|------------|--------|-------|
| `State::apply_block()` | §4 State Transitions | ⚠️ **Partial** | Basic slot validation, missing complex state logic |
| `State::validate_state_consistency()` | §4.1 Consistency | ⚠️ **Basic** | Simple counter check, needs GP v0.8 rules |
| `State::get_last_slot()` | §4.2 Slot Tracking | ✅ **OK** | Simple getter, compatible |
| `State::get_counter()` | §4.3 Metrics | ⚠️ **Basic** | May not reflect v0.8 state metrics |

**Key Issues in state.rs:**
1. Missing Service Account state management
2. No PVM state integration
3. Basic validation logic vs comprehensive GP v0.8 rules

### src/coretime.rs

| Struct/Function | GP Section | Status | Notes |
|----------------|------------|--------|-------|
| `CoreTimeLedger` | §6 CoreTime | ✅ **Likely OK** | Basic accounting structure |
| `CoreUsage` | §6.1 Per-Core Tracking | ✅ **OK** | Simple usage tracking |
| `CoreTimeGuarantee` | §6.2 Guarantees | ⚠️ **Needs Review** | Format may have changed in v0.8 |
| `validate_and_apply()` | §6.3 Validation | ⚠️ **Partial** | Basic validation, may miss v0.8 rules |
| Constants (MAX_*) | §6.4 Limits | ⚠️ **Needs Review** | May need updates for v0.8 |

**Key Issues in coretime.rs:**
1. CoreTime logic appears mostly compatible
2. Constants may need verification against v0.8 limits
3. Validation rules may be incomplete

### src/importer.rs

| Struct/Function | GP Section | Status | Notes |
|----------------|------------|--------|-------|
| `Importer` | §3 Block Processing | ✅ **OK** | Basic importer structure |
| `import_block()` | §3.4 Block Import | ⚠️ **Partial** | Missing v0.8 validation steps |
| `validate_and_apply_block()` | §3.5 Validation Pipeline | ⚠️ **Incomplete** | Missing validator judgments, PVM |
| `validate_block_structure()` | §3.1 Structure | ⚠️ **Basic** | May not catch v0.8 structural issues |
| `validate_header()` | §3.2 Header Validation | ⚠️ **Partial** | Missing entropy source validation |
| `validate_extrinsic()` | §3.3 Extrinsic Validation | ⚠️ **Basic** | No PVM validation |

**Key Issues in importer.rs:**
1. Missing validator judgment logic
2. No PVM boot/validation (required for M1)
3. Basic header validation vs comprehensive v0.8 rules
4. No Service Account validation

### src/main.rs

| Function | GP Section | Status | Notes |
|----------|------------|--------|-------|
| `main()` | - | ✅ **OK** | CLI entry point |
| `test_block_import()` | Test Harness | ✅ **OK** | Basic test, needs v0.8 vectors |

## Breaking Changes from v0.6.7 → v0.8

Based on TODO.md and code analysis:

### 1. Service Account Elements (June 2025)
- **Affected:** `EpochMark`, `State`, `Importer`
- **Impact:** High - Service Accounts are core to v0.8
- **Effort:** 8-12 hours

### 2. Storage Key Representation (June 2025)  
- **Affected:** `Preimage`, state transition logic
- **Impact:** High - Affects all storage operations
- **Effort:** 4-6 hours

### 3. PVM Integration (June 2025)
- **Affected:** `Extrinsic`, `Importer`, validation logic
- **Impact:** Critical - Required for M1 conformance
- **Effort:** 16-24 hours

### 4. Aggregating Scheduler (June 2025)
- **Affected:** CoreTime logic, state management
- **Impact:** Medium - May affect CoreTime validation
- **Effort:** 6-8 hours

### 5. Proof-of-Provisioning (June 2025)
- **Affected:** Header validation, entropy sources
- **Impact:** Medium - May affect `entropy_source` field
- **Effort:** 4-6 hours

## Implementation Timeline vs Gray Paper

| Git Commit | Date | Likely GP Version | Notes |
|------------|------|-------------------|-------|
| f4726a7 | 2024-?? | Pre-v0.6.7 | Initial implementation |
| 2429370 | 2025-06-?? | v0.6.7 | "task 2.1 JAM datastructure" |
| 13d5c29 | 2025-08-?? | v0.7.0 | After breaking changes |
| Latest | 2025-09-?? | v0.7.0 | Last active development |

**Conclusion:** Most code was written against v0.6.7 or early v0.7.0, before major breaking changes.

## Priority Assessment

### Priority 1: M1 Blockers (Critical)
- PVM integration in `Extrinsic` and validation
- Service Account elements in `EpochMark`
- Storage key representation in `Preimage`

### Priority 2: Conformance Issues (High)
- Header validation updates for PoP
- Validator judgment logic in `Importer`
- State transition completeness

### Priority 3: Logic Updates (Medium)
- CoreTime validation rules
- Error type updates
- Constants verification

## Next Steps

1. **Run official vectors** - Identify specific failures
2. **Map each failure** - To specific breaking changes
3. **Prioritize fixes** - Based on M1 requirements
4. **Implement incrementally** - Start with PVM integration

---

**Status:** Audit complete, ready for vector testing phase.
