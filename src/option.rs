#[macro_export]
macro_rules! option_formatter {
    (#[target($buffer:ty)]
    $name:ident
    ) => (
        impl Formatter<Option<$name>> for $buffer {

            fn serialize(&mut self, offset: u64, value: Option<$name>) -> ZeroFormatterResult<i32> {
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

            fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<Option<$name>> {
                let len: i32 = try!(self.deserialize(offset));
                if len == -1 {
                    Ok(None)
                }
                else if len < -1 {
                    ZeroFormatterError::invalid_binary(*offset)
                }
                else {
                    *offset -= 4;
                    self.deserialize(offset).map(|v| Some(v))
                }
            }
        }
    )
}
