//! Comprehensive integration tests for JamLiquor core functionality
//!
//! This module provides tests using test vectors to validate
//! core system behaviors and invariants.

use anyhow::Result;
use jamliquor::state::State;
use jamliquor::Importer;

use jamliquor::schema::{
    Block, Extrinsic, Header, OpaqueHash, Preimage, TicketBody, TicketEnvelope,
};
use serde_json::{to_value, Value};
use std::fs::File;
use tempfile::tempdir;

fn build_sample_block() -> (Block, Value) {
    let mut block = Block {
        header: Header {
            parent: OpaqueHash::new([
                0x5c, 0x74, 0x3d, 0xbc, 0x51, 0x42, 0x84, 0xb2, 0xea, 0x57, 0x79, 0x87, 0x87, 0xc5,
                0xa1, 0x55, 0xef, 0x9d, 0x7a, 0xc1, 0xe9, 0x49, 0x9e, 0xc6, 0x59, 0x10, 0xa7, 0xa3,
                0xd6, 0x58, 0x97, 0xb7,
            ]),
            parent_state_root: OpaqueHash::new([0x25; 32]),
            extrinsic_hash: OpaqueHash::new([0u8; 32]),
            slot: 42,
            epoch_mark: None,
            tickets_mark: Some(vec![TicketBody {
                id: OpaqueHash::new([3u8; 32]),
                attempt: 1,
            }]),
            offenders_mark: vec![OpaqueHash::new([4u8; 32])],
            author_index: 0,
            entropy_source: vec![5u8; 96],
            seal: vec![6u8; 32],
        },
        extrinsic: Extrinsic {
            tickets: vec![TicketEnvelope {
                attempt: 1,
                signature: vec![7u8; 64],
            }],
            preimages: vec![Preimage {
                requester: 1,
                blob: vec![8u8; 16],
            }],
            guarantees: Vec::new(),
            assurances: Vec::new(),
            disputes: Value::Null,
        },
    };

    let extrinsic_hash = block
        .extrinsic
        .compute_hash()
        .expect("failed to compute extrinsic hash");
    block.header.extrinsic_hash = OpaqueHash::new(extrinsic_hash);

    let block_json = to_value(&block).expect("failed to serialize block to JSON");
    (block, block_json)
}

fn write_block_json(value: &Value) -> Result<std::path::PathBuf> {
    let dir = tempdir()?;
    let file_path = dir.path().join("block.json");
    serde_json::to_writer(File::create(&file_path)?, value)?;
    std::mem::forget(dir);
    Ok(file_path)
}

/// Test block import from vector
#[test]
fn test_block_import_from_vector() -> Result<()> {
    let mut importer = Importer::new();
    let (expected_block, block_json) = build_sample_block();
    let vector_path = write_block_json(&block_json)?;

    // Try importing the block
    let block = importer.import_block(&vector_path)?;

    // Validate block properties from the vector
    assert_eq!(
        hex::encode(block.header.parent.as_bytes()),
        hex::encode(expected_block.header.parent.as_bytes()),
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
    let (block_template, block_json) = build_sample_block();
    let block_path = write_block_json(&block_json)?;
    let block = importer.import_block(&block_path)?;

    // Modify block to have a valid slot
    let mut block = block;
    block.header.slot = 43;
    block.header.parent = block_template.header.parent;

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
        let (_, block_json) = build_sample_block();
        let vector_path = write_block_json(&block_json).expect("failed to write block json");

        let result = importer.import_block(&vector_path);
        assert!(result.is_ok(), "Block import should succeed");

        let block = result.unwrap();
        assert_eq!(block.header.slot, 42, "Block should have slot 42");
    }

    #[test]
    fn prop_state_transition_stability() {
        let mut state = State::new();
        let mut importer = Importer::new();

        let (_, block_json) = build_sample_block();
        let vector_path = write_block_json(&block_json).expect("failed to write block json");
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
    let (_, block_json) = build_sample_block();
    let vector_contents = serde_json::to_string_pretty(&block_json)?;

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
