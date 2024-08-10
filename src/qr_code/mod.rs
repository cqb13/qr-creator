pub mod version;

pub enum EncodingMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

impl EncodingMode {
    pub fn to_bits(&self) -> Vec<u8> {
        match self {
            EncodingMode::Numeric => vec![0, 0, 0, 1],
            EncodingMode::Alphanumeric => vec![0, 0, 1, 0],
            EncodingMode::Byte => vec![0, 1, 0, 0],
            EncodingMode::Kanji => vec![1, 0, 0, 0],
        }
    }
}

pub enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl ErrorCorrectionLevel {}

pub enum Version {
    Normal(i16),
}

impl Version {
    pub fn size(&self) -> i16 {
        match self {
            Version::Normal(version) => version * 4 + 17,
        }
    }
}

pub struct QrCode {
    encoding_mode: EncodingMode,
    error_correction_level: ErrorCorrectionLevel,
    version: Version,
}
