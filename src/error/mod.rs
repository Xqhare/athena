//! Error handling for the Athena crate.
//!
//! Defines a unified error type `AthenaError` and a result alias `AthenaResult`
//! used throughout the library for consistent error reporting.

/// The primary error type for all Athena operations.
#[derive(Debug)]
pub enum AthenaError {
    /// Provided input data is invalid for the operation.
    InvalidInput,
    /// The operation resulted in an invalid state or output.
    InvalidOutput,
    /// Data structure or format is corrupted or unexpected.
    InvalidData,
    /// The provided buffer is too small to contain the result.
    InsufficientBuffer,
    /// An underlying I/O error occurred.
    IoError(std::io::Error),
    /// Error parsing an integer from a string or bytes.
    ParseErrorInt(std::num::ParseIntError),
    /// Error converting a byte sequence to a UTF-8 string.
    ParseErrorUtf8(std::string::FromUtf8Error),
    /// A LEB128 continuation bit was found where it wasn't expected.
    ContinuationBitInLastByte,
    /// Numerical overflow occurred during an operation.
    Overflow,
    /// Parity bit validation failed.
    ParityError,
    /// UUID byte sequence has an incorrect length.
    InvalidUuidLength,
    /// Table schema does not match the provided data.
    TableSchemaMismatch,
    /// The requested operation is not supported on the current platform.
    UnsupportedPlatform,
}

/// A specialized `Result` type for Athena operations.
pub type AthenaResult<T> = Result<T, AthenaError>;

impl std::fmt::Display for AthenaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AthenaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AthenaError::IoError(e) => Some(e),
            AthenaError::ParseErrorInt(e) => Some(e),
            AthenaError::ParseErrorUtf8(e) => Some(e),
            _ => None,
        }
    }
}

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
