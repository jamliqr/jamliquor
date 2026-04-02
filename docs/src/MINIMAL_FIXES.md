# Minimal Fixes for M1 Conformance

**Priority:** Based on audit failure analysis and M1 requirements  
**Estimated Total Effort:** 40-58 hours  
**Target:** Get JamLiquor passing official M1 test vectors

## Priority 1: Critical Blockers (Required for any test to pass)

### 1.1 Fix Ticket Count Validation Logic
**Estimated Effort:** 2-4 hours  
**Files:** `src/importer.rs`  
**Issue:** `Ticket count mismatch: header marks 0 tickets but found 3`

**Root Cause:** Current validation expects exact match between `header.tickets_mark` length and `extrinsic.tickets` length, but v0.8 allows tickets when `tickets_mark` is null.

**Fix Steps:**
```rust
// In validate_block_structure() - update logic:
if header.tickets_mark.is_some() {
    let expected_count = header.tickets_mark.as_ref().unwrap().len();
    let actual_count = extrinsic.tickets.len();
    if expected_count != actual_count {
        return Err(BlockchainError::InvalidBlockStructure {
            reason: format!("Ticket count mismatch: header marks {} tickets but found {}", 
                          expected_count, actual_count)
        });
    }
} else {
    // v0.8: tickets_mark = null means no epoch transition, but tickets may still exist
    // Allow any number of tickets when tickets_mark is null
}
```

**Verification:** Run `test_importer_with_official_vectors` should pass

### 1.2 Add Basic PVM Integration
**Estimated Effort:** 16-24 hours  
**Files:** `src/schema.rs`, `src/importer.rs`  
**Issue:** Missing PVM fields and validation (required for M1)

**Fix Steps:**
1. **Add PVM fields to Extrinsic:**
```rust
pub struct Extrinsic {
    pub tickets: Vec<TicketEnvelope>,
    pub preimages: Vec<Preimage>,
    pub guarantees: Vec<serde_json::Value>,
    pub assurances: Vec<serde_json::Value>,
    pub disputes: serde_json::Value,
    // NEW: PVM integration
    pub pvm_boot: Option<PVMBootData>,
    pub pvm_code: Option<Vec<u8>>,
}
```

2. **Add PVMBootData structure:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct PVMBootData {
    pub code_hash: OpaqueHash,
    pub input_data: Vec<u8>,
    pub gas_limit: u64,
}
```

3. **Add PVM validation:**
```rust
fn validate_pvm(&self, extrinsic: &Extrinsic) -> Result<()> {
    if let Some(pvm_boot) = &extrinsic.pvm_boot {
        // Validate PVM boot parameters
        // Check code hash format
        // Validate gas limit
        // Add host call validation
    }
    Ok(())
}
```

**Verification:** PVM-related test vectors should pass (when available)

## Priority 2: High Impact (Required for M1 conformance)

### 2.1 Implement Service Account Integration
**Estimated Effort:** 8-12 hours  
**Files:** `src/schema.rs`, `src/importer.rs`  
**Issue:** Missing Service Account elements in EpochMark

**Fix Steps:**
1. **Update EpochMark structure:**
```rust
pub struct EpochMark {
    pub entropy: OpaqueHash,
    pub tickets_entropy: OpaqueHash,
    pub validators: Vec<OpaqueHash>,
    // NEW: Service Account elements
    pub service_accounts: Vec<ServiceAccount>,
    pub service_account_root: OpaqueHash,
}
```

2. **Add ServiceAccount structure:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    pub id: u32,
    pub balance: u64,
    pub code_hash: Option<OpaqueHash>,
    pub storage_root: OpaqueHash,
}
```

3. **Update validation logic:**
```rust
fn validate_service_accounts(&self, epoch_mark: &EpochMark) -> Result<()> {
    // Validate service account consistency
    // Check storage root format
    // Validate code hashes
    Ok(())
}
```

**Verification:** Epoch validation should pass with Service Account data

### 2.2 Implement Validator Judgment Logic
**Estimated Effort:** 8-12 hours  
**Files:** `src/importer.rs`, `src/schema.rs`  
**Issue:** No validator judgment processing

