//! Generational arena (`SlotMap`) providing stable, versioned keys and $O(1)$ operations.

mod xff_serde;

use std::ops::{Index, IndexMut};

/// A versioned key into a [`SlotMap`].
///
/// Useless on its own, and cannot be manually constructed.
///
/// Created by [`SlotMap::insert`].
///
/// Used for indexing into a [`SlotMap`].
///
/// # Implements
/// - [`Ord`], [`PartialOrd`]: Sorts by index first, then by version.
/// - [`Copy`], [`Clone`], [`Eq`], [`Hash`]: To make it usable for Hash-maps and the like
/// - [`Debug`], [`PartialEq`]: Useful for debugging.
///
/// In general, you will never have to interface with this type directly nor use its associated methods.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SlotMapIndex {
    index: usize,
    version: u32,
    is_dummy: bool,
}

impl SlotMapIndex {
    /// Creates a dummy key.
    ///
    /// This key is invalid and cannot be used to index a [`SlotMap`].
    ///
    /// Using it will not do anything and be silently ignored.
    ///
    /// Actual [`SlotMapIndex`]es are created by [`SlotMap::insert`].
    pub fn new_dummy() -> Self {
        Self {
            index: 0,
            version: 0,
            is_dummy: true,
        }
    }
}

impl Ord for SlotMapIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.index == other.index {
            self.version.cmp(&other.version)
        } else {
            self.index.cmp(&other.index)
        }
    }
}

impl PartialOrd for SlotMapIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl SlotMapIndex {
    /// Returns the underlying slot index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Returns the generation version of this key.
    #[must_use]
    pub const fn version(self) -> u32 {
        self.version
    }

    /// Returns `true` if this key is a dummy key.
    #[must_use]
    pub const fn is_dummy(self) -> bool {
        self.is_dummy
    }
}

/// A single slot in a [`SlotMap`].
#[derive(Debug, Clone)]
struct Slot<T> {
    value: Option<T>,
    version: u32,
}

/// A generational slot map providing stable keys ([`SlotMapIndex`]) and O(1)
/// insertion, lookup, and removal.
///
/// As it is vector based, it uses the default vector growth strategy: Double the capacity when full.
/// It is advisable, as with any vector, to use [`with_capacity`] when the amount of elements can
/// be estimated.
///
/// The entire `SlotMap` along with each `SlotMapIndex` can be persisted and restored from / to disk.
/// To do so, make sure each element implements the [`XffSerde`] trait found in my `Aequa` library.
///
/// The capacity of the `SlotMap` is also persisted and restored.
#[derive(Debug, Clone)]
pub struct SlotMap<T> {
    slots: Vec<Slot<T>>,
    free: Vec<usize>,
}

impl<T> Default for SlotMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SlotMap<T> {
    /// Creates an empty [`SlotMap`].
    ///
    /// Consider using [`with_capacity`] if the amount of elements can be estimated to avoid unnecessary reallocations.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            slots: Vec::new(),
            free: Vec::new(),
        }
    }

    /// Creates an empty [`SlotMap`] with at least the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            free: Vec::new(),
        }
    }

    /// Returns the number of active elements in the map.
    #[must_use]
    pub fn len(&self) -> usize {
        self.slots.len() - self.free.len()
    }

    /// Returns `true` if the map contains no active elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the total number of slots the map can hold without reallocating.
    ///
    /// To find the amount of free slots, use [`free_slots`].
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Returns the number of free slots in the map.
    #[must_use]
    pub fn free_slots(&self) -> usize {
        self.free.len()
    }

    /// Returns `true` if the map contains a value for the specified `index`.
    #[must_use]
    pub fn contains_key(&self, index: SlotMapIndex) -> bool {
        if index.is_dummy() {
            return false;
        }
        self.get(index).is_some()
    }

    /// Returns a shared reference to the value corresponding to `index`,
    /// or `None` if the slot is vacant or the version does not match.
    #[must_use]
    pub fn get(&self, index: SlotMapIndex) -> Option<&T> {
        if index.is_dummy() {
            return None;
        }
        self.slots
            .get(index.index)
            .filter(|slot| slot.version == index.version)
            .and_then(|slot| slot.value.as_ref())
    }

    /// Returns a mutable reference to the value corresponding to `index`,
    /// or `None` if the slot is vacant or the version does not match.
    #[must_use]
    pub fn get_mut(&mut self, index: SlotMapIndex) -> Option<&mut T> {
        if index.is_dummy() {
            return None;
        }
        self.slots
            .get_mut(index.index)
            .filter(|slot| slot.version == index.version)
            .and_then(|slot| slot.value.as_mut())
    }

    /// Inserts a value into the map, returning its unique [`SlotMapIndex`].
    #[must_use]
    pub fn insert(&mut self, value: T) -> SlotMapIndex {
        if let Some(free_index) = self.free.pop() {
            let slot = &mut self.slots[free_index];
            slot.value = Some(value);
            SlotMapIndex {
                index: free_index,
                version: slot.version,
                is_dummy: false,
            }
        } else {
            let index = self.slots.len();
            self.slots.push(Slot {
                value: Some(value),
                version: 0,
            });
            SlotMapIndex {
                index,
                version: 0,
                is_dummy: false,
            }
        }
    }

    /// Removes the value associated with `index` and returns it, or `None`
    /// if the key is stale or out of bounds.
    pub fn remove(&mut self, index: SlotMapIndex) -> Option<T> {
        if index.is_dummy {
            return None;
        }
        let slot = self.slots.get_mut(index.index)?;
        if slot.version != index.version {
            return None;
        }
        let value = slot.value.take()?;
        // Advance the slot generation immediately to invalidate existing keys.
        // Only push to the free list if the generation did not saturate u32::MAX.
        if let Some(next_version) = slot.version.checked_add(1) {
            slot.version = next_version;
            self.free.push(index.index);
        }
        Some(value)
    }

    /// Removes and drops the value associated with `index`, if present.
    pub fn delete(&mut self, index: SlotMapIndex) {
        let _ = self.remove(index);
    }

    /// Clears the map, removing all values while retaining allocated capacity
    /// and advancing generations so previously issued keys remain invalid.
    pub fn clear(&mut self) {
        for (idx, slot) in self.slots.iter_mut().enumerate() {
            if slot.value.take().is_some()
                && let Some(next_version) = slot.version.checked_add(1)
            {
                slot.version = next_version;
                self.free.push(idx);
            }
        }
    }

    /// Returns an iterator over `(SlotMapIndex, &T)` pairs of all occupied slots.
    pub fn iter(&self) -> impl Iterator<Item = (SlotMapIndex, &T)> {
        self.slots.iter().enumerate().filter_map(|(index, slot)| {
            slot.value.as_ref().map(|val| {
                (
                    SlotMapIndex {
                        index,
                        version: slot.version,
                        is_dummy: false,
                    },
                    val,
                )
            })
        })
    }

    /// Returns an iterator over `(SlotMapIndex, &mut T)` pairs of all occupied slots.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (SlotMapIndex, &mut T)> {
        self.slots
            .iter_mut()
            .enumerate()
            .filter_map(|(index, slot)| {
                let version = slot.version;
                slot.value.as_mut().map(|val| {
                    (
                        SlotMapIndex {
                            index,
                            version,
                            is_dummy: false,
                        },
                        val,
                    )
                })
            })
    }

    /// Returns an iterator over all active [`SlotMapIndex`] keys.
    pub fn keys(&self) -> impl Iterator<Item = SlotMapIndex> + '_ {
        self.iter().map(|(k, _)| k)
    }

    /// Returns an iterator over shared references to all active values.
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.iter().map(|(_, v)| v)
    }

    /// Returns an iterator over mutable references to all active values.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut().map(|(_, v)| v)
    }
}

