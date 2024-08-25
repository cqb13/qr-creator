use qr_code::qr_code::encoding::encode;
use qr_code::qr_code::EncodingMode;

#[test]
fn test_numeric_encoding_proper() {
    let data = "8675309";

    let encoded_data = encode(data, &EncodingMode::Numeric);

    assert!(
        encoded_data.is_ok(),
        "Failed to encode a proper numeric string: {}",
        encoded_data.unwrap()
    );

    let encoded_data = encoded_data.unwrap();

    assert_eq!(
        encoded_data, "110110001110000100101001",
        "Encoded data \"{}\" did not match expected result \"110110001110000100101001\"",
        encoded_data
    )
}

#[test]
fn test_numeric_encoding_fail() {
    let data = "839k29-39";

    let encded_data = encode(data, &EncodingMode::Numeric);

    assert!(
        !encded_data.is_ok(),
        "Failed to detect invalid characters in numeric encoding data"
    )
}
