use std::io;
use std::ops::Range;

use termcolor::{Buffer, BufferWriter, Color, ColorChoice, WriteColor};

use super::byte_mapping;
use super::color::{ColorRange, ColorlessString, Colors, Spec};

/// The HexView struct represents the configuration of how to display the data.
pub struct HexView<'a> {
    address_offset: usize,
    codepage: &'a [char],
    data: &'a [u8],
    replacement_character: char,
    row_width: usize,
    colors: Colors,
    force_color: bool,
}

macro_rules! color {
    ($fmt:ident, $color:ident, $str:expr) => {{
        $fmt.set_color(&$color)?;
        write!($fmt, "{}", $str)?;
        $fmt.reset()
    }};
}

impl<'a> HexView<'a> {
    /// Prints the hextable to stdout. If any colors were given during construction, the specified ranges will be printed in color.
    pub fn print(&self) -> io::Result<()> {
        let cc = if self.force_color || std::io::IsTerminal::is_terminal(&std::io::stdout()) {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        };
        let writer = BufferWriter::stdout(cc);
        let mut buffer: Buffer = writer.buffer();
        self.fmt(&mut buffer)?;
        writer.print(&buffer)?;
        Ok(())
    }

    /// Constructs a new HexView for the given data without offset and using codepage 850, a row width
    /// of 16 and `.` as replacement character.
    pub fn new(data: &[u8]) -> HexView {
        HexView {
            address_offset: 0,
            codepage: &byte_mapping::CODEPAGE_0850,
            data: data,
            replacement_character: '.',
            row_width: 16,
            colors: Colors::new(),
            force_color: false,
        }
    }

    pub fn fmt<W: WriteColor>(&self, buffer: &mut W) -> io::Result<()> {
        let begin_padding = calculate_begin_padding(self.address_offset, self.row_width);
        let end_padding = calculate_end_padding(begin_padding + self.data.len(), self.row_width);
        let mut address = self.address_offset - begin_padding;
        let mut offset = 0;
        let mut color_range = ColorRange::new(&self.colors);
        let mut separator = "";

        if self.data.len() + begin_padding + end_padding <= self.row_width {
            fmt_line(
                buffer,
                address,
                &self.codepage,
                self.replacement_character,
                &self.data,
                &mut color_range,
                &Padding::new(begin_padding, end_padding),
            )?;
            return Ok(());
        }

        if begin_padding != 0 {
            let slice = &self.data[offset..offset + self.row_width - begin_padding];
            fmt_line(
                buffer,
                address,
                &self.codepage,
                self.replacement_character,
                &slice,
                &mut color_range,
                &Padding::from_left(begin_padding),
            )?;
            offset += self.row_width - begin_padding;
            address += self.row_width;
            separator = "\n";
            color_range.update_offset(offset);
        }

        while offset + (self.row_width - 1) < self.data.len() {
            let slice = &self.data[offset..offset + self.row_width];
            write!(buffer, "{}", separator)?;
            fmt_line(
                buffer,
                address,
                &self.codepage,
                self.replacement_character,
                &slice,
                &mut color_range,
                &Padding::default(),
            )?;
            offset += self.row_width;
            address += self.row_width;
            separator = "\n";
            color_range.update_offset(offset);
        }

        if end_padding != 0 {
            let slice = &self.data[offset..];
            writeln!(buffer, "")?;
            fmt_line(
                buffer,
                address,
                &self.codepage,
                self.replacement_character,
                &slice,
                &mut color_range,
                &Padding::from_right(end_padding),
            )?;
        }
        Ok(())
    }
}

/// A builder for the [HexView](struct.HexView.html) struct.
pub struct HexViewBuilder<'a> {
    hex_view: HexView<'a>,
}

impl<'a> HexViewBuilder<'a> {
    /// Constructs a new HexViewBuilder for the given data.
    pub fn new(data: &[u8]) -> HexViewBuilder {
        HexViewBuilder {
            hex_view: HexView::new(&data),
        }
    }

    /// Configures the address offset of the HexView under construction.
    pub fn address_offset(mut self, offset: usize) -> HexViewBuilder<'a> {
        self.hex_view.address_offset = offset;
        self
    }

    /// Forces any color data to be printed in `print`, even if redirected to a file or pipe.
    pub fn force_color(mut self) -> Self {
        self.hex_view.force_color = true;
        self
    }

    /// Configures the codepage of the HexView under construction.
    pub fn codepage<'b: 'a>(mut self, codepage: &'b [char]) -> HexViewBuilder<'a> {
        self.hex_view.codepage = codepage;
        self
    }

    /// Configures the replacement character of the HexView under construction.
    ///
    /// The replacement character is the character that will be used for nonprintable
    /// characters in the codepage.
    pub fn replacement_character(mut self, ch: char) -> HexViewBuilder<'a> {
        self.hex_view.replacement_character = ch;
        self
    }

    /// Configures the row width of the HexView under construction.
    pub fn row_width(mut self, width: usize) -> HexViewBuilder<'a> {
        self.hex_view.row_width = width;
        self
    }
    /// Adds the vector of `colors` to the range color printer
    pub fn add_colors(mut self, colors: Colors) -> HexViewBuilder<'a> {
        self.hex_view.colors.extend(colors);
        self
    }
    /// Adds the `color` to the given `range`, using a more ergonomic API
    pub fn add_color(mut self, color: &str, range: Range<usize>) -> HexViewBuilder<'a> {
        use std::str::FromStr;
        self.hex_view.colors.push((
            Spec::new()
                .set_fg(Some(Color::from_str(color).unwrap()))
                .clone(),
            range,
        ));
        self
    }
    /// Constructs the HexView.
    pub fn finish(mut self) -> HexView<'a> {
        self.hex_view
            .colors
            .sort_by(|&(_, ref r1), &(_, ref r2)| r1.start.cmp(&r2.start));
        self.hex_view
    }
}

