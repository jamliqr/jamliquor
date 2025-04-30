//! Unit tests for JamLiquor core components

use proptest::prelude::*;

/// Basic sanity check to ensure testing infrastructure works
#[test]
fn test_project_setup() {}

proptest! {
    #[test]
    fn prop_test_example(x: u16, y: u16) {
        // Example property: addition is commutative
        assert_eq!(x as u32 + y as u32, y as u32 + x as u32);
    }
}

mod importer_tests;
pub mod prop_state_tests;
pub mod state_tests;
