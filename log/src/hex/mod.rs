//! Vendored from the [`hexplay`](https://crates.io/crates/hexplay) crate
//! (v0.3.0) by Tom Moers, dual-licensed `MIT OR Apache-2.0`. Inlined here with
//! its single `atty` call replaced by `std::io::IsTerminal`, since `atty` is
//! unmaintained (RUSTSEC-2024-0375) and unsound (RUSTSEC-2021-0145).
//!
//! This module provides a way to [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//! a byte slice as it is commonly done in a hex-editor.
//!
//! The configuration of the visualization are stored in the [HexView](struct.HexView.html),
//! struct, which can be easily constructed using the [HexViewBuilder](struct.HexViewBuilder.html).
//!
//! # Examples
//!
//! Usage is very simple, just build a `HexView` and use it for formatting:
//!
//! ```ignore
//! use hexplay::HexViewBuilder;
//!
//! // The buffer we want to display
//! let data : Vec<u8> = (0u8..200u8).collect();
//!
//! // Build a new HexView using the provider builder
//! let view = HexViewBuilder::new(&data[40..72])
//!     .address_offset(40)
//!     .row_width(16)
//!     .finish();
//!
//! println!("{}", view);
//!
//! # let result = format!("{}", view);
//! # let mut lines = result.lines();
//! # assert_eq!("00000020                          28 29 2A 2B 2C 2D 2E 2F  |         ()*+,-./ |", lines.next().unwrap());
//! # assert_eq!("00000030  30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F  | 0123456789:;<=>? |", lines.next().unwrap());
//! # assert_eq!("00000040  40 41 42 43 44 45 46 47                          | @ABCDEFG         |", lines.next().unwrap());
//! ```
//!
//! This will result in the following output:
//!
//! ```text
//! 00000020                          28 29 2A 2B 2C 2D 2E 2F  |         ()*+,-./ |
//! 00000030  30 31 32 33 34 35 36 37 38 39 3A 3B 3C 3D 3E 3F  | 0123456789:;<=>? |
//! 00000040  40 41 42 43 44 45 46 47                          | @ABCDEFG         |
//! ```
//!
//! # Color
//!
//! You can add color to the hextable by specifying a [color::Spec](color/struct.Spec.html) and a range in the hextable to color,
//! using HexViewBuilder's [add_colors](struct.HexViewBuilder.html#method.add_colors) method.
//!
//! **NB**: overlapping color ranges have unspecified behavior (not unsafe though, of course)
//!
//! ```ignore
//! use hexplay::HexViewBuilder;
//!
//! let data : Vec<u8> = (0u8..200u8).collect();
//!
//! let view = HexViewBuilder::new(&data[40..72])
//!     .address_offset(40)
//!     .row_width(16)
//!     .add_colors(vec![
//!         (hexplay::color::red(), 6..15),
//!         (hexplay::color::blue(), 21..26),
//!         (hexplay::color::yellow_bold(), 15..21),
//!         (hexplay::color::green(), 0..6),
//!     ])
//!     .finish();
//!
//! // this will print to stdout
//! view.print().unwrap();
//! ```

// Vendored verbatim from hexplay 0.3.0 (aside from the atty->IsTerminal fix);
// suppress style lints to keep this module close to upstream.
#![allow(clippy::all, dead_code, unused_imports, mismatched_lifetime_syntaxes)]

mod byte_mapping;
pub mod color;
mod format;

pub use byte_mapping::CODEPAGE_0850;
pub use byte_mapping::CODEPAGE_1252;
pub use byte_mapping::CODEPAGE_ASCII;
pub use format::HexView;
pub use format::HexViewBuilder;
