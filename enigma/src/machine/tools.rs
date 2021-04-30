/// char_to_idx returns the u8 position of the character in the alphabet, 0-indexed and
/// case-insensitive
pub fn char_to_idx(c: char) -> u8 {
    // todo: add checks to verify c is in range [aA-zZ]
    let ci = c as u8;
    // keep in mind lower-case & uppercase characters
    if ci >= 97 {
        return ci - 97;
    }
    ci - 65
}
