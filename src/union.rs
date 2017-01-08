#[macro_export]
macro_rules! union_formatter {
    (#[target($buffer:ty)]
    enum $name:ident : $key_type:ty {
        $($key_value:expr; $case_name:ident($field_type:ty)),*
    }) => {
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum $name {
            $($case_name($field_type)),*
        }

        impl Formatter<$name> for $buffer {

            fn serialize(&mut self, offset: u64, value: $name) -> ZeroFormatterResult<i32> {
                let mut byte_size: i32 = 4;

                match value {
                    $(
                    $name::$case_name(v) => {
                        byte_size += try!(self.serialize(offset + (byte_size as u64), $key_value));
                        byte_size += try!(self.serialize(offset + (byte_size as u64), v))
                    }
                    ),*
                }

                try!(self.serialize(offset, byte_size));

                Ok(byte_size)
            }

            fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<$name> {

                try!(util::check_non_null(self, offset));

                let key: $key_type = try!(self.deserialize(offset));
                match key {
                    $(
                    $key_value => {
                        let v: $field_type = try!(self.deserialize(offset));
                        Ok( $name::$case_name (v) )
                    }
                    ),*,
                    _ => ZeroFormatterError::invalid_binary(*offset)
                }
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
            0; a: i32
        }
    }

    struct_formatter! {
        #[target(Cursor<Vec<u8>>)]
        S {
            b: i64
        }
    }

    union_formatter! {
        #[target(Cursor<Vec<u8>>)]
        enum U: i32 {
            1; A(O),
            2; B(S)
        }
    }

    #[test]
    fn serialize_deserialize_union_a() {
        let mut c = Cursor::new(Vec::new());
        let input: U = U::A(O{ a: 1 });
        assert_eq!(c.serialize(0, input).unwrap(), 24);
        let mut offset = 0;
        assert_eq!(input, c.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_deserialize_union_b() {
        let mut c = Cursor::new(Vec::new());
        let input: U = U::B(S{ b: 2 });
        assert_eq!(c.serialize(0, input).unwrap(), 16);
        let mut offset = 0;
        assert_eq!(input, c.deserialize(&mut offset).unwrap());
    }
}
