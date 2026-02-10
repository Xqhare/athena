
/// A u8 flag
///
/// Flags are counted from the least significant bit to the most significant bit
///
/// # Example
///
/// ```
/// use athena::bit_flags::U8Flag;
///
/// let mut flag = U8Flag::new();
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
#[derive(Debug)]
pub struct U8Flag {
    flag: u8
}

impl U8Flag {
    /// Creates a new `U16Flag`
    ///
    /// All flags are set to `false`
    pub fn new() -> U8Flag {
        U8Flag {
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
    /// use athena::bit_flags::U8Flag;
    ///
    /// let mut flag = U8Flag::new();
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
    /// use athena::bit_flags::U8Flag;
    ///
    /// let mut flag = U8Flag::new();
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
    /// use athena::bit_flags::U8Flag;
    ///
    /// let mut flag = U8Flag::new();
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
    use super::U8Flag;
    #[test]
    fn first_bit() {
        let mut flag = U8Flag::new();
        flag.set(0);
        assert!(flag.read(0));
    }
    #[test]
    fn all_bits() {
        let mut flag = U8Flag::new();
        for i in 0..8 {
            flag.set(i);
        }
        for i in 0..8 {
            assert!(flag.read(i));
        }
    }
    #[test]
    fn all_bits_clear() {
        let mut flag = U8Flag::new();
        for i in 0..8 {
            if i % 2 == 0 {
                continue
            }
            flag.set(i);
        }
        for i in 0..8 {
            flag.clear(i);
        }
        for i in 0..8 {
            assert!(!flag.read(i));
        }
    }
}
