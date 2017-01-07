use error::*;
use formatter::*;

use std::io::Seek;
use byteorder::{ReadBytesExt, WriteBytesExt};
use chrono::{UTC, DateTime, TimeZone};
use std::time::Duration;

impl<R> Formatter<DateTime<UTC>> for R where R: Seek + ReadBytesExt + WriteBytesExt {

    fn serialize(&mut self, offset: u64, value: DateTime<UTC>) -> ZeroFormatterResult<i32> {
        let seconds = try!(self.serialize(offset, value.timestamp()));
        let nanos = try!(self.serialize(offset + 8, value.timestamp_subsec_nanos() as i32));
        Ok(seconds + nanos)
    }

    fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<DateTime<UTC>> {
        let seconds: i64 = try!(self.deserialize(offset));
        let nanos: i32 = try!(self.deserialize(offset));
        Ok(UTC.timestamp(seconds, nanos as u32))
    }
}

impl<R> Formatter<Duration> for R where R: Seek + ReadBytesExt + WriteBytesExt {

    fn serialize(&mut self, offset: u64, value: Duration) -> ZeroFormatterResult<i32> {
        let seconds = try!(self.serialize(offset, value.as_secs() as i64));
        let nanos = try!(self.serialize(offset + 8, value.subsec_nanos() as i32));
        Ok(seconds + nanos)
    }

    fn deserialize(&mut self, offset: &mut u64) -> ZeroFormatterResult<Duration> {
        let seconds: i64 = try!(self.deserialize(offset));
        let nanos: i32 = try!(self.deserialize(offset));
        Ok(Duration::new(seconds as u64, nanos as u32))
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;
    use chrono::UTC;
    use formatter::*;
    use std::time::Duration;

    #[test]
    fn serialize_deserialize_datetime_utc() {
        let dt = UTC::now();
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, dt).unwrap(), 12);
        let mut offset = 0;
        assert_eq!(dt, wtr.deserialize(&mut offset).unwrap());
    }

    #[test]
    fn serialize_duration() {
        let mut wtr = Cursor::new(Vec::new());
        assert_eq!(wtr.serialize(0, Duration::new(1, 2)).unwrap(), 12);
        assert_eq!(wtr.into_inner(), vec![1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0]);
    }

    #[test]
    fn deserialize_duration() {
        let mut rdr = Cursor::new(vec![1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0]);
        let mut offset = 0;
        assert_eq!(Duration::new(1, 2), rdr.deserialize(&mut offset).unwrap());
    }
}
