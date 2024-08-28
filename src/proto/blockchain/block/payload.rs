use base64::{engine::general_purpose, Engine as _};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::proto::blockchain::constants::BLOCK_PAYLOAD_LEN;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BlockPayload(Box<[u8; BLOCK_PAYLOAD_LEN]>);

impl From<Box<[u8; BLOCK_PAYLOAD_LEN]>> for BlockPayload {
    fn from(value: Box<[u8; BLOCK_PAYLOAD_LEN]>) -> Self {
        Self(value)
    }
}

impl BlockPayload {
    pub(crate) fn get(&self) -> &[u8; BLOCK_PAYLOAD_LEN] {
        &self.0
    }
}

impl Serialize for BlockPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let base64_string = general_purpose::STANDARD.encode(*self.0);
        serializer.serialize_str(&base64_string)
    }
}

impl<'de> Deserialize<'de> for BlockPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BlockPayloadVisitor;

        impl Visitor<'_> for BlockPayloadVisitor {
            type Value = BlockPayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a base64 encoded string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let decoded = general_purpose::STANDARD
                    .decode(value)
                    .map_err(|e| E::custom(format!("Failed to decode base64: {}", e)))?;

                if decoded.len() != BLOCK_PAYLOAD_LEN {
                    return Err(E::custom(format!(
                        "Decoded length {} does not match expected length {}",
                        decoded.len(),
                        BLOCK_PAYLOAD_LEN
                    )));
                }

                let mut array = Box::new([0; BLOCK_PAYLOAD_LEN]);
                array.copy_from_slice(&decoded);
                Ok(BlockPayload(array))
            }
        }

        deserializer.deserialize_str(BlockPayloadVisitor)
    }
}
