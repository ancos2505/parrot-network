use ed25519_dalek::{Signature, PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};

use super::{
    result::BlockchainProtoResult, tokens::Wings, traits::Serializable, wallet::PublicKey,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Transaction {
    /// The sender's address (public key)
    pub(crate) from: PublicKey,
    /// The recipient's address (public key)
    pub(crate) to: PublicKey,
    /// The amount of **Parrot** tokens to be transferred (in **Wings**)
    pub(crate) tokens: Wings,
    /// A nonce to ensure the transaction is unique
    pub(crate) nonce: u64,

    // /// Optional data field (used for smart contract calls)
    // data: Option<Vec<u8>>,
    ///  In the context of an **Parrot** transaction, the signature plays a
    /// crucial role in verifying the authenticity and integrity of the
    /// transaction. The signature ensures that the transaction was indeed
    /// created by the owner of the account and that it hasn't been tampered
    /// with during transmission.
    pub(crate) signature: Signature,
}

impl Transaction {
    pub(crate) const PAYLOAD_LEN: usize = 144;
}

impl Serializable for Transaction {
    type Bytes = [u8; Self::PAYLOAD_LEN];

    fn serialize_to_bytes(&self) -> BlockchainProtoResult<Self::Bytes> {
        let mut buf: [u8; Self::PAYLOAD_LEN] = [0; Self::PAYLOAD_LEN];
        let from: [u8; 32] = *self.from;
        let to: [u8; 32] = *self.to;
        let tokens: [u8; 8] = self.tokens.as_bytes();
        let nonce: [u8; 8] = self.nonce.to_be_bytes();
        let signature: [u8; 64] = self.signature.to_bytes();

        buf[0..32].copy_from_slice(&from);
        buf[32..64].copy_from_slice(&to);
        buf[64..72].copy_from_slice(&tokens);
        buf[72..80].copy_from_slice(&nonce);
        buf[80..144].copy_from_slice(&signature);
        Ok(buf)
    }

    fn deserialize_from_bytes(bytes: Self::Bytes) -> BlockchainProtoResult<Self> {
        let from = {
            let inner: [u8; PUBLIC_KEY_LENGTH] = bytes[0..32].try_into()?;
            PublicKey::from_bytes(&inner)?
        };

        let to = {
            let inner: [u8; PUBLIC_KEY_LENGTH] = bytes[32..64].try_into()?;
            PublicKey::from_bytes(&inner)?
        };

        let tokens = {
            let inner: [u8; size_of::<u64>()] = bytes[64..72].try_into()?;
            let value = u64::from_be_bytes(inner);
            Wings::new(value)
        };
        let nonce = {
            let inner: [u8; size_of::<u64>()] = bytes[72..80].try_into()?;
            u64::from_be_bytes(inner)
        };
        let signature = {
            let inner: [u8; SIGNATURE_LENGTH] = bytes[80..Self::PAYLOAD_LEN].try_into()?;
            Signature::from_bytes(&inner)
        };

        Ok(Self {
            from,
            to,
            tokens,
            nonce,
            signature,
        })
    }
}
