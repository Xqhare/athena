#[derive(Debug)]
pub enum AthenaError {
    InvalidInput,
    InvalidOutput,
    InvalidData,
    InsufficientBuffer,
    IoError(std::io::Error),
    ParseErrorInt(std::num::ParseIntError),
    ParseErrorUtf8(std::string::FromUtf8Error),
    ContinuationBitInLastByte,
    Overflow,
    ParityError,
    InvalidUuidLength,
    TableSchemaMismatch,
    UnsupportedPlatform,
}

pub type AthenaResult<T> = Result<T, AthenaError>;

impl std::fmt::Display for AthenaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AthenaError {}

impl From<std::io::Error> for AthenaError {
    fn from(e: std::io::Error) -> Self {
        AthenaError::IoError(e)
    }
}

impl From<std::num::ParseIntError> for AthenaError {
    fn from(e: std::num::ParseIntError) -> Self {
        AthenaError::ParseErrorInt(e)
    }
}

impl From<std::string::FromUtf8Error> for AthenaError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        AthenaError::ParseErrorUtf8(e)
    }
}
