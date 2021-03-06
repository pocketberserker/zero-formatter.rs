use error::*;
use formatter::*;

use std::io::Seek;
use byteorder::{ReadBytesExt, WriteBytesExt};

/// `struct_formatter` define sturct type and provide sequential fields formatter.
/// But, `struct_formatter` does not support [versioning](https://github.com/neuecc/ZeroFormatter/tree/1.6.0#versioning).
///
/// ```
/// # #[macro_use] extern crate zero_formatter;
/// # extern crate byteorder;
/// # use zero_formatter::*;
/// # use std::io::{Seek, SeekFrom, Read, Write, Cursor, Result};
/// # use byteorder::{ReadBytesExt, WriteBytesExt};
/// #
/// # declare_buffer! { Buffer }
/// #
/// struct_formatter! {
///     #[target(Buffer<Cursor<Vec<u8>>>)]
///     StructSample {
///         a: i32,
///         b: i64
///     }
/// }
///
/// # fn example() -> Result<()> {
/// # let mut writer = Buffer::new(Cursor::new(Vec::new()));
/// try!(writer.serialize(0, StructSample { a: 1, b: 2 }));
/// # Ok(())
/// # }
/// #
/// # fn main() {
/// # example();
/// # }
/// ```
#[macro_export]
macro_rules! struct_formatter {
    (#[target($buffer:ty)]
    $name:ident {
        $($field_name:ident: $field_type:ty),*
    }) => {
        #[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
        pub struct $name {
            $(pub $field_name: $field_type),*
        }

        impl Formatter<$name> for $buffer {

            fn serialize(&mut self, offset: u64, value: $name) -> ZeroFormatterResult<i32> {
                let mut byte_size: i32 = 0;

                $(
                let $field_name = try!(self.serialize(offset + (byte_size as u64), value.$field_name));
                byte_size += $field_name;
                )*

                Ok(byte_size)
            }

            fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<$name> {

                $(
                let $field_name: $field_type = try!(self.deserialize(offset));
                )*

                Ok($name { $($field_name: $field_name),* })
            }
        }

        has_value_formatter! {
            #[target($buffer)]
            $name
        }
    }
}

impl<R, A1, A2> Formatter<(A1, A2)> for R
  where R: Seek + ReadBytesExt + WriteBytesExt + Formatter<A1> + Formatter<A2> {

    fn serialize(&mut self, offset: u64, value: (A1, A2)) -> ZeroFormatterResult<i32> {
        let r1 = try!(self.serialize(offset, value.0));
        let r2 = try!(self.serialize(offset + (r1 as u64), value.1));
        Ok(r1 + r2)
    }

    fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<(A1, A2)> {
        let a1: A1 = try!(self.deserialize(offset));
        let a2: A2 = try!(self.deserialize(offset));
        Ok((a1, a2))
    }
}

impl<R, A1, A2> Formatter<Option<(A1, A2)>> for R
    where R: Seek + ReadBytesExt + WriteBytesExt + Formatter<A1> + Formatter<A2> {

    fn serialize(&mut self, offset: u64, value: Option<(A1, A2)>) -> ZeroFormatterResult<i32> {
        match value {
            None => {
                self.serialize(offset, false)
            },
            Some((v1, v2)) => {
                let r0 = try!(self.serialize(offset, true));
                let r1 = try!(self.serialize(offset + (r0 as u64), v1));
                let r2 = try!(self.serialize(offset + ((r0 + r1) as u64), v2));
                Ok(r0 + r1 + r2)
            }
        }
    }

    fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<Option<(A1, A2)>> {

        let has_value: bool = try!(self.deserialize(offset));
        if has_value {
            let a1: A1 = try!(self.deserialize(offset));
            let a2: A2 = try!(self.deserialize(offset));
            Ok(Some((a1, a2)))
        }
        else {
            Ok(None)
        }
    }
}

