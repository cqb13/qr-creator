use qr_code::qr_code::encoding::encode;
use qr_code::qr_code::EncodingMode;

#[test]
fn test_alphanumeric_proper() {
    let data = "HELLO WORLD";

    let encoded_data = encode(data, &EncodingMode::Alphanumeric);

    assert!(
        encoded_data.is_ok(),
        "Failed to encode a proper alphanumeric string: {}",
        encoded_data.unwrap()
    );

    let encoded_data = encoded_data.unwrap();

    assert_eq!(
        encoded_data, "0110000101101111000110100010111001011011100010011010100001101",
        "Encoded dada \"{}\" did not match expected result \"0110000101101111000110100010111001011011100010011010100001101\"",
        encoded_data
    )
}

#[test]
fn test_numeric_encoding_fail() {
    let data = "HElsk039('!";

    let encoded_data = encode(data, &EncodingMode::Alphanumeric);

    assert!(
        !encoded_data.is_ok(),
        "Failed to detect invalid characters in numeric encoding data"
    )
}
