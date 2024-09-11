mod generate_code_words;

use crate::qr_code::{ErrorCorrectionLevel, Version};

pub fn generate_error_correction(
    data_bits: String,
    error_correction_level: &ErrorCorrectionLevel,
    version: &Version,
) -> Result<String, String> {
    Ok("this is fine".to_string())
}
