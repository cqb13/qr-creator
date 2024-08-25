use crate::qr_code::utils::{convert_hex_to_binary, left_pad};
use crate::qr_code::{EncodingMode, Version};

pub fn create_character_count_indicator(
    data: &str,
    encoding_mode: &EncodingMode,
    version: &Version,
) -> String {
    let binary_indicator_bit_length = calculate_binary_indicator_bit_length(encoding_mode, version);

    let count = data.len() as i32;

    let hex = format!("{:02X?}", count);

    let binary = left_pad(
        convert_hex_to_binary(&hex),
        binary_indicator_bit_length,
        "0",
    );

    binary
}

fn calculate_binary_indicator_bit_length(encoding_mode: &EncodingMode, version: &Version) -> i32 {
    let version = version.version();

    if version >= 1 && version <= 9 {
        match encoding_mode {
            EncodingMode::Numeric => 10,
            EncodingMode::Alphanumeric => 9,
            EncodingMode::Byte => 8,
        }
    } else if version >= 10 && version <= 26 {
        match encoding_mode {
            EncodingMode::Numeric => 12,
            EncodingMode::Alphanumeric => 11,
            EncodingMode::Byte => 16,
        }
    } else {
        match encoding_mode {
            EncodingMode::Numeric => 14,
            EncodingMode::Alphanumeric => 13,
            EncodingMode::Byte => 16,
        }
    }
}
