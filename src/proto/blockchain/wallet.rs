use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use falcon_rust::falcon1024;

use crate::proto::helpers::hex_to_string::{hex_pubkey, hex_signature};

use super::{
    block::BlockIndex,
    // constants::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH},
    result::{BlockchainProtoError, BlockchainProtoResult},
    tokens::Wings,
    transaction::Transaction,
};

#[derive(Debug, PartialEq)]
pub(crate) struct Wallet {
    secret_key: SecretKey,
    pubkey: PublicKey,
    synced_block_index: BlockIndex,
    // TODO
    confirmed_transactions: ConfirmedTransactions,
    // TODO
    balance: Wings,
}

impl Wallet {
    pub(crate) const TRANSACTION_PAYLOAD_LEN: usize = 80;

    pub(crate) fn random() -> BlockchainProtoResult<Self> {
        use falcon_rust::falcon1024;
        use rand::thread_rng;
        use rand::Rng;
        let mut rng = thread_rng();

        let (sk, pk) = falcon1024::keygen(rng.r#gen());

        Ok(Self {
            secret_key: sk.into(),
            pubkey: pk.into(),
            // TODO: Implement ledger scanning
            synced_block_index: BlockIndex::zero(),
            // TODO: Implement ledger scanning
            confirmed_transactions: ConfirmedTransactions::empty(),
            // TODO: Implement ledger scanning
            balance: Wings::zero(),
        })
    }

    pub(crate) fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    pub(crate) fn pubkey(&self) -> &PublicKey {
        &self.pubkey
    }

    pub(crate) fn synced_block_index(&self) -> &BlockIndex {
        &self.synced_block_index
    }

    pub(crate) fn confirmed_transactions(&self) -> &ConfirmedTransactions {
        &self.confirmed_transactions
    }

    pub(crate) fn balance(&self) -> &Wings {
        &self.balance
    }

    // fn generate_nonce(&self, to: &PublicKey, tokens: &Wings) -> BlockchainProtoResult<u64> {
    //     use sha2::{Digest, Sha256};

    //     const NONCE_LENGTH: usize = 112;

    //     let mut buf: [u8; NONCE_LENGTH] = [0; NONCE_LENGTH];

    //     let confirmed_transactions_hash: [u8; 32] = {
    //         let mut hasher = Sha256::new();
    //         let mut buf: Vec<u8> = vec![];
    //         for transaction in self.confirmed_transactions.iter() {
    //             buf.extend(transaction.serialize_to_bytes()?.to_vec())
    //         }

    //         hasher.update(buf);
    //         hasher.finalize().into()
    //     };

    //     buf[0..32].copy_from_slice(&*self.pubkey().as_bytes());
    //     buf[32..64].copy_from_slice(&*to.as_bytes());
    //     buf[64..72].copy_from_slice(&self.synced_block_index.to_be_bytes());
    //     buf[72..104].copy_from_slice(&confirmed_transactions_hash);
    //     buf[104..NONCE_LENGTH].copy_from_slice(&self.synced_block_index.to_be_bytes());

    //     let mut hasher = Sha256::new();
    //     hasher.update(buf);

    //     let hash_result = hasher.finalize();

    //     let nonce = u64::from_le_bytes(hash_result[0..8].try_into()?);

    //     Ok(nonce)
    // }

    // pub(crate) fn transfer(
    //     &mut self,
    //     to: PublicKey,
    //     tokens: Wings,
    // ) -> BlockchainProtoResult<Transaction> {
    //     let nonce = self.generate_nonce(&to, &tokens)?;
    //     let mut buf: [u8; Self::TRANSACTION_PAYLOAD_LEN] = [0; Self::TRANSACTION_PAYLOAD_LEN];

    //     let from = self.pubkey();

    //     buf[0..32].copy_from_slice(&**from);
    //     buf[32..64].copy_from_slice(&*to);
    //     buf[64..72].copy_from_slice(&tokens.as_bytes());
    //     buf[72..Self::TRANSACTION_PAYLOAD_LEN].copy_from_slice(&nonce.to_be_bytes());

    //     let signature = self.sign(&buf);

    //     let transaction = Transaction {
    //         from: self.pubkey().clone(),
    //         to,
    //         tokens,
    //         nonce,
    //         signature,
    //     };
    //     self.confirmed_transactions.push(transaction.clone());
    //     Ok(transaction)
    // }

    // pub(crate) fn keypair_import(keypair: &[u8; KEYPAIR_LENGTH]) -> BlockchainProtoResult<Self> {
    //     let signing_key = SigningKey::from_keypair_bytes(keypair)?;

    //     let keypair_bytes = signing_key.to_keypair_bytes();
    //     let (secret_key_slice, pubkey_slice) = keypair_bytes.split_at(SECRET_KEY_LENGTH);

    //     let secret_key_bytes: [u8; SECRET_KEY_LENGTH] = secret_key_slice.try_into()?;
    //     let pubkey_bytes: [u8; PUBLIC_KEY_LENGTH] = pubkey_slice.try_into()?;

    //     Ok(Wallet {
    //         private_key: secret_key_bytes.into(),
    //         pubkey: PublicKey::from_bytes(&pubkey_bytes)?,
    //         // TODO: Implement ledger scanning
    //         synced_block_index: BlockIndex::zero(),
    //         // TODO: Implement ledger scanning
    //         confirmed_transactions: ConfirmedTransactions::empty(),
    //         // TODO: Implement ledger scanning
    //         balance: Wings::zero(),
    //         signing_key,
    //     })
    // }

    // pub(crate) fn keypair_export(&self) -> [u8; KEYPAIR_LENGTH] {
    //     self.signing_key.to_keypair_bytes()
    // }

    // pub(crate) fn sign(
    //     &self,
    //     transaction_payload: &[u8; Self::TRANSACTION_PAYLOAD_LEN],
    // ) -> Signature {
    //     self.signing_key.sign(transaction_payload)
    // }

    // pub(crate) fn verify(
    //     &self,
    //     message: &[u8],
    //     signature: &Signature,
    //     other_pubkey: &PublicKey,
    // ) -> BlockchainProtoResult<()> {
    //     use ed25519_dalek::Verifier;
    //     let verifying_key: VerifyingKey = other_pubkey.try_into()?;
    //     verifying_key.verify(message, signature)?;
    //     Ok(())
    // }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SecretKey(falcon1024::SecretKey);

impl SecretKey {
    pub(crate) fn random() -> Self {
        use falcon_rust::falcon1024;
        use rand::thread_rng;
        use rand::Rng;
        let mut rng = thread_rng();

        let (sk, _) = falcon1024::keygen(rng.r#gen());

        Self(sk)
    }
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
    pub(crate) fn sign(&self, msg: &[u8]) -> Signature {
        Signature(falcon1024::sign(msg, &self.0))
    }
}

impl From<falcon1024::SecretKey> for SecretKey {
    fn from(value: falcon1024::SecretKey) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PublicKey(falcon1024::PublicKey);

impl PublicKey {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

impl From<falcon1024::PublicKey> for PublicKey {
    fn from(value: falcon1024::PublicKey) -> Self {
        Self(value)
    }
}

impl From<&SecretKey> for PublicKey {
    fn from(value: &SecretKey) -> Self {
        Self(falcon1024::PublicKey::from_secret_key(&value.0))
    }
}

impl FromStr for PublicKey {
    type Err = BlockchainProtoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use array_bytes::hex2bytes;
        let bytes = hex2bytes(s)?;
        let pk = falcon1024::PublicKey::from_bytes(&bytes)
            .map_err(|err| BlockchainProtoError::FalconDeserializationError(format!("{err:?}")))?;
        Ok(Self(pk))
    }
}

// impl TryFrom<Box<[u8; PUBLIC_KEY_LENGTH]>> for PublicKey {
//     type Error = BlockchainProtoError;

//     fn try_from(value: Box<[u8; PUBLIC_KEY_LENGTH]>) -> Result<Self, Self::Error> {
//         let pk = falcon1024::PublicKey::from_bytes(&*value)
//             .map_err(|err| BlockchainProtoError::FalconDeserializationError(format!("{err:?}")))?;
//         Ok(Self(pk))
//     }
// }

impl Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex_pubkey(&self))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConfirmedTransactions(Vec<Transaction>);
impl ConfirmedTransactions {
    pub(crate) fn empty() -> Self {
        Self(vec![])
    }
}

impl Deref for ConfirmedTransactions {
    type Target = Vec<Transaction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConfirmedTransactions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Signature(falcon1024::Signature);

impl Signature {
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}

// impl TryFrom<[u8; SIGNATURE_LENGTH]> for Signature {
//     type Error = BlockchainProtoError;

//     fn try_from(value: [u8; SIGNATURE_LENGTH]) -> Result<Self, Self::Error> {
//         Ok(Self(falcon1024::Signature::from_bytes(&value).map_err(
//             |err| BlockchainProtoError::FalconDeserializationError(format!("{err:?}")),
//         )?))
//     }
// }

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex_signature(&self))
    }
}

#[cfg(test)]
mod tests {
    use crate::proto::helpers::hex_to_string::hex_pubkey;

    use super::*;

    // #[test]
    // fn test_wallet_hex_pubkey() {
    //     let wallet = Wallet::random().unwrap();
    //     let pubkey_hex = hex_pubkey(wallet.pubkey());
    //     assert_eq!(pubkey_hex.chars().count(), PUBLIC_KEY_LENGTH * 2);
    //     assert!(pubkey_hex.chars().all(|c| c.is_ascii_hexdigit()));
    // }

    // #[test]
    // fn test_wallet_creation() {
    //     let wallet = Wallet::random().unwrap();
    //     assert_eq!(wallet.pubkey().to_bytes().len(), PUBLIC_KEY_LENGTH);
    // }

    #[test]
    fn test_pubkey_conversion() {
        let wallet = Wallet::random().unwrap();
        let pubkey = wallet.pubkey();
        let byte_array = pubkey.to_bytes();

        let converted_pk = falcon1024::PublicKey::from_bytes(&byte_array).unwrap();

        assert_eq!(byte_array, converted_pk.to_bytes());
    }
}
