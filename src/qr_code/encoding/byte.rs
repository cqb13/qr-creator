use crate::qr_code::utils::convert_hex_to_binary;

/**
* Hello, World!
*
* Step 1:
* convert data into ISO 8859-1, or UTF-8
* ISO 8859-1 is preferred
*
* Step 2:
* convert each character in the string into hex bytes
*
* Step 3:
* convert each byte into an 8-bit binary string, pad to the left with zeros if neeeded
*/
pub fn byte_encoding(data: &str) -> Result<String, String> {
    let mut bits: String = String::new();

    for char in data.chars() {
        let string = char.to_string();

        let byte = format!("{:02X?}", string.as_bytes());

        let binary_bits = convert_hex_to_binary(&byte);

        bits = bits + &binary_bits;
    }

    Ok(bits)
}
