/**
* Kanji mode can only encode double-byte Shift JIS characters whose bytes are in the ranges 0x8140 to 0x9FFC and 0xE040 to 0xEBBF (hexadecimal)
*
* Step 1:
* convert characters to bytes
*
* 茗 -> 0xE4AA
* 荷 -> 0x89D7
*
* Step 2:
* Kanji mode has two methods for encoding double-byte Kanji characters.
* One method is for characters whose bytes are in the range 0x8140 to 0x9FFC.
* The other method is for characters whose bytes are in the range 0xE040 to 0xEBBF.
*
*   Method 1: (0x8140 to 0x9FFC)
*
*       Step 1:
*       From the example above, the character 荷 is 0x89D7 in Shift JIS, so it is in the range 0x8140 to 0x9FFC.
*       The first step is to subtract 0x8140 from the hex value.
*
*       0x89D7 - 0x8140 = 0x0897
*
*       Step 2:
*       Split the number resulting from step 1 into its most and least significant byte
*
*       Most significant byte of 0x0897 is 0x08
*       Least significant byte of 0x0897 is 0x97
*
*       Step 3:
*       Multiply the most significant byte by 0xC0, then add the least significant byte to the result
*
*       (0x08 * 0xC0) + 0x97 = (0x600) + 0x97 = 0x697
*
*       Step 4:
*       Convert the result to 13-bit binary
*
*       0x697 = 0 0110 1001 0111
*
*   Method 2:
*
*       Step 1:
*       From the example above, the character 茗 is 0xE4AA in Shift JIS, so it is in the range 0xE040 to 0xEBBF.
*       The first step is to subtract 0xC140 from the hex value.
*
*       0xE4AA - 0xC140 = 0x236A
*
*       Step 2:
*       Split the number resulting from step 1 into its most and least significant byte
*       
*       Most significant byte of 0x236A is 0x23
*       Least significant byte of 0x236A is 0x6A
*
*       Step 3:
*       Multiply the most significant byte by 0xC0, then add the least significant byte to the result
*
*       (0x23 * 0xC0) + 0x6A = (0x1A40) + 0x6A = 0x1AAA
*
*       Step 4:
*       Convert the result to 13-bit binary
*
*       0x1AAA = 1 1010 1010 1010
*
* Step 3:
* Put the string back together
*
* 茗荷 encoded for QR code is 11010101010100011010010111
*/
pub fn kanji_encoding(data: &str) -> Result<String, String> {
    let mut bits: String = String::new();

    Ok(bits)
}
