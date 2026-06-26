pub const NIL: char = std::char::REPLACEMENT_CHARACTER;

/// The mapping for [ASCII](https://en.wikipedia.org/wiki/ASCII)
///
/// This mapping uses the standard ascii character set which is composed of a
/// 7-bit code (or 128 characters). The first 32 characters and the last one
/// are non-printable control characters.
pub const CODEPAGE_ASCII: &'static [char] = &[
    //   0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // 0
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // 1
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', // 2
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', // 3
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', // 4
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', // 5
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', // 6
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', NIL, // 7
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // 8
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // 9
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // A
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // B
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // C
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // D
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // E
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // F
];

/// The mapping for [code page 850](https://en.wikipedia.org/wiki/Code_page_850)
///
/// This code page is also known as `DOS/IBM-ASCII` and is used as the default
/// code page by this library.
pub const CODEPAGE_0850: &'static [char] = &[
    //   0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F
    NIL, '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', // 0
    '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', // 1
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', // 2
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', // 3
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', // 4
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', // 5
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', // 6
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂', // 7
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', // 8
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', 'ø', '£', 'Ø', '×', 'ƒ', // 9
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', // A
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', // B
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', // C
    '╨', '╤', '╥', '╙', '╘', '╒', '╕', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', // D
    'α', 'ß', 'Γ', '∏', '∑', 'σ', 'µ', 'τ', 'Φ', 'θ', 'Ω', 'δ', '∞', 'Ø', 'ε', '∩', // E
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '▫', '¨', '·', '√', 'ⁿ', '²', '■', NIL, // F
];

/// The mapping for [code page 1252](https://en.wikipedia.org/wiki/Code_page_1252)
///
/// This code page is also known as `Latin 1 Windows` or `ANSI`.
pub const CODEPAGE_1252: &'static [char] = &[
    //   0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // 0
    NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, NIL, // 1
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', // 2
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', // 3
    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', // 4
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', // 5
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', // 6
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', NIL, // 7
    '€', NIL, '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', NIL, 'Ž', NIL, // 8
    NIL, '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'Š', '›', 'œ', NIL, 'ž', 'Ÿ', // 9
    ' ', '¡', '¢', '£', '¤', '¥', '¦', '§', '¨', '©', 'ª', '«', '¬', NIL, '®', '¯', // A
    '°', '±', '²', '³', '´', 'µ', '¶', '·', '¸', '¹', 'º', '»', '¼', '½', '¾', '¿', // B
    'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'Æ', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï', // C
    'Ð', 'Ñ', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', '×', 'Ø', 'Ù', 'Ú', 'Û', 'Ü', 'Ý', 'Þ', 'ß', // D
    'à', 'á', 'â', 'ã', 'ä', 'å', 'æ', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï', // E
    'ð', 'ñ', 'ò', 'ó', 'ô', 'õ', 'ö', '÷', 'ø', 'ù', 'ú', 'û', 'ü', 'ý', 'þ', 'ÿ', // F
];

fn contains(byte: u8, codepage: &[char]) -> bool {
    (byte as usize) < codepage.len()
}

fn is_nil(byte: u8, codepage: &[char]) -> bool {
    codepage[byte as usize] == NIL
}

fn is_printable(byte: u8, codepage: &[char]) -> bool {
    contains(byte, codepage) && !is_nil(byte, codepage)
}

/// Returns a byte's character representation given a specific codepage
pub fn as_char(byte: u8, codepage: &[char], repl_char: char) -> char {
    if !is_printable(byte, codepage) {
        return repl_char;
    }

    return codepage[byte as usize];
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;

    #[test]
    fn test_hardcoded_ascii_table_matches_the_generated_one() {
        let codepage: Vec<char> = std::iter::empty()
            .chain(std::iter::repeat(super::NIL).take(32)) // The first 32 control characters
            .chain((32..127).map(|c| std::char::from_u32(c).unwrap())) // The following 95 printable chars
            .chain(std::iter::once(super::NIL)) // The DEL character
            .chain(std::iter::repeat(super::NIL).take(128)) // The characters for the 8th bit
            .collect();

        assert_eq!(CODEPAGE_ASCII, &*codepage);
    }
}
