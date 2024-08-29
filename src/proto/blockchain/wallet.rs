use std::ops::{Deref, DerefMut};

use ed25519_dalek::{
    SecretKey, Signature, Signer, SigningKey, VerifyingKey, KEYPAIR_LENGTH, PUBLIC_KEY_LENGTH,
    SECRET_KEY_LENGTH,
};

use crate::proto::blockchain::traits::Serializable;

use super::{
    block::BlockIndex,
    result::{BlockchainProtoError, BlockchainProtoResult},
    tokens::Wings,
    transaction::Transaction,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Wallet {
    private_key: PrivateKey,
    pubkey: PublicKey,
    signing_key: SigningKey,
    synced_block_index: BlockIndex,
    // TODO
    confirmed_transactions: ConfirmedTransactions,
    // TODO
    balance: Wings,
}

impl Wallet {
    pub(crate) const TRANSACTION_PAYLOAD_LEN: usize = 80;

    pub(crate) fn random() -> Self {
        let private_key: PrivateKey = {
            use rand::{rngs::OsRng, RngCore};
            let mut inner_buf: SecretKey = [0u8; SECRET_KEY_LENGTH];
            OsRng.fill_bytes(&mut inner_buf);
            inner_buf
        }
        .into();

        let signing_key = SigningKey::from_bytes(&private_key);

        let pubkey = (&signing_key.verifying_key()).into();

        Self {
            private_key,
            pubkey,
            signing_key,
            // TODO: Implement ledger scanning
            synced_block_index: BlockIndex::zero(),
            // TODO: Implement ledger scanning
            confirmed_transactions: ConfirmedTransactions::empty(),
            // TODO: Implement ledger scanning
            balance: Wings::zero(),
        }
    }

    fn generate_nonce(&self, to: &PublicKey, tokens: &Wings) -> BlockchainProtoResult<u64> {
        use sha2::{Digest, Sha256};

        const NONCE_LENGTH: usize = 112;

        let mut buf: [u8; NONCE_LENGTH] = [0; NONCE_LENGTH];

        let confirmed_transactions_hash: [u8; 32] = {
            let mut hasher = Sha256::new();
            let mut buf: Vec<u8> = vec![];
            for transaction in self.confirmed_transactions.iter() {
                buf.extend(transaction.serialize_to_bytes()?.to_vec())
            }

            hasher.update(buf);
            hasher.finalize().into()
        };

        buf[0..32].copy_from_slice(&*self.pubkey().as_bytes());
        buf[32..64].copy_from_slice(&*to.as_bytes());
        buf[64..72].copy_from_slice(&self.synced_block_index.to_be_bytes());
        buf[72..104].copy_from_slice(&confirmed_transactions_hash);
        buf[104..NONCE_LENGTH].copy_from_slice(&self.synced_block_index.to_be_bytes());

        let mut hasher = Sha256::new();
        hasher.update(buf);

        let hash_result = hasher.finalize();

        let nonce = u64::from_le_bytes(hash_result[0..8].try_into()?);

        Ok(nonce)
    }
    pub(crate) fn transfer(
        &mut self,
        to: PublicKey,
        tokens: Wings,
    ) -> BlockchainProtoResult<Transaction> {
        let nonce = self.generate_nonce(&to, &tokens)?;
        let mut buf: [u8; Self::TRANSACTION_PAYLOAD_LEN] = [0; Self::TRANSACTION_PAYLOAD_LEN];

        let from = self.pubkey();

        buf[0..32].copy_from_slice(&**from);
        buf[32..64].copy_from_slice(&*to);
        buf[64..72].copy_from_slice(&tokens.as_bytes());
        buf[72..Self::TRANSACTION_PAYLOAD_LEN].copy_from_slice(&nonce.to_be_bytes());

        let signature = self.sign(&buf);

        let transaction = Transaction {
            from: self.pubkey().clone(),
            to,
            tokens,
            nonce,
            signature,
        };
        self.confirmed_transactions.push(transaction.clone());
        Ok(transaction)
    }

    pub(crate) fn keypair_import(keypair: &[u8; KEYPAIR_LENGTH]) -> BlockchainProtoResult<Self> {
        let signing_key = SigningKey::from_keypair_bytes(keypair)?;

        let keypair_bytes = signing_key.to_keypair_bytes();
        let (secret_key_slice, pubkey_slice) = keypair_bytes.split_at(SECRET_KEY_LENGTH);

        let secret_key_bytes: [u8; SECRET_KEY_LENGTH] = secret_key_slice.try_into()?;
        let pubkey_bytes: [u8; PUBLIC_KEY_LENGTH] = pubkey_slice.try_into()?;

        Ok(Wallet {
            private_key: secret_key_bytes.into(),
            pubkey: PublicKey::from_bytes(&pubkey_bytes)?,
            // TODO: Implement ledger scanning
            synced_block_index: BlockIndex::zero(),
            // TODO: Implement ledger scanning
            confirmed_transactions: ConfirmedTransactions::empty(),
            // TODO: Implement ledger scanning
            balance: Wings::zero(),
            signing_key,
        })
    }

    pub(crate) fn keypair_export(&self) -> [u8; KEYPAIR_LENGTH] {
        self.signing_key.to_keypair_bytes()
    }

    pub(crate) fn sign(
        &self,
        transaction_payload: &[u8; Self::TRANSACTION_PAYLOAD_LEN],
    ) -> Signature {
        self.signing_key.sign(transaction_payload)
    }

    pub(crate) fn verify(
        &self,
        message: &[u8],
        signature: &Signature,
        other_pubkey: &PublicKey,
    ) -> BlockchainProtoResult<()> {
        use ed25519_dalek::Verifier;
        let verifying_key: VerifyingKey = other_pubkey.try_into()?;
        verifying_key.verify(message, signature)?;
        Ok(())
    }

    pub(crate) fn pubkey(&self) -> &PublicKey {
        &self.pubkey
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PrivateKey([u8; SECRET_KEY_LENGTH]);

impl From<[u8; SECRET_KEY_LENGTH]> for PrivateKey {
    fn from(value: [u8; SECRET_KEY_LENGTH]) -> Self {
        Self(value)
    }
}

impl Deref for PrivateKey {
    type Target = [u8; SECRET_KEY_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PublicKey([u8; PUBLIC_KEY_LENGTH]);

impl PublicKey {
    pub(crate) fn from_bytes(value: &[u8; PUBLIC_KEY_LENGTH]) -> BlockchainProtoResult<Self> {
        let verifying_key = VerifyingKey::from_bytes(value)?;
        Ok(Self(*verifying_key.as_bytes()))
    }
    pub(crate) fn as_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }
}

impl From<&VerifyingKey> for PublicKey {
    fn from(value: &VerifyingKey) -> Self {
        Self(*value.as_bytes())
    }
}

impl TryFrom<&PublicKey> for VerifyingKey {
    type Error = BlockchainProtoError;

    fn try_from(value: &PublicKey) -> Result<Self, Self::Error> {
        Ok(VerifyingKey::from_bytes(&value.0)?)
    }
}

impl Deref for PublicKey {
    type Target = [u8; PUBLIC_KEY_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use crate::proto::helpers::hex_to_string::{hex_pubkey, hex_signing_key};

    use super::*;

    #[test]
    fn test_wallet_hex_pubkey() {
        let wallet = Wallet::random();
        let pubkey_hex = hex_pubkey(wallet.pubkey());
        assert_eq!(pubkey_hex.chars().count(), PUBLIC_KEY_LENGTH * 2);
        assert!(pubkey_hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_wallet_hex_signing_key() {
        let wallet = Wallet::random();
        let signing_key_hex = hex_signing_key(&wallet.signing_key);
        assert_eq!(signing_key_hex.chars().count(), KEYPAIR_LENGTH * 2);
        assert!(signing_key_hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_wallet_invalid_keypair_import() {
        let invalid_keypair = [0u8; KEYPAIR_LENGTH];
        assert!(Wallet::keypair_import(&invalid_keypair).is_err());
    }
}

#[test]
fn test_wallet_creation() {
    let wallet = Wallet::random();
    assert_eq!(wallet.pubkey().0.len(), PUBLIC_KEY_LENGTH);
}

#[test]
fn test_pubkey_conversion() {
    let wallet = Wallet::random();
    let pubkey = wallet.pubkey();

    let verifying_key: VerifyingKey = pubkey.try_into().unwrap();
    let converted_pubkey: PublicKey = (&verifying_key).into();

    assert_eq!(pubkey, &converted_pubkey);
}

#[test]
fn test_private_key_creation() {
    let private_key: PrivateKey = [0u8; SECRET_KEY_LENGTH].into();
    assert_eq!(private_key.0.len(), SECRET_KEY_LENGTH);
}
