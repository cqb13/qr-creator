use crate::qr_code::Version;

pub struct ErrorCorrectionGroups {
    pub group_one: Vec<ErrorCorrectionBlock>,
    pub group_two: Vec<ErrorCorrectionBlock>,
}

impl ErrorCorrectionGroups {
    pub fn new() -> ErrorCorrectionGroups {
        ErrorCorrectionGroups {
            group_one: Vec::new(),
            group_two: Vec::new(),
        }
    }
}

pub struct ErrorCorrectionBlock {
    pub codewords: String,
}

pub fn generate_code_words(
    data_bits: String,
    version: &Version,
) -> Result<ErrorCorrectionGroups, String> {
    let error_correction_groups = ErrorCorrectionGroups::new();

    Ok(error_correction_groups)
}
