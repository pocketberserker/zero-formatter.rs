#[macro_export]
macro_rules! declare_buffer {
    ($name:ident) => (

        pub struct $name<T: Seek + ReadBytesExt + WriteBytesExt> {
            pub inner: T
        }

        impl<T> $name<T>
            where T: Seek + ReadBytesExt + WriteBytesExt {

            pub fn new(inner: T) -> Buffer<T> { Buffer { inner: inner } }
        }

        impl<T> Seek for $name<T>
            where T: Seek + ReadBytesExt + WriteBytesExt {

            fn seek(&mut self, style: SeekFrom) -> Result<u64> {
                self.inner.seek(style)
            }
        }

        impl<T> Read for $name<T>
            where T: Seek + ReadBytesExt + WriteBytesExt {

            fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
                self.inner.read(buf)
            }
        }

        impl<T> Write for $name<T>
            where T: Seek + ReadBytesExt + WriteBytesExt {

            fn write(&mut self, buf: &[u8]) -> Result<usize> {
                self.inner.write(buf)
            }

            fn flush(&mut self) -> Result<()> {
                self.inner.flush()
            }
        }
    )
}
