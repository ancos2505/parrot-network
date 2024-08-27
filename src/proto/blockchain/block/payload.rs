use crate::proto::blockchain::constants::BLOCK_PAYLOAD_LEN;

#[derive(Debug, PartialEq, Eq)]
pub struct BlockPayload([u8; BLOCK_PAYLOAD_LEN]);
