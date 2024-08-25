/**
* 8675309
*
* Step 1:
* first we need to break up the string into groups of 3 digits
* if the length is not a multiple of 3, the final group will be only 1 or 2 digits long
*
* 867 530 9
*
* Step 2:
* treat each group as a three digit number (or fewer than 3 if the final group is 1 or 2  digits long)
* if a group starts with 0, it should be interpreted as a 2 digit number and should be converted to 7 binary bits
* if there are 2 zeros at the start of a group it should be interpreted as a 1 digit number and converted in to 4 binary bits
* conditions above also apply to to the last group if it is 1 or 2 digits long
*
* 867 → 1101100011
* 530 → 1000010010
* 9 → 1001
*/
pub fn numeric_encoding(data: &str) -> Result<Vec<String>, String> {
    let grouped_data = data
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut bits: Vec<String> = Vec::new();

    for group in grouped_data {
        let number = match group.parse::<i32>() {
            Ok(number) => number,
            Err(err) => return Err(err.to_string()),
        };

        let value = format!("{:b}", number);

        bits.push(value)
    }

    Ok(bits)
}
