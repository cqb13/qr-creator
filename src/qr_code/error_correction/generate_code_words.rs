use crate::qr_code::{ErrorCorrectionLevel, Version};

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
    pub codewords: Vec<String>,
}

pub fn generate_code_words(
    data_bits: String,
    error_correction_level: &ErrorCorrectionLevel,
    version: &Version,
) -> Result<ErrorCorrectionGroups, String> {
    let error_correction_groups = ErrorCorrectionGroups::new();

    Ok(error_correction_groups)
}

pub struct GroupConfig {
    pub blocks_in_group_one: i32,
    pub code_words_in_group_one_blocks: i32,
    pub blocks_in_group_two: i32,
    pub code_words_in_group_two_blocks: i32,
}

impl GroupConfig {
    pub fn new(
        blocks_in_group_one: i32,
        code_words_in_group_one_blocks: i32,
        blocks_in_group_two: i32,
        code_words_in_group_two_blocks: i32,
    ) -> GroupConfig {
        GroupConfig {
            blocks_in_group_one,
            code_words_in_group_one_blocks,
            blocks_in_group_two,
            code_words_in_group_two_blocks,
        }
    }
}

pub fn get_group_layout(error_correction_level: &ErrorCorrectionLevel, version: &Version) {
    // let group_configs: [GroupConfig; 40] = match error_correction_level {
    //     ErrorCorrectionLevel::Low => [],
    //     ErrorCorrectionLevel::Medium => [],
    //     ErrorCorrectionLevel::Quartile => [],
    //     ErrorCorrectionLevel::High => [],
    // };
}
