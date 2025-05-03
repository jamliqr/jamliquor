use jamliquor::schema::{Block, Extrinsic, Header, OpaqueHash, State, ValidationResult};
use serde_json;

#[test]
fn test_serde_roundtrip_state() {
    let state = State::default();
    let json = serde_json::to_string(&state).unwrap();
    let state2: State = serde_json::from_str(&json).unwrap();
    assert_eq!(state.last_slot, state2.last_slot);
    assert_eq!(state.counter, state2.counter);
}

#[test]
fn test_serde_roundtrip_block() {
    let block = Block {
        header: Header {
            slot: 1,
            parent: OpaqueHash([0; 32]),
            parent_state_root: OpaqueHash([0; 32]),
            extrinsic_hash: OpaqueHash([0; 32]),
            entropy_source: vec![0; 96],
            epoch_mark: None,
            tickets_mark: None,
            offenders_mark: vec![],
            author_index: 0,
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
    let json = serde_json::to_string(&block).unwrap();
    let block2: Block = serde_json::from_str(&json).unwrap();
    assert_eq!(block.header.slot, block2.header.slot);
}

#[test]
fn test_validation_result_serde() {
    let ok = ValidationResult::Success;
    let fail = ValidationResult::Failure {
        code: 42,
        message: Some("fail".into()),
    };
    let ok_json = serde_json::to_string(&ok).unwrap();
    let fail_json = serde_json::to_string(&fail).unwrap();
    assert_eq!(ok, serde_json::from_str(&ok_json).unwrap());
    assert_eq!(fail, serde_json::from_str(&fail_json).unwrap());
}
