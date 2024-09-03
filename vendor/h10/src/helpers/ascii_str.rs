use crate::http::result::{H10LibError, H10LibResult};

#[allow(dead_code)]
pub fn str_from_ascii<'a>(bytes: &'a [u8]) -> H10LibResult<&'a str> {
    let valid_bytes = {
        let mut found = 0;
        for (idx, c) in bytes.iter().enumerate() {
            if *c == b'\0' {
                found = idx;
                break;
            }
        }
        let (valid, _) = bytes.split_at(found);
        valid
    };

    for c in valid_bytes {
        if !c.is_ascii() {
            return Err(H10LibError::StrFromAscii(
                "Invalid input for str_from_ascii".into(),
            ));
        }
    }
    let s = std::str::from_utf8(valid_bytes)?;
    Ok(s)
}
