//! Diagnostic tests to capture specific validation failures

use jamliquor::{Importer, schema::Block};
use std::path::Path;

#[test]
fn test_detailed_block_import_failure() {
    let mut importer = Importer::new();
    
    // Test importing the official block vector with detailed error capture
    let result = importer.import_block("tests/vectors/codec/full/block.json");
    
    match result {
        Ok(block) => {
            println!("✅ Unexpected success - block imported:");
            println!("   Slot: {}", block.header.slot);
            println!("   Parent: {:?}", block.header.parent);
        }
        Err(e) => {
            println!("❌ Expected failure captured:");
            println!("   Error: {}", e);
            println!("   Error chain:");
            
            let mut source = e.source();
            let mut depth = 1;
            while let Some(err) = source {
                println!("     {}. {}", depth, err);
                source = err.source();
                depth += 1;
            }
            
            // Try to identify the specific validation step that failed
            let error_str = e.to_string();
            
            if error_str.contains("Ticket count mismatch") {
                println!("   🔍 Diagnosis: Ticket count mismatch - header.tickets_mark vs extrinsic.tickets");
            } else if error_str.contains("Invalid block structure") && error_str.contains("Ticket count") {
                println!("   🔍 Diagnosis: Ticket count mismatch - header.tickets_mark vs extrinsic.tickets");
            } else if error_str.contains("slot") {
                println!("   🔍 Diagnosis: Slot validation failed");
            } else if error_str.contains("parent") {
                println!("   🔍 Diagnosis: Parent hash validation failed");
            } else if error_str.contains("structure") {
                println!("   🔍 Diagnosis: Block structure validation failed");
            } else if error_str.contains("extrinsic") {
                println!("   🔍 Diagnosis: Extrinsic validation failed");
            } else if error_str.contains("coretime") {
                println!("   🔍 Diagnosis: CoreTime validation failed");
            } else if error_str.contains("entropy") {
                println!("   🔍 Diagnosis: Entropy validation failed");
            } else if error_str.contains("signature") {
                println!("   🔍 Diagnosis: Signature validation failed");
            } else {
                println!("   🔍 Diagnosis: Unknown validation failure");
            }
        }
    }
}

#[test]
fn test_block_structure_only() {
    // Test if we can at least parse the JSON structure without validation
    
    let path = Path::new("tests/vectors/codec/full/block.json");
    let file = std::fs::File::open(path).expect("Failed to open block file");
    let reader = std::io::BufReader::new(file);
    
    match serde_json::from_reader::<_, Block>(reader) {
        Ok(block) => {
            println!("✅ JSON parsing successful:");
            println!("   Slot: {}", block.header.slot);
            println!("   Has epoch_mark: {}", block.header.epoch_mark.is_some());
            println!("   Has tickets_mark: {}", block.header.tickets_mark.is_some());
            println!("   Tickets count: {}", block.extrinsic.tickets.len());
            println!("   Preimages count: {}", block.extrinsic.preimages.len());
            println!("   Guarantees count: {}", block.extrinsic.guarantees.len());
            println!("   Assurances count: {}", block.extrinsic.assurances.len());
        }
        Err(e) => {
            println!("❌ JSON parsing failed:");
            println!("   Error: {}", e);
        }
    }
}
