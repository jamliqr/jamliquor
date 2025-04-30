//! Shared test utilities and module declarations for JamLiquor

use std::path::PathBuf;
use tempfile::TempDir;

/// Utility functions for testing
pub mod utils {
    use super::*;

    /// Get the path to test vectors
    pub fn get_vector_path(vector_name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/vectors");
        path.push(vector_name);
        path
    }

    /// Create a temporary test directory
    pub fn create_temp_test_dir() -> TempDir {
        tempfile::tempdir().expect("Failed to create temporary test directory")
    }

    /// Generate a predictable random seed for reproducible tests
    pub fn get_test_seed() -> u64 {
        42 // Consistent seed for property-based testing
    }
}

/// Declare test modules
pub mod integration;
pub mod unit;
