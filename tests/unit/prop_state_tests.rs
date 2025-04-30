use jamliquor::schema::{Block, Extrinsic, Header, OpaqueHash};
use jamliquor::state::State;
use proptest::prelude::*;
use serde_json;

proptest! {
    /// Property test to validate state transition stability
    #[test]
    fn prop_state_transition_stability(
        initial_slot in 43..1000u32,
        preimage_count in 0..100usize
    ) {
        // Create a valid block
        let block = Block {
            header: Header {
                parent: OpaqueHash::new([0u8; 32]),
                slot: initial_slot,
                extrinsic_hash: OpaqueHash::new([1u8; 32]),
                parent_state_root: OpaqueHash::new([2u8; 32]),
                epoch_mark: None,
                tickets_mark: None,
                offenders_mark: vec![],
                author_index: 0,
                entropy_source: vec![0u8; 32],
                seal: vec![0u8; 64],
            },
            extrinsic: Extrinsic {
                preimages: vec![],
                tickets: vec![],
                guarantees: vec![],
                assurances: vec![],
                disputes: serde_json::Value::Null,
            },
        };

        // Initialize state
        let mut state = State::new();

        // Apply block
        prop_assert!(state.apply_block(&block).is_ok(), "Block application should succeed");

        // Validate state transitions
        prop_assert_eq!(state.get_last_slot(), initial_slot as u64, "Last slot should be updated correctly");
        prop_assert_eq!(state.get_counter(), 0, "Counter should be zero");
    }

    /// Property test to validate slot progression constraints
    #[test]
    fn prop_slot_progression(
        current_slot in 43..1000u32,
        previous_slot in 43..1000u32
    ) {
        let block = Block {
            header: Header {
                parent: OpaqueHash::new([0u8; 32]),
                slot: current_slot,
                extrinsic_hash: OpaqueHash::new([1u8; 32]),
                parent_state_root: OpaqueHash::new([2u8; 32]),
                epoch_mark: None,
                tickets_mark: None,
                offenders_mark: vec![],
                author_index: 0,
                entropy_source: vec![0u8; 32],
                seal: vec![0u8; 64],
            },
            extrinsic: Extrinsic {
                preimages: vec![],
                tickets: vec![],
                guarantees: vec![],
                assurances: vec![],
                disputes: serde_json::Value::Null,
            },
        };

        let mut state = State::new();

        // Explicitly set a previous slot to simulate state progression
        let previous_block = Block {
            header: Header {
                parent: OpaqueHash::new([0u8; 32]),
                slot: previous_slot,
                extrinsic_hash: OpaqueHash::new([1u8; 32]),
                parent_state_root: OpaqueHash::new([2u8; 32]),
                epoch_mark: None,
                tickets_mark: None,
                offenders_mark: vec![],
                author_index: 0,
                entropy_source: vec![0u8; 32],
                seal: vec![0u8; 64],
            },
            extrinsic: Extrinsic {
                preimages: vec![],
                tickets: vec![],
                guarantees: vec![],
                assurances: vec![],
                disputes: serde_json::Value::Null,
            },
        };

        // Validate slot progression
        prop_assert!(state.apply_block(&previous_block).is_ok(), "Previous block application should succeed");
        prop_assert!(state.apply_block(&block).is_ok(), "Block application should succeed for valid slot progression");
    }
}
