# JAM Test Vectors Documentation

## Overview

JamLiquor uses official and unofficial test vectors to ensure conformance with the JAM Gray Paper specification.

## Repository Structure

```
tests/vectors/           # Official W3F test vectors (w3f/jamtestvectors)
├── codec/             # Codec test vectors
│   ├── full/          # Full-size test vectors
│   │   ├── block.json
│   │   ├── header_0.json, header_1.json
│   │   ├── extrinsic.json
│   │   └── *_extrinsic.json (various extrinsic types)
│   └── tiny/          # Minimal test vectors
├── stf/               # State Transition Function test vectors
├── traces/            # Execution traces
├── trie/              # Merkle trie test vectors
└── erasure/           # Erasure coding test vectors

tests/vectors-davxy/   # Unofficial test vectors (davxy/jam-test-vectors)
└── [same structure as official]
```

## Test Vector Formats

### Block Vectors (`block.json`)

```json
{
    "header": {
        "parent": "0x...",
        "parent_state_root": "0x...",
        "extrinsic_hash": "0x...",
        "slot": 42,
        "epoch_mark": {
            "entropy": "0x...",
            "tickets_entropy": "0x...",
            "validators": [
                {
                    "bandersnatch": "0x...",
                    "ed25519": "0x..."
                }
            ]
        },
        "author_index": 0,
        "entropy": "0x...",
        "signature": "0x...",
        "work_report": "0x..."
    },
    "extrinsic": {
        "type": "safrole",
        "data": "0x..."
    }
}
```

### Header Vectors (`header_N.json`)

Individual header components for targeted testing.

### Extrinsic Vectors (`*_extrinsic.json`)

Different extrinsic types:
- `assurances_extrinsic.json` - Assurance-related extrinsics
- `disputes_extrinsic.json` - Dispute-related extrinsics  
- `preimages_extrinsic.json` - Preimage-related extrinsics
- `tickets_extrinsic.json` - Ticket-related extrinsics
- `guarantees_extrinsic.json` - Guarantee-related extrinsics

## Test Harness Usage

The `VectorTestHarness` in `tests/integration/vector_tests.rs` provides:

```rust
let harness = VectorTestHarness::new();

// Load official block vector
let block = harness.load_block_vector()?;

// Load specific header
let header = harness.load_header_vector(0)?;

// Load extrinsic vector
let extrinsic = harness.load_extrinsic_vector()?;
```

## Updating Test Vectors

When the Gray Paper version updates:

```bash
# Update official vectors
git submodule update --remote tests/vectors

# Update unofficial vectors  
git submodule update --remote tests/vectors-davxy

# Commit the update
git add tests/vectors tests/vectors-davxy
git commit -m "chore: update test vector submodules"
```

## Fresh Clone Setup

For developers cloning the repository:

```bash
git clone --recurse-submodules https://github.com/jamliqr/jamliquor.git

# Or after cloning without submodules:
git submodule update --init --recursive
```

## Conformance Testing

Run tests against official vectors:

```bash
# Run vector integration tests
cargo test --test integration vector_tests

# Run all tests with verbose output
cargo test --all-targets -- --nocapture

# Run specific vector test
cargo test test_load_official_block_vector -- --nocapture
```

## Expected Failures (Audit Phase)

During the audit phase, many tests are expected to fail due to:

1. **Codec mismatches** - Field names/types may differ from Gray Paper v0.8
2. **Logic mismatches** - Validation rules may be outdated
3. **Missing features** - PVM, validator judgments not yet implemented
4. **Breaking changes** - Service Account, storage key format changes

All failures are documented in `docs/src/AUDIT_FAILURES.md`.

## M1 Conformance

The M1 conformance suite requires passing:
- Block import vectors (`tests/vectors/codec/full/block.json`)
- Header validation vectors (`tests/vectors/codec/full/header_*.json`)
- Extrinsic processing vectors (`tests/vectors/codec/full/*_extrinsic.json`)

These vectors exercise:
- Host calls and PVM boot
- SAFROLE validation
- CoreTime accounting
- State transitions

## Notes

- Official vectors are the authoritative source for conformance
- Unofficial vectors (davxy) provide additional edge cases
- Binary files (`.bin`) contain SCALE-encoded data for codec testing
- JSON files (`.json`) contain human-readable test data
- Test vectors are pinned to specific commits for reproducibility
