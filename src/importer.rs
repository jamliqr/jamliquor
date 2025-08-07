use crate::schema::{Block, BlockchainError, Extrinsic, Header, State};
use anyhow::{Context, Result};
use log::{info, warn};
use std::fs::File;
use std::path::Path;

pub struct Importer {
    state: State,
    last_block_hash: Option<[u8; 32]>,
    last_state_root: Option<[u8; 32]>,
}

impl Importer {
    pub fn new() -> Self {
        Importer {
            state: State::new(),
            last_block_hash: None,
            last_state_root: None,
        }
    }

    #[allow(dead_code)]
    pub fn set_initial_state(&mut self, last_hash: [u8; 32], last_root: [u8; 32]) {
        self.last_block_hash = Some(last_hash);
        self.last_state_root = Some(last_root);
    }

    /// Import and validate a block from a given path
    pub fn import_block<P: AsRef<Path>>(&mut self, path: P) -> Result<Block> {
        info!("Importing block from path: {}", path.as_ref().display());

        // Open and parse block file
        let file = File::open(&path)
            .with_context(|| format!("Failed to open block file: {}", path.as_ref().display()))?;
        let block: Block =
            serde_json::from_reader(file).with_context(|| "Failed to parse block JSON")?;

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

    fn validate_and_apply_block(&mut self, block: &Block) -> Result<()> {
        self.validate_header(&block.header)?;
        self.validate_extrinsic(&block.header, &block.extrinsic)?;
        self.state.apply_block(block)?;
        Ok(())
    }

    /// Validate block header with comprehensive checks
    fn validate_header(&self, header: &Header) -> Result<()> {
        let current_slot = u64::from(header.slot);
        let last_slot = self.state.get_last_slot();

        // Slot validation
        if current_slot <= last_slot {
            warn!(
                "Invalid slot progression: current {current_slot} <= last {last_slot}"
            );
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

        // Entropy validation (placeholder)
        let _entropy_source = header.validate_entropy()?;

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

    fn validate_extrinsic(&self, header: &Header, extrinsic: &Extrinsic) -> Result<()> {
        if let Some(tickets_mark) = &header.tickets_mark {
            if tickets_mark.len() != extrinsic.tickets.len() {
                return Err(anyhow::anyhow!(
                    "Tickets mark count ({}) mismatches extrinsic tickets count ({})",
                    tickets_mark.len(),
                    extrinsic.tickets.len()
                ));
            }

            for (ticket, mark) in extrinsic.tickets.iter().zip(tickets_mark.iter()) {
                if ticket.attempt != mark.attempt {
                    return Err(anyhow::anyhow!(
                        "Ticket attempt mismatch: {} vs {}",
                        ticket.attempt,
                        mark.attempt
                    ));
                }
            }
        }
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
