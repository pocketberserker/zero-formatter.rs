use error::ZeroFormatterResult;

use std::io::Seek;
use byteorder::{ReadBytesExt, WriteBytesExt};

pub trait Formatter<T>: Seek + ReadBytesExt + WriteBytesExt {
    fn serialize(&mut self, offset: u64, value: T) -> ZeroFormatterResult<i32>;
    fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<T>;
}