impl<T> Index<SlotMapIndex> for SlotMap<T> {
    type Output = T;

    fn index(&self, index: SlotMapIndex) -> &Self::Output {
        self.get(index)
            .expect("invalid or stale SlotMapIndex used to index SlotMap")
    }
}

impl<T> IndexMut<SlotMapIndex> for SlotMap<T> {
    fn index_mut(&mut self, index: SlotMapIndex) -> &mut Self::Output {
        self.get_mut(index)
            .expect("invalid or stale SlotMapIndex used to mutably index SlotMap")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_get_remove() {
        let mut map = SlotMap::new();
        assert!(map.is_empty());

        let k1 = map.insert("alpha");
        let k2 = map.insert("beta");
        assert_eq!(map.len(), 2);
        assert!(!map.is_empty());

        assert_eq!(map.get(k1), Some(&"alpha"));
        assert_eq!(map.get(k2), Some(&"beta"));
        assert_eq!(map[k1], "alpha");

        if let Some(v) = map.get_mut(k1) {
            *v = "alpha_modified";
        }
        assert_eq!(map.get(k1), Some(&"alpha_modified"));

        assert_eq!(map.remove(k1), Some("alpha_modified"));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(k1), None);
        assert!(!map.contains_key(k1));
    }

    #[test]
    fn slot_reuse_and_generational_aba_protection() {
        let mut map = SlotMap::with_capacity(4);
        assert!(map.capacity() >= 4);

        let k_old = map.insert(10);
        assert_eq!(k_old.index(), 0);
        assert_eq!(k_old.version(), 0);

        // Remove once; a second remove with the same stale key must return None
        assert_eq!(map.remove(k_old), Some(10));
        assert_eq!(map.remove(k_old), None);

        // Insert a new element; it should reuse index 0 with version 1
        let k_new = map.insert(20);
        assert_eq!(k_new.index(), 0);
        assert_eq!(k_new.version(), 1);

        // Old key must NOT access or remove the new value
        assert_eq!(map.get(k_old), None);
        assert_eq!(map.remove(k_old), None);

        // New key must work as expected
        assert_eq!(map.get(k_new), Some(&20));
        assert_eq!(map.remove(k_new), Some(20));
        assert!(map.is_empty());
    }

    #[test]
    fn iterators_and_clear() {
        let mut map = SlotMap::new();
        let k1 = map.insert(1);
        let k2 = map.insert(2);
        let k3 = map.insert(3);

        map.delete(k2);
        assert_eq!(map.len(), 2);

        for (_, val) in map.iter_mut() {
            *val *= 10;
        }

        let collected: Vec<(SlotMapIndex, i32)> = map.iter().map(|(k, &v)| (k, v)).collect();
        assert_eq!(collected, vec![(k1, 10), (k3, 30)]);

        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.get(k1), None);
        assert_eq!(map.get(k3), None);

        // Re-inserting after clear reuses slots with incremented versions
        let k4 = map.insert(40);
        assert_eq!(map.get(k4), Some(&40));
        assert_ne!(k4.version(), 0);
    }
}