#[derive(Default)]
struct Padding {
    left: usize,
    right: usize,
}

impl Padding {
    fn new(left_padding: usize, right_padding: usize) -> Padding {
        Padding {
            left: left_padding,
            right: right_padding,
        }
    }

    fn from_left(left_padding: usize) -> Padding {
        Padding {
            left: left_padding,
            right: 0,
        }
    }

    fn from_right(right_padding: usize) -> Padding {
        Padding {
            left: 0,
            right: right_padding,
        }
    }
}

fn fmt_bytes_as_hex<W: WriteColor>(
    f: &mut W,
    bytes: &[u8],
    color_range: &ColorRange,
    padding: &Padding,
) -> io::Result<()> {
    let mut separator = "";

    for _ in 0..padding.left {
        write!(f, "{}  ", separator)?;
        separator = " ";
    }

    for (i, byte) in bytes.iter().enumerate() {
        match color_range.get(i) {
            Some(rgb) => {
                write!(f, "{}", separator)?;
                color!(f, rgb, format!("{:02X}", byte))?;
            }
            None => write!(f, "{}{:02X}", separator, byte)?,
        }
        separator = " ";
    }

    for _ in 0..padding.right {
        write!(f, "{}  ", separator)?;
        separator = " ";
    }

    Ok(())
}

fn fmt_bytes_as_char<W: WriteColor>(
    f: &mut W,
    cp: &[char],
    repl_char: char,
    bytes: &[u8],
    color_range: &ColorRange,
    padding: &Padding,
) -> io::Result<()> {
    for _ in 0..padding.left {
        write!(f, " ")?;
    }

    for (i, &byte) in bytes.iter().enumerate() {
        let byte = byte_mapping::as_char(byte, cp, repl_char);
        match color_range.get(i) {
            Some(rgb) => {
                color!(f, rgb, format!("{}", byte))?;
            }
            _ => write!(f, "{}", byte)?,
        }
    }

    for _ in 0..padding.right {
        write!(f, " ")?;
    }

    Ok(())
}

fn fmt_line<W: WriteColor>(
    f: &mut W,
    address: usize,
    cp: &[char],
    repl_char: char,
    bytes: &[u8],
    color_range: &mut ColorRange,
    padding: &Padding,
) -> io::Result<()> {
    write!(f, "{:0width$X}", address, width = 8)?;

    write!(f, "  ")?;
    fmt_bytes_as_hex(f, bytes, &color_range, &padding)?;
    write!(f, "  ")?;

    write!(f, "| ")?;
    fmt_bytes_as_char(f, cp, repl_char, bytes, &color_range, &padding)?;
    write!(f, " |")?;

    Ok(())
}

fn calculate_begin_padding(address_offset: usize, row_width: usize) -> usize {
    debug_assert!(
        row_width != 0,
        "A zero row width is can not be used to calculate the begin padding"
    );
    address_offset % row_width
}

fn calculate_end_padding(data_size: usize, row_width: usize) -> usize {
    debug_assert!(
        row_width != 0,
        "A zero row width is can not be used to calculate the end padding"
    );
    (row_width - data_size % row_width) % row_width
}