/// `object_formatter` define struct type and provide formatter.
/// `object_formatter` support [versioning](https://github.com/neuecc/ZeroFormatter/tree/1.6.0#versioning).
///
/// ```
/// # #[macro_use] extern crate zero_formatter;
/// # extern crate byteorder;
/// # use zero_formatter::*;
/// # use std::io::{Seek, SeekFrom, Read, Write, Cursor, Result};
/// # use byteorder::{ReadBytesExt, WriteBytesExt};
/// #
/// # declare_buffer! { Buffer }
/// #
/// object_formatter! {
///     #[target(Buffer<Cursor<Vec<u8>>>)]
///     ObjectSample {
///         0; a: i32,
///         1; b: i64
///     }
/// }
///
/// # fn example() -> Result<()> {
/// # let mut writer = Buffer::new(Cursor::new(Vec::new()));
/// try!(writer.serialize(0, ObjectSample { a: 1, b: 2 }));
/// # Ok(())
/// # }
/// #
/// # fn main() {
/// # example();
/// # }
/// ```
#[macro_export]
macro_rules! object_formatter {
    (#[target($buffer:ty)]
    $name:ident {
        $($index:expr; $field_name:ident: $field_type:ty),*
    }) => {
        #[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
        pub struct $name {
            $(pub $field_name: $field_type),*
        }

        impl Formatter<$name> for $buffer {

            fn serialize(&mut self, offset: u64, value: $name) -> ZeroFormatterResult<i32> {
                let last_index: i32 = *([$($index),*].iter().max().unwrap());
                let mut byte_size: i32 = 4 + 4 + 4 * (last_index + 1);

                try!(self.serialize(offset + 4, last_index));

                $(
                try!(self.serialize(offset + 4 + 4 + 4 * $index, (offset as i32) + byte_size));
                let $field_name = try!(self.serialize(offset + (byte_size as u64), value.$field_name));
                byte_size += $field_name;
                )*

                try!(self.serialize(offset, byte_size));
                try!(self.seek(SeekFrom::Start(offset + (byte_size as u64))));
                Ok(byte_size)
            }

            fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<$name> {

                let start_offset: u64 = *offset;
                let byte_size = try!(util::check_non_null(self, offset));
                let last_index: i32 = try!(self.deserialize(offset));

                $(
                let $field_name: $field_type = try!(if $index <= last_index {
                    *offset = start_offset + 4 + 4 + 4 * $index;
                    let o: i32 = try!(self.deserialize(offset));
                    if o == 0 {
                        Ok(Default::default())
                    } else {
                        *offset = o as u64;
                        self.deserialize(offset)
                    }
                } else {
                    Ok(Default::default())
                });
                )*

                *offset = start_offset + (byte_size as u64);
                Ok($name { $($field_name: $field_name),* })
            }
        }

        option_formatter! {
            #[target($buffer)]
            $name
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;
    use std::io::{Seek, SeekFrom};
    use error::*;
    use formatter::*;
    use util;

    object_formatter! {
        #[target(Cursor<Vec<u8>>)]
        O {
            0; a: i32,
            1; b: i64
        }
    }

    #[test]
    fn serialize_object() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, O { a: 1, b: 2 }).unwrap(), 28);
        assert_eq!(wtr.into_inner(), vec![28, 0, 0, 0, 1, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize_object() {
        let mut rdr = Cursor::new(vec![28, 0, 0, 0, 1, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(O { a: 1, b: 2 }, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_object_some() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Some(O { a: 1, b: 2 })).unwrap(), 28);
        assert_eq!(wtr.into_inner(), vec![28, 0, 0, 0, 1, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize_object_some() {
        let mut rdr = Cursor::new(vec![28, 0, 0, 0, 1, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(Some(O { a: 1, b: 2 }), rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_object_none() {
        let mut wtr = Cursor::new(Vec::new());
        let input: Option<O> = None;
        assert_eq!(wtr.serialize(0, input).unwrap(), 4);
        assert_eq!(wtr.into_inner(), vec![0xff, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn deserialize_object_none() {
        let mut rdr = Cursor::new(vec![0xff, 0xff, 0xff, 0xff]);
        let mut offset = 0;
        let expected: Option<O> = None;
        assert_eq!(expected, rdr.deserialize(&mut offset).unwrap());
    }

    object_formatter! {
        #[target(Cursor<Vec<u8>>)]
        O2 {
            0; a: i32,
            1; b: i64,
            2; c: i8
        }
    }

    #[test]
    fn deserialize_object_versioning() {
        let mut rdr = Cursor::new(vec![28, 0, 0, 0, 1, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(O2 { a: 1, b: 2, c: 0 }, rdr.deserialize(&mut offset).unwrap());
    }

    struct_formatter! {
        #[target(Cursor<Vec<u8>>)]
        S {
            a: i32,
            b: i64
        }
    }

    #[test]
    fn serialize_struct() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, S { a: 1, b: 2 }).unwrap(), 12);
        assert_eq!(wtr.into_inner(), vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize_struct() {
        let mut rdr = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(S { a: 1, b: 2 }, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_struct_some() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Some(S { a: 1, b: 2 })).unwrap(), 13);
        assert_eq!(wtr.into_inner(), vec![1, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize_struct_some() {
        let mut rdr = Cursor::new(vec![1, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(Some(S { a: 1, b: 2 }), rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_struct_none() {
        let mut wtr = Cursor::new(Vec::new());
        let input: Option<S> = None;
        assert_eq!(wtr.serialize(0, input).unwrap(), 1);
        assert_eq!(wtr.into_inner(), vec![0]);
    }

    #[test]
    fn deserialize_struct_none() {
        let mut rdr = Cursor::new(vec![0]);
        let mut offset = 0;
        let expected: Option<S> = None;
        assert_eq!(expected, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_2_tuple() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, (1u8, 2u8)).unwrap(), 2);
        assert_eq!(wtr.into_inner(), vec![1, 2]);
    }

    #[test]
    fn deserialize_2_tuple() {
        let mut rdr = Cursor::new(vec![1, 2]);
        let mut offset = 0;
        assert_eq!((1u8, 2u8), rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_some_2_tuple() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Some((1u8, 2u8))).unwrap(), 3);
        assert_eq!(wtr.into_inner(), vec![1, 1, 2]);
    }

    #[test]
    fn deserialize_some_2_tuple() {
        let mut rdr = Cursor::new(vec![1, 1, 2]);
        let mut offset = 0;
        assert_eq!(Some((1u8, 2u8)), rdr.deserialize(&mut offset).unwrap());
    }
}
