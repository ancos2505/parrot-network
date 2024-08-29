use super::result::BlockchainProtoResult;

pub(crate) trait Serializable<const N: usize>
where
    Self: Sized,
{
    fn serialize_to_bytes(&self) -> BlockchainProtoResult<[u8; N]>;
    fn deserialize_from_bytes(bytes: [u8; N]) -> BlockchainProtoResult<Self>;
}
