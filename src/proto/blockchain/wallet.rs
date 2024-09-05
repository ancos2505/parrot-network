use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use array_bytes::hex2array;

use falcon_rust::falcon1024;

use crate::proto::helpers::hex_to_string::{hex_pubkey, hex_signature};

use super::{
    block::BlockIndex,
    constants::SIGNATURE_BYTES_LENGTH,
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
    pub(crate) fn as_inner(&self) -> &falcon1024::PublicKey {
        &self.0
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
    pub(crate) fn as_inner(&self) -> &falcon1024::Signature {
        &self.0
    }
}

impl FromStr for Signature {
    type Err = BlockchainProtoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; SIGNATURE_BYTES_LENGTH] = hex2array(s)?;
        let signature = falcon1024::Signature::from_bytes(&bytes)
            .map_err(|err| BlockchainProtoError::FalconDeserializationError(format!("{err:?}")))?;
        Ok(Self(signature))
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex_signature(&self))
    }
}

#[cfg(test)]
mod tests {
    // use crate::proto::helpers::hex_to_string::hex_pubkey;

    use super::*;
    fn mock_pubkey1() -> PublicKey {
        let hex = "0a1699b732f3e88362d0d699eb5c582f99aa4733875e648d18549011dd5e4c25a25348388f828623422fa3a52823e1aa9d18ab9eec15b443d5205a10b2dc01f440426e819ce3638ae134fbf449e1a7cc7ed64e2f3644b85600ce8c070ef084c89be2485b095e15ba152a497919e8221933746908f3c2a2ef964afa545e81983924ec0a69862d9c52bcdb2f00d94d3eb172d82c045e4197f8f9085abc725901c1e3ccb02c5f219361a233b08320cf230311da18ca27150c99a24deaccac9f9db1d9174a672c8a691b07ec943599408f52505b527400976af06a0b9528e293e0c931393a7e4b9396d7b440fecb9ad0cfaf982b992faade209e205744e45420c4efc4b31db159c473245f0d4c2a6cb1e205cb41ad4572047cd7b37d4c5c07d543c32504845ba09d7d0d01b355eae2049770a72a98018b93dd083335ee835541d0d69108ce8f7434d0f44d030c86cc0043809a44ac5b802a96d8568e156a2865216c0eb5227511e41314d3b9ace602b16d7a4459571f435ea71b174e78831aba68f007ab673254937e4b2f5fd629a40ea6fa216c6519aa42ac977dbd6b77ef57379663906dda0301d578d94e48e73f7660f4e8070cb174249942874c2c6edd7394a938555e1af24850f97efea232304aafbb46fe15b79923538e84251f16f72c2c1d33ed6281b47910c490d64054d5ca896ce366382acc9890e2e63328c972ad47272c76ad7b9ba10e3e6e4811bb9d27bc686b42479d82b0001598dae16c6f55c3c826ddf0554a179280801c099c9cda6401bb8a3d14c8a91bea4db698e44d08430aacf646cda73491611901a54add5e7e4a0037217c2d59da08975c6f45be2e96ef162022b132d3b290173e3c9e854928a185f66edbe3c7d464b9e887b548a923d61673d36dfa383ab030994a311d9475c2c454d88235c162d2ef2dabdac86533296d75a943f80b56052a909805f34cf47555d5f4ea8ee8090e4f09d96fcf4bfc31570252c237e6d088d4d6c2bea1a60243d2369625431410e5854485bd9445549310901759c753e11175ade056254e3aeceefb8e1f4a57d5176a80825829dde24bc6d0b69130ea1a1cd7a25c41e1815219787b310e009ce0579c964ac1a35ec842393bbb8b0ad284183e6864dde3aad05ba9a4cf094806631b616167b76d1853ce1107575199496f5a0652b833f1179ac164809abafeec1b538e9776d49f8fb63da49637419d9a0e7f659d6a2b86565e8a9dc2f1a8a459395c9a311d2ac3da86e11044b2caa1134a499d322208a584745c96cf3be3916d7451f87bad5e68b119378ad5629c07809bf74411256386ca39ca49d86639a3536105a933b1c79a8d4fe368898081a9c47a81513e8bbb08054b1276998ec6c83e5236e992859d66ec96317d40948c74b348f581c54daad6fb3fa0dc5d306403dfab7f656ed443ef2acb719e0819218b3d52126943ff5d3a60ce86a95014ce1980957384e3385bdd8ae54f1375166e9f4028cca9a2b9a807309ef48da1a859deca1d66dc1d435808bc2221e6518febb748f21365e6072299e0aff661d1cd956474b9fa1f532ed1e57ad74d4d5b6454fac66edea6e93987c9547c8b59e0f262530567b4bc37b2685c1ddd1a889586783358316638c98798dccf890b866e23699718d9e46a00721811b41e957a384a49f20df11839b2e3b5c7cd8f34f2465250910af24219b5c7613fcd81b28d4d80bf063f12c0648c2e2a374a87284554fa8d97ad29b921e6a4997287ea824d35e0c99acb55f66282a953816b6994352a12fdb2697daac81a35040e9e70ce25bc85099cca3f8ea211de5323eb4e5a99d8af24c0a3b47931463032517e6d461252dc87d01d163184985c9242b50462a32ecc54467c612deb815ff4b6fb6f9a19055aa0d6b3ebb39bad3f5a2e19e183d90893e1abcc876edd9802eee08105e6d9a5e26aa1b5431677cc8119690200bba693321cf4d2fd6853af35a8d593d2ceb9e43caa9208c71af983b77f2e0c659a0d35da45220e2d4518da5de6677ccf8a5e62ca3186cdd07ad10260c40fd67bda0182f8416354e2677bfcbe05df679f95d09d5ac604493836d57868e437b4a8cc897784b81422145203118eb0d1d4f830d0371724da2af9e88d7ec995f26704c197a4acb6a169f3ac618489af914008f112f5889ce25169a1862a48601184fa106bd39da360bc95e98ddd8be5bc868641bfbefc45866755bf406889591eeeb6bb819410c856f39ab45fd0c90ef9938540678d8599a1b642257706e98156ffa2b252d79afdbce070cf429cfa40b0135c5912457f272496942632d3927b981c62c63af873c92a7c105d08f21b0a3ea52bb7eb4f2789558d6325051bb2f5c76bb6226b5321b5563d6c2a27269ac50ae2670d6eef81e4edbc4655e1c23de09039f63e70c75964695ecf0026913e826ed56667549c9dd687852d65b828eced94f28a6c96b89a39d1cd8535d04dd36b069b81690855e3209c2b6583d9b6108362f6d0ebf61fdf0b11cf05a1c56c826807a6346112";
        PublicKey::from_str(hex).unwrap()
    }

