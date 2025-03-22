use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpaqueHash(#[serde(with = "hex")] [u8; 32]);

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
