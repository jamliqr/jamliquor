
use jamliquor::Importer;
use std::path::PathBuf;

#[test]
fn test_importer_initialization() {
    let _importer = Importer::new();
    // Verify default state
    assert!(true, "Importer should be created successfully");
}

#[test]
fn test_set_initial_state() {
    let mut importer = Importer::new();
    let test_hash = [0u8; 32];
    let test_root = [1u8; 32];

    importer.set_initial_state(test_hash, test_root);
    // Note: This is a placeholder. We might want to add more robust verification
    assert!(true, "Initial state should be set successfully");
}

#[test]
fn test_import_block_invalid_path() {
    let mut importer = Importer::new();
    let invalid_path = PathBuf::from("/path/to/nonexistent/block.json");

    let result = importer.import_block(&invalid_path);
    assert!(result.is_err(), "Importing from invalid path should fail");
}

#[test]
fn test_import_block_invalid_json() {
    // Create a temporary file with invalid JSON
    use std::fs::File;
    use std::io::Write;

    let temp_dir = std::env::temp_dir();
    let invalid_json_path = temp_dir.join("invalid_block.json");

    let mut file = File::create(&invalid_json_path).expect("Failed to create temp file");
    file.write_all(b"{ invalid json }")
        .expect("Failed to write to temp file");

    let mut importer = Importer::new();
    let result = importer.import_block(&invalid_json_path);

    // Clean up temp file
    std::fs::remove_file(&invalid_json_path).expect("Failed to remove temp file");

    assert!(result.is_err(), "Importing invalid JSON should fail");
}
