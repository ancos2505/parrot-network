use crate::proto::blockchain::{
    tokens::Wings, traits::Serializable, transaction::Transaction, wallet::Wallet,
};

use super::result::BlockchainProtoResult;

#[test]
fn test_transaction_creation_and_signing() -> BlockchainProtoResult<()> {
    let mut sender_wallet = Wallet::random();
    let receiver_wallet = Wallet::random();
    let amount = Wings::new(1_000_000); // 1 Parrot

    let transaction = sender_wallet.transfer(receiver_wallet.pubkey().clone(), amount.clone())?;

    // Verify the transaction fields
    assert_eq!(transaction.from, *sender_wallet.pubkey());
    assert_eq!(transaction.to, *receiver_wallet.pubkey());
    assert_eq!(transaction.tokens, amount);

    // Verify the signature
    let message = {
        let mut buf = [0u8; Wallet::TRANSACTION_PAYLOAD_LEN];
        buf[0..32].copy_from_slice(&*transaction.from.as_bytes());
        buf[32..64].copy_from_slice(&*transaction.to.as_bytes());
        buf[64..72].copy_from_slice(&transaction.tokens.as_bytes());
        buf[72..80].copy_from_slice(&transaction.nonce.to_be_bytes());
        buf
    };

    sender_wallet.verify(&message, &transaction.signature, &transaction.from)?;

    Ok(())
}

#[test]
fn test_transaction_verification_with_wrong_pubkey() -> BlockchainProtoResult<()> {
    let mut sender_wallet = Wallet::random();
    let receiver_wallet = Wallet::random();
    let wrong_wallet = Wallet::random();
    let amount = Wings::new(1_000_000); // 1 Parrot

    let transaction = sender_wallet.transfer(receiver_wallet.pubkey().clone(), amount.clone())?;

    let message = {
        let mut buf = [0u8; Wallet::TRANSACTION_PAYLOAD_LEN];
        buf[0..32].copy_from_slice(&*transaction.from.as_bytes());
        buf[32..64].copy_from_slice(&*transaction.to.as_bytes());
        buf[64..72].copy_from_slice(&transaction.tokens.as_bytes());
        buf[72..80].copy_from_slice(&transaction.nonce.to_be_bytes());
        buf
    };

    // This should fail because we're using the wrong public key
    assert!(sender_wallet
        .verify(&message, &transaction.signature, wrong_wallet.pubkey())
        .is_err());

    Ok(())
}

#[test]
fn test_transaction_tamper_resistance() -> BlockchainProtoResult<()> {
    let mut sender_wallet = Wallet::random();
    let receiver_wallet = Wallet::random();
    let amount = Wings::new(1_000_000); // 1 Parrot

    let mut transaction =
        sender_wallet.transfer(receiver_wallet.pubkey().clone(), amount.clone())?;

    // Attempt to tamper with the transaction
    transaction.tokens = Wings::new(2_000_000); // Changing the amount

    let message = {
        let mut buf = [0u8; Wallet::TRANSACTION_PAYLOAD_LEN];
        buf[0..32].copy_from_slice(&*transaction.from.as_bytes());
        buf[32..64].copy_from_slice(&*transaction.to.as_bytes());
        buf[64..72].copy_from_slice(&transaction.tokens.as_bytes());
        buf[72..80].copy_from_slice(&transaction.nonce.to_be_bytes());
        buf
    };

    // This should fail because the transaction has been tampered with
    assert!(sender_wallet
        .verify(&message, &transaction.signature, &transaction.from)
        .is_err());

    Ok(())
}

#[test]
fn test_transaction_serialization_and_deserialization() -> BlockchainProtoResult<()> {
    let mut sender_wallet = Wallet::random();
    let receiver_wallet = Wallet::random();
    let amount = Wings::new(1_000_000); // 1 Parrot

    let original_transaction =
        sender_wallet.transfer(receiver_wallet.pubkey().clone(), amount.clone())?;

    // Serialize the transaction
    let serialized = original_transaction.serialize_to_bytes()?;

    // Deserialize the transaction
    let deserialized_transaction = Transaction::deserialize_from_bytes(serialized)?;

    // Verify that the deserialized transaction matches the original
    assert_eq!(original_transaction, deserialized_transaction);

    // Verify the signature of the deserialized transaction
    let message = {
        let mut buf = [0u8; Wallet::TRANSACTION_PAYLOAD_LEN];
        buf[0..32].copy_from_slice(&*deserialized_transaction.from.as_bytes());
        buf[32..64].copy_from_slice(&*deserialized_transaction.to.as_bytes());
        buf[64..72].copy_from_slice(&deserialized_transaction.tokens.as_bytes());
        buf[72..80].copy_from_slice(&deserialized_transaction.nonce.to_be_bytes());
        buf
    };

    sender_wallet.verify(
        &message,
        &deserialized_transaction.signature,
        &deserialized_transaction.from,
    )?;

    Ok(())
}

#[test]
fn test_multiple_transactions_from_same_wallet() -> BlockchainProtoResult<()> {
    let mut sender_wallet = Wallet::random();
    let receiver_wallet1 = Wallet::random();
    let receiver_wallet2 = Wallet::random();
    let amount1 = Wings::new(1_000_000); // 1 Parrot
    let amount2 = Wings::new(2_000_000); // 2 Parrots

    let transaction1 =
        sender_wallet.transfer(receiver_wallet1.pubkey().clone(), amount1.clone())?;
    let transaction2 =
        sender_wallet.transfer(receiver_wallet2.pubkey().clone(), amount2.clone())?;

    // Verify that the nonces are different
    assert_ne!(transaction1.nonce, transaction2.nonce);

    // Verify both transactions
    for transaction in [transaction1, transaction2].iter() {
        let message = {
            let mut buf = [0u8; Wallet::TRANSACTION_PAYLOAD_LEN];
            buf[0..32].copy_from_slice(&*transaction.from.as_bytes());
            buf[32..64].copy_from_slice(&*transaction.to.as_bytes());
            buf[64..72].copy_from_slice(&transaction.tokens.as_bytes());
            buf[72..80].copy_from_slice(&transaction.nonce.to_be_bytes());
            buf
        };

        sender_wallet.verify(&message, &transaction.signature, &transaction.from)?;
    }

    Ok(())
}
