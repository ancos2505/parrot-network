use std::fmt::Display;

pub(crate) const ONE_MBYTE: usize = 1024 * 1024;

pub(crate) const TEN_MBYTES: usize = ONE_MBYTE * 10;

pub(crate) const URL_PARTS_MAX_CHARS: usize = 1024;

pub const MAX_REQUEST_LENGTH: usize = if cfg!(debug_assertions) {
    ONE_MBYTE
} else {
    TEN_MBYTES
};

pub const MAX_RESPONSE_LENGTH: usize = MAX_REQUEST_LENGTH;

pub(crate) struct AsciiWhiteSpace;
impl AsciiWhiteSpace {
    pub const fn as_str() -> &'static str {
        " "
    }
    pub const fn len() -> usize {
        1
    }
}

impl Display for AsciiWhiteSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}
