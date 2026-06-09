//! Ergonomic bit flag management for fixed-width unsigned integers.
//!
//! This module provides the `BitFlag` struct and type aliases for common bit-widths
//! (`u8`, `u16`, `u32`), allowing for safe and readable manipulation of individual bits
//! as boolean flags.

use crate::utils::traits::unsigned::Unsigned;

/// A generic bit flag container.
///
/// Flags are indexed from the least significant bit (0) to the most significant bit.
///
/// # Example
///
/// ```
/// use athena::bit_flags::BitFlag;
///
/// let mut flag = BitFlag::<u8>::new();
/// flag.set(0);
/// flag.set(1);
/// flag.set(7);
///
/// flag.clear(1);
///
/// assert!(flag.read(0));
/// assert!(!flag.read(1));
/// assert!(flag.read(7));
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BitFlag<T: Unsigned> {
    flag: T,
}

impl<T: Unsigned> BitFlag<T> {
    /// Creates a new `BitFlag` with all bits initialized to `false` (zero).
    #[must_use]
    pub fn new() -> Self {
        Self { flag: T::zero() }
    }

    /// Sets the bit at the specified index to `true`.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the bit to set.
    pub fn set(&mut self, index: usize) {
        self.flag = self.flag | (T::one() << index);
    }

    /// Clears the bit at the specified index (sets it to `false`).
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the bit to clear.
    pub fn clear(&mut self, index: usize) {
        self.flag = self.flag & !(T::one() << index);
    }

    /// Reads the state of the bit at the specified index.
    ///
    /// # Arguments
    /// * `index` - The zero-based index of the bit to read.
    ///
    /// # Returns
    /// * `true` if the bit is set, `false` otherwise.
    pub fn read(&self, index: usize) -> bool {
        (self.flag & (T::one() << index)) != T::zero()
    }
}

/// An 8-bit bit flag container.
pub type U8Flag = BitFlag<u8>;
/// A 16-bit bit flag container.
pub type U16Flag = BitFlag<u16>;
/// A 32-bit bit flag container.
pub type U32Flag = BitFlag<u32>;

#[cfg(test)]
mod tests {
    use super::BitFlag;

    #[test]
    fn test_generic_flag() {
        let mut flag = BitFlag::<u64>::new();
        flag.set(63);
        assert!(flag.read(63));
        flag.clear(63);
        assert!(!flag.read(63));
    }
}
