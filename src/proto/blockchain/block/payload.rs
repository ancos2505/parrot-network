use crate::proto::blockchain::constants::BLOCK_PAYLOAD_LEN;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BlockPayload([u8; BLOCK_PAYLOAD_LEN]);
