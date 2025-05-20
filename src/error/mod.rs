
#[derive(Debug)]
pub enum AthenaError {
    InvalidInput,
    InvalidOutput,
    InvalidData,
    InsufficientBuffer,
    IoError(std::io::Error),
    ContinuationBitInLastByte,
    Overflow,
}

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

