use crate::qr_code::{ErrorCorrectionLevel, Version};

pub struct ErrorCorrectionGroups {
    pub group_one: Vec<Vec<String>>,
    pub group_two: Vec<Vec<String>>,
}

impl ErrorCorrectionGroups {
    pub fn new() -> ErrorCorrectionGroups {
        ErrorCorrectionGroups {
            group_one: Vec::new(),
            group_two: Vec::new(),
        }
    }
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

pub fn generate_code_words(
    data_bits: String,
    error_correction_level: &ErrorCorrectionLevel,
    version: &Version,
) -> Result<ErrorCorrectionGroups, String> {
    let byte_blocks = data_bits_to_byte_blocks(data_bits)?;
    let error_correction_group_config = get_group_layout(&error_correction_level, &version);
    let error_correction_groups = ErrorCorrectionGroups::new();

    Ok(error_correction_groups)
}

fn data_bits_to_byte_blocks(data_bits: String) -> Result<Vec<String>, String> {
    if data_bits.len() % 8 != 0 {
        return Err("Failed to split bits into bytes".to_string());
    }

    let chunks: Vec<String> = (0..data_bits.len())
        .step_by(8)
        .map(|i| data_bits[i..i + 8].to_string())
        .collect();

    Ok(chunks)
}

fn get_group_layout(
    error_correction_level: &ErrorCorrectionLevel,
    version: &Version,
) -> Result<GroupConfig, String> {
    if version.version() > 40 || version.version() < 1 {
        return Err("Version must be between 1 and 40".to_string());
    }
    let group_configs = match error_correction_level {
        ErrorCorrectionLevel::Low => [
            (1, 19, 0, 0),
            (1, 34, 0, 0),
            (1, 55, 0, 0),
            (1, 80, 0, 0),
            (1, 108, 0, 0),
            (2, 68, 0, 0),
            (2, 78, 0, 0),
            (2, 97, 0, 0),
            (2, 116, 0, 0),
            (2, 68, 2, 69),
            (4, 81, 0, 0),
            (2, 92, 2, 93),
            (4, 107, 0, 0),
            (3, 115, 1, 116),
            (5, 87, 1, 88),
            (5, 98, 1, 99),
            (1, 107, 5, 108),
            (5, 120, 1, 121),
            (3, 113, 4, 114),
            (3, 107, 5, 108),
            (4, 116, 4, 117),
            (2, 111, 7, 112),
            (4, 121, 5, 122),
            (6, 117, 4, 118),
            (8, 106, 4, 107),
            (10, 114, 2, 115),
            (8, 122, 4, 123),
            (3, 117, 10, 118),
            (7, 116, 7, 117),
            (5, 115, 10, 116),
            (13, 115, 3, 116),
            (17, 115, 0, 0),
            (17, 115, 1, 116),
            (13, 115, 6, 116),
            (12, 121, 7, 122),
            (6, 121, 14, 122),
            (17, 122, 4, 123),
            (4, 122, 18, 123),
            (20, 117, 4, 118),
            (19, 118, 6, 119),
        ],
        ErrorCorrectionLevel::Medium => [
            (1, 16, 0, 0),
            (1, 28, 0, 0),
            (1, 44, 0, 0),
            (2, 32, 0, 0),
            (2, 43, 0, 0),
            (4, 27, 0, 0),
            (4, 31, 0, 0),
            (2, 38, 2, 39),
            (3, 36, 2, 37),
            (4, 43, 1, 44),
            (1, 50, 4, 51),
            (6, 36, 2, 37),
            (8, 37, 1, 38),
            (4, 40, 5, 41),
            (5, 41, 5, 42),
            (7, 45, 3, 46),
            (10, 46, 1, 47),
            (9, 43, 4, 44),
            (3, 44, 11, 45),
            (3, 41, 13, 42),
            (17, 42, 0, 0),
            (17, 46, 0, 0),
            (4, 47, 14, 48),
            (6, 45, 14, 46),
            (8, 47, 13, 48),
            (19, 46, 4, 47),
            (22, 45, 3, 46),
            (3, 45, 23, 46),
            (21, 45, 7, 46),
            (19, 47, 10, 48),
            (2, 46, 29, 47),
            (10, 46, 23, 47),
            (14, 46, 21, 47),
            (14, 46, 23, 47),
            (12, 47, 26, 48),
            (6, 47, 34, 48),
            (29, 46, 14, 47),
            (13, 46, 32, 47),
            (40, 47, 7, 48),
            (18, 47, 31, 48),
        ],
        ErrorCorrectionLevel::Quartile => [
            (1, 13, 0, 0),
            (1, 22, 0, 0),
            (2, 17, 0, 0),
            (2, 24, 0, 0),
            (2, 15, 2, 16),
            (4, 19, 0, 0),
            (2, 14, 4, 15),
            (4, 18, 2, 19),
            (4, 16, 4, 17),
            (6, 19, 2, 20),
            (4, 22, 4, 23),
            (4, 20, 6, 21),
            (8, 20, 4, 21),
            (11, 16, 5, 17),
            (5, 24, 7, 25),
            (15, 19, 2, 20),
            (1, 22, 15, 23),
            (17, 22, 1, 23),
            (17, 21, 4, 22),
            (15, 24, 5, 25),
            (17, 22, 6, 23),
            (7, 24, 16, 25),
            (11, 24, 14, 25),
            (11, 24, 16, 25),
            (7, 24, 22, 25),
            (28, 22, 6, 23),
            (8, 23, 26, 24),
            (4, 24, 31, 25),
            (1, 23, 37, 24),
            (15, 24, 25, 25),
            (42, 24, 1, 25),
            (10, 24, 35, 25),
            (29, 24, 19, 25),
            (44, 24, 7, 25),
            (39, 24, 14, 25),
            (46, 24, 10, 25),
            (49, 24, 10, 25),
            (48, 24, 14, 25),
            (43, 24, 22, 25),
            (34, 24, 34, 25),
        ],
        ErrorCorrectionLevel::High => [
            (1, 9, 0, 0),
            (1, 16, 0, 0),
            (2, 13, 0, 0),
            (4, 9, 0, 0),
            (2, 11, 2, 12),
            (4, 15, 0, 0),
            (4, 13, 1, 14),
            (4, 14, 2, 15),
            (4, 12, 4, 13),
            (6, 15, 2, 16),
            (3, 12, 8, 13),
            (7, 14, 4, 15),
            (12, 11, 4, 12),
            (11, 12, 5, 13),
            (11, 12, 7, 13),
            (3, 15, 13, 16),
            (2, 14, 17, 15),
            (2, 14, 19, 15),
            (9, 13, 16, 14),
            (15, 15, 10, 16),
            (19, 16, 6, 17),
            (34, 13, 0, 0),
            (16, 15, 14, 16),
            (30, 16, 2, 17),
            (22, 15, 13, 16),
            (33, 16, 4, 17),
            (12, 15, 28, 16),
            (11, 15, 31, 16),
            (19, 15, 26, 16),
            (23, 15, 25, 16),
            (23, 15, 28, 16),
            (19, 15, 35, 16),
            (11, 15, 46, 16),
            (59, 16, 1, 17),
            (22, 15, 41, 16),
            (2, 15, 64, 16),
            (24, 15, 46, 16),
            (42, 15, 32, 16),
            (10, 15, 67, 16),
            (20, 15, 61, 16),
        ],
    };

    let group_config = group_configs.get(version.version() as usize);

    match group_config {
        Some(config) => return Ok(GroupConfig::new(config.0, config.1, config.2, config.3)),
        None => return Err("Failed to find config".to_string()),
    }
}
