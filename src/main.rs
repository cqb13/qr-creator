pub mod qr_code;

use crate::qr_code::{EncodingMode, ErrorCorrectionLevel, QrCode};

fn main() {
    let data = "HELLO WORLD".to_string();

    let qr_code = match QrCode::create(
        data,
        EncodingMode::Alphanumeric,
        ErrorCorrectionLevel::Quartile,
    ) {
        Ok(qr_code) => qr_code,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1)
        }
    };

    qr_code.details()
}
