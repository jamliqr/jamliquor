use anyhow::Result;
use jamliquor::Importer;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut importer = Importer::new();
    let block_path = PathBuf::from("tests/vectors/codec/tiny/block.json");
    let block = importer.import_block(&block_path)?;
    println!("Block: {block:?}");
    println!("State: {:?}", importer.state());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use jamliquor::schema::{
        Block, Extrinsic, Header, OpaqueHash, Preimage, TicketBody, TicketEnvelope,
    };
    use serde_json::{to_value, Value};
    use std::fs::File;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn build_sample_block() -> (Block, Value) {
        let mut block = Block {
            header: Header {
                parent: OpaqueHash::new([0u8; 32]),
                parent_state_root: OpaqueHash::new([1u8; 32]),
                extrinsic_hash: OpaqueHash::new([0u8; 32]),
                slot: 43,
                epoch_mark: None,
                tickets_mark: Some(vec![TicketBody {
                    id: OpaqueHash::new([2u8; 32]),
                    attempt: 1,
                }]),
                offenders_mark: vec![OpaqueHash::new([3u8; 32])],
                author_index: 0,
                entropy_source: vec![4u8; 96],
                seal: vec![5u8; 32],
            },
            extrinsic: Extrinsic {
                tickets: vec![TicketEnvelope {
                    attempt: 1,
                    signature: vec![6u8; 64],
                }],
                preimages: vec![Preimage {
                    requester: 1,
                    blob: vec![1u8, 2, 3, 4],
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

    fn write_block_json(value: &Value) -> Result<PathBuf> {
        let dir = tempdir()?;
        let file_path = dir.path().join("block.json");
        serde_json::to_writer(File::create(&file_path)?, value)?;
        std::mem::forget(dir);
        Ok(file_path)
    }

    #[test]
    fn test_block_import() -> Result<()> {
        let mut importer = Importer::new();
        let (block, block_json) = build_sample_block();

        importer.set_initial_state(
            *block.header.parent.as_bytes(),
            *block.header.parent_state_root.as_bytes(),
        );

        let block_path = write_block_json(&block_json)?;
        let imported_block = importer.import_block(&block_path)?;

        assert_eq!(
            imported_block.header.extrinsic_hash.as_bytes(),
            block.header.extrinsic_hash.as_bytes(),
            "Extrinsic hash should match header commitment"
        );

        assert_eq!(
            importer.state().get_last_slot(),
            imported_block.header.slot as u64,
            "Last slot should update after import"
        );

        let (total_tickets, valid_tickets, invalid_tickets) = importer.state().get_ticket_stats();
        assert_eq!(total_tickets, 1, "Should track one ticket");
        assert_eq!(valid_tickets, 1, "Ticket should be valid");
        assert_eq!(invalid_tickets, 0, "No invalid tickets expected");

        assert_eq!(
            importer.state().get_counter(),
            valid_tickets + imported_block.extrinsic.preimages.len() as u64,
            "State counter should equal valid tickets plus preimages"
        );

        Ok(())
    }
}
