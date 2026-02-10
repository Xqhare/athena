
/// A `u32` flag
///
/// Flags are counted from the least significant bit to the most significant bit
///
/// # Example
///
/// ```
/// use athena::bit_flags::U32Flag;
///
/// let mut flag = U32Flag::new();
/// flag.set(0);
/// flag.set(1);
/// flag.set(12);
/// flag.set(31);
///
/// flag.clear(1);
///
/// assert!(flag.read(0));
/// assert!(!flag.read(1));
/// assert!(flag.read(12));
/// assert!(flag.read(31));
/// ```
#[derive(Debug)]
pub struct U32Flag {
    flag: u32
}

impl U32Flag {
    /// Creates a new `U32Flag`
    ///
    /// All flags are set to `false`
    pub fn new() -> U32Flag {
        U32Flag {
            flag: 0
        }
    }

    /// Sets a flag
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the flag
    ///
    /// # Example
    ///
    /// ```
    /// use athena::bit_flags::U32Flag;
    ///
    /// let mut flag = U32Flag::new();
    ///
    /// flag.set(0);
    ///
    /// assert!(flag.read(0));
    /// ```
    pub fn set(&mut self, index: u8) {
        self.flag |= 1 << index;
    }

    /// Clears a flag
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the flag
    ///
    /// # Example
    ///
    /// ```
    /// use athena::bit_flags::U32Flag;
    ///
    /// let mut flag = U32Flag::new();
    ///
    /// flag.set(0);
    /// flag.clear(0);
    ///
    /// assert!(!flag.read(0));
    /// ```
    pub fn clear(&mut self, index: u8) {
        self.flag &= !(1 << index);
    }

    /// Reads a flag
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the flag
    ///
    /// # Example
    ///
    /// ```
    /// use athena::bit_flags::U32Flag;
    ///
    /// let mut flag = U32Flag::new();
    ///
    /// flag.set(0);
    ///
    /// assert!(flag.read(0));
    /// ```
    pub fn read(&self, index: u8) -> bool {
        (self.flag & (1 << index)) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::U32Flag;
    #[test]
    fn first_bit() {
        let mut flag = U32Flag::new();
        flag.set(0);
        assert!(flag.read(0));
    }
    #[test]
    fn all_bits() {
        let mut flag = U32Flag::new();
        for i in 0..32 {
            flag.set(i);
        }
        for i in 0..32 {
            assert!(flag.read(i));
        }
    }
    #[test]
    fn all_bits_clear() {
        let mut flag = U32Flag::new();
        for i in 0..32 {
            if i % 2 == 0 {
                continue
            }
            flag.set(i);
        }
        for i in 0..32 {
            flag.clear(i);
        }
        for i in 0..32 {
            assert!(!flag.read(i));
        }
    }
}
