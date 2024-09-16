mod generate_code_words;

use generate_code_words::generate_code_words;

use crate::qr_code::{ErrorCorrectionLevel, Version};

pub fn generate_error_correction(
    data_bits: String,
    error_correction_level: &ErrorCorrectionLevel,
    version: &Version,
) -> Result<String, String> {
    let code_words = generate_code_words(data_bits, &error_correction_level, &version)?;
    Ok("this is fine".to_string())
}
