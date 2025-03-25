use crate::schema::{Block, Header, Extrinsic, State};
use anyhow::Result;
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

    pub fn set_initial_state(&mut self, last_hash: [u8; 32], last_root: [u8; 32]) {
        self.last_block_hash = Some(last_hash);
        self.last_state_root = Some(last_root);
    }

    pub fn import_block<P: AsRef<Path>>(&mut self, path: P) -> Result<Block> {
        let file = File::open(path)?;
        let block: Block = serde_json::from_reader(file)?;
        self.validate_and_apply_block(&block)?;
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

    fn validate_header(&self, header: &Header) -> Result<()> {
        if u64::from(header.slot) <= self.state.get_last_slot() {
            return Err(anyhow::anyhow!("Slot must increase: {}", header.slot));
        }
        if let Some(last_hash) = self.last_block_hash {
            if header.parent.as_bytes() != &last_hash {
                return Err(anyhow::anyhow!("Parent hash mismatch"));
            }
        }
        if let Some(last_root) = self.last_state_root {
            if header.parent_state_root.as_bytes() != &last_root {
                return Err(anyhow::anyhow!("Parent state root mismatch"));
            }
        }
        if let Some(epoch_mark) = &header.epoch_mark {
            if header.author_index as usize >= epoch_mark.validators.len() {
                return Err(anyhow::anyhow!(
                    "Author index {} out of bounds (max {})",
                    header.author_index,
                    epoch_mark.validators.len() - 1
                ));
            }
        }
        if header.offenders_mark.is_empty() {
            return Err(anyhow::anyhow!("Offenders mark cannot be empty"));
        }
        if header.seal.is_empty() {
            return Err(anyhow::anyhow!("Seal cannot be empty"));
        }
        if header.entropy_source.is_empty() {
            return Err(anyhow::anyhow!("Entropy source cannot be empty"));
        }
        Ok(())
    }

    fn validate_extrinsic(&self, header: &Header, extrinsic: &Extrinsic) -> Result<()> {
        // Restore tickets_mark validation here
        if let Some(tickets_mark) = &header.tickets_mark {
            if tickets_mark.len() != extrinsic.tickets.len() {
                return Err(anyhow::anyhow!(
                    "Tickets mark count ({}) mismatches extrinsic tickets count ({})",
                    tickets_mark.len(),
                    extrinsic.tickets.len()
                ));
            }
        }
        Ok(())
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}