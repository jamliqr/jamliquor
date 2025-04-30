use jamliquor::schema::{Block, Extrinsic, Header, OpaqueHash};
use jamliquor::state::State;

#[test]
fn test_state_initialization() {
    let state = State::new();
    assert_eq!(
        state.get_last_slot(),
        0,
        "New state should start with slot 0"
    );
}

#[test]
fn test_apply_block_updates_last_slot() {
    let mut state = State::new();

    // Create a mock block
    let block = Block {
        header: Header {
            parent: OpaqueHash::new([0u8; 32]),
            slot: 43,
            extrinsic_hash: OpaqueHash::new([1u8; 32]),
            parent_state_root: OpaqueHash::new([2u8; 32]),
            epoch_mark: None,
            tickets_mark: None,
            offenders_mark: vec![],
            author_index: 0,
            entropy_source: vec![],
            seal: vec![],
        },
        extrinsic: Extrinsic {
            tickets: vec![],
            preimages: vec![],
            guarantees: vec![],
            assurances: vec![],
            disputes: serde_json::Value::Null,
        },
    };

    let result = state.apply_block(&block);
    assert!(result.is_ok(), "Block application should succeed");
    assert_eq!(state.get_last_slot(), 43, "Last slot should be updated");
}

#[test]
fn test_apply_block_invalid_slot() {
    let mut state = State::new();

    // Create a mock block with invalid slot (less than current)
    let block = Block {
        header: Header {
            parent: OpaqueHash::new([0u8; 32]),
            slot: 41, // Invalid slot
            extrinsic_hash: OpaqueHash::new([1u8; 32]),
            parent_state_root: OpaqueHash::new([2u8; 32]),
            epoch_mark: None,
            tickets_mark: None,
            offenders_mark: vec![],
            author_index: 0,
            entropy_source: vec![],
            seal: vec![],
        },
        extrinsic: Extrinsic {
            tickets: vec![],
            preimages: vec![],
            guarantees: vec![],
            assurances: vec![],
            disputes: serde_json::Value::Null,
        },
    };

    let result = state.apply_block(&block);
    assert!(result.is_err(), "Block with invalid slot should fail");
}

#[test]
fn test_state_reset() {
    let mut state = State::new();

    // Create a mock block
    let block = Block {
        header: Header {
            parent: OpaqueHash::new([0u8; 32]),
            slot: 43,
            extrinsic_hash: OpaqueHash::new([1u8; 32]),
            parent_state_root: OpaqueHash::new([2u8; 32]),
            epoch_mark: None,
            tickets_mark: None,
            offenders_mark: vec![],
            author_index: 0,
            entropy_source: vec![],
            seal: vec![],
        },
        extrinsic: Extrinsic {
            tickets: vec![],
            preimages: vec![],
            guarantees: vec![],
            assurances: vec![],
            disputes: serde_json::Value::Null,
        },
    };

    // Apply block
    state
        .apply_block(&block)
        .expect("Block application should succeed");

    // Reset state
    state = State::new();

    assert_eq!(
        state.get_last_slot(),
        0,
        "State should reset to initial state"
    );
}
