//! **Parrot** token
//! 1_000_000 **tails** => 1 **Parrot**

use ed25519_dalek::Signature;

use super::wallet::PublicKey;

#[derive(Debug, PartialEq, Eq)]
struct Transaction {
    /// The sender's address (public key)
    from: PublicKey,
    /// The recipient's address (public key)
    to: PublicKey,
    /// The amount of **Parrot** tokens to be transferred (in **tail**)
    tokens: u64,
    /// A nonce to ensure the transaction is unique
    nonce: u64,
    /// Optional data field (used for smart contract calls)
    data: Option<Vec<u8>>,
    ///  In the context of an **Parrot** transaction, the signature plays a
    /// crucial role in verifying the authenticity and integrity of the
    /// transaction. The signature ensures that the transaction was indeed
    /// created by the owner of the account and that it hasn't been tampered
    /// with during transmission.
    signature: Signature,
}
