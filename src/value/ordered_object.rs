use super::XffValue;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// An object made up of key-value pairs of XFF values where the order of insertion is preserved.
///
/// Internally uses a `Vec<(String, XffValue)>`.
pub struct OrderedObject {
    /// The underlying key-value pairs
    pub pairs: Vec<(String, XffValue)>,
}

impl OrderedObject {
    /// Creates a new, empty ordered object.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }

    /// Creates an ordered object from a vector of pairs.
    pub fn from_vec(pairs: Vec<(String, XffValue)>) -> Self {
        Self { pairs }
    }

    /// Returns the number of elements in the ordered object.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Returns `true` if the ordered object is empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Adds a key-value pair to the end of the ordered object.
    pub fn push<K: Into<String>, V: Into<XffValue>>(&mut self, key: K, value: V) {
        self.pairs.push((key.into(), value.into()));
    }

    /// Clears the ordered object.
    pub fn clear(&mut self) {
        self.pairs.clear();
    }

    /// Returns a reference to the value associated with the key, if it exists.
    ///
    /// Note: This is an O(n) operation.
    pub fn get(&self, key: &str) -> Option<&XffValue> {
        self.pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Returns a mutable reference to the value associated with the key, if it exists.
    ///
    /// Note: This is an O(n) operation.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut XffValue> {
        self.pairs.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Returns a reference to the pair at the given index.
    pub fn get_index(&self, index: usize) -> Option<&(String, XffValue)> {
        self.pairs.get(index)
    }

    /// Returns a mutable reference to the pair at the given index.
    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut (String, XffValue)> {
        self.pairs.get_mut(index)
    }

    /// Returns an iterator over the key-value pairs.
    pub fn iter(&self) -> std::slice::Iter<'_, (String, XffValue)> {
        self.pairs.iter()
    }

    /// Returns a mutable iterator over the key-value pairs.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (String, XffValue)> {
        self.pairs.iter_mut()
    }
}

impl From<Vec<(String, XffValue)>> for OrderedObject {
    fn from(pairs: Vec<(String, XffValue)>) -> Self {
        Self { pairs }
    }
}

impl From<OrderedObject> for Vec<(String, XffValue)> {
    fn from(obj: OrderedObject) -> Self {
        obj.pairs
    }
}

impl std::ops::Index<usize> for OrderedObject {
    type Output = (String, XffValue);

    fn index(&self, index: usize) -> &Self::Output {
        &self.pairs[index]
    }
}

impl std::ops::IndexMut<usize> for OrderedObject {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pairs[index]
    }
}

impl std::ops::Index<&str> for OrderedObject {
    type Output = XffValue;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl std::fmt::Display for OrderedObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{(ordered) ")?;
        for (i, (k, v)) in self.pairs.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, "}}")
    }
}
