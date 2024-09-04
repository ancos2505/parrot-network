pub(crate) const KEYGEN_SEED_LENGTH: usize = 32;

// pub(crate) const SECRET_KEY_LENGTH: usize = Falcon1024::SECRET_KEY_LENGTH;
// pub(crate) const PUBLIC_KEY_LENGTH: usize = Falcon1024::PUBLIC_KEY_LENGTH;
// pub(crate) const SIGNATURE_LENGTH: usize = Falcon1024::SIGNATURE_LENGTH;

/// ## Falcon 1024
///
/// **Key sizes:**
/// - Secret key size: **2305** bytes (fixed-length)
/// - Public key size: **1793** bytes (fixed-length)
///
///  The "1024" in Falcon 1024 refers to a higher security level. It's designed
/// to provide **256 bits** of post-quantum security (equivalent to **AES-256**)
/// .
struct Falcon1024;
// impl Falcon1024 {
//     pub(crate) const SECRET_KEY_LENGTH: usize = 2305;
//     pub(crate) const PUBLIC_KEY_LENGTH: usize = 1793;
//     pub(crate) const SIGNATURE_LENGTH: usize = 1280;
// }

// pub(crate) const LOCAL_NET_GENESIS_WALLET_SECRET_KEY: [u8; Falcon1024::SECRET_KEY_LENGTH] =
//     *(include_bytes!("../../../misc/localnet_genesis_wallet.dat"));
