use jamliquor::coretime::{
    CoreTimeLedger, MAX_CORETIME_PER_CORE, MAX_DISPUTE_AGE, MAX_GUARANTEE_LOOKBACK,
};
use jamliquor::schema::BlockchainError;
use serde_json::{json, Value};

#[test]
fn validate_and_apply_updates_coretime_ledger_state() {
    let mut ledger = CoreTimeLedger::default();

    let guarantees = vec![json!({
        "slot": 90,
        "report": {
            "core_index": 3,
            "auth_gas_used": 10,
            "results": [
                {
                    "accumulate_gas": 20,
                    "service_id": 0u64,
                }
            ]
        }
    })];

    let assurances = vec![json!({
        "bitfield": "0x1",
        "validator_index": 0u16,
    })];

    let disputes = Value::Null;

    ledger
        .validate_and_apply(
            90 + MAX_GUARANTEE_LOOKBACK,
            &guarantees,
            &assurances,
            &disputes,
        )
        .expect("CoreTime validation should succeed for valid data");

    assert_eq!(ledger.total_allocated(), 30);
    assert_eq!(ledger.total_consumed(), 30);
    assert_eq!(ledger.per_core_consumed(3), Some(30));
    assert_eq!(ledger.last_block_slot(), Some(90 + MAX_GUARANTEE_LOOKBACK));
}

#[test]
fn validate_and_apply_rejects_coretime_overuse() {
    let mut ledger = CoreTimeLedger::default();

    let guarantees = vec![json!({
        "slot": 50,
        "report": {
            "core_index": 1,
            "results": [
                {
                    "accumulate_gas": MAX_CORETIME_PER_CORE + 1,
                    "service_id": 42u64,
                }
            ]
        }
    })];

    let err = ledger
        .validate_and_apply(50, &guarantees, &[], &Value::Null)
        .expect_err("CoreTime validation should fail when consumption exceeds limits");

    assert!(matches!(err, BlockchainError::CoreTimeBalanceError { .. }));
}

#[test]
fn validate_and_apply_rejects_stale_disputes() {
    let mut ledger = CoreTimeLedger::default();

    let guarantees = Vec::new();
    let assurances = Vec::new();
    let disputes = json!({
        "verdicts": [
            {
                "target": "0xdeadbeef",
                "age": MAX_DISPUTE_AGE + 1,
                "votes": [
                    {
                        "vote": true,
                        "index": 0u16,
                        "signature": "0x01",
                    }
                ]
            }
        ]
    });

    let err = ledger
        .validate_and_apply(10, &guarantees, &assurances, &disputes)
        .expect_err("CoreTime validation should fail for stale disputes");

    assert!(matches!(
        err,
        BlockchainError::CoreTimeValidationError { .. }
    ));
}