    fn mock_pubkey2() -> PublicKey {
        let hex ="0a0138f60948862c24aa3da94d021d3b05f0a58813c881ea6c60026fad8719355570a20da348a1d754441c34309508251cc27f708a24f84f9dbddd110454df81326e652924505c07bcfa039e180188b0c7c47c882caa407e37a051f43a6173d52e10df5751e43405895c86114552c8a23a98046e94cb10f00b9a25cb510bdf18ad89c2b5e3a52271b0e0ce1ee58b3da4bb1f07b1834199b8175840656430976e08c7a64e0ecbdb4ea3934a98c761840502f60b59b28321b1f23b951c16208c912d79605f3d135ee9850385f991c5b8747a0f146461b75b2cccea1f52ec174082341a84c4f99bd6b07eba6d959381957246d5e539a4f39a354ce444594c594049878108616640f509f3e05d58c153725a4d123ffeca10d5eac5b96481465669a3215128caa180f8a43174d1c085ae27a5b7fad6496dbfa149eb9613323ed9309704e8df9de83d21f0d3e3870090120f110413f13657fb8565bdc6892364191c9eacbd00a69b1fb92dbc6a6f2b5c71ad3468fd041507c512c392cad70d025d5a669af1069646c006dbeb14742279f7449ab87d69741f2f421165b0d9f613ab95f6f9a902275b7eddb4291e025345fcd5f30f24895cad671b3768dd9fee16771f40b11fc9819a052f3b9a62ad215b8ef14a6485dba38cc471a6d32903077508ee6812e85b8ebc5e16b1b882aa15294f4b79e571a1f4203a80984776e2379149049d9896887b5f40b557c80b74c8133b8e0e005a6da03eaf4cba880081d74a25b5239c1d0187c553d28e1790582cd7b598636482f851aba7262f77e2d0d4855af60d92033811d53926242a68c82b5d6401f0f7fdd2956548ff74ae15970956da330f0525910080e922202e1f3efb74650d626880d5dad11e4460a489a6a8987aae0a5c80d4c45b56ba79caba09238155022ab6a41a66784605e0a7415bb5575301cbce4b4d00694c21534df128148b9ba674febda0eb8e45b6a178d47ed872ac509d83b30f16559ee50a0d297d0591768a2953e863f40423526faa0ee92e88a32beeb798c0684e655011204886bd4049034637415594f63e209a426d0b9aba46d7d78aca218f718cf3cec9a11ddad7b65313172b79fe344b0ae1460ea137f303a99b7635982e5b211329c00b2d5b446da69527ba1016b75cb0849da38648480d2444e66e9932b910f8ee92929a90501a176caf2c34ba55a7aa28783332baa7556ab5aa20a558fe1984469bb92166254414d56dc32a7fe5a454e904c67c999546e16b62ba49fe672a43c84f1a493590aa685ad2a773f82e84bd9108f617532eb368bc78caee916c8693b5d469f2af2a4f3815a22d5b118b59abe3e965b62745cce74db12c4f76c351e13cba081aad64ba48ba8afa8656200c25d36bb057c4202676493b94f6b11e3582c2ebca42b469ea0725686581aa1378db7e087e6b532a19b5752e14121e1893491074f0d0db551889c35929db1ea12552cbe9c1b506aa3b798fdcdc7122ef4254a1c31d12f4242da3f03d76b6e067060cb9e9f770a545e13a30a799a789278826cc15f9eace550137217f8768b2cd4da6d78ac8033af27a1f550b310e0996fc6c4420e1ea972ea7bf9b6cf1066c1bfd76bc2443b74c66260682c35e80652c2176fac443ba8118470b9ddf2b5261bb2389de4565d82f87576ec39ff14cd0ea436841445355a0ddcab6766c9b3f17bc842d00507bcf1539e0069b719b03ae461d85edc2d489d81e426c1d004efadd800986d81b0026664c9c438952cb690500581da48c8b7aa8aa6aa52fe792d7cf0becaf12bf9e425c02e54ba38ae05d9188b6a6166b08f2809300cd20ab7a75b811d3703456e431fe0adc5ff292b47882d07544e210b0e155600067d0895e15792cf795e7a80900850bcbc8daa90d20a11388991d454ce0d1dae32c2606b0a5ac049946eca46a6f92504e501595c920918b36b0d53ba83c1f6c2eb38a569d5a1c0ba1db6ba74d82f287eb20313ff5f702f055f8a43ecea1d56198883dbd31f7e1e81fe590976e961e3e0a14f796d0df55e8f9c539a1c421b180970fdf3c2558bc420d5c6e6ca09d63a9abf269fa1c697f0f3248a0be4a2a49be3cadad6446d523660c9b8938de79a6b472ba6b1b402e39f8d82799a771cd257bf369cb516c378ba5de2e536e97584e14ce33ce63358b943b9db041ab36c688b6b57fc2804442a6501bf5bb6f101bdae8b3d6c088aee0b618eb58882a0803a475ddd32fe8ddb9c1e8faac8183a9de8e25574644366142e6b945f0b6e3854705d975430462f568302306b2297250859d15e9c738a3c49ed6065b910431f5ba0ed9473aa8a200fa1938bc006c6155c41dab18309b560a671a8962f8a61de45745d4719ccd28b3a27734d2d9f0b7b9fc0b748612704c5e651001f90db948e751c28b549c41c5d910832d15bcf5e1e7562e0c3364459d6d6978fb6457018e847e6d54c7dda58d4a4d90b32d52b2e296e5b0272aa4d541cbed93b06ede4523a887341e3ae3e59353000bb87922198a6158482e1008ea4a";
        PublicKey::from_str(hex).unwrap()
    }

