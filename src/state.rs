use crate::schema::{Block, BlockchainError};
use anyhow::Result;
use log::{info, warn};

#[derive(Debug, Default)]
pub struct State {
    last_slot: u64, // Tracks last processed slot
    counter: u64,   // Placeholder for state (e.g., from extrinsics)
}

impl State {
    pub fn get_last_slot(&self) -> u64 {
        self.last_slot
    }

    pub fn get_counter(&self) -> u64 {
        self.counter
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// Apply block to the current state with comprehensive validation
    pub fn apply_block(&mut self, block: &Block) -> Result<()> {
        // Validate slot progression
        let current_slot = block.header.slot as u64;
        if self.last_slot > 0 {
            // Reject slots less than or equal to the last processed slot
            if current_slot <= self.last_slot {
                warn!("Slot must be strictly greater than last processed slot: current {} <= last processed slot {}", current_slot, self.last_slot);
                return Err(BlockchainError::InvalidSlot {
                    last_slot: self.last_slot,
                    current_slot,
                }
                .into());
            }
        } else if current_slot < 43 {
            // First active block must be at or above slot 43
            warn!(
                "First active block must be at or above slot 43, got {}",
                current_slot
            );
            return Err(BlockchainError::InvalidSlot {
                last_slot: 0,
                current_slot,
            }
            .into());
        }

        // Update slot
        info!("Processing block at slot: {}", current_slot);
        self.last_slot = current_slot;

        // Advanced state transition logic
        let preimage_count = block.extrinsic.preimages.len() as u64;
        if preimage_count > 0 {
            info!("Processing {} preimages", preimage_count);
            self.counter += preimage_count;
        }

        // Log state metrics
        info!(
            "State update: last_slot={}, counter={}",
            self.last_slot, self.counter
        );

        Ok(())
    }

    /// Validate state consistency
    pub fn validate_state_consistency(&self) -> Result<()> {
        if self.counter < self.last_slot {
            warn!(
                "Potential state inconsistency: counter {} < last_slot {}",
                self.counter, self.last_slot
            );
            return Err(BlockchainError::StateTransitionError {
                reason: "Counter lower than last processed slot".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
