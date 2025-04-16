use crate::schema::Block;
use anyhow::Result;

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

    pub fn apply_block(&mut self, block: &Block) -> Result<()> {
        // Update slot
        self.last_slot = block.header.slot as u64;
        // Simple transition: increment counter (replace with JAM logic later)
        self.counter += block.extrinsic.preimages.len() as u64; // Count preimages instead of extrinsics
        Ok(())
    }
}
