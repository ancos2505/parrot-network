pub(crate) const BLOCK_PAYLOAD_LEN: usize = if cfg!(debug_assertions) {
    u8::MAX as usize + 1
} else {
    // 1024 * 1024 // 1 MByte
    u16::MAX as usize + 1 // 64 KBytes
};
