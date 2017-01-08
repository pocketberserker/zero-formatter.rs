use std::io;
use std::string::FromUtf8Error;
use std::fmt;
use std::error::Error;

pub type ZeroFormatterResult<T> = Result<T, ZeroFormatterError>;

#[derive(Debug)]
pub enum ZeroFormatterError {
    IoError(io::Error),
    FromUtf8Error(FromUtf8Error),
    InvalidBinary(u64)
}

impl ZeroFormatterError {
    pub fn invalid_binary<T>(offset: u64) -> ZeroFormatterResult<T> {
        Err(ZeroFormatterError::InvalidBinary(offset))
    }
}

impl fmt::Display for ZeroFormatterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZeroFormatterError::IoError(_) | ZeroFormatterError::FromUtf8Error(_) => fmt::Debug::fmt(self, f),
            ZeroFormatterError::InvalidBinary(ref offset) =>
                write!(f, "[offset {}] Binary does not valid.", *offset)
        }
    }
}

impl Error for ZeroFormatterError {
    fn description(&self) -> &str {
        match self {
            &ZeroFormatterError::IoError(ref e) => e.description(),
            &ZeroFormatterError::FromUtf8Error(ref e) => e.description(),
            &ZeroFormatterError::InvalidBinary(_) => "Binary does not valid."
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ZeroFormatterError::IoError(ref e) => Some(e),
            &ZeroFormatterError::FromUtf8Error(ref e) => Some(e),
            &ZeroFormatterError::InvalidBinary(_) => None
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
        ZeroFormatterError::FromUtf8Error(err)
    }
}

impl From<ZeroFormatterError> for io::Error {
    fn from(err: ZeroFormatterError) -> Self {
        match err {
            ZeroFormatterError::IoError(e) => e,
            e @ ZeroFormatterError::FromUtf8Error(_) => io::Error::new(io::ErrorKind::InvalidData, e),
            e @ ZeroFormatterError::InvalidBinary(_) => io::Error::new(io::ErrorKind::InvalidData, e)
        }
    }
}
