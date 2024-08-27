use std::ops::Deref;

use ed25519_dalek::{
    SecretKey, Signature, Signer, SigningKey, VerifyingKey, KEYPAIR_LENGTH, PUBLIC_KEY_LENGTH,
    SECRET_KEY_LENGTH, SIGNATURE_LENGTH,
};

use super::result::{H10BlockchainProtoError, H10BlockchainProtoResult};

#[derive(Debug, PartialEq, Eq)]
pub struct Wallet {
    private_key: PrivateKey,
    pubkey: PublicKey,
    signing_key: SigningKey,
}
impl Wallet {
    pub fn new() -> Self {
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
        }
    }

    pub fn keypair_import(keypair: &[u8; KEYPAIR_LENGTH]) -> H10BlockchainProtoResult<Self> {
        let signing_key = SigningKey::from_keypair_bytes(keypair)?;

        let keypair_bytes = signing_key.to_keypair_bytes();
        let (secret_key_slice, pubkey_slice) = keypair_bytes.split_at(SECRET_KEY_LENGTH);

        let secret_key: [u8; SECRET_KEY_LENGTH] = secret_key_slice.try_into()?;
        let pubkey: [u8; PUBLIC_KEY_LENGTH] = pubkey_slice.try_into()?;

        Ok(Wallet {
            private_key: secret_key.into(),
            pubkey: (&pubkey).try_into()?,
            signing_key,
        })
    }

    pub fn keypair_export(&self) -> [u8; KEYPAIR_LENGTH] {
        self.signing_key.to_keypair_bytes()
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn verify(
        &self,
        message: &[u8],
        signature: &Signature,
        other_pubkey: &PublicKey,
    ) -> H10BlockchainProtoResult<()> {
        use ed25519_dalek::Verifier;
        let verifying_key: VerifyingKey = other_pubkey.try_into()?;
        verifying_key.verify(message, signature)?;
        Ok(())
    }

    pub fn pubkey(&self) -> &PublicKey {
        &self.pubkey
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrivateKey([u8; SECRET_KEY_LENGTH]);

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

#[derive(Debug, PartialEq, Eq)]
pub struct PublicKey([u8; PUBLIC_KEY_LENGTH]);

impl From<&VerifyingKey> for PublicKey {
    fn from(value: &VerifyingKey) -> Self {
        Self(*value.as_bytes())
    }
}

impl TryFrom<&PublicKey> for VerifyingKey {
    type Error = H10BlockchainProtoError;

    fn try_from(value: &PublicKey) -> Result<Self, Self::Error> {
        Ok(VerifyingKey::from_bytes(&value.0)?)
    }
}

impl TryFrom<&[u8; PUBLIC_KEY_LENGTH]> for PublicKey {
    type Error = H10BlockchainProtoError;

    fn try_from(value: &[u8; PUBLIC_KEY_LENGTH]) -> Result<Self, Self::Error> {
        let verifying_key = VerifyingKey::from_bytes(value)?;
        Ok(Self(*verifying_key.as_bytes()))
    }
}

impl Deref for PublicKey {
    type Target = [u8; PUBLIC_KEY_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Signature;

    #[test]
    fn test_wallet_keypair_export_import() {
        let original_wallet = Wallet::new();
        let exported_keypair = original_wallet.keypair_export();
        let imported_wallet = Wallet::keypair_import(&exported_keypair).unwrap();

        assert_eq!(original_wallet.pubkey(), imported_wallet.pubkey());

        // Test signing with both wallets
        let message = b"Test message";
        let signature1 = original_wallet.sign(message);
        let signature2 = imported_wallet.sign(message);

        assert_eq!(signature1, signature2);
    }

    #[test]
    fn test_wallet_pubkey_to_hex() {
        let wallet = Wallet::new();
        let pubkey_hex = pubkey_to_hex(wallet.pubkey());
        assert_eq!(pubkey_hex.len(), PUBLIC_KEY_LENGTH * 2);
        assert!(pubkey_hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_wallet_signing_key_to_hex() {
        let wallet = Wallet::new();
        let signing_key_hex = signing_key_to_hex(&wallet.signing_key);
        assert_eq!(signing_key_hex.len(), KEYPAIR_LENGTH * 2);
        assert!(signing_key_hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_wallet_signature_to_hex() {
        let wallet = Wallet::new();
        let message = b"Test message";
        let signature = wallet.sign(message);
        let signature_hex = signature_to_hex(&signature);
        assert_eq!(signature_hex.len(), SIGNATURE_LENGTH * 2);
        assert!(signature_hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_wallet_verify_with_multiple_messages() {
        let wallet = Wallet::new();
        let messages = ["Message 1", "Another message", "Yet another message"];

        for message in &messages {
            let signature = wallet.sign(message.as_bytes());
            assert!(wallet
                .verify(message.as_bytes(), &signature, wallet.pubkey())
                .is_ok());
        }
    }

    #[test]
    fn test_wallet_verify_cross_wallet() {
        let wallet1 = Wallet::new();
        let wallet2 = Wallet::new();
        let message = b"Cross-wallet verification test";

        let signature1 = wallet1.sign(message);
        let signature2 = wallet2.sign(message);

        // Wallet 1 should verify its own signature
        assert!(wallet1
            .verify(message, &signature1, wallet1.pubkey())
            .is_ok());

        // Wallet 2 should verify its own signature
        assert!(wallet2
            .verify(message, &signature2, wallet2.pubkey())
            .is_ok());

        // Wallet 1 should verify Wallet 2's signature
        assert!(wallet1
            .verify(message, &signature2, wallet2.pubkey())
            .is_ok());

        // Wallet 2 should verify Wallet 1's signature
        assert!(wallet2
            .verify(message, &signature1, wallet1.pubkey())
            .is_ok());
    }

    #[test]
    fn test_wallet_invalid_keypair_import() {
        let invalid_keypair = [0u8; KEYPAIR_LENGTH];
        assert!(Wallet::keypair_import(&invalid_keypair).is_err());
    }

    #[test]
    fn test_wallet_sign_different_messages() {
        let wallet = Wallet::new();
        let message1 = b"First message";
        let message2 = b"Second message";

        let signature1 = wallet.sign(message1);
        let signature2 = wallet.sign(message2);

        assert_ne!(signature1, signature2);
        assert!(wallet
            .verify(message1, &signature1, wallet.pubkey())
            .is_ok());
        assert!(wallet
            .verify(message2, &signature2, wallet.pubkey())
            .is_ok());
        assert!(wallet
            .verify(message1, &signature2, wallet.pubkey())
            .is_err());
        assert!(wallet
            .verify(message2, &signature1, wallet.pubkey())
            .is_err());
    }
}

#[test]
fn test_wallet_creation() {
    let wallet = Wallet::new();
    assert_eq!(wallet.pubkey().0.len(), PUBLIC_KEY_LENGTH);
}

#[test]
fn test_wallet_sign_and_verify() {
    let wallet = Wallet::new();
    let message = "The quick brown fox jumps over the lazy dog.";

    let signature = wallet.sign(message.as_bytes());

    let result = wallet.verify(message.as_bytes(), &signature, wallet.pubkey());
    assert!(result.is_ok());
}

#[test]
fn test_wallet_verify_with_different_pubkey() {
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    let message = "The quick brown fox jumps over the lazy dog.";

    let signature = wallet1.sign(message.as_bytes());

    let result = wallet1.verify(message.as_bytes(), &signature, wallet2.pubkey());
    assert!(result.is_err());
}

#[test]
fn test_wallet_verify_tampered_message() {
    let wallet = Wallet::new();
    let original_message = "The quick brown fox jumps over the lazy dog.";
    let tampered_message = "The quick brown f0x jumps over the lazy dog.";

    let signature = wallet.sign(original_message.as_bytes());

    let result = wallet.verify(tampered_message.as_bytes(), &signature, wallet.pubkey());
    assert!(result.is_err());
}

#[test]
fn test_pubkey_conversion() {
    let wallet = Wallet::new();
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

fn pubkey_to_hex(pubkey: &PublicKey) -> String {
    let mut output = "".to_string();
    for n in **pubkey {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}

fn signing_key_to_hex(signing_key: &SigningKey) -> String {
    let mut output = "".to_string();
    for n in signing_key.to_keypair_bytes() {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}

fn signature_to_hex(signature: &Signature) -> String {
    let mut output = "".to_string();
    for n in signature.to_bytes() {
        output.push_str(format!("{:02x}", n).as_str())
    }
    output
}
