

#[derive(Debug)]
pub struct State {
    last_slot: u64,    // Tracks last processed slot
    counter: u64,      // Placeholder for state (e.g., from extrinsics)
}

impl Default for State {
    pub fn get_last_slot(&self) -> u64 {
        self.last_slot
    }
    pub fn get_counter(&self) -> u64 {
        self.counter
    }
}

impl State {
    pub fn new() -> Self {
        State { last_slot: 0, counter: 0 }
    }

    pub fn apply_block(&mut self, block: &Block) -> Result<()> {
        // Update slot
        self.last_slot = block.header.slot;
        // Simple transition: increment counter (replace with JAM logic later)
        self.counter += block.extrinsics.len() as u64; // Example: count extrinsics
        Ok(())
    }
}