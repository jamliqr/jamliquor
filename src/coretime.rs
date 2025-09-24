use crate::schema::BlockchainError;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Maximum CoreTime that can be consumed by a single core within one block.
pub const MAX_CORETIME_PER_CORE: u64 = 1_024;
/// Maximum total CoreTime that can be consumed within a single block.
pub const MAX_CORETIME_PER_BLOCK: u64 = 4_096;
/// Maximum number of assurances expected per block. Used as a sanity check.
pub const MAX_ASSURANCES_PER_BLOCK: usize = 256;
/// Maximum dispute age allowed to be processed alongside a block.
pub const MAX_DISPUTE_AGE: u64 = 64;
/// Maximum difference between the current block slot and a guarantee slot.
pub const MAX_GUARANTEE_LOOKBACK: u64 = 8;

#[derive(Debug, Default)]
struct CoreUsage {
    total_consumed: u64,
    last_block_slot: u64,
    last_block_consumed: u64,
}

/// Tracks CoreTime allocations and consumption over time.
#[derive(Debug, Default)]
pub struct CoreTimeLedger {
    total_allocated: u64,
    total_consumed: u64,
    last_block_slot: Option<u64>,
    per_core_usage: HashMap<u16, CoreUsage>,
}

impl CoreTimeLedger {
    /// Total CoreTime allocated across all processed blocks.
    pub fn total_allocated(&self) -> u64 {
        self.total_allocated
    }

    /// Total CoreTime consumed across all processed blocks.
    pub fn total_consumed(&self) -> u64 {
        self.total_consumed
    }

    /// Returns the last block slot processed by the ledger, if any.
    pub fn last_block_slot(&self) -> Option<u64> {
        self.last_block_slot
    }

    /// Returns the total CoreTime consumed by a specific core, if tracked.
    pub fn per_core_consumed(&self, core_index: u16) -> Option<u64> {
        self.per_core_usage
            .get(&core_index)
            .map(|usage| usage.total_consumed)
    }

