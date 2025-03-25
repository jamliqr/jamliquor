use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct State {
    last_slot: u64, // Tracks last processed slot
    counter: u64,   // Placeholder for state (e.g., from extrinsics)
}

impl State {
    pub fn new() -> Self {
        State {
            last_slot: 0,
            counter: 0,
        }
    }

    pub fn get_last_slot(&self) -> u64 {
        self.last_slot
    }

    pub fn get_counter(&self) -> u64 {
        self.counter
    }

    pub fn apply_block(&mut self, block: &Block) -> Result<(), anyhow::Error> {
        self.last_slot = block.header.slot as u64;
        // Count only valid tickets
        let valid_ticket_count = block
            .extrinsic
            .tickets
            .iter()
            .filter(|ticket| ticket.attempt <= 255) // u8 max, always true, placeholder for stricter rules
            .count() as u64;
        // Count valid preimages (non-empty blob)
        let valid_preimage_count = block
            .extrinsic
            .preimages
            .iter()
            .filter(|preimage| !preimage.blob.is_empty())
            .count() as u64;
        self.counter += valid_ticket_count + valid_preimage_count;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketBody {
    pub id: OpaqueHash,
    pub attempt: u8,
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
