//! Comprehensive integration tests for JamLiquor core functionality
//!
//! This module provides tests using test vectors to validate
//! core system behaviors and invariants.

use anyhow::Result;
use jamliquor::state::State;
use jamliquor::Importer;

use crate::utils;

/// Test block import from vector
#[test]
fn test_block_import_from_vector() -> Result<()> {
    let mut importer = Importer::new();
    let vector_path = utils::get_vector_path("codec/data/block.json");

    // Ensure the vector exists
    assert!(vector_path.exists(), "Block vector should exist");

    // Try importing the block
    let block = importer.import_block(&vector_path)?;

    // Validate block properties from the vector
    assert_eq!(
        hex::encode(block.header.parent.as_bytes()),
        "5c743dbc514284b2ea57798787c5a155ef9d7ac1e9499ec65910a7a3d65897b7",
        "Block parent hash should match vector"
    );
    assert_eq!(block.header.slot, 42, "Block slot should match vector");

    Ok(())
}

/// Test Safrole test vectors
/// TODO: Implement comprehensive Safrole vector testing
#[test]
#[ignore]
fn test_safrole_vectors() -> Result<()> {
    // Placeholder for future Safrole vector testing
    // This test is currently disabled and serves as a reminder
    // to implement comprehensive Safrole vector validation
    println!("Safrole vector testing is not yet fully implemented");
    Ok(())
}

/// Test state transition and validation
#[test]
fn test_state_transition() -> Result<()> {
    let mut state = State::new();
    let mut importer = Importer::new();

    // Simulate block import and state transition
    let block_path = utils::get_vector_path("codec/data/block.json");
    let block = importer.import_block(&block_path)?;

    // Modify block to have a valid slot
    let mut block = block;
    block.header.slot = 43;

    // Apply block to state
    state.apply_block(&block)?;

    // Validate state properties
    assert_eq!(
        state.get_last_slot(),
        43,
        "State last slot should be updated"
    );

    Ok(())
}

/// Property-based tests for core functionality
#[cfg(test)]
mod property_tests {
    use super::*;

    #[test]
    fn prop_block_import_sanity() {
        let mut importer = Importer::new();
        let vector_path = utils::get_vector_path("codec/data/block.json");

        let result = importer.import_block(&vector_path);
        assert!(result.is_ok(), "Block import should succeed");

        let block = result.unwrap();
        assert_eq!(block.header.slot, 42, "Block should have slot 42");
    }

    #[test]
    fn prop_state_transition_stability() {
        let mut state = State::new();
        let mut importer = Importer::new();

        let vector_path = utils::get_vector_path("codec/data/block.json");
        let mut block = importer.import_block(&vector_path).unwrap();

        // Modify block to have a valid slot
        block.header.slot = 43;

        let result = state.apply_block(&block);
        assert!(result.is_ok(), "State transition should be stable");
    }
}

/// Verify vector data integrity
#[test]
fn verify_vector_data_integrity() -> Result<()> {
    let vector_path = utils::get_vector_path("codec/data/block.json");

    // Read vector contents
    let vector_contents = std::fs::read_to_string(&vector_path)?;

    // Basic checks on vector data
    assert!(
        !vector_contents.is_empty(),
        "Vector data should not be empty"
    );
    assert!(
        vector_contents.contains("header"),
        "Vector should contain block header"
    );
    assert!(
        vector_contents.contains("slot"),
        "Vector should contain slot information"
    );

    Ok(())
}
