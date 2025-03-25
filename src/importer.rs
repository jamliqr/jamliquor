use crate::schema::{Block, State};
use anyhow::Result;
use std::path::Path;
use std::fs::File;

pub struct Importer {
    state: State,
    last_block_hash: Option<[u8; 32]>, // Track last block hash for parent validation
}

impl Importer {
    pub fn new() -> Self {
        Importer { state: State::new(), last_block_hash: None }
    }

    pub fn import_block<P: AsRef<Path>>(&mut self, path: P) -> Result<Block> {
        let file = File::open(path)?;
        let block: Block = serde_json::from_reader(file)?;
        self.validate_and_apply_block(&block)?;
        self.last_block_hash = Some(*block.header.extrinsic_hash.as_bytes()); // Update last block hash
        Ok(block)
    }

    fn validate_and_apply_block(&mut self, block: &Block) -> Result<()> {
        // Basic validation: ensure slot increases
        if u64::from(block.header.slot) <= self.state.get_last_slot() {
            return Err(anyhow::anyhow!("Slot must increase: {}", block.header.slot));
        }

        // Validate parent hash (if not genersis)
        if let Some(last_hash) = self.last_block_hash {
            if block.header.parent.as_bytes() != &last_hash {
                return Err(anyhow::anyhow!("Parent hash mismatch"));
            }
        }

        // Apply state transition (to be refined with JAM rules)
        self.state.apply_block(block)?;
        Ok(())
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}
