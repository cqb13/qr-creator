mod alphanumeric;
mod byte;
mod numeric;

use crate::qr_code::encoding::alphanumeric::alphanumeric_encoding;
use crate::qr_code::encoding::byte::byte_encoding;
use crate::qr_code::encoding::numeric::numeric_encoding;
use crate::qr_code::EncodingMode;

pub fn encode(data: &str, mode: &EncodingMode) -> Result<Vec<String>, String> {
    match mode {
        EncodingMode::Numeric => numeric_encoding(data),
        EncodingMode::Alphanumeric => alphanumeric_encoding(data),
        EncodingMode::Byte => byte_encoding(data),
        EncodingMode::Kanji => Ok(Vec::new()),
    }
}
