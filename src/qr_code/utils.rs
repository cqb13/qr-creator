pub fn left_pad(string: &str, length: i32, char: &str) -> String {
    let mut padded = string.to_string();

    while length > padded.len() as i32 {
        padded = format!("{}{}", char, padded);
    }

    padded
}

pub fn right_pad(string: &str, length: i32, char: &str) -> String {
    let mut padded = string.to_string();

    while length > padded.len() as i32 {
        padded += char;
    }

    padded
}

pub fn convert_hex_to_binary(hex: &str) -> String {
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
