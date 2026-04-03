pub type RngResult<T> = Result<T, RngError>;

/// Error Type for Random Number Generator
pub enum RngError {
    /// I/O Error
    Io(std::io::Error),
    /// Used for other errors
    Generic(String),
}

impl From<std::io::Error> for RngError {
    fn from(e: std::io::Error) -> Self {
        RngError::Io(e)
    }
}

pub trait RngApi {
    /// Generate a random `u8`
    ///
    /// # Return
    ///
    /// `Result<u8, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_u8(&mut self) -> Result<u8, RngError>;
    /// Generate a random `u16`
    ///
    /// # Return
    ///
    /// `Result<u16, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_u16(&mut self) -> Result<u16, RngError>;
    /// Generate a random `u32`
    ///
    /// # Return
    ///
    /// `Result<u32, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_u32(&mut self) -> Result<u32, RngError>;
    /// Generate a random `u64`
    ///
    /// # Return
    ///
    /// `Result<u64, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_u64(&mut self) -> Result<u64, RngError>;
    /// Generate a random `usize`
    ///
    /// # Return
    ///
    /// `Result<usize, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_usize(&mut self) -> Result<usize, RngError>;

    /// Generate a random `i8`
    ///
    /// # Return
    ///
    /// `Result<i8, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_i8(&mut self) -> Result<i8, RngError>;
    /// Generate a random `i16`
    ///
    /// # Return
    ///
    /// `Result<i16, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_i16(&mut self) -> Result<i16, RngError>;
    /// Generate a random `i32`
    ///
    /// # Return
    ///
    /// `Result<i32, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_i32(&mut self) -> Result<i32, RngError>;
    /// Generate a random `i64`
    ///
    /// # Return
    ///
    /// `Result<i64, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_i64(&mut self) -> Result<i64, RngError>;

    /// Generate a random `f32`
    ///
    /// # Return
    ///
    /// `Result<f32, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_f32(&mut self) -> Result<f32, RngError>;
    /// Generate a random `f64`
    ///
    /// # Return
    ///
    /// `Result<f64, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_f64(&mut self) -> Result<f64, RngError>;

    /// Generate a random byte array
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the byte array
    ///
    /// # Return
    ///
    /// `Result<Vec<u8>, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_bytes(&mut self, len: usize) -> Result<Vec<u8>, RngError>;
    /// Generate a random string of length `len`
    ///
    /// # Arguments
    ///
    /// * `len` - The length of the string
    ///
    /// # Return
    ///
    /// `Result<String, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_string(&mut self, len: usize) -> Result<String, RngError>;
    /// Generate a random Latin character
    ///
    /// Generates either a lowercase or uppercase character from A-Z
    ///
    /// # Arguments
    ///
    /// * `uppercase` - Whether to generate an uppercase character
    ///
    /// # Return
    ///
    /// `Result<char, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_latin_char(&mut self, uppercase: bool) -> Result<char, RngError>;
    /// Generate a random ASCII character
    ///
    /// Valid ASCII characters are 32-126
    ///
    /// # Return
    ///
    /// `Result<char, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_ascii_char(&mut self) -> Result<char, RngError>;

    /// Generate a random boolean
    ///
    /// # Return
    ///
    /// `Result<bool, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_bool(&mut self) -> Result<bool, RngError>;

    /// Generate a random number in the range `min` to `max` inclusive on both ends
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<usize, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_from_range_inclusive(&mut self, min: usize, max: usize) -> Result<usize, RngError>;
    /// Generate a random number in the range `min` to `max` exclusive on the max end
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<usize, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_from_range(&mut self, min: usize, max: usize) -> Result<usize, RngError>;
    /// Generate a random number in the range `min` to `max` exclusive on the max end
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<u64, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_from_u64_range(&mut self, min: u64, max: u64) -> Result<u64, RngError>;
    /// Generate a random number in the range `min` to `max` exclusive on the max end
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<isize, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_from_i_range(&mut self, min: isize, max: isize) -> Result<isize, RngError>;
    /// Generate a random number in the range `min` to `max` exclusive on the max end
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<i64, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_from_i64_range(&mut self, min: i64, max: i64) -> Result<i64, RngError>;
    /// Generate a random number in the range `min` to `max` exclusive on the max end
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<i32, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_from_i32_range(&mut self, min: i32, max: i32) -> Result<i32, RngError>;
    /// Generate a random number in the range `min` to `max` exclusive on the max end
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<f32, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_from_f32_range(&mut self, min: f32, max: f32) -> Result<f32, RngError>;
    /// Generate a random number in the range `min` to `max` exclusive on the max end
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<f64, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_from_f64_range(&mut self, min: f64, max: f64) -> Result<f64, RngError>;

    /// Generate a random index
    ///
    /// # Arguments
    ///
    /// * `collection_length` - The length of the collection
    ///
    /// # Return
    ///
    /// `Result<usize, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_index(&mut self, collection_length: usize) -> Result<usize, RngError>;

    /// Generate a random number in the range `0` to `max` inclusive
    ///
    /// # Arguments
    ///
    /// * `max` - The maximum value
    ///
    /// # Return
    ///
    /// `Result<usize, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_with_ceiling(&mut self, max: usize) -> Result<usize, RngError>;
    /// Generate a random number in the range `min` to `usize::MAX` inclusive
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value
    ///
    /// # Return
    ///
    /// `Result<usize, RngError>`
    ///
    /// # Errors
    ///
    /// May error if the random number generator fails
    fn random_with_floor(&mut self, min: usize) -> Result<usize, RngError>;
}