impl<'a> std::fmt::Display for HexView<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.row_width == 0 {
            write!(f, "Invalid HexView::width")?;
            return Err(std::fmt::Error);
        }
        let mut string = ColorlessString(String::new());
        match self.fmt(&mut string) {
            Ok(()) => {
                write!(f, "{}", string.0)
            }
            Err(e) => write!(f, "{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;

    #[test]
    fn test_begin_padding() {
        // Rust 1.13 needs the fully qualified name here
        assert_eq!(super::calculate_begin_padding(0, 16), 0);
        assert_eq!(super::calculate_begin_padding(16, 16), 0);
        assert_eq!(super::calculate_begin_padding(54, 16), 6);
    }

    #[test]
    fn a_full_line_is_formatted_as_expected() {
        let data: Vec<u8> = (0x40..0x40 + 0xF + 1).collect();

        let row_view = HexViewBuilder::new(&data).row_width(data.len()).finish();

        let result = format!("{}", row_view);

        assert_eq!(
            result,
            "00000000  40 41 42 43 44 45 46 47 48 49 4A 4B 4C 4D 4E 4F  | @ABCDEFGHIJKLMNO |"
        );
    }

    #[test]
    fn an_incomplete_line_is_padded_on_the_right() {
        let data = ['a' as u8; 10];

        let row_view = HexViewBuilder::new(&data).row_width(16).finish();

        let result = format!("{}", row_view);

        assert_eq!(
            result,
            "00000000  61 61 61 61 61 61 61 61 61 61                    | aaaaaaaaaa       |"
        );
    }

    #[test]
    fn an_unaligned_address_causes_padded_on_the_left() {
        let data = ['a' as u8; 11];

        let row_view = HexViewBuilder::new(&data)
            .address_offset(5)
            .row_width(16)
            .finish();

        let result = format!("{}", row_view);
        println!("{}", result);

        assert_eq!(
            result,
            "00000000                 61 61 61 61 61 61 61 61 61 61 61  |      aaaaaaaaaaa |"
        );
    }

    #[test]
    fn an_unaligned_incomplete_line_causes_padding_on_both_sides() {
        let data = ['a' as u8; 8];

        let row_view = HexViewBuilder::new(&data)
            .address_offset(5)
            .row_width(16)
            .finish();

        let result = format!("{}", row_view);
        println!("{}", result);

        assert_eq!(
            result,
            "00000000                 61 61 61 61 61 61 61 61           |      aaaaaaaa    |"
        );
    }

    #[test]
    fn decreasing_the_row_width_increases_the_total_character_count() {
        let data: Vec<u8> = (0..64).collect();

        let short_row_view = HexViewBuilder::new(&data).row_width(1).finish();
        let long_row_view = HexViewBuilder::new(&data).row_width(16).finish();

        let short_row_result = format!("{}", short_row_view);
        let long_row_result = format!("{}", long_row_view);

        assert!(long_row_result.len() < short_row_result.len());
    }

    #[test]
    fn the_address_offset_is_zero_by_default() {
        let data = [99; 16];

        let row_view = HexViewBuilder::new(&data).row_width(16).finish();

        let result = format!("{}", row_view);
        let address_offset_str = format!("{:X}", 0);

        assert!(result.contains(&address_offset_str));
    }

    #[test]
    fn the_address_offset_is_used_when_given() {
        let data = [0; 16];

        let address_offset = data.len() * 10;
        let row_view = HexViewBuilder::new(&data)
            .row_width(16)
            .address_offset(address_offset)
            .finish();

        let result = format!("{}", row_view);
        let address_offset_str = format!("{:X}", address_offset);

        assert!(result.contains(&address_offset_str));
    }

    #[test]
    fn the_address_offset_increases_by_the_row_width_for_each_row() {
        let data = [0; 16 * 5];

        let address_offset = data.len() * 10;
        let row_view = HexViewBuilder::new(&data)
            .row_width(16)
            .address_offset(address_offset)
            .finish();

        let result = format!("{}", row_view);
        let row_2_address_offset_str = format!("{:X}", address_offset + 2 * row_view.row_width);
        let row_4_address_offset_str = format!("{:X}", address_offset + 4 * row_view.row_width);

        assert!(result.contains(&row_2_address_offset_str));
        assert!(result.contains(&row_4_address_offset_str));
    }

    #[test]
    fn there_is_no_superfluous_whitespace() {
        let data = [0; 17];

        let one_line_result = format!("{}", HexViewBuilder::new(&data[0..16]).finish());
        let two_line_result = format!("{}", HexViewBuilder::new(&data[0..17]).finish());

        println!("{}", one_line_result);
        println!("{}", two_line_result);

        assert_eq!(one_line_result, one_line_result.trim());
        assert_eq!(two_line_result, two_line_result.trim());
    }

    #[test]
    fn the_row_width_is_16_by_default() {
        let data = [0; 17];

        let one_line_result = format!("{}", HexViewBuilder::new(&data[0..16]).finish());
        let two_line_result = format!("{}", HexViewBuilder::new(&data[0..17]).finish());

        println!("{}", one_line_result);
        println!("{}", two_line_result);

        assert_eq!(1, one_line_result.lines().count());
        assert_eq!(2, two_line_result.lines().count());
    }

    #[test]
    fn the_replacement_character_is_dot_by_default() {
        let data = [0; 1];
        let empty_cp = [];

        let result = format!(
            "{}",
            HexViewBuilder::new(&data).codepage(&empty_cp).finish()
        );

        println!("{}", result);

        assert!(result.contains('.'));
    }

    #[test]
    fn the_replacement_character_can_be_changed() {
        let data = [0; 1];
        let empty_cp = [];

        let result = format!(
            "{}",
            HexViewBuilder::new(&data)
                .codepage(&empty_cp)
                .replacement_character(std::char::REPLACEMENT_CHARACTER)
                .finish()
        );

        println!("{}", result);

        assert!(result.contains(std::char::REPLACEMENT_CHARACTER));
    }

    #[test]
    fn all_characters_can_be_printed() {
        let data: Vec<u8> = (0u16..256u16).map(|v| v as u8).collect();

        let dump_view = HexViewBuilder::new(&data)
            .address_offset(20)
            .row_width(8)
            .finish();

        let result = format!("{}", dump_view);
        println!("{}", result);

        assert!(!result.is_empty());
    }
}