    /// Validate and apply the CoreTime information contained within a block.
    pub fn validate_and_apply(
        &mut self,
        block_slot: u64,
        guarantees: &[Value],
        assurances: &[Value],
        disputes: &Value,
    ) -> Result<(), BlockchainError> {
        if guarantees.is_empty() && assurances.is_empty() && disputes.is_null() {
            self.last_block_slot = Some(block_slot);
            return Ok(());
        }

        let mut per_block_usage: HashMap<u16, u64> = HashMap::new();
        let mut total_block_usage: u64 = 0;
        let mut unique_allocations: HashSet<(u64, u16)> = HashSet::new();

        for value in guarantees {
            let guarantee: CoreTimeGuarantee =
                serde_json::from_value(value.clone()).map_err(|e| {
                    BlockchainError::CoreTimeValidationError {
                        reason: format!("Invalid guarantee format: {e}"),
                    }
                })?;

            if guarantee.slot > block_slot {
                return Err(BlockchainError::CoreTimeValidationError {
                    reason: format!(
                        "Guarantee references future slot {} (current block slot {})",
                        guarantee.slot, block_slot
                    ),
                });
            }

            if block_slot - guarantee.slot > MAX_GUARANTEE_LOOKBACK {
                return Err(BlockchainError::CoreTimeValidationError {
                    reason: format!(
                        "Guarantee slot {} exceeds lookback window {}",
                        guarantee.slot, MAX_GUARANTEE_LOOKBACK
                    ),
                });
            }

            let allocation_key = (guarantee.slot, guarantee.report.core_index);
            if !unique_allocations.insert(allocation_key) {
                return Err(BlockchainError::CoreTimeValidationError {
                    reason: format!(
                        "Duplicate guarantee for core {} at slot {}",
                        guarantee.report.core_index, guarantee.slot
                    ),
                });
            }

            let mut core_consumption = 0u64;
            for result in &guarantee.report.results {
                core_consumption = core_consumption
                    .checked_add(result.accumulate_gas)
                    .ok_or_else(|| BlockchainError::CoreTimeBalanceError {
                        reason: "CoreTime consumption overflow".to_string(),
                    })?;
            }

            if let Some(auth_gas_used) = guarantee.report.auth_gas_used {
                core_consumption =
                    core_consumption.checked_add(auth_gas_used).ok_or_else(|| {
                        BlockchainError::CoreTimeBalanceError {
                            reason: "CoreTime authentication gas overflow".to_string(),
                        }
                    })?;
            }

            let entry = per_block_usage
                .entry(guarantee.report.core_index)
                .or_insert(0);
            *entry = entry.checked_add(core_consumption).ok_or_else(|| {
                BlockchainError::CoreTimeBalanceError {
                    reason: format!(
                        "Core {} CoreTime consumption overflow",
                        guarantee.report.core_index
                    ),
                }
            })?;

            if *entry > MAX_CORETIME_PER_CORE {
                return Err(BlockchainError::CoreTimeBalanceError {
                    reason: format!(
                        "Core {} exceeds per-block CoreTime limit ({} > {})",
                        guarantee.report.core_index, *entry, MAX_CORETIME_PER_CORE
                    ),
                });
            }

            total_block_usage =
                total_block_usage
                    .checked_add(core_consumption)
                    .ok_or_else(|| BlockchainError::CoreTimeBalanceError {
                        reason: "Block CoreTime consumption overflow".to_string(),
                    })?;
        }

        if total_block_usage > MAX_CORETIME_PER_BLOCK {
            return Err(BlockchainError::CoreTimeBalanceError {
                reason: format!(
                    "Block exceeds CoreTime limit ({} > {})",
                    total_block_usage, MAX_CORETIME_PER_BLOCK
                ),
            });
        }

        if assurances.len() > MAX_ASSURANCES_PER_BLOCK {
            return Err(BlockchainError::CoreTimeValidationError {
                reason: format!("Too many assurances in block: {}", assurances.len()),
            });
        }

        let mut seen_validators = HashSet::new();
        for value in assurances {
            let assurance: CoreTimeAssurance =
                serde_json::from_value(value.clone()).map_err(|e| {
                    BlockchainError::CoreTimeValidationError {
                        reason: format!("Invalid assurance format: {e}"),
                    }
                })?;

            if assurance.bitfield.trim().is_empty() {
                return Err(BlockchainError::CoreTimeValidationError {
                    reason: format!(
                        "Assurance for validator {} has empty bitfield",
                        assurance.validator_index
                    ),
                });
            }

            if !seen_validators.insert(assurance.validator_index) {
                return Err(BlockchainError::CoreTimeValidationError {
                    reason: format!(
                        "Duplicate assurance for validator {}",
                        assurance.validator_index
                    ),
                });
            }
        }

        if !disputes.is_null() {
            let disputes: CoreTimeDisputes =
                serde_json::from_value(disputes.clone()).map_err(|e| {
                    BlockchainError::CoreTimeValidationError {
                        reason: format!("Invalid dispute format: {e}"),
                    }
                })?;

            for verdict in disputes.verdicts {
                if verdict.age > MAX_DISPUTE_AGE {
                    return Err(BlockchainError::CoreTimeValidationError {
                        reason: format!(
                            "Dispute verdict age {} exceeds limit {}",
                            verdict.age, MAX_DISPUTE_AGE
                        ),
                    });
                }

                if verdict.votes.is_empty() {
                    return Err(BlockchainError::CoreTimeValidationError {
                        reason: format!("Dispute verdict {} has no votes", verdict.target),
                    });
                }
            }
        }

        self.total_allocated = self
            .total_allocated
            .checked_add(total_block_usage)
            .ok_or_else(|| BlockchainError::CoreTimeBalanceError {
                reason: "CoreTime allocation overflow".to_string(),
            })?;

        self.total_consumed = self
            .total_consumed
            .checked_add(total_block_usage)
            .ok_or_else(|| BlockchainError::CoreTimeBalanceError {
                reason: "CoreTime consumption overflow".to_string(),
            })?;

        self.last_block_slot = Some(block_slot);

        for (core_index, consumed) in per_block_usage {
            let usage = self
                .per_core_usage
                .entry(core_index)
                .or_insert_with(CoreUsage::default);
            usage.total_consumed = usage.total_consumed.checked_add(consumed).ok_or_else(|| {
                BlockchainError::CoreTimeBalanceError {
                    reason: format!("Core {} total consumption overflow", core_index),
                }
            })?;
            usage.last_block_slot = block_slot;
            usage.last_block_consumed = consumed;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct CoreTimeGuarantee {
    slot: u64,
    report: GuaranteeReport,
}

#[derive(Debug, Deserialize)]
struct GuaranteeReport {
    #[serde(default)]
    auth_gas_used: Option<u64>,
    core_index: u16,
    #[serde(default)]
    results: Vec<GuaranteeResult>,
}

#[derive(Debug, Deserialize)]
struct GuaranteeResult {
    #[serde(default)]
    accumulate_gas: u64,
    #[serde(default)]
    #[allow(dead_code)]
    service_id: u64,
}

#[derive(Debug, Deserialize)]
struct CoreTimeAssurance {
    bitfield: String,
    validator_index: u16,
}

#[derive(Debug, Deserialize)]
struct CoreTimeDisputes {
    #[serde(default)]
    verdicts: Vec<DisputeVerdict>,
}

#[derive(Debug, Deserialize)]
struct DisputeVerdict {
    target: String,
    #[serde(default)]
    age: u64,
    #[serde(default)]
    votes: Vec<DisputeVote>,
}

#[derive(Debug, Deserialize)]
struct DisputeVote {
    #[allow(dead_code)]
    vote: bool,
    #[allow(dead_code)]
    index: u16,
    #[allow(dead_code)]
    signature: String,
}
