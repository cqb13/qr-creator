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
pub fn byte_encoding(data: &str) -> Result<Vec<String>, String> {
    let mut bits: Vec<String> = Vec::new();

    for char in data.chars() {
        let string = char.to_string();

        let byte = format!("{:02X?}", string.as_bytes());

        let binary_bits = convert_to_binary_from_hex(&byte);

        bits.push(binary_bits);
    }

    Ok(bits)
}

fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
