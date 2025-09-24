use anyhow::Result;
use jamliquor::schema::{
    Block, BlockchainError, Extrinsic, Header, OpaqueHash, Preimage, TicketBody, TicketEnvelope,
};
use jamliquor::importer::Importer;
use std::fs::File;
use tempfile::tempdir;

/// Helper function to create a minimal valid block for testing
fn create_test_block() -> Block {
    // Create a zero hash with proper hex encoding
    let zero_hash = OpaqueHash::new([0u8; 32]);

    // Create a valid entropy source (96 bytes = 3 x 32-byte chunks)
    let mut entropy_source = Vec::with_capacity(96);
    entropy_source.extend_from_slice(&[0u8; 32]); // current entropy
    entropy_source.extend_from_slice(&[1u8; 32]); // next entropy
    entropy_source.extend_from_slice(&[2u8; 32]); // final entropy

    let mut block = Block {
        header: Header {
            parent: zero_hash,
            slot: 1,
            parent_state_root: zero_hash,
            extrinsic_hash: zero_hash,
            author_index: 0,
            entropy_source,
            seal: vec![0u8; 32],
            epoch_mark: None,
            tickets_mark: None,
            offenders_mark: Vec::new(),
        },
        extrinsic: Extrinsic {
            tickets: Vec::new(),
            preimages: Vec::new(),
            guarantees: Vec::new(),
            assurances: Vec::new(),
            disputes: serde_json::Value::Null,
        },
    };

    let extrinsic_hash = block
        .extrinsic
        .compute_hash()
        .expect("failed to compute extrinsic hash for test block");
    block.header.extrinsic_hash = OpaqueHash::new(extrinsic_hash);

    block
}

fn write_block_to_temp_file(block: &Block) -> Result<std::path::PathBuf> {
    let dir = tempdir()?;
    let file_path = dir.path().join("block.json");
    let file = File::create(&file_path)?;
    serde_json::to_writer(file, block)?;
    // Keep the temp directory alive by leaking it (will be cleaned up on program exit)
    std::mem::forget(dir);
    Ok(file_path.to_path_buf())
}

#[test]
fn test_empty_block_validation() -> Result<()> {
    let mut importer = Importer::new();
    let block = create_test_block();
    let block_path = write_block_to_temp_file(&block)?;

    // Should pass with default valid block
    importer.import_block(block_path)?;

    Ok(())
}

#[test]
fn test_invalid_slot() -> Result<()> {
    let mut importer = Importer::new();
    let mut block = create_test_block();

    // Set slot to 0 (invalid)
    block.header.slot = 0;

    let block_path = write_block_to_temp_file(&block)?;
    let result = importer.import_block(block_path);

    // Check for specific error type
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<BlockchainError>().is_some());

    Ok(())
}

#[test]
fn test_ticket_validation() -> Result<()> {
    let mut importer = Importer::new();
    let mut block = create_test_block();

    // Add a ticket with invalid signature (empty)
    block.extrinsic.tickets.push(TicketEnvelope {
        attempt: 1,
        signature: Vec::new(),
    });

    // Add matching ticket mark
    block.header.tickets_mark = Some(vec![TicketBody {
        id: OpaqueHash::new([0u8; 32]),
        attempt: 1,
    }]);

    let block_path = write_block_to_temp_file(&block)?;
    let result = importer.import_block(block_path);

    // Check for specific error type
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<BlockchainError>().is_some());

    Ok(())
}

#[test]
fn test_preimage_validation() -> Result<()> {
    let mut importer = Importer::new();
    let mut block = create_test_block();

    // Add invalid preimage with empty blob
    block.extrinsic.preimages.push(Preimage {
        requester: 1,
        blob: Vec::new(),
    });

    let block_path = write_block_to_temp_file(&block)?;
    let result = importer.import_block(block_path);

    // Check for specific error type
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<BlockchainError>().is_some());

    // Test invalid requester ID 0
    block.extrinsic.preimages[0].requester = 0;
    block.extrinsic.preimages[0].blob = vec![1, 2, 3];

    let block_path = write_block_to_temp_file(&block)?;
    let result = importer.import_block(block_path);

    // Check for specific error type
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<BlockchainError>().is_some());

    Ok(())
}

#[test]
fn test_ticket_count_mismatch() -> Result<()> {
    let mut importer = Importer::new();
    let mut block = create_test_block();

    // Add a ticket but no corresponding mark
    block.extrinsic.tickets.push(TicketEnvelope {
        attempt: 1,
        signature: vec![1; 64], // Valid signature length
    });

    let block_path = write_block_to_temp_file(&block)?;
    let result = importer.import_block(block_path);

    // Check for specific error type
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<BlockchainError>().is_some());

    // Add a mark that doesn't match the ticket count
    block.header.tickets_mark = Some(vec![TicketBody {
        id: OpaqueHash::new([0u8; 32]),
        attempt: 1,
    }]);
    block.extrinsic.tickets.push(TicketEnvelope {
        attempt: 1,
        signature: vec![1; 64],
    });

    let block_path = write_block_to_temp_file(&block)?;
    let result = importer.import_block(block_path);

    // Check for specific error type
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<BlockchainError>().is_some());

    Ok(())
}

#[test]
fn test_entropy_validation() -> Result<()> {
    let mut importer = Importer::new();
    let mut block = create_test_block();

    // Set invalid entropy source (wrong size)
    block.header.entropy_source = vec![0u8; 31]; // Invalid size

    let block_path = write_block_to_temp_file(&block)?;
    let result = importer.import_block(block_path);

    // Check for specific error type
    let err = result.unwrap_err();
    assert!(err.downcast_ref::<BlockchainError>().is_some());

    Ok(())
}
