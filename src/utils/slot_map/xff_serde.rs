use aequa::{Array, Object, XffSerde, XffValue, xff};
use nemesis::NemesisError;

use crate::utils::slot_map::{Slot, SlotMap, SlotMapIndex};

impl<T> XffSerde for SlotMap<T>
where
    T: XffSerde,
{
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        let slots = {
            let mut ary = Array::new();
            for slot in &self.slots {
                ary.push(slot.to_xff());
            }
            ary
        };
        let free = {
            let mut ary = Array::new();
            for index in &self.free {
                ary.push(xff!(*index));
            }
            ary
        };
        obj.insert("slots_cap", self.slots.capacity());
        obj.insert("slots", slots);
        obj.insert("free", free);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(slots) = obj.get("slots")
                && let Some(slots) = slots.as_array()
                && let Some(free) = obj.get("free")
                && let Some(free) = free.as_array()
                && let Some(slots_cap) = obj.get("slots_cap")
                && let Some(slots_cap) = slots_cap.as_number()
                && let Some(slots_cap) = slots_cap.into_usize()
            {
                let slots = {
                    let mut tmp_slots: Vec<Slot<T>> = Vec::with_capacity(slots_cap);
                    for slot in slots.iter() {
                        tmp_slots.push(Slot::<T>::from_xff(slot)?);
                    }
                    tmp_slots
                };
                let free = {
                    let mut tmp_free = Vec::with_capacity(free.len());
                    for index in free.iter() {
                        if let Some(index) = index.as_number()
                            && let Some(index) = index.into_usize()
                        {
                            tmp_free.push(index);
                        }
                    }
                    tmp_free
                };
                Ok(SlotMap { slots, free })
            } else {
                Err(NemesisError::new(
                    "SlotMap loading from XffValue",
                    "Invalid XffValue, missing slots or free key or invalid type",
                )
                .add_ctx(format!("Value: {xff:?}")))
            }
        } else {
            Err(NemesisError::new(
                "SlotMap loading from XffValue",
                "Invalid XffValue, not an object",
            )
            .add_ctx(format!("Value: {xff:?}")))
        }
    }
}

impl<T> XffSerde for Slot<T>
where
    T: XffSerde,
{
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        if let Some(value) = &self.value {
            obj.insert("value", value.to_xff());
        }
        obj.insert("version", self.version);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(version) = obj.get("version")
                && let Some(version) = version.as_number()
                && let Some(version) = version.into_usize()
            {
                let value = if let Some(value) = obj.get("value") {
                    Some(T::from_xff(value)?)
                } else {
                    None
                };
                let version = version as u32;
                Ok(Slot { value, version })
            } else {
                Err(NemesisError::new(
                    "Slot loading from XffValue",
                    "Invalid XffValue, missing version or invalid version type",
                )
                .add_ctx(format!("Value: {xff:?}")))
            }
        } else {
            Err(NemesisError::new(
                "Slot loading from XffValue",
                "Invalid XffValue, not an object",
            )
            .add_ctx(format!("Value: {xff:?}")))
        }
    }
}

impl XffSerde for SlotMapIndex {
    fn to_xff(&self) -> XffValue {
        let mut obj = Object::new();
        obj.insert("index", self.index);
        obj.insert("version", self.version);
        obj.insert("is_dummy", self.is_dummy);
        xff!(obj)
    }
    fn from_xff(xff: &XffValue) -> Result<Self, NemesisError>
    where
        Self: Sized,
    {
        if let Some(obj) = xff.as_object() {
            if let Some(index) = obj.get("index")
                && let Some(index) = index.as_number()
                && let Some(index) = index.into_usize()
                && let Some(version) = obj.get("version")
                && let Some(version) = version.as_number()
                && let Some(version) = version.into_usize()
                && let Some(is_dummy) = obj.get("is_dummy")
                && let Some(is_dummy) = is_dummy.as_boolean()
            {
                Ok(SlotMapIndex {
                    index,
                    version: version as u32,
                    is_dummy: *is_dummy,
                })
            } else {
                Err(NemesisError::new(
                    "SlotMapIndex loading from XffValue",
                    "Invalid XffValue, either missing index or version or of wrong type",
                )
                .add_ctx(format!("Value: {xff:?}")))
            }
        } else {
            Err(NemesisError::new(
                "SlotMapIndex loading from XffValue",
                "Invalid XffValue, not an object",
            )
            .add_ctx(format!("Value: {xff:?}")))
        }
    }
}
