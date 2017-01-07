use error::*;
use formatter::*;

use std::io::Seek;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::borrow::Cow;

fn try_deserialize<R, A>(s: &mut R, n: i32, v: &mut Vec<A>, offset: &mut u64) -> ZeroFormatterResult<()>
    where R: Seek + ReadBytesExt + WriteBytesExt + Formatter<A> {
    if n < 0 {
      Ok(())
    } else {
      let a = try!(s.deserialize(offset));
      v.push(a);
      try_deserialize(s, n - 1, v, offset)
    }
}

impl<'a, R, A: Clone> Formatter<Cow<'a, [A]>> for R
    where R: Seek + ReadBytesExt + WriteBytesExt + Formatter<A> + Formatter<i32> {

    fn serialize(&mut self, offset: u64, value: Cow<'a, [A]>) -> ZeroFormatterResult<i32> {
        let v: Vec<A> = value.into_owned();
        let lr = try!(self.serialize(offset, v.len() as i32));
        let byte_size = try!(v.iter().fold(
            Ok(lr),
            |b, a| {
                b.and_then(|bs| {
                    self.serialize(offset + (bs as u64), a.clone())
                        .map(|s| bs + s)
                })
            }
        ));
        Ok(byte_size as i32)
    }

    fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<Cow<'a, [A]>> {
        let l: i32 = try!(self.deserialize(offset));
        let mut v: Vec<A> = Vec::with_capacity(l as usize);
        try!(try_deserialize(self, l - 1, &mut v, offset));
        Ok(Cow::from(v))
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;
    use std::borrow::Cow;
    use std::io::{Seek, SeekFrom};
    use error::*;
    use formatter::*;
    use byteorder::{ReadBytesExt, WriteBytesExt};

    #[test]
    fn serialize_vec() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Cow::from(vec![1i32, 2i32, 3i32])).unwrap(), 16);
        assert_eq!(wtr.into_inner(), vec![3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
    }

    #[test]
    fn deserialize_vec() {
        let mut rdr = Cursor::new(vec![3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
        let mut offset = 0;
        let actual: Cow<'static, [i32]> = rdr.deserialize(&mut offset).unwrap();
        assert_eq!(offset, 16);
        assert_eq!(Cow::from(vec![1, 2, 3]), actual);
    }

    struct_formatter! {
        struct S {
            a: i32
        }
    }

    #[test]
    fn serialize_vec_struct() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Cow::from(vec![S{a: 1}, S{a: 2}, S{a: 3}])).unwrap(), 16);
        assert_eq!(wtr.into_inner(), vec![3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
    }

    #[test]
    fn deserialize_vec_struct() {
        let mut rdr = Cursor::new(vec![3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
        let mut offset = 0;
        let actual: Cow<'static, [S]> = rdr.deserialize(&mut offset).unwrap();
        assert_eq!(offset, 16);
        assert_eq!(Cow::from(vec![S{a: 1}, S{a: 2}, S{a: 3}]), actual);
    }

    object_formatter! {
        struct O {
            0; a: i32
        }
    }

    #[test]
    fn serialize_vec_object() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Cow::from(vec![O{a: 1}, O{a: 2}, O{a: 3}])).unwrap(), 52);
        let expected = vec![
            3, 0, 0, 0,

            16, 0, 0, 0,
            0, 0, 0, 0,
            16, 0, 0, 0,
            1, 0, 0, 0,

            16, 0, 0, 0,
            0, 0, 0, 0,
            32, 0, 0, 0,
            2, 0, 0, 0,

            16, 0, 0, 0,
            0, 0, 0, 0,
            48, 0, 0, 0,
            3, 0, 0, 0,
        ];
        assert_eq!(wtr.into_inner(), expected);
    }

    #[test]
    fn deserialize_vec_object() {
        let mut rdr = Cursor::new(vec![
            3, 0, 0, 0,

            16, 0, 0, 0,
            0, 0, 0, 0,
            16, 0, 0, 0,
            1, 0, 0, 0,

            16, 0, 0, 0,
            0, 0, 0, 0,
            32, 0, 0, 0,
            2, 0, 0, 0,

            16, 0, 0, 0,
            0, 0, 0, 0,
            48, 0, 0, 0,
            3, 0, 0, 0,
        ]);
        let mut offset = 0;
        let actual: Cow<'static, [O]> = rdr.deserialize(&mut offset).unwrap();
        assert_eq!(offset, 52);
        assert_eq!(Cow::from(vec![O{a: 1}, O{a: 2}, O{a: 3}]), actual);
    }
}
