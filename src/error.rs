use std::io;
use std::string::FromUtf8Error;
use std::fmt;
use std::error::Error;

pub type ZeroFormatterResult<T> = Result<T, ZeroFormatterError>;

#[derive(Debug)]
pub enum ZeroFormatterError {
    IoError(io::Error),
    Utf8Error(FromUtf8Error),
    UnionKeyNotFound
}

impl ZeroFormatterError {
    pub fn union_key_not_found<T>() -> ZeroFormatterResult<T> {
        Err(ZeroFormatterError::UnionKeyNotFound)
    }
}

impl fmt::Display for ZeroFormatterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for ZeroFormatterError {
    fn description(&self) -> &str {
        match self {
            &ZeroFormatterError::IoError(ref e) => e.description(),
            &ZeroFormatterError::Utf8Error(ref e) => e.description(),
            &ZeroFormatterError::UnionKeyNotFound => "Union key does not found."
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ZeroFormatterError::IoError(ref e) => Some(e),
            &ZeroFormatterError::Utf8Error(ref e) => Some(e),
            &ZeroFormatterError::UnionKeyNotFound => None
        }
    }
}

impl From<io::Error> for ZeroFormatterError {
    fn from(err: io::Error) -> Self {
        ZeroFormatterError::IoError(err)
    }
}

impl From<FromUtf8Error> for ZeroFormatterError {
    fn from(err: FromUtf8Error) -> Self {
        ZeroFormatterError::Utf8Error(err)
    }
}

impl From<ZeroFormatterError> for io::Error {
    fn from(err: ZeroFormatterError) -> Self {
        match err {
            ZeroFormatterError::IoError(e) => e,
            e @ ZeroFormatterError::Utf8Error(_) => io::Error::new(io::ErrorKind::InvalidData, e),
            e @ ZeroFormatterError::UnionKeyNotFound => io::Error::new(io::ErrorKind::InvalidData, e)
        }
    }
}
