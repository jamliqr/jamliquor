use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct State {
    pub last_slot: u64, // Tracks last processed slot
    pub counter: u64,   // Total valid tickets and preimages
    pub ticket_state: TicketState,
}

#[derive(Debug, Default)]
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
pub struct OpaqueHash(#[serde(with = "hex")] [u8; 32]);

impl OpaqueHash {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpochMark {
    pub entropy: OpaqueHash,
    #[serde(rename = "tickets_entropy")]
    pub tickets_entropy: OpaqueHash,
    pub validators: Vec<OpaqueHash>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub parent: OpaqueHash,
    #[serde(rename = "parent_state_root")] // Changed to snake_case
    pub parent_state_root: OpaqueHash,
    #[serde(rename = "extrinsic_hash")] // Changed to snake_case
    pub extrinsic_hash: OpaqueHash,
    pub slot: u32,
    #[serde(rename = "epoch_mark")]
    pub epoch_mark: Option<EpochMark>,
    #[serde(rename = "tickets_mark")]
    pub tickets_mark: Option<Vec<TicketBody>>,
    #[serde(rename = "offenders_mark")]
    pub offenders_mark: Vec<OpaqueHash>,
    #[serde(rename = "author_index")]
    pub author_index: u16,
    #[serde(rename = "entropy_source", with = "hex_vec")]
    pub entropy_source: Vec<u8>,
    #[serde(with = "hex_vec")]
    pub seal: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketEnvelope {
    pub attempt: u8,
    #[serde(with = "hex_vec")]
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
pub struct TicketBody {
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
pub struct Preimage {
    pub requester: u32,
    #[serde(with = "hex_vec")]
    pub blob: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extrinsic {
    pub tickets: Vec<TicketEnvelope>,
    pub preimages: Vec<Preimage>,
    pub guarantees: Vec<serde_json::Value>,
    pub assurances: Vec<serde_json::Value>,
    pub disputes: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: Header,
    pub extrinsic: Extrinsic,
}

mod hex {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s
            .strip_prefix("0x")
            .ok_or_else(|| D::Error::custom("expected hex string with or without 0x"))?;
        let bytes = hex::decode(s).map_err(D::Error::custom)?;
        if bytes.len() != 32 {
            return Err(D::Error::custom("expected 32 bytes"));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(array)
    }
}

mod hex_vec {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s
            .strip_prefix("0x")
            .ok_or_else(|| D::Error::custom("expected hex string with or without 0x"))?;
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