**Fix Steps:**
1. **Add ValidatorJudgment structure:**
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatorJudgment {
    pub validator_index: u16,
    pub judgment_type: JudgmentType,
    pub evidence: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JudgmentType {
    Valid,
    Invalid,
    Abstain,
}
```

2. **Add to Extrinsic:**
```rust
pub struct Extrinsic {
    // ... existing fields
    pub validator_judgments: Vec<ValidatorJudgment>,
}
```

3. **Add validation logic:**
```rust
fn validate_validator_judgments(&self, extrinsic: &Extrinsic, header: &Header) -> Result<()> {
    for judgment in &extrinsic.validator_judgments {
        // Validate validator index bounds
        if judgment.validator_index >= header.epoch_mark.validators.len() as u16 {
            return Err(BlockchainError::InvalidAuthorIndex {
                author_index: judgment.validator_index as u64,
                max_validators: header.epoch_mark.validators.len(),
            });
        }
        // Validate judgment format
        // Check evidence integrity
    }
    Ok(())
}
```

**Verification:** Validator judgment test vectors should pass

## Priority 3: Medium Impact (Improves conformance)

### 3.1 Update Storage Key Representation
**Estimated Effort:** 4-6 hours  
**Files:** `src/schema.rs`, `src/state.rs`  
**Issue:** Storage key format may have changed in v0.8

**Fix Steps:**
1. **Update Preimage structure:**
```rust
pub struct Preimage {
    pub requester: u32,
    pub blob: Vec<u8>,
    // NEW: v0.8 storage key format
    pub storage_key: StorageKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageKey {
    pub account_id: Option<u32>,
    pub key_path: Vec<u8>,
}
```

2. **Update state transition logic:**
```rust
fn apply_preimage(&mut self, preimage: &Preimage) -> Result<()> {
    // Use new storage key format
    let storage_location = match &preimage.storage_key.account_id {
        Some(account_id) => format!("account:{}:{}", account_id, hex::encode(&preimage.storage_key.key_path)),
        None => format!("global:{}", hex::encode(&preimage.storage_key.key_path)),
    };
    // Apply to state
    Ok(())
}
```

**Verification:** Preimage test vectors should pass

### 3.2 Update CoreTime Validation Rules
**Estimated Effort:** 4-6 hours  
**Files:** `src/coretime.rs`  
**Issue:** Constants and validation may need v0.8 updates

**Fix Steps:**
1. **Review and update constants:**
```rust
// Check against v0.8 specification
pub const MAX_CORETIME_PER_CORE: u64 = 1_024;  // Verify this value
pub const MAX_CORETIME_PER_BLOCK: u64 = 4_096; // Verify this value
```

2. **Add new validation rules:**
```rust
fn validate_coretime_v08(&self, block_slot: u64, guarantees: &[CoreTimeGuarantee]) -> Result<()> {
    // Add any new v0.8 specific rules
    // Check aggregating scheduler requirements
    // Validate proof-of-provisioning integration
    Ok(())
}
```

**Verification:** CoreTime test vectors should pass

## Implementation Order

### Week 1 (20-28 hours)
1. **Day 1:** Fix ticket count validation (2-4 hours)
2. **Day 2-4:** Implement basic PVM integration (16-24 hours)

### Week 2 (20-30 hours)  
1. **Day 5-6:** Implement Service Account integration (8-12 hours)
2. **Day 7-8:** Implement validator judgment logic (8-12 hours)
3. **Day 9:** Update storage key representation (4-6 hours)

### Week 3 (Optional - 8-12 hours)
1. **Day 10:** Update CoreTime validation rules (4-6 hours)
2. **Day 11:** Final testing and cleanup (4-6 hours)

## Success Criteria

### Phase Complete When:
- [ ] `test_importer_with_official_vectors` passes
- [ ] All official M1 block import vectors pass
- [ ] PVM boot validation works
- [ ] Service Account validation works
- [ ] Validator judgment processing works
- [ ] Storage key operations work

### M1 Conformance Achieved When:
- [ ] All official W3F test vectors pass
- [ ] Block import pipeline complete
- [ ] State transitions verified
- [ ] PVM execution validated
- [ ] CoreTime accounting correct

## Risk Mitigation

### Technical Risks
- **PVM Complexity:** Start with basic validation, extend as needed
- **Service Account Scope:** Focus on validation first, defer full state management
- **Breaking Changes:** Test each fix independently to isolate issues

### Timeline Risks
- **Underestimation:** Each fix includes 25% buffer for unknown issues
- **Dependencies:** PVM integration may uncover other issues
- **Testing:** Allocate time for comprehensive testing after each fix

## Notes

- All fixes should be backward compatible where possible
- Test vectors should be run after each major fix
- Documentation should be updated as structures change
- Consider feature flags for experimental features

---

**Total Estimated Effort:** 40-58 hours over 2-3 weeks  
**Confidence:** High - All issues have clear solutions  
**Next Step:** Begin with ticket count validation fix (Priority 1.1)
