use crate::schema::Block;
use anyhow::{Context, Result};
use std::fs;

pub struct Importer {
    test_vectors_dir: String,
}

impl Importer {
    pub fn new(test_vectors_dir: &str) -> Self {
        Importer {
            test_vectors_dir: test_vectors_dir.to_string(),
        }
    }

    pub fn import_block(&self, filename: &str) -> Result<Block> {
        let path = format!("{}/codec/data/{}", self.test_vectors_dir, filename);
        let json =
            fs::read_to_string(&path).with_context(|| format!("Failed to read file: {}", path))?;
        let block: Block =
            serde_json::from_str(&json).with_context(|| "Failed to parse block JSON")?;
        println!("Imported block at slot: {}", block.header.slot);
        Ok(block)
    }
}
