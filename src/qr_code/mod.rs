pub mod encoding;
pub mod version;

use encoding::encode;
use version::determine_optimal_qr_code_version;

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

    pub fn to_string(&self) -> String {
        match self {
            EncodingMode::Numeric => "Numeric",
            EncodingMode::Alphanumeric => "Alphanumeric",
            EncodingMode::Byte => "Byte",
            EncodingMode::Kanji => "Kanji",
        }
        .to_string()
    }
}

pub enum ErrorCorrectionLevel {
    Low,
    Medium,
    Quartile,
    High,
}

impl ErrorCorrectionLevel {
    pub fn to_string(&self) -> String {
        match self {
            ErrorCorrectionLevel::Low => "Low",
            ErrorCorrectionLevel::Medium => "Medium",
            ErrorCorrectionLevel::Quartile => "Quartile",
            ErrorCorrectionLevel::High => "High",
        }
        .to_string()
    }
}

pub enum Version {
    Normal(i16),
}

impl Version {
    pub fn size(&self) -> i16 {
        match self {
            Version::Normal(version) => version * 4 + 17,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Version::Normal(version) => format!("Version {}", version),
        }
    }
}

pub struct QrCode {
    pub encoding_mode: EncodingMode,
    pub error_correction_level: ErrorCorrectionLevel,
    pub version: Version,
    pub data: String,
}

impl QrCode {
    pub fn create(
        data: String,
        encoding_mode: EncodingMode,
        error_correction_level: ErrorCorrectionLevel,
    ) -> Result<QrCode, String> {
        let version = match determine_optimal_qr_code_version(
            &encoding_mode,
            &error_correction_level,
            data.len() as i32,
        ) {
            Ok(version) => version,
            Err(err) => return Err(err),
        };

        let encoded_data = match encode(&data, &encoding_mode) {
            Ok(encoded_data) => encoded_data,
            Err(err) => return Err(err),
        };

        println!("{:?}", encoded_data);

        Ok(QrCode {
            encoding_mode,
            error_correction_level,
            version,
            data,
        })
    }

    /**
     * Prints information about the qr code
     */
    pub fn details(&self) {
        println!("---- {} QR Code ----\n", self.version.to_string());
        println!("Encoding Mode: {}", self.encoding_mode.to_string());
        println!(
            "Error Correction Level: {}",
            self.error_correction_level.to_string()
        );
    }
}
