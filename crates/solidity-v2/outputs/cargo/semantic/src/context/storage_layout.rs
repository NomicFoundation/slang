use ruint::aliases::U256;

pub(super) const SLOT_SIZE: usize = 32;

/// How much storage a type occupies, for slot packing and layout.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageSize {
    /// A slot-aligned type occupying this many whole slots.
    Slots(U256),
    /// A value type that can share a slot with its neighbours. Its width in
    /// bytes, always in `1..=32`.
    Bytes(usize),
}

/// A variable's location in storage, as a slot and a byte offset within it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StoragePosition {
    pub slot: U256,
    pub offset: usize,
}

/// Assigns each state variable or struct member its storage position in
/// declaration order, packing [`StorageSize::Bytes`] values into a shared slot
/// and slot-aligning [`StorageSize::Slots`] aggregates.
pub struct StorageLayoutBuilder {
    slot: U256,
    byte_offset: usize,
}

impl StorageLayoutBuilder {
    pub fn new(base_slot: U256) -> Self {
        Self {
            slot: base_slot,
            byte_offset: 0,
        }
    }

    /// Allocates the next item, returning its [`StoragePosition`] and advancing
    /// past it. Returns `None` when the item would extend past the end of
    /// storage.
    pub fn allocate(&mut self, size: StorageSize) -> Option<StoragePosition> {
        match size {
            StorageSize::Slots(count) => {
                // Aggregates start on a fresh slot, so finish the current one.
                if self.byte_offset > 0 {
                    self.slot = self.slot.checked_add(U256::from(1))?;
                    self.byte_offset = 0;
                }
                let position = StoragePosition {
                    slot: self.slot,
                    offset: 0,
                };

                // Update for the next item.
                self.slot = self.slot.checked_add(count)?;
                Some(position)
            }
            StorageSize::Bytes(bytes) => {
                // Move to the next slot when the value does not fit this one.
                if self.byte_offset + bytes > SLOT_SIZE {
                    self.slot = self.slot.checked_add(U256::from(1))?;
                    self.byte_offset = 0;
                }
                let position = StoragePosition {
                    slot: self.slot,
                    offset: self.byte_offset,
                };

                // Update for the next item.
                self.byte_offset += bytes;
                Some(position)
            }
        }
    }

    /// The number of whole slots consumed, rounding a partially-filled final
    /// slot up to a slot boundary. `None` if that rounding runs past the end
    /// of storage.
    pub(super) fn slots_used(&self) -> Option<U256> {
        if self.byte_offset > 0 {
            self.slot.checked_add(U256::from(1))
        } else {
            Some(self.slot)
        }
    }
}
