//! Implementation of [ZeroFormatter](https://github.com/neuecc/ZeroFormatter) in Rust.
//!
//! ## Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! zero-formatter = "0.1"
//! ```
//!
//! ## Examples
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
//!
//! ## Supported Type
//!
//! Currently, this library support only [Stage1](https://github.com/neuecc/ZeroFormatter/tree/1.6.0#cross-platform).
//! See also [WireFormat Specification](https://github.com/neuecc/ZeroFormatter/tree/1.6.0#wireformat-specification).
//!
//! ### Primitive Format
//!
//! | Rust | C# | Note |
//! | ---- | ---- | --- |
//! | `i16` | `Int16` | |
//! | `i32` | `Int32`| |
//! | `i64` | `Int64` | |
//! | `u16` | `UInt16` | |
//! | `u32` | `UInt32` | |
//! | `u64` | `UInt64` | |
//! | `f32` | `Single` | |
//! | `f64` | `Double` | |
//! | `bool` | `Boolean` | |
//! | `u8` | `Byte` | |
//! | `i8` | `SByte` | |
//! | `time::Duration` | `TimeSpan` | |
//! | `chrono::DateTime<chrono::UTC>` | `DateTime` | |
//! | | `DateTimeOffset` | |
//! | `Cow<'a, str>` | `String` | |
//! | `Option<i16>` | `Int16?` | |
//! | `Option<i32>` | `Int32?`| |
//! | `Option<i64>` | `Int64?` | |
//! | `Option<u16>` | `UInt16?` | |
//! | `Option<u32>` | `UInt32?` | |
//! | `Option<u64>` | `UInt64?` | |
//! | `Option<f32>` | `Single?` | |
//! | `Option<f64>` | `Double?` | |
//! | `Option<bool>` | `Boolean?` | |
//! | `Option<u8>` | `Byte?` | |
//! | `Option<i8>` | `SByte?` | |
//! | `Option<time::Duration>` | `TimeSpan?` | |
//! | `Option<chrono::DateTime<chrono::UTC>>` | `DateTime?` | |
//! | | `DateTimeOffset?` | |
//!
//! ### Sequence Format
//!
//! | Rust | C# | Note |
//! | ---- | ---- | --- |
//! | `Cow<'a, [T]>` | `Sequence<T>` | |
//!
//! ### List Format
//!
//! | Rust | C# | Note |
//! | ---- | ---- | --- |
//! | | FixedSizeList | |
//! | | VariableSizeList | |
//!
//! ### Object Format
//!
//! | Rust | C# | Note |
//! | ---- | ---- | --- |
//! | struct | Object | use `object_formatter` macro |
//! | `Option<struct>` | Object | if byteSize = -1, indicates `None` |
//! | struct | Struct | |
//! | `Option<struct>` | Struct? | |
//! | `Option<(A1, A2)>` | Tuple<A1, A2> | |
//!
//! ### Union Format
//!
//! | Rust | C# | Note |
//! | ---- | ---- | --- |
//! | enum | Union | use `union_formatter` macro |
//! | Option<enum> | | if byte_size = 1, indicates `None` |


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
