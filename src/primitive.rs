use formatter::*;

use std::{i32, usize};
use std::borrow::Cow;
use std::ops::Deref;
use std::string::String;
//use std::convert::TryFrom;
use std::io::{Seek, SeekFrom};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

impl<R: Seek + ReadBytesExt + WriteBytesExt> Formatter<u8> for R {

    fn serialize(&mut self, offset: u64, value: u8) -> Result<i32> {
        try!(self.seek(SeekFrom::Start(offset)));
        try!(self.write_u8(value));
        Ok(1)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<u8> {
        try!(self.seek(SeekFrom::Start(*(offset as &u64))));
        let n = try!(self.read_u8());
        *offset += 1;
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

#[cfg(test)]
mod tests {

    use std::io::Cursor;
    use std::borrow::Cow;
    use formatter::*;

    #[test]
    fn serialize_true() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, true).unwrap(), 1);
        assert_eq!(wtr.into_inner(), vec![1]);
    }

    #[test]
    fn deserialize_true() {
        let mut rdr = Cursor::new(vec![1]);
        let mut offset = 0;
        assert_eq!(true, rdr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_false() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, false).unwrap(), 1);
        assert_eq!(wtr.into_inner(), vec![0]);
    }

    #[test]
    fn deserialize_false() {
        let mut rdr = Cursor::new(vec![0]);
        let mut offset = 0;
        assert_eq!(false, rdr.deserialize(&mut offset).unwrap());
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
}
