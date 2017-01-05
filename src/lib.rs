
extern crate byteorder;

use std::io::{Result, Seek, Cursor};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

pub trait Formatter<T>: Seek + ReadBytesExt + WriteBytesExt {

    fn len() -> Option<i32> { None }
    fn serialize(&mut self, offset: u64, value: T) -> Result<i32>;
    fn deserialize(&mut self, offset: &mut u64) -> Result<T>;
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<u8> for R {
    fn len() -> Option<i32> { Some(1) }

    fn serialize(&mut self, offset: u64, value: u8) -> Result<i32> {
        self.write_u8(value)
            .map(|_| 1)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<u8> {
        self.read_u8()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<u16> for R {
    fn len() -> Option<i32> { Some(2) }

    fn serialize(&mut self, offset: u64, value: u16) -> Result<i32> {
        self.write_u16::<LittleEndian>(value)
            .map(|_| 2)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<u16> {
        self.read_u16::<LittleEndian>()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<u32> for R {
    fn len() -> Option<i32> { Some(4) }

    fn serialize(&mut self, offset: u64, value: u32) -> Result<i32> {
        self.write_u32::<LittleEndian>(value)
            .map(|_| 4)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<u32> {
        self.read_u32::<LittleEndian>()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<u64> for R {
    fn len() -> Option<i32> { Some(8) }

    fn serialize(&mut self, offset: u64, value: u64) -> Result<i32> {
        self.write_u64::<LittleEndian>(value)
            .map(|_| 8)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<u64> {
        self.read_u64::<LittleEndian>()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<i8> for R {
    fn len() -> Option<i32> { Some(1) }

    fn serialize(&mut self, offset: u64, value: i8) -> Result<i32> {
        self.write_i8(value)
            .map(|_| 1)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<i8> {
        self.read_i8()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<i16> for R {
    fn len() -> Option<i32> { Some(8) }

    fn serialize(&mut self, offset: u64, value: i16) -> Result<i32> {
        self.write_i16::<LittleEndian>(value)
            .map(|_| 2)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<i16> {
        self.read_i16::<LittleEndian>()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<i32> for R {
    fn len() -> Option<i32> { Some(8) }

    fn serialize(&mut self, offset: u64, value: i32) -> Result<i32> {
        self.write_i32::<LittleEndian>(value)
            .map(|_| 4)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<i32> {
        self.read_i32::<LittleEndian>()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<i64> for R {
    fn len() -> Option<i32> { Some(8) }

    fn serialize(&mut self, offset: u64, value: i64) -> Result<i32> {
        self.write_i64::<LittleEndian>(value)
            .map(|_| 8)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<i64> {
        self.read_i64::<LittleEndian>()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<f32> for R {
    fn len() -> Option<i32> { Some(4) }

    fn serialize(&mut self, offset: u64, value: f32) -> Result<i32> {
        self.write_f32::<LittleEndian>(value)
            .map(|_| 4)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<f32> {
        self.read_f32::<LittleEndian>()
    }
}

impl<R: Seek + ReadBytesExt + WriteBytesExt + ?Sized> Formatter<f64> for R {
    fn len() -> Option<i32> { Some(8) }

    fn serialize(&mut self, offset: u64, value: f64) -> Result<i32> {
        self.write_f64::<LittleEndian>(value)
            .map(|_| 8)
    }

    fn deserialize(&mut self, offset: &mut u64) -> Result<f64> {
        self.read_f64::<LittleEndian>()
    }
}

#[cfg(test)]
mod tests {

    use std::io::{Cursor};
    use Formatter;

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
}
