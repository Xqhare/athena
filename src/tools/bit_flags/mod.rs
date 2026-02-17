use crate::utils::traits::unsigned::Unsigned;

/// A generic bit flag
///
/// Flags are counted from the least significant bit to the most significant bit
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
    flag: T
}

impl<T: Unsigned> BitFlag<T> {
    /// Creates a new `BitFlag`
    ///
    /// All flags are set to `false`
    pub fn new() -> Self {
        Self {
            flag: T::zero()
        }
    }

    /// Sets a flag
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the flag
    pub fn set(&mut self, index: usize) {
        self.flag = self.flag | (T::one() << index);
    }

    /// Clears a flag
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the flag
    pub fn clear(&mut self, index: usize) {
        self.flag = self.flag & !(T::one() << index);
    }

    /// Reads a flag
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the flag
    pub fn read(&self, index: usize) -> bool {
        (self.flag & (T::one() << index)) != T::zero()
    }
}

pub type U8Flag = BitFlag<u8>;
pub type U16Flag = BitFlag<u16>;
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
