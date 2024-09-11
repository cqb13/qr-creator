mod character_count;
pub mod encoding;
mod utils;
pub mod version;

use character_count::create_character_count_indicator;
use encoding::encode;
use utils::right_pad;
use version::{determine_data_bits_required_for_version, determine_optimal_qr_code_version};

pub enum EncodingMode {
    Numeric,
    Alphanumeric,
    Byte,
}

impl EncodingMode {
    pub fn to_bits(&self) -> String {
        match self {
            EncodingMode::Numeric => "0001",
            EncodingMode::Alphanumeric => "0010",
            EncodingMode::Byte => "0100",
        }
        .to_string()
    }

    pub fn to_string(&self) -> String {
        match self {
            EncodingMode::Numeric => "Numeric",
            EncodingMode::Alphanumeric => "Alphanumeric",
            EncodingMode::Byte => "Byte",
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

    pub fn version(&self) -> i16 {
        match self {
            Version::Normal(version) => version.to_owned(),
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
    pub encoded_data: String,
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

        let character_count_indicator =
            create_character_count_indicator(&data, &encoding_mode, &version);

        let data_bits_required_for_version =
            match determine_data_bits_required_for_version(&version, &error_correction_level) {
                Ok(data_bits_required_for_version) => data_bits_required_for_version,
                Err(err) => return Err(err),
            };

        let constructed_data = match construct_data(
            encoding_mode.to_bits(),
            &character_count_indicator,
            &encoded_data,
            data_bits_required_for_version,
        ) {
            Ok(construct_data) => construct_data,
            Err(err) => return Err(err),
        };

        println!(
            "req: {}, current: {}",
            data_bits_required_for_version,
            constructed_data.len()
        );
        println!("{}", constructed_data);

        //TODO: error correction

        Ok(QrCode {
            encoding_mode,
            error_correction_level,
            version,
            data,
            encoded_data,
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

fn construct_data(
    encoding_mode_bits: String,
    character_count_indicator_bits: &str,
    encoded_data_bits: &str,
    data_bits_required_for_version: i32,
) -> Result<String, String> {
    let mut bits = format!(
        "{}{}{}",
        encoding_mode_bits, character_count_indicator_bits, encoded_data_bits
    );

    if bits.len() > data_bits_required_for_version as usize {
        return Err("Data bits are longer than what the QR Code can hold".to_string());
    }

    // adding terminator bits
    if bits.len() + 4 != data_bits_required_for_version as usize {
        bits += "0000";

        if bits.len() == data_bits_required_for_version as usize {
            return Ok(bits);
        }

        // making multiple of 8;
        let len = bits.len() % 8;
        let dif = 8 - len;

        bits = right_pad(&bits, (dif + bits.len()) as i32, "0");
        if bits.len() == data_bits_required_for_version as usize {
            return Ok(bits);
        }

        // TODO: improve this: required_len - current_len = x; x/8 = bytes_to_pad
        let pad_string_odd = "11101100";
        let pad_string_even = "00010001";

        while bits.len() != data_bits_required_for_version as usize {
            bits += pad_string_odd;
            if bits.len() == data_bits_required_for_version as usize {
                return Ok(bits);
            }

            bits += pad_string_even;

            if bits.len() == data_bits_required_for_version as usize {
                return Ok(bits);
            }
        }
    } else {
        while bits.len() != data_bits_required_for_version as usize {
            bits += "0";
        }
    }

    Ok(bits)
}
