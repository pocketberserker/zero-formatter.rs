extern crate byteorder;

use std::{i32, usize};
use std::borrow::Cow;
use std::ops::Deref;
use std::string::String;
//use std::convert::TryFrom;
use std::io::{Seek, SeekFrom};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

pub trait Formatter<T>: Seek + ReadBytesExt + WriteBytesExt {
    fn serialize(&mut self, offset: u64, value: T) -> Result<i32>;
    fn deserialize(&mut self, offset: &mut u64) -> Result<T>;
}

impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<u8> for R {

    fn serialize(&mut self, offset: u64, value: u8) -> Result<i32> {
        try!(self.seek(SeekFrom::Start(offset)));
        try!(self.write_u8(value));
        Ok(1)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<u8> {
        try!(self.seek(SeekFrom::Start(*(offset as &u64))));
        let n = try!(self.read_u8());
        Ok(n)
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<bool> for R {

    fn serialize(&mut self, offset: u64, value: bool) -> Result<i32> {
        let i: u8 = if value { 1 } else { 0 };
        self.serialize(offset, i)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<bool> {
        let n: u8 = try!(self.deserialize(offset));
        Ok(n == 1)
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<i8> for R {

    fn serialize(&mut self, offset: u64, value: i8) -> Result<i32> {
        try!(self.seek(SeekFrom::Start(offset)));
        try!(self.write_i8(value));
        Ok(1)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<i8> {
        try!(self.seek(SeekFrom::Start(*(offset as &u64))));
        let n = try!(self.read_i8());
        *offset += 1;
        Ok(n)
    }
}

macro_rules! primitive_formatter_impl {
    ($($t:ty; $w:tt; $r:tt; $l:expr),*) => ($(
        impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<$t> for R {

            fn serialize(&mut self, offset: u64, value: $t) -> Result<i32> {
                try!(self.seek(SeekFrom::Start(offset)));
                try!(self.$w::<LittleEndian>(value));
                Ok($l)
            }

            fn deserialize(&mut self, offset: &mut u64) -> Result<$t> {
                try!(self.seek(SeekFrom::Start(*(offset as &u64))));
                let n = try!(self.$r::<LittleEndian>());
                *offset += $l;
                Ok(n)
            }
        }
    )*)
}

primitive_formatter_impl! {
    u16; write_u16; read_u16; 2,
    u32; write_u32; read_u32; 4,
    u64; write_u64; read_u64; 8,
    i16; write_i16; read_i16; 2,
    i32; write_i32; read_i32; 4,
    i64; write_i64; read_i64; 8,
    f32; write_f32; read_f32; 4,
    f64; write_f64; read_f64; 8
}

impl<'a, R: Seek + ReadBytesExt + WriteBytesExt> Formatter<Cow<'a, str>> for R {

    fn serialize(&mut self, offset: u64, value: Cow<'a, str>) -> Result<i32> {
        let bytes = value.deref().as_bytes();
        let l = bytes.len();
        //let i = try!(i32::try_from(l));
        let i = l as i32;
        try!(self.seek(SeekFrom::Start(offset)));
        try!(self.write_i32::<LittleEndian>(i));
        try!(self.write(bytes));
        Ok(i + 4)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<Cow<'a, str>> {
        try!(self.seek(SeekFrom::Start(*(offset as &u64))));
        let i: i32 = try!(self.deserialize(offset));
        //let l = try!(usize::try_from(i));
        let l = i as usize;
        let mut buf = Vec::with_capacity(l);
        unsafe { buf.set_len(l); }
        try!(self.read(&mut buf));
        *offset += l as u64;
        let s = try!(String::from_utf8(buf));
        Ok(s.into())
    }
}

#[macro_export]
macro_rules! has_value_formatter {
    ($($t:ident)*) => ($(
        impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<Option<$t>> for R {

            fn serialize(&mut self, offset: u64, value: Option<$t>) -> std::result::Result<i32, Box<std::error::Error>> {
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

            fn deserialize(&mut self, offset: &mut u64) -> std::result::Result<Option<$t>, Box<std::error::Error>> {
                let has_value: bool = try!(self.deserialize(offset));
                *offset += 1;
                if has_value {
                    self.deserialize(offset).map(|v| Some(v))
                }
                else {
                    Ok(None)
                }
            }
        }
    )*)
}

has_value_formatter! { u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 bool }

#[macro_export]
macro_rules! option_formatter {
    ($($name:ident)*) => ($(
        impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<Option<$name>> for R {

            fn serialize(&mut self, offset: u64, value: Option<$name>) -> std::result::Result<i32, Box<std::error::Error>> {
                try!(self.seek(SeekFrom::Start(offset)));
                match value {
                    None => {
                        self.serialize(offset, -1i32)
                    },
                    Some(v) => {
                        self.serialize(offset, v)
                    }
                }
            }

            fn deserialize(&mut self, offset: &mut u64) -> Result<Option<$name>, Box<std::error::Error>> {
                let len: i32 = try!(self.deserialize(offset));
                if len == -1 {
                    Ok(None)
                }
                else {
                    *offset -= 4;
                    self.deserialize(offset).map(|v| Some(v))
                }
            }
        }
    )*)
}

#[macro_export]
macro_rules! object_formatter {
    (struct $name:ident {
        $($index:expr; $field_name:ident: $field_type:ty),*
    }) => {
        #[derive(Default, Debug, PartialEq, Eq)]
        struct $name {
            $($field_name: $field_type),*
        }

        impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<$name> for R {

            fn serialize(&mut self, offset: u64, value: $name) -> std::result::Result<i32, Box<std::error::Error>> {
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

            fn deserialize(&mut self, offset: &mut u64) -> Result<$name, Box<std::error::Error>> {

                let start_offset: u64 = *offset;
                let byte_size: i32 = try!(self.deserialize(offset));
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

        option_formatter! { $name }
    }
}

#[macro_export]
macro_rules! struct_formatter {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty),*
    }) => {
        #[derive(Default, Debug, PartialEq, Eq)]
        struct $name {
            $($field_name: $field_type),*
        }

        impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<$name> for R {

            fn serialize(&mut self, offset: u64, value: $name) -> std::result::Result<i32, Box<std::error::Error>> {
                let mut byte_size: i32 = 0;

                $(
                let $field_name = try!(self.serialize(offset + (byte_size as u64), value.$field_name));
                byte_size += $field_name;
                )*

                Ok(byte_size)
            }

            fn deserialize(&mut self, offset: &mut u64) -> Result<$name, Box<std::error::Error>> {

                $(
                let $field_name: $field_type = try!(self.deserialize(offset));
                )*

                Ok($name { $($field_name: $field_name),* })
            }
        }

        has_value_formatter! { $name }
    }
}

#[cfg(test)]
mod tests {

    use std;
    use std::borrow::Cow;
    use std::io::Cursor;
    use std::io::{Seek, SeekFrom};
    use Formatter;
    use byteorder::{ReadBytesExt, WriteBytesExt};

    #[test]
    fn serialize_bool() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, true).unwrap(), 1);
        assert_eq!(wtr.into_inner(), vec![1]);
    }

    #[test]
    fn deserialize_bool() {
        let mut rdr = Cursor::new(vec![1]);
        let mut offset = 0;
        assert_eq!(true, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_u8() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 1u8).unwrap(), 1);
        assert_eq!(wtr.into_inner(), vec![1]);
    }

    #[test]
    fn deserialize_u8() {
        let mut rdr = Cursor::new(vec![1]);
        let mut offset = 0;
        assert_eq!(1u8, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_u16() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 1u16).unwrap(), 2);
        assert_eq!(wtr.into_inner(), vec![1, 0]);
    }

    #[test]
    fn deserialize_u16() {
        let mut rdr = Cursor::new(vec![1, 0]);
        let mut offset = 0;
        assert_eq!(1u16, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_u32() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 1u32).unwrap(), 4);
        assert_eq!(wtr.into_inner(), vec![1, 0, 0, 0]);
    }

    #[test]
    fn deserialize_u32() {
        let mut rdr = Cursor::new(vec![1, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(1u32, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_u64() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 1u64).unwrap(), 8);
        assert_eq!(wtr.into_inner(), vec![1, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize_u64() {
        let mut rdr = Cursor::new(vec![1, 0, 0, 0, 0, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(1u64, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_i8() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 1i8).unwrap(), 1);
        assert_eq!(wtr.into_inner(), vec![1]);
    }

    #[test]
    fn deserialize_i8() {
        let mut rdr = Cursor::new(vec![1]);
        let mut offset = 0;
        assert_eq!(1i8, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_i16() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 1i16).unwrap(), 2);
        assert_eq!(wtr.into_inner(), vec![1, 0]);
    }

    #[test]
    fn deserialize_i16() {
        let mut rdr = Cursor::new(vec![1, 0]);
        let mut offset = 0;
        assert_eq!(1i16, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_i32() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 1i32).unwrap(), 4);
        assert_eq!(wtr.into_inner(), vec![1, 0, 0, 0]);
    }

    #[test]
    fn deserialize_i32() {
        let mut rdr = Cursor::new(vec![1, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(1i32, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_i64() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 1i64).unwrap(), 8);
        assert_eq!(wtr.into_inner(), vec![1, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize_i64() {
        let mut rdr = Cursor::new(vec![1, 0, 0, 0, 0, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(1i64, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_f32() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 123.0f32).unwrap(), 4);
        assert_eq!(wtr.into_inner(), vec![0, 0, 0xf6, 0x42]);
    }

    #[test]
    fn deserialize_f32() {
        let mut rdr = Cursor::new(vec![0, 0, 0xf6, 0x42]);
        let mut offset = 0;
        assert_eq!(123.0f32, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_f64() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, 123.0f64).unwrap(), 8);
        assert_eq!(wtr.into_inner(), vec![0, 0, 0, 0, 0, 0xc0, 0x5e, 0x40]);
    }

    #[test]
    fn deserialize_f64() {
        let mut rdr = Cursor::new(vec![0, 0, 0, 0, 0, 0xc0, 0x5e, 0x40]);
        let mut offset = 0;
        assert_eq!(123.0f64, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_str() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Cow::Borrowed("あいうえお")).unwrap(), 19);
        assert_eq!(wtr.into_inner(), vec![0x0f, 0, 0, 0, 0xe3, 0x81, 0x82, 0xe3, 0x81, 0x84, 0xe3, 0x81, 0x86, 0xe3, 0x81, 0x88, 0xe3, 0x81, 0x8a]);
    }

    #[test]
    fn deserialize_str() {
        let mut rdr = Cursor::new(vec![0x0f, 0, 0, 0, 0xe3, 0x81, 0x82, 0xe3, 0x81, 0x84, 0xe3, 0x81, 0x86, 0xe3, 0x81, 0x88, 0xe3, 0x81, 0x8a]);
        let mut offset = 0;
        let actual: Cow<'static, str> = rdr.deserialize(&mut offset).unwrap();
        assert_eq!(offset, 19);
        assert_eq!(Cow::Borrowed("あいうえお"), actual);
    }

    #[test]
    fn serialize_option_u8() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Some(1u8)).unwrap(), 2);
        assert_eq!(wtr.into_inner(), vec![1, 1]);
    }

    #[test]
    fn deserialize_option_u8() {
        let mut rdr = Cursor::new(vec![1, 1]);
        let mut offset = 0;
        assert_eq!(Some(1u8), rdr.deserialize(&mut offset).unwrap());
    }

    object_formatter! {
        struct O {
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
        struct O2 {
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
        struct S {
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
}
