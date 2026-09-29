use ruint::aliases::U256;
use slang_solidity_v2_common::nodes::NodeId;

use crate::types::TypeId;

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

/// How a type is laid out in storage: its size, and how its contents are
/// arranged. Every `TypeId` it refers to is the type as it is in storage, so
/// a struct member and a state variable of the same type share one.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageTypeLayout {
    pub size: StorageSize,
    pub kind: StorageTypeKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageTypeKind {
    /// A value type, stored in place.
    Value,
    /// A `bytes` or `string`, stored in its slot when short and at the slot's
    /// hash otherwise.
    Bytes,
    /// A dynamic array: its slot holds the length, and the elements start at
    /// the slot's hash.
    DynamicArray { element: TypeId },
    /// A fixed-size array, with its elements stored in place.
    FixedSizeArray { element: TypeId },
    /// A mapping: its slot is empty, each value is at the hash of its key and
    /// the slot.
    Mapping { key: TypeId, value: TypeId },
    /// A struct, with its members stored in place, in declaration order.
    Struct { members: Vec<StorageMember> },
}

/// A struct member's position, relative to the start of the struct.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageMember {
    pub node_id: NodeId,
    pub type_id: TypeId,
    pub position: StoragePosition,
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
