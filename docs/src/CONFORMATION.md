# JAM Protocol Test Vector Conformation Guide

## Overview

This document provides a comprehensive guide to the test vectors used in the JamLiquor implementation, detailing their purpose, structure, and significance in validating the JAM protocol implementation.

## Vector Categories

### 1. Safrole Consensus Vectors

#### Purpose
Validate the Safrole consensus mechanism, which is distinct from Sassafras.

#### Configuration Types
- **Tiny Configuration**
  - Validators: 6
  - Epoch Duration: 12
  - Use Case: Prototyping and rapid development

- **Full Configuration**
  - Validators: 1023
  - Epoch Duration: 600
  - Use Case: Production-level testing

#### Key Characteristics
- Entire ticket accumulator must be filled
- No application-specific data in ticket envelope
- Ring proofs constructed using Zcash SRS parameters

#### Unique Differences from Sassafras
- No threshold for ticket score consideration
- Ticket persistence based on accumulator state
- Strict ticket accumulator filling requirement

### 2. Codec Vectors

#### Purpose
Test encoding and decoding of JAM protocol data structures

#### Covered Data Structures
- refine_context
- work_item
- work_package
- work_result (0 and 1)

#### Encoding Characteristics
- Variable-length encoding similar to SCALE
- Syntactically correct but not logically constrained
- Designed to test codec handling, not protocol logic

### 3. JAM Types ASN Schema

#### Purpose
Define type structures for protocol data

#### Components
- Tiny configuration schema
- Full configuration schema
- Utility scripts for schema manipulation

### 4. Shuffle Vectors

#### Purpose
Test randomization and selection mechanisms

### 5. Trie Vectors

#### Purpose
Validate state storage and Merkle tree implementations

## Conformance Testing Guidelines

### 1. Codec Validation
- Ensure correct encoding/decoding of all defined types
- Handle variable-length sequences
- Validate against provided ASN.1 schemas

### 2. Consensus Mechanism Testing
- Test both tiny and full validator configurations
- Validate ring proof mechanisms
- Verify ticket accumulator logic

### 3. State Transition Validation
- Confirm state consistency across different vector scenarios
- Validate error handling and state preservation
- Test edge cases in state transitions

### 4. Performance Considerations
- Verify performance characteristics for both tiny and full configurations
- Ensure scalability from prototype to production environments

## Error Handling Principles

- On State Transition Function (STF) execution error, post-state must match pre-state
- Error codes are for testing purposes and may not reflect final implementation
- Prioritize clear, consistent error reporting

## Recommended Testing Approach

1. Start with tiny configuration vectors
2. Progressively test full configuration vectors
3. Validate each data structure independently
4. Perform comprehensive integration testing
5. Continuously update test vectors as protocol evolves

## References

- [Graypaper](https://github.com/paritytech/graypaper) - Authoritative source for JAM protocol
- [Zcash Powers of Tau Ceremony](https://zfnd.org/conclusion-of-the-powers-of-tau-ceremony)
- [Bandersnatch VRFs Specification](https://github.com/davxy/bandersnatch-vrfs-spec/tree/main/example)

## Version

Last Updated: 2025-04-30
