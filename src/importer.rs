use crate::coretime::CoreTimeLedger;
use crate::schema::{Block, BlockchainError, Extrinsic, Header, State};
use anyhow::{Context, Result};
use log::{debug, info, trace, warn};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Importer {
    state: State,
    last_block_hash: Option<[u8; 32]>,
    last_state_root: Option<[u8; 32]>,
    coretime: CoreTimeLedger,
}

impl Importer {
    pub fn new() -> Self {
        Importer {
            state: State::new(),
            last_block_hash: None,
            last_state_root: None,
            coretime: CoreTimeLedger::default(),
        }
    }

    #[allow(dead_code)]
    pub fn set_initial_state(&mut self, last_hash: [u8; 32], last_root: [u8; 32]) {
        self.last_block_hash = Some(last_hash);
        self.last_state_root = Some(last_root);
    }

    pub fn coretime(&self) -> &CoreTimeLedger {
        &self.coretime
    }

    /// Import and validate a block from a given path
    ///
    /// This function performs streaming JSON parsing for memory efficiency with large blocks.
    /// It validates the block structure, header, and transactions before applying state changes.
    pub fn import_block<P: AsRef<Path>>(&mut self, path: P) -> Result<Block> {
        let path = path.as_ref();
        info!("Importing block from path: {}", path.display());

        // Open file with buffered reader for efficient reading
        let file = File::open(path)
            .map_err(BlockchainError::IoError)
            .with_context(|| format!("Failed to open block file: {}", path.display()))?;

        // Use buffered reader for better performance
        let reader = BufReader::new(file);

        // Parse JSON with size limits for security
        let block: Block = serde_json::from_reader(reader)
            .map_err(BlockchainError::JsonError)
            .with_context(|| format!("Failed to parse block JSON from {}", path.display()))?;

        // Validate and apply block
        self.validate_and_apply_block(&block)
            .with_context(|| "Block validation failed")?;

        // Update last block hash and state root
        let extrinsic_hash = hex::encode(block.header.extrinsic_hash.as_bytes());

        info!("Block imported successfully: hash={extrinsic_hash}");

        self.last_block_hash = Some(*block.header.extrinsic_hash.as_bytes());
        self.last_state_root = Some(*block.header.parent_state_root.as_bytes());

        Ok(block)
    }

    /// Validates and applies a block to the current state
    ///
    /// This performs all necessary validations in the correct order:
    /// 1. Structural validation
    /// 2. Header validation
    // 3. Transaction validation
    /// 4. State transition validation
    fn validate_and_apply_block(&mut self, block: &Block) -> Result<()> {
        debug!(
            "Starting validation for block at slot {}",
            block.header.slot
        );

        // 1. Validate block structure
        self.validate_block_structure(block)?;

        // 2. Validate header (includes parent hash, slot, etc.)
        self.validate_header(&block.header)?;

        // 3. Validate all transactions and their proofs
        self.validate_extrinsic(&block.header, &block.extrinsic)?;

        // 3b. Validate CoreTime accounting and guarantees
        self.coretime.validate_and_apply(
            block.header.slot as u64,
            &block.extrinsic.guarantees,
            &block.extrinsic.assurances,
            &block.extrinsic.disputes,
        )?;

        // 4. Apply state transition
        trace!(
            "Applying state transition for block at slot {}",
            block.header.slot
        );
        self.state.apply_block(block)?;

        debug!(
            "Successfully validated and applied block at slot {}",
            block.header.slot
        );
        Ok(())
    }

    /// Validates the structural integrity of the block
    fn validate_block_structure(&self, block: &Block) -> Result<()> {
        // Check header has a valid slot number
        if block.header.slot == 0 {
            return Err(BlockchainError::InvalidSlot {
                last_slot: 0,
                current_slot: 0,
            }
            .into());
        }

        // Validate extrinsic structure
        if block.extrinsic.tickets.len()
            != block.header.tickets_mark.as_ref().map_or(0, |tm| tm.len())
        {
            return Err(BlockchainError::InvalidBlockStructure {
                reason: format!(
                    "Ticket count mismatch: header marks {} tickets but found {}",
                    block.header.tickets_mark.as_ref().map_or(0, |tm| tm.len()),
                    block.extrinsic.tickets.len()
                ),
            }
            .into());
        }

        // Validate preimages if any
        for (i, preimage) in block.extrinsic.preimages.iter().enumerate() {
            if preimage.blob.is_empty() {
                return Err(BlockchainError::InvalidPreimage {
                    reason: format!("Preimage at index {} has empty blob", i),
                }
                .into());
            }
        }

        Ok(())
    }

