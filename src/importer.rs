use crate::schema::{Block, State};
use anyhow::Result;
use std::path::Path;
use std::fs::File;

pub struct Importer {
    state: State,
}

impl Importer {
    pub fn new() -> Self {
        Importer { state: State::new() }
    }

    pub fn import_block<P: AsRef<Path>>(&mut self, path: P) -> Result<Block> {
        let file = File::open(path)?;
        let block: Block = serde_json::from_reader(file)?;
        self.validate_and_apply_block(&block)?;
        Ok(block)
    }

    fn validate_and_apply_block(&mut self, block: &Block) -> Result<()> {
        // Basic validation: ensure slot increases
        if u64::from(block.header.slot) <= self.state.get_last_slot() {
            return Err(anyhow::anyhow!("Slot must increase: {}", block.header.slot));
        }
        // Apply state transition (to be refined with JAM rules)
        self.state.apply_block(block)?;
        Ok(())
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}
