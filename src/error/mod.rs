
#[derive(Debug)]
pub enum AthenaError {
    InvalidInput,
    InvalidOutput,
    InsufficientBuffer,
    IoError(std::io::Error),
}
