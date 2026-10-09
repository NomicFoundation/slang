use ruint::aliases::U256;
use slang_solidity_v2_common::collections::{Set, SortedMap};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::SemanticContext;
use crate::binder::{Definition, StructDefinition};
use crate::types::{
    ArrayType, ByteArrayType, FixedPointNumberType, FixedSizeArrayType, IntegerType, MappingType,
    StructType, Type, TypeId, UserDefinedValueType,
};

const SLOT_SIZE: usize = 32;

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

/// The output type of `storage_type_table`, used to compute the type table for
/// a contract storage layout.
pub type StorageTypeTable<T> = SortedMap<TypeId, T>;

/// How a type is laid out in storage: its size, and how its contents are
/// arranged.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageTypeLayout {
    pub size: StorageSize,
    pub kind: StorageTypeKind,
}

/// How a type's contents are arranged in storage.
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
    /// A fixed-size array of `length` elements, stored in place.
    FixedSizeArray { element: TypeId, length: U256 },
    /// A mapping: its slot is empty, each value is at the hash of its key and
    /// the slot.
    Mapping { key: TypeId, value: TypeId },
    /// A struct, with its members stored in place, in declaration order.
    Struct { members: Vec<StorageVariable> },
}

/// A state variable or struct member, and where it is stored. The `type_id` is
/// guaranteed to have the `Storage` location in the registry, regardless of how
/// a struct member is typed in the `Binder`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageVariable {
    node_id: NodeId,
    name: String,
    type_id: TypeId,
    position: StoragePosition,
}

