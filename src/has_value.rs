use error::*;
use formatter::*;

use std::io::Seek;
use byteorder::{ReadBytesExt, WriteBytesExt};
use chrono::{UTC, DateTime};
use std::time::Duration;

#[macro_export]
macro_rules! has_value_formatter_methods {
    ($t:ty) => (
        fn serialize(&mut self, offset: u64, value: Option<$t>) -> ZeroFormatterResult<i32> {
            match value {
                None => {
                    self.serialize(offset, false)
                },
                Some(v) => {
                    let r1 = try!(self.serialize(offset, true));
                    let r2 = try!(self.serialize(offset + 1, v));
                    Ok(r1 + r2)
                }
            }
        }

        fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<Option<$t>> {
            let has_value: bool = try!(self.deserialize(offset));
            if has_value {
                self.deserialize(offset).map(|v| Some(v))
            }
            else {
                Ok(None)
            }
        }
    )
}

macro_rules! primitive_has_value_formatter {
    ($($t:ty),*) => ($(
        impl<R> Formatter<Option<$t>> for R where R: Seek + ReadBytesExt + WriteBytesExt {
            has_value_formatter_methods! { $t }
        }
    )*)
}

primitive_has_value_formatter! {
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    f32,
    f64,
    bool,
    DateTime<UTC>,
    Duration
}

#[macro_export]
macro_rules! has_value_formatter {
    (#[target($buffer:ty)]
    $t:ty
    ) => (
        impl Formatter<Option<$t>> for $buffer {
            has_value_formatter_methods! { $t }
        }
    )
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;
    use formatter::*;

    #[test]
    fn serialize_u8_some() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Some(1u8)).unwrap(), 2);
        assert_eq!(wtr.into_inner(), vec![1, 1]);
    }

    #[test]
    fn deserialize_u8_some() {
        let mut rdr = Cursor::new(vec![1, 1]);
        let mut offset = 0;
        assert_eq!(Some(1u8), rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_u8_none() {
        let mut wtr = Cursor::new(Vec::new());
        let input: Option<u8> = None;
        assert_eq!(wtr.serialize(0, input).unwrap(), 1);
        assert_eq!(wtr.into_inner(), vec![0]);
    }

    #[test]
    fn deserialize_u8_none() {
        let mut rdr = Cursor::new(vec![0]);
        let mut offset = 0;
        let expected: Option<u8> = None;
        assert_eq!(expected, rdr.deserialize(&mut offset).unwrap());
    }
}
