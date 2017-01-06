use std;
use std::error::Error;
use std::io::Seek;
use byteorder::{ReadBytesExt, WriteBytesExt};

pub type Result<T> = std::result::Result<T, Box<Error>>;

pub trait Formatter<T>: Seek + ReadBytesExt + WriteBytesExt {
    fn serialize(&mut self, offset: u64, value: T) -> Result<i32>;
    fn deserialize(&mut self, offset: &mut u64) -> Result<T>;
}
