use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Comprehensive error types for JAM blockchain operations
///
/// Memory Usage:
/// - Fixed: ~32 bytes (enum tag + variant fields)
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum BlockchainError {
    /// Slot validation failed
    #[error("Invalid slot: expected > {last_slot}, got {current_slot}")]
    InvalidSlot { last_slot: u64, current_slot: u64 },

    /// Parent hash validation failed
    #[error("Parent hash mismatch: expected {expected}, got {actual}")]
    ParentHashMismatch { expected: String, actual: String },

    /// Ticket validation failed
    #[error("Ticket validation error: {reason}")]
    TicketValidationError { reason: String },

    /// State transition error
    #[error("State transition failed: {reason}")]
    StateTransitionError { reason: String },

    /// Invalid author index in block header
    #[error("Invalid author index: {author_index} out of bounds (max {max_validators})")]
    InvalidAuthorIndex {
        author_index: u64,
        max_validators: usize,
    },

    /// Parent state root mismatch
    #[error("Parent state root mismatch: expected {expected}, got {actual}")]
    ParentStateRootMismatch { expected: String, actual: String },
}

/// ValidationResult captures the outcome of block or state validation with detailed context.
///
/// Memory Usage:
/// - Fixed: ~16 bytes (enum tag + u64 for code + Option<String> for message)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ValidationResult {
    /// Validation succeeded
    Success,
    /// Validation failed with a code and optional message
    Failure { code: u64, message: Option<String> },
}

#[derive(Debug, Default)]
/// State structure for tracking JAM protocol state.
///
/// Memory Usage:
/// - Fixed: ~64 bytes (u64s + TicketState)
/// - Per ticket: ~40 bytes
/// - Per preimage: ~32 bytes + blob size
///
/// Total memory usage grows linearly with processed blocks
/// but can be pruned based on configuration.
pub struct State {
    pub last_slot: u64, // Tracks last processed slot
    pub counter: u64,   // Total valid tickets and preimages
    pub ticket_state: TicketState,
}

#[derive(Debug, Default)]
/// TicketState tracks ticket statistics and last ticket ID.
///
/// Memory Usage:
/// - Fixed: ~56 bytes (3 x u64 + Option<[u8;32]>)
pub struct TicketState {
    pub total_tickets: u64,
    pub valid_tickets: u64,
    pub invalid_tickets: u64,
    pub last_ticket_id: Option<OpaqueHash>,
}