    fn mock_pubkey3() -> PublicKey {
        let hex = "0a4b7562333ba3ec05fef863d14e8f93adb1866a50ed266c19d3c8a58801f4d010649a6a98f024664fd61c91709df892cb57b4e08d51751ddd403c21058e01ba6a319e79f45ed51d1ea44807293e5c3111d8ffe7cda3ba11b360896322045bb24467d22a883120b653d496190b8919c3bc7ab6818066d8946900b6851fe51699f2c1186883e8adb8d2e6aa9e6962c032f0850a6519b1f4501065920d7a0d27e3d0b64d6db053641d4c8e5167ebecc2866c02df161f50f5a3a9cf1228c69481fea7307d99f98592f329ba1fe87d7e3014c706705da979b331c1ef99e9f05b6fa6a762da3d97b3459e87a87c83434b991ee822b78550bb58a899d7a8935593504c4acf48e57c546c57cc2ee2360e04646097b955c83822181ad09aa23079a194227be6480b9d95c3a6bad28a1cae593229e3897d1ab81c2ca66ac489f76da528e1386943b3064864122995669ec87ae3eb538b2e3143dca1cf932661765adbef0e82a656615c2db8e05b9b33d2d86618c198131488b519ef463268cc22eae5c560cfb83bda8ac636e1908e501966c2540146a84cd6d5a25004c8c916d84e950e86d4075897e149cda238780ed8bcdc5520a6d9e267d81619056ad077e039e64ddf70aaa6db017a9b61432c32214e58ae14a97e86b3e31e35f246473ce3075ab84a68be4c16bbc1bd08aa6304504c7b76fe9331026d0e8a2ec7c275223079fa11364f3a69d51642fc9a8884da8ba9d10d780c1257df70be097a8a0148729d7cd48942ac986576132ba1a6ae763932dab8926e430234c1f4932f5af470748bcb566ea6bc680a29cab4be3282f3aa76240a1a885a4cf06392c8005a538d4c48a0662d19057b7aec50965f5c0308bf78bd2c139e767a1bd9e3da45de9d63d74dd34386f0553a06c08848b3e7ec1e8785e60b70cdeeb747f20d9a6514e0ad8ad462d3ac3ab304358539eb8dab5172aaf5e36604880b52679c066824fe4a28a8fea82a89c2d85d3b73f11c0244e48508456b8b16a63d7ee19e383e0d5203651c36f6966eaf784e294a543cbb08130287e86a45aeb9713fae608cda33871a12b25f96b0a49d54bebdd50bf449a91c7efd2ec8e7d8f17f12694535d30764b21c361f4a9267425450ac62180acecae982a37b8a0d8f19962c60bab47efa5a4c6db8390914f66ad4578aa8f7d390958b4eadd615ed0c75f9c38838567a855550c231dd42871405e010ae893be4f3eb4c88051b9a6aba3c07fc6e9cc3682b4995a70ec0b6988ca06c8c722328db4211050418a7c5ab0be189c1e25cd5b5145f91f495895b74ef57dbde4c87b669e91bc3f2a8b4bd37cf060b8832e3f58740804e6d0f020e188323d8d6c0bd42065d58728979c1817c04a41a226e158afec698ee2cff317a50513d00d9ad71bfaa9887a2a89544589ddcc5513e03c985e6e81601338233453d5ed0c7b3244e7a321e6c4052162b1a268695c42afe005d97bbbb41f91125e467fdae712c6e4ff76c661248d6ae37616eee1772c8740ecb5f6ecdef28479f4f8794222b33e98f82861e53385e07aecd793035843308e8e16828d21a308c89a3e79ba6838e2e30da0f8f1d0dd50bbd2d908faaacdafbc7b010d0575af217e346680351365fd61889bef4726cb515219f1505122b7b8d7ed3494d677cc5d9c681c8372c48f97415129d2524ac99938ff56542e8775c6b95b5aeb7f3834e324996909a06a8efad602fe55981988c75e9570f6b0b42667561499e426d6e440449ddb79ac9163bc8113b5b952c91fbe60042923130f32ce54ea81530734d48bd9584b9ca123e30d936d207ac765285b3f56cc0fdb25c1fa3490766bef0bdf0d8c1419ce90b58e0a63043a94b6bb9d306beb08ad8fee32e89dee902c04a493e041880b0468f8d4a2c7973e73221dd55507bc614420e000b989db21b0075a1f8cc7760582fbb79499d2f62bbeba950731a715ff25da99fb31ac2f05caaed555aee3b3eaa73af22d2d1a8a2c6bff526f64ee07751041e537e871b45c1d825259e6a9e486972c8443e336ebf5364ac3d00ceb0d39e1aa619c53c5901e88e646a2cc84a2091495d18d626ca1aa8be0a6000d4dea198d9b5585e57483ea2f68d272aa6a155433707442c2e238017ed7e0f8ed8a62ee745a7e14df0ec6fd53dc636d60194dc15d6a380ea54851bc2a1da17600e5c01ea1a6ca21b02ce1279a0909c68ba66c0eba65ab654df5ab81e278f0c6533f7859e154dca279c0b9d78d9fca6e964277c84f9c5a4d8bd1dd1fc568ce5754f4c9aa61356346b66b2b22f98341e28a0372bc70f3ac636bbed18022e81de826c29422352e074fbda2508f0a6215a429862dabe31e18baea5a1ac8181a5b42a9e07768e66fa93491377904ddd520dafb6a4c419abfaa19aee6e9d08288c565bd54ebeddfd3bb7c577b552a600dc9c9ca60c0872f55452a0e9de2b1b1883602d9559689a69796e607001091a837d10c0ad5a1c630cc6d4868e535f398e7208cfdba616b5fb32e3c23a8299e8adda5ea32a1d023504dd";
        PublicKey::from_str(hex).unwrap()
    }

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
        let pubkey = mock_pubkey1();
        let byte_array = pubkey.to_bytes();

        let converted_pk = falcon1024::PublicKey::from_bytes(&byte_array).unwrap();

        assert_eq!(byte_array, converted_pk.to_bytes());
    }
}
