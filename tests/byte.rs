use qr_code::qr_code::encoding::encode;
use qr_code::qr_code::EncodingMode;

#[test]
fn test_byte_encoding_proper() {
    let data = "Hello, World!";

    let encoded_data = encode(data, &EncodingMode::Byte);

    assert!(
        encoded_data.is_ok(),
        "Failed to encode a proper byte string: {}",
        encoded_data.unwrap()
    );

    let encoded_data = encoded_data.unwrap();

    assert_eq!(
        encoded_data, "01001000011001010110110001101100011011110010110000100000010101110110111101110010011011000110010000100001",
        "Encoded data \"{}\" did not match expected result \"01001000011001010110110001101100011011110010110000100000010101110110111101110010011011000110010000100001\"",
        encoded_data
    );
}
