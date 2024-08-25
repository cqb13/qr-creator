use crate::qr_code::utils::left_pad;
use std::collections::HashMap;

/**
* HELLO WORLD
*
* notes: Alphanumeric mode can only encode uppercase letters
*   - https://www.thonky.com/qr-code-tutorial/alphanumeric-table
*
* Step 1:
* first we need to break up the data into pairs of characters
*
* HE, LL, O , WO, RL, D
*
* Step 2:
* for each pair of characters get the value of first character from the ALPHANUMERIC_VALUES and
* multiply it by 45. Then add that number to the number representation of the second character.
*
* H -> 17
* E -> 14
*
* (17 * 45) + 14 = 779
*
* Step 3:
* convert the resulting number from step 2 in to an 11-bit binary string, padding on the left with
* 0s if needed.
*
* 779 → 01100001011
*
* if encoding an odd number of characters, as we are here, take the number representation of the
* final character and convert it into a 6-bit binary string.
*/
pub fn alphanumeric_encoding(data: &str) -> Result<String, String> {
    let alphanumeric_values: HashMap<&str, i32> = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("A", 10),
        ("B", 11),
        ("C", 12),
        ("D", 13),
        ("E", 14),
        ("F", 15),
        ("G", 16),
        ("H", 17),
        ("I", 18),
        ("J", 19),
        ("K", 20),
        ("L", 21),
        ("M", 22),
        ("N", 23),
        ("O", 24),
        ("P", 25),
        ("Q", 26),
        ("R", 27),
        ("S", 28),
        ("T", 29),
        ("U", 30),
        ("V", 31),
        ("W", 32),
        ("X", 33),
        ("Y", 34),
        ("Z", 35),
        (" ", 36),
        ("$", 37),
        ("%", 38),
        ("*", 39),
        ("+", 40),
        ("-", 41),
        (".", 42),
        ("/", 43),
        (":", 44),
    ]
    .iter()
    .cloned()
    .collect();

    let grouped_data = data
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut bits: String = String::new();

    for group in grouped_data {
        let first_letter = group.chars().nth(0).unwrap().to_string();

        let first_number = match alphanumeric_values.get(&first_letter.as_str()) {
            Some(number) => number * 45,
            None => return Err(format!("Failed to find alphanumeric character \"{}\" in alphanumeric_values, consider using a different encoding mode", first_letter)),
        };

        let second_letter = group.chars().nth(1);

        match second_letter {
            Some(second_letter) => {
                let second_number = match alphanumeric_values.get(&second_letter.to_string().as_str()) {
                    Some(number) => number.to_owned(),
                    None => return Err(format!("Failed to find alphanumeric character \"{}\" in alphanumeric_values, consider using a different encoding mode", second_letter)),
                };

                let total = first_number + second_number;

                let value = left_pad(format!("{:b}", total), 11, "0");

                bits = bits + &value;
            }
            None => {
                // triggers on the last group when encoding an odd number of characters
                let value = left_pad(format!("{:b}", first_number / 45), 6, "0");

                bits = bits + &value;
            }
        }
    }

    Ok(bits)
}
