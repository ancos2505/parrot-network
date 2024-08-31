use super::result::BlockchainProtoResult;

pub(crate) trait Serializable: Sized {
    type Bytes: AsRef<[u8]> + AsMut<[u8]>;
    fn serialize_to_bytes(&self) -> BlockchainProtoResult<Self::Bytes>;
    fn deserialize_from_bytes(bytes: Self::Bytes) -> BlockchainProtoResult<Self>;
}