impl StorageVariable {
    /// A variable at `position`, measured as [`Self::position`] describes.
    /// `type_id` must have the `Storage` location.
    pub fn new(node_id: NodeId, name: String, type_id: TypeId, position: StoragePosition) -> Self {
        Self {
            node_id,
            name,
            type_id,
            position,
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// The variable's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The variable's type, as it is in storage.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Where the variable starts: in the contract's storage, past its base
    /// slot, for a state variable, and relative to the start of the struct for
    /// a struct member.
    pub fn position(&self) -> StoragePosition {
        self.position
    }

    /// The slot of [`Self::position`].
    pub fn slot(&self) -> U256 {
        self.position.slot
    }

    /// The byte offset within the slot of [`Self::position`].
    pub fn offset(&self) -> usize {
        self.position.offset
    }
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
    fn slots_used(&self) -> Option<U256> {
        if self.byte_offset > 0 {
            self.slot.checked_add(U256::from(1))
        } else {
            Some(self.slot)
        }
    }
}

impl SemanticContext {
    pub(crate) const ADDRESS_BYTE_SIZE: usize = 20;
    pub(crate) const SELECTOR_SIZE: usize = 4;

    pub fn storage_size_of_type_id(&self, type_id: TypeId) -> Option<StorageSize> {
        self.storage_size_of_type_id_impl(type_id, &mut Set::default())
    }

    fn storage_size_of_type_id_impl(
        &self,
        type_id: TypeId,
        visited_structs: &mut Set<NodeId>,
    ) -> Option<StorageSize> {
        use StorageSize::{Bytes, Slots};
        match self.types.get_type_by_id(type_id) {
            Type::Address(_) | Type::Contract(_) | Type::Interface(_) => {
                Some(Bytes(Self::ADDRESS_BYTE_SIZE))
            }
            Type::Boolean => Some(Bytes(1)),
            Type::FixedPointNumber(FixedPointNumberType { bits, .. })
            | Type::Integer(IntegerType { bits, .. }) => {
                Some(Bytes((bits.div_ceil(8)).try_into().unwrap()))
            }
            Type::ByteArray(ByteArrayType { width }) => Some(Bytes((*width).try_into().unwrap())),
            Type::Enum(_) => Some(Bytes(1)),
            Type::Bytes(_) | Type::String(_) => Some(Slots(U256::from(1))),
            Type::Mapping(_) => Some(Slots(U256::from(1))),

            Type::Array(_) => Some(Slots(U256::from(1))),
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => Some(Slots(
                match self.storage_size_of_type_id_impl(*element_type, visited_structs)? {
                    Slots(slots_per_element) => size.checked_mul(slots_per_element)?,
                    Bytes(bytes) => {
                        let elements_per_slot = U256::from(SLOT_SIZE / bytes);
                        size.div_ceil(elements_per_slot)
                    }
                },
            )),

            Type::Function(function_type) => {
                if function_type.is_externally_visible() {
                    Some(Bytes(Self::ADDRESS_BYTE_SIZE + Self::SELECTOR_SIZE))
                } else {
                    // NOTE: an internal function ref type is 8 bytes long, it's
                    // opaque and its meaning not documented
                    Some(Bytes(8))
                }
            }
            Type::Struct(StructType { definition_id, .. }) => {
                let Definition::Struct(struct_definition) =
                    self.binder.find_definition_by_id(*definition_id)?
                else {
                    return None;
                };
                self.lay_out_struct_members(struct_definition, visited_structs, |_, _, _| {})
                    .map(Slots)
            }
            Type::UserDefinedValue(UserDefinedValueType { definition_id }) => self
                .storage_size_of_type_id_impl(
                    self.user_defined_value_target_type_id(*definition_id)?,
                    visited_structs,
                ),

            Type::ArraySlice(_)
            | Type::Error(_)
            | Type::Event(_)
            | Type::Library(_)
            | Type::Literal(_)
            | Type::MetaType(_)
            | Type::Tuple(_)
            | Type::UserMetaType(_) => None,
        }
    }

    /// Positions each member of `struct_definition` from slot 0, passing it to
    /// `on_member` with its declared type, and returns the number of slots the
    /// struct occupies. `None` when the struct is recursive, a member has no
    /// storage size, or it overflows storage.
    fn lay_out_struct_members(
        &self,
        struct_definition: &StructDefinition,
        visited_structs: &mut Set<NodeId>,
        mut on_member: impl FnMut(&ir::StructMember, TypeId, StoragePosition),
    ) -> Option<U256> {
        // Recursive structs are not valid Solidity, but guard against cycles
        // to avoid unbounded recursion if malformed types reach this point.
        // Such recursion should already have been reported by the recursive-struct analysis.
        let definition_id = struct_definition.ir_node.id();
        if !visited_structs.insert(definition_id) {
            return None;
        }
        let mut builder = StorageLayoutBuilder::new(U256::ZERO);
        for member in struct_definition.ir_node.members.iter() {
            let member_type_id = self.binder.node_typing(member.id()).as_type_id()?;
            let member_size = self.storage_size_of_type_id_impl(member_type_id, visited_structs)?;
            let position = builder.allocate(member_size)?;
            on_member(member, member_type_id, position);
        }
        visited_structs.remove(&definition_id);
        builder.slots_used()
    }

    /// The type table of a storage layout: `roots` (the types of its state
    /// variables) and every type those refer to, each with `describe` applied
    /// to how it is laid out. `None` when one of them cannot be stored or
    /// overflows storage.
    pub fn storage_type_table<T>(
        &self,
        roots: impl IntoIterator<Item = TypeId>,
        mut describe: impl FnMut(TypeId, StorageTypeLayout) -> T,
    ) -> Option<StorageTypeTable<T>> {
        let mut table = SortedMap::default();
        for type_id in roots {
            self.add_to_storage_type_table(type_id, &mut table, &mut describe)?;
        }
        Some(table)
    }

    /// Adds `type_id` to `table`, then every type it refers to that `table`
    /// does not have yet. Checking `table` first also stops at recursive
    /// structs, which refer back to themselves through a mapping or an array.
    fn add_to_storage_type_table<T>(
        &self,
        type_id: TypeId,
        table: &mut StorageTypeTable<T>,
        describe: &mut impl FnMut(TypeId, StorageTypeLayout) -> T,
    ) -> Option<()> {
        if table.contains_key(&type_id) {
            return Some(());
        }
        let layout = self.storage_type_layout(type_id)?;
        // `describe` takes the layout, so keep what it refers to first. Only a
        // struct refers to more than two types.
        let (first, second, member_type_ids) = match &layout.kind {
            StorageTypeKind::Value | StorageTypeKind::Bytes => (None, None, Vec::new()),
            StorageTypeKind::DynamicArray { element }
            | StorageTypeKind::FixedSizeArray { element, .. } => (Some(*element), None, Vec::new()),
            StorageTypeKind::Mapping { key, value } => (Some(*key), Some(*value), Vec::new()),
            StorageTypeKind::Struct { members } => (
                None,
                None,
                members.iter().map(|member| member.type_id).collect(),
            ),
        };
        table.insert(type_id, describe(type_id, layout));
        for referenced_type_id in first.into_iter().chain(second).chain(member_type_ids) {
            self.add_to_storage_type_table(referenced_type_id, table, describe)?;
        }
        Some(())
    }

    /// How a type in storage is laid out, as an entry of
    /// [`Self::storage_type_table`]. `None` when the type cannot be stored or
    /// overflows storage.
    fn storage_type_layout(&self, type_id: TypeId) -> Option<StorageTypeLayout> {
        let kind = match self.types.get_type_by_id(type_id) {
            Type::Struct(StructType { definition_id, .. }) => {
                let Definition::Struct(struct_definition) =
                    self.binder.find_definition_by_id(*definition_id)?
                else {
                    return None;
                };
                let mut members = Vec::with_capacity(struct_definition.ir_node.members.len());
                let slots = self.lay_out_struct_members(
                    struct_definition,
                    &mut Set::default(),
                    |member, type_id, position| {
                        members.push(StorageVariable::new(
                            member.id(),
                            member.name.unparse().to_string(),
                            self.types.storage_type_id(type_id),
                            position,
                        ));
                    },
                )?;
                return Some(StorageTypeLayout {
                    size: StorageSize::Slots(slots),
                    kind: StorageTypeKind::Struct { members },
                });
            }
            Type::Array(ArrayType { element_type, .. }) => StorageTypeKind::DynamicArray {
                element: *element_type,
            },
            Type::Bytes(_) | Type::String(_) => StorageTypeKind::Bytes,
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => StorageTypeKind::FixedSizeArray {
                element: *element_type,
                length: *size,
            },
            Type::Mapping(MappingType {
                key_type_id,
                value_type_id,
            }) => StorageTypeKind::Mapping {
                key: *key_type_id,
                value: *value_type_id,
            },
            Type::Address(_)
            | Type::Boolean
            | Type::ByteArray(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::FixedPointNumber(_)
            | Type::Function(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::UserDefinedValue(_) => StorageTypeKind::Value,

            // None of these can be stored. They should never reach a storage
            // layout, but describe nothing if they do.
            Type::ArraySlice(_)
            | Type::Error(_)
            | Type::Event(_)
            | Type::Library(_)
            | Type::Literal(_)
            | Type::MetaType(_)
            | Type::Tuple(_)
            | Type::UserMetaType(_) => return None,
        };
        Some(StorageTypeLayout {
            size: self.storage_size_of_type_id(type_id)?,
            kind,
        })
    }
}
