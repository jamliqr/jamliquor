use jamliquor::Importer;
use std::path::PathBuf;

#[test]
fn test_importer_initialization() {}

#[test]
fn test_set_initial_state() {}

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
