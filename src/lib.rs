//! Implementation of [ZeroFormatter](https://github.com/neuecc/ZeroFormatter) in Rust.
//!
//! # Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! zero-formatter = "0.1"
//! ```
//!
//! # Examples
//!
//! ```
//! #[macro_use] extern crate zero_formatter;
//! extern crate byteorder;
//! use zero_formatter::*;
//! use std::io::{Seek, SeekFrom, Read, Write, Cursor, Result};
//! use byteorder::{ReadBytesExt, WriteBytesExt};
//!
//! declare_buffer! { Buffer }
//!
//! object_formatter! {
//!     #[target(Buffer<Cursor<Vec<u8>>>)]
//!     ObjectSample {
//!         0; a: i32,
//!         1; b: i64
//!     }
//! }
//!
//! # fn example() -> Result<()> {
//! let mut writer = Buffer::new(Cursor::new(Vec::new()));
//! try!(writer.serialize(0, ObjectSample { a: 1, b: 2 }));
//! # Ok(())
//! # }
//! #
//! # fn main() {
//! # example();
//! # }
//! ```

extern crate byteorder;
extern crate chrono;

mod error;
mod formatter;
#[macro_use]
mod buffer;
mod primitive;
pub mod util;
#[macro_use]
mod has_value;
#[macro_use]
mod option;
#[macro_use]
mod object;
mod time;
mod sequence;
mod union;

pub use error::ZeroFormatterResult;
pub use error::ZeroFormatterError;
pub use formatter::Formatter;