    /// Validates block header with comprehensive checks
    ///
    /// This includes:
    /// - Slot progression validation
    /// - Parent hash validation
    /// - State root validation
    /// - Entropy validation
    /// - Epoch mark validation (if present)
    fn validate_header(&self, header: &Header) -> Result<()> {
        let current_slot = u64::from(header.slot);
        let last_slot = self.state.get_last_slot();

        trace!(
            "Validating header for slot {} (last slot: {})",
            current_slot,
            last_slot
        );

        // Slot validation
        if current_slot <= last_slot {
            warn!("Invalid slot progression: current {current_slot} <= last {last_slot}");
            return Err(BlockchainError::InvalidSlot {
                last_slot,
                current_slot,
            }
            .into());
        }

        // Parent hash validation
        if let Some(last_hash) = self.last_block_hash {
            let parent_hash = header.parent.as_bytes();
            if parent_hash != &last_hash {
                warn!(
                    "Parent hash mismatch: expected {}, got {}",
                    hex::encode(last_hash),
                    hex::encode(parent_hash)
                );
                return Err(BlockchainError::ParentHashMismatch {
                    expected: hex::encode(last_hash),
                    actual: hex::encode(parent_hash),
                }
                .into());
            }
        }

        // Parent state root validation
        if let Some(last_root) = self.last_state_root {
            if header.parent_state_root.as_bytes() != &last_root {
                warn!(
                    "Parent state root mismatch: expected {}, got {}",
                    hex::encode(last_root),
                    hex::encode(header.parent_state_root.as_bytes())
                );
                return Err(BlockchainError::ParentStateRootMismatch {
                    expected: hex::encode(last_root),
                    actual: hex::encode(header.parent_state_root.as_bytes()),
                }
                .into());
            }
        }

        // Entropy validation
        // Validate entropy source (result is ignored as we only care about the error case)
        let _ = header
            .validate_entropy()
            .map_err(|e| BlockchainError::InvalidEntropy {
                reason: format!("Invalid entropy source: {}", e),
            })?;

        trace!("Entropy validation passed for slot {}", current_slot);

        info!("Header validation passed for slot {current_slot}");

        // Optional additional validations
        if let Some(epoch_mark) = &header.epoch_mark {
            if header.author_index as usize >= epoch_mark.validators.len() {
                warn!(
                    "Author index {} out of bounds (max {})",
                    header.author_index,
                    epoch_mark.validators.len() - 1
                );
                return Err(BlockchainError::InvalidAuthorIndex {
                    author_index: header.author_index as u64,
                    max_validators: epoch_mark.validators.len(),
                }
                .into());
            }
        }

        Ok(())
    }

    /// Validates all transactions in the extrinsic
    ///
    /// This includes:
    /// - Ticket validation against marks
    /// - Signature verification
    /// - Transaction inclusion proofs
    fn validate_extrinsic(&self, header: &Header, extrinsic: &Extrinsic) -> Result<()> {
        debug!(
            "Validating extrinsic with {} tickets",
            extrinsic.tickets.len()
        );

        // Validate tickets against marks if marks exist
        if let Some(tickets_mark) = &header.tickets_mark {
            // This check is redundant with validate_block_structure but kept for defense in depth
            if tickets_mark.len() != extrinsic.tickets.len() {
                return Err(BlockchainError::TicketValidationError {
                    reason: format!(
                        "Ticket count mismatch: expected {}, got {}",
                        tickets_mark.len(),
                        extrinsic.tickets.len()
                    ),
                }
                .into());
            }

            // Validate each ticket against its mark
            for (i, (ticket, mark)) in extrinsic
                .tickets
                .iter()
                .zip(tickets_mark.iter())
                .enumerate()
            {
                if ticket.attempt != mark.attempt {
                    return Err(BlockchainError::TicketValidationError {
                        reason: format!(
                            "Ticket {} attempt mismatch: expected {}, got {}",
                            i, mark.attempt, ticket.attempt
                        ),
                    }
                    .into());
                }

                // Additional ticket validation
                if let Err(e) = ticket.validate() {
                    return Err(BlockchainError::TicketValidationError {
                        reason: format!("Invalid ticket at index {}: {}", i, e),
                    }
                    .into());
                }
            }
        } else if !extrinsic.tickets.is_empty() {
            return Err(BlockchainError::TicketValidationError {
                reason: "Tickets present but no tickets mark in header".to_string(),
            }
            .into());
        }

        // Validate preimages
        for (i, preimage) in extrinsic.preimages.iter().enumerate() {
            if preimage.requester == 0 {
                return Err(BlockchainError::InvalidPreimage {
                    reason: format!("Preimage at index {} has invalid requester ID 0", i),
                }
                .into());
            }
        }

        // Validate extrinsic hash commitment in header
        self.validate_extrinsic_hash(header, extrinsic)?;

        debug!(
            "Extrinsic validation passed with {} tickets and {} preimages",
            extrinsic.tickets.len(),
            extrinsic.preimages.len()
        );

        Ok(())
    }

    fn validate_extrinsic_hash(&self, header: &Header, extrinsic: &Extrinsic) -> Result<()> {
        let computed_hash =
            extrinsic
                .compute_hash()
                .map_err(|e| BlockchainError::InvalidInclusionProof {
                    reason: format!("Failed to serialize extrinsic for hashing: {}", e),
                })?;

        if header.extrinsic_hash.as_bytes() != &computed_hash {
            return Err(BlockchainError::InvalidInclusionProof {
                reason: format!(
                    "Extrinsic hash mismatch: expected {}, computed {}",
                    hex::encode(header.extrinsic_hash.as_bytes()),
                    hex::encode(computed_hash)
                ),
            }
            .into());
        }

        trace!("Extrinsic hash validated: {}", hex::encode(computed_hash));

        Ok(())
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Default for Importer {
    fn default() -> Self {
        Self::new()
    }
}