impl TicketState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_tickets(
        &mut self,
        tickets: &[TicketEnvelope],
        tickets_mark: &[TicketBody],
    ) -> Result<(), anyhow::Error> {
        // Validate ticket counts match
        if tickets.len() != tickets_mark.len() {
            return Err(anyhow::anyhow!(
                "Ticket count mismatch: {} tickets vs {} marks",
                tickets.len(),
                tickets_mark.len()
            ));
        }

        // Update total tickets
        self.total_tickets += tickets.len() as u64;

        // Process each ticket
        for (ticket, mark) in tickets.iter().zip(tickets_mark.iter()) {
            // Validate ticket attempt matches mark
            if ticket.attempt != mark.attempt {
                self.invalid_tickets += 1;
                continue;
            }

            // Validate ticket
            if ticket.validate().is_err() {
                self.invalid_tickets += 1;
                continue;
            }

            // Update state
            self.valid_tickets += 1;
            self.last_ticket_id = Some(mark.id);
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_stats(&self) -> (u64, u64, u64) {
        (self.total_tickets, self.valid_tickets, self.invalid_tickets)
    }
}

impl State {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_last_slot(&self) -> u64 {
        self.last_slot
    }

    #[allow(dead_code)]
    pub fn get_counter(&self) -> u64 {
        self.counter
    }

    #[allow(dead_code)]
    pub fn get_ticket_stats(&self) -> (u64, u64, u64) {
        self.ticket_state.get_stats()
    }

    pub fn apply_block(&mut self, block: &Block) -> Result<(), anyhow::Error> {
        // Update slot
        self.last_slot = block.header.slot as u64;

        // Process tickets if tickets_mark exists
        if let Some(tickets_mark) = &block.header.tickets_mark {
            self.ticket_state
                .apply_tickets(&block.extrinsic.tickets, tickets_mark)?;
        }

        // Count valid preimages (non-empty blob)
        let valid_preimage_count = block
            .extrinsic
            .preimages
            .iter()
            .filter(|preimage| !preimage.blob.is_empty())
            .count() as u64;

        // Update counter with valid tickets and preimages
        self.counter = self.ticket_state.valid_tickets + valid_preimage_count;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
/// OpaqueHash is a fixed-size 32-byte array used for hashes and IDs.
///
/// Memory Usage:
/// - Fixed: 32 bytes
#[serde(transparent)]
pub struct OpaqueHash(
    #[serde(
        serialize_with = "hex::serialize",
        deserialize_with = "hex::deserialize"
    )]
    [u8; 32],
);

impl OpaqueHash {
    /// Get the inner byte array
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Create a new OpaqueHash from a byte array
    #[allow(dead_code)]
    pub fn new(bytes: [u8; 32]) -> Self {
        OpaqueHash(bytes)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// EpochMark represents epoch metadata and entropy.
///
/// Memory Usage:
/// - Fixed: ~96 bytes (2 x OpaqueHash + Vec<Validator>)
pub struct EpochMark {
    #[serde(deserialize_with = "hex::deserialize_opaque_hash")]
    pub entropy: OpaqueHash,
    #[serde(deserialize_with = "hex::deserialize_opaque_hash")]
    pub tickets_entropy: OpaqueHash,
    #[serde(deserialize_with = "hex::deserialize_vec_opaque_hash")]
    pub validators: Vec<OpaqueHash>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Header contains block metadata and consensus information.
///
/// Memory Usage:
/// - Fixed: ~200 bytes (slot, hashes, entropy, etc.)
/// - With tickets_mark: grows with number of tickets
pub struct Header {
    #[serde(deserialize_with = "hex::deserialize_opaque_hash")]
    pub parent: OpaqueHash,
    #[serde(deserialize_with = "hex::deserialize_opaque_hash")]
    pub parent_state_root: OpaqueHash,
    #[serde(deserialize_with = "hex::deserialize_opaque_hash")]
    pub extrinsic_hash: OpaqueHash,
    pub slot: u32,
    pub epoch_mark: Option<EpochMark>,
    pub tickets_mark: Option<Vec<TicketBody>>,
    #[serde(deserialize_with = "hex::deserialize_vec_opaque_hash")]
    pub offenders_mark: Vec<OpaqueHash>,
    pub author_index: u16,
    #[serde(deserialize_with = "hex::deserialize_vec_u8")]
    pub entropy_source: Vec<u8>,
    #[serde(deserialize_with = "hex::deserialize_vec_u8")]
    pub seal: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
/// TicketEnvelope represents a ticket submitted by a validator.
///
/// Memory Usage:
/// - Fixed: ~40 bytes (fields + hash)
pub struct TicketEnvelope {
    pub attempt: u8,
    #[serde(deserialize_with = "hex::deserialize_vec_u8")]
    pub signature: Vec<u8>,
}

impl TicketEnvelope {
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        // Validate signature length (should be 64 bytes for ed25519)
        if self.signature.len() != 64 {
            return Err(anyhow::anyhow!(
                "Invalid signature length: {} (expected 64)",
                self.signature.len()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// TicketBody represents the actual content of a ticket.
///
/// Memory Usage:
/// - Fixed: ~40 bytes (fields + hash)
pub struct TicketBody {
    #[serde(deserialize_with = "hex::deserialize_opaque_hash")]
    pub id: OpaqueHash,
    pub attempt: u8,
}

impl TicketBody {
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        // No validation needed for attempt since u8 already enforces 0-255 range
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Preimage represents a preimage for state transition proofs.
///
/// Memory Usage:
/// - Fixed: ~32 bytes + blob size
pub struct Preimage {
    pub requester: u32,
    #[serde(deserialize_with = "hex::deserialize_vec_u8")]
    pub blob: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Extrinsic contains tickets and preimages for a block.
///
/// Memory Usage:
/// - Fixed: Small; grows with number of tickets/preimages
pub struct Extrinsic {
    pub tickets: Vec<TicketEnvelope>,
    pub preimages: Vec<Preimage>,
    pub guarantees: Vec<serde_json::Value>,
    pub assurances: Vec<serde_json::Value>,
    pub disputes: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
/// Block is the top-level structure for JAM blocks.
///
/// Memory Usage:
/// - Fixed: header + extrinsic (see above)
#[serde(deny_unknown_fields)]
pub struct Block {
    pub header: Header,
    pub extrinsic: Extrinsic,
}

mod hex {
    use serde::de::Error;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(bytes)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        let bytes = hex::decode(s).map_err(D::Error::custom)?;
        if bytes.len() != 32 {
            return Err(D::Error::custom("expected 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(arr)
    }

    pub fn deserialize_opaque_hash<'de, D>(deserializer: D) -> Result<super::OpaqueHash, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(super::OpaqueHash(deserialize(deserializer)?))
    }

    pub fn deserialize_vec_opaque_hash<'de, D>(
        deserializer: D,
    ) -> Result<Vec<super::OpaqueHash>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Vec<String> = Deserialize::deserialize(deserializer)?;
        v.into_iter()
            .map(|s| {
                let s = s.strip_prefix("0x").unwrap_or(&s);
                let bytes = hex::decode(s).map_err(D::Error::custom)?;
                if bytes.len() != 32 {
                    return Err(D::Error::custom("expected 32 bytes"));
                }
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&bytes);
                Ok(super::OpaqueHash(arr))
            })
            .collect()
    }

    pub fn deserialize_vec_u8<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        hex::decode(s).map_err(D::Error::custom)
    }
}

// Constants for entropy validation
const ENTROPY_SOURCE_SIZE: usize = 96; // Total entropy source size
const ENTROPY_CHUNK_SIZE: usize = 32; // Size of each entropy value

#[derive(Debug)]
pub struct EntropySource {
    current_entropy: [u8; ENTROPY_CHUNK_SIZE],
    next_entropy: [u8; ENTROPY_CHUNK_SIZE],
    final_entropy: [u8; ENTROPY_CHUNK_SIZE],
}

impl EntropySource {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, anyhow::Error> {
        if bytes.len() != ENTROPY_SOURCE_SIZE {
            return Err(anyhow::anyhow!(
                "Invalid entropy source size: {} (expected {})",
                bytes.len(),
                ENTROPY_SOURCE_SIZE
            ));
        }

        let mut current_entropy = [0u8; ENTROPY_CHUNK_SIZE];
        let mut next_entropy = [0u8; ENTROPY_CHUNK_SIZE];
        let mut final_entropy = [0u8; ENTROPY_CHUNK_SIZE];

        current_entropy.copy_from_slice(&bytes[0..ENTROPY_CHUNK_SIZE]);
        next_entropy.copy_from_slice(&bytes[ENTROPY_CHUNK_SIZE..2 * ENTROPY_CHUNK_SIZE]);
        final_entropy.copy_from_slice(&bytes[2 * ENTROPY_CHUNK_SIZE..]);

        Ok(Self {
            current_entropy,
            next_entropy,
            final_entropy,
        })
    }

    pub fn validate_with_epoch_mark(&self, epoch_mark: &EpochMark) -> Result<(), anyhow::Error> {
        // In JAM protocol, the current entropy should match the epoch mark entropy
        if self.current_entropy != *epoch_mark.entropy.as_bytes() {
            return Err(anyhow::anyhow!(
                "Current entropy mismatch with epoch mark entropy"
            ));
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_current_entropy(&self) -> &[u8; ENTROPY_CHUNK_SIZE] {
        &self.current_entropy
    }

    #[allow(dead_code)]
    pub fn get_next_entropy(&self) -> &[u8; ENTROPY_CHUNK_SIZE] {
        &self.next_entropy
    }

    #[allow(dead_code)]
    pub fn get_final_entropy(&self) -> &[u8; ENTROPY_CHUNK_SIZE] {
        &self.final_entropy
    }
}

impl Header {
    pub fn validate_entropy(&self) -> Result<EntropySource, anyhow::Error> {
        // Parse and validate entropy source format
        let entropy_source = EntropySource::from_bytes(&self.entropy_source)?;

        // If we have an epoch mark, validate entropy values match
        if let Some(epoch_mark) = &self.epoch_mark {
            entropy_source.validate_with_epoch_mark(epoch_mark)?;
        }

        Ok(entropy_source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_entropy_source_validation() -> Result<(), anyhow::Error> {
        // Create a valid entropy source (96 bytes)
        let mut entropy_bytes = Vec::with_capacity(96);
        entropy_bytes.extend_from_slice(&[1u8; 32]); // current entropy
        entropy_bytes.extend_from_slice(&[2u8; 32]); // next entropy
        entropy_bytes.extend_from_slice(&[3u8; 32]); // final entropy

        // Test valid entropy source
        let entropy_source = EntropySource::from_bytes(&entropy_bytes)?;
        assert_eq!(entropy_source.get_current_entropy(), &[1u8; 32]);
        assert_eq!(entropy_source.get_next_entropy(), &[2u8; 32]);
        assert_eq!(entropy_source.get_final_entropy(), &[3u8; 32]);

        // Test invalid size
        let invalid_bytes = vec![0u8; 64];
        assert!(EntropySource::from_bytes(&invalid_bytes).is_err());

        // Test epoch mark validation
        let epoch_mark = EpochMark {
            entropy: OpaqueHash([1u8; 32]),
            tickets_entropy: OpaqueHash([2u8; 32]),
            validators: vec![],
        };
        assert!(entropy_source.validate_with_epoch_mark(&epoch_mark).is_ok());

        // Test epoch mark mismatch
        let invalid_epoch_mark = EpochMark {
            entropy: OpaqueHash([4u8; 32]),
            tickets_entropy: OpaqueHash([5u8; 32]),
            validators: vec![],
        };
        assert!(entropy_source
            .validate_with_epoch_mark(&invalid_epoch_mark)
            .is_err());

        Ok(())
    }
}
