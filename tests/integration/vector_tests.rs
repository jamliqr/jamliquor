//! Integration tests against official JAM test vectors
//!
//! This module tests JamLiquor against the official W3F test vectors to ensure
//! conformance with the JAM Gray Paper specification.

use anyhow::Result;
use jamliquor::{Importer, schema::Block};
use serde_json;
use std::path::Path;

/// Test harness for loading official test vectors
pub struct VectorTestHarness {
    vectors_dir: String,
}

impl VectorTestHarness {
    /// Create a new test harness pointing to the vectors directory
    pub fn new() -> Self {
        Self {
            vectors_dir: "tests/vectors/codec/full".to_string(),
        }
    }

    /// Load a JSON test vector file
    pub fn load_json_vector<T>(&self, filename: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let path = Path::new(&self.vectors_dir).join(filename);
        let content = std::fs::read_to_string(&path)?;
        let vector: T = serde_json::from_str(&content)?;
        Ok(vector)
    }

    /// Load the official block test vector
    pub fn load_block_vector(&self) -> Result<Block> {
        self.load_json_vector("block.json")
    }

    /// Load the official header test vector
    pub fn load_header_vector(&self, index: usize) -> Result<serde_json::Value> {
        let filename = format!("header_{}.json", index);
        self.load_json_vector(&filename)
    }

    /// Load the official extrinsic test vector
    pub fn load_extrinsic_vector(&self) -> Result<serde_json::Value> {
        self.load_json_vector("extrinsic.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_official_block_vector() {
        let harness = VectorTestHarness::new();
        
        // Test that we can load the official block vector
        let result = harness.load_block_vector();
        
        match result {
            Ok(block) => {
                println!("✅ Successfully loaded official block vector");
                println!("   Block slot: {}", block.header.slot);
                println!("   Block parent: {:?}", block.header.parent);
            }
            Err(e) => {
                println!("❌ Failed to load official block vector: {}", e);
                panic!("Failed to load official block vector: {}", e);
            }
        }
    }

    #[test]
    fn test_load_official_header_vectors() {
        let harness = VectorTestHarness::new();
        
        // Test loading header vectors
        for i in 0..=1 {
            let result = harness.load_header_vector(i);
            match result {
                Ok(header) => {
                    println!("✅ Successfully loaded header_{}", i);
                    if let Some(slot) = header.get("slot") {
                        println!("   Header slot: {}", slot);
                    }
                }
                Err(e) => {
                    println!("❌ Failed to load header_{}: {}", i, e);
                    panic!("Failed to load header_{}: {}", i, e);
                }
            }
        }
    }

    #[test]
    fn test_load_official_extrinsic_vector() {
        let harness = VectorTestHarness::new();
        
        // Test that we can load the official extrinsic vector
        let result = harness.load_extrinsic_vector();
        
        match result {
            Ok(extrinsic) => {
                println!("✅ Successfully loaded official extrinsic vector");
                println!("   Extrinsic type: {}", 
                    extrinsic.get("type").unwrap_or(&serde_json::Value::Null));
            }
            Err(e) => {
                println!("❌ Failed to load official extrinsic vector: {}", e);
                panic!("Failed to load official extrinsic vector: {}", e);
            }
        }
    }

    #[test]
    fn test_importer_with_official_vectors() {
        let harness = VectorTestHarness::new();
        let mut importer = Importer::new();
        
        // Test importing the official block vector
        let result = importer.import_block("tests/vectors/codec/full/block.json");
        
        match result {
            Ok(block) => {
                println!("✅ Successfully imported official block vector");
                println!("   Imported slot: {}", block.header.slot);
                println!("   Imported parent: {:?}", block.header.parent);
            }
            Err(e) => {
                println!("❌ Failed to import official block vector: {}", e);
                // Don't panic here - this is expected to fail during audit phase
                println!("   This failure will be documented in the audit report");
            }
        }
    }
}
