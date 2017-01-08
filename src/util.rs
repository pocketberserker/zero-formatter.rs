use error::*;
use formatter::*;

use std::io::Seek;
use byteorder::{ReadBytesExt, WriteBytesExt};

pub fn check_non_null<R>(r: &mut R, offset: &mut u64) -> ZeroFormatterResult<i32>
    where R: Seek + ReadBytesExt + WriteBytesExt + Formatter<i32> {
    r.deserialize(offset)
        .and_then(|bs| if bs >= 0 {
            Ok(bs)
        } else {
            ZeroFormatterError::invalid_binary(*offset)
        })
}
