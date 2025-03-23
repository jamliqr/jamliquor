mod schema;
mod importer;

use anyhow::Result;
use importer::Importer;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut importer = Importer::new();
    let block_path = PathBuf::from("../../kd/jamtestvectors/codec/data/block.json");
    let block = importer.import_block(&block_path)?;
    println!("Block: {:?}", block);
    println!("State: {:?}", importer.state());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_block_import() -> Result<()> {
        let mut importer = Importer::new();
        let block_path = PathBuf::from("../../kd/jamtestvectors/codec/data/block.json");
        let block = importer.import_block(&block_path)?;
        
        // Verify the block was imported successfully
        assert!(block.header.slot > 0, "Block slot should be positive");
        assert_eq!(importer.state().last_slot, block.header.slot);
        assert_eq!(importer.state().counter, block.extrinsics.len() as u64);
        Ok(())
    }
}