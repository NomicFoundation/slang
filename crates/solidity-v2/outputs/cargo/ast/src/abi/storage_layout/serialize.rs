//! The JSON storage layout: the `storage` items, and the `types` table keyed and
//! ordered by [`StorageType::type_identifier`], `null` when there are no items. Keys
//! are in alphabetical order, and slots and sizes are decimal strings.

use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use slang_solidity_v2_semantic::types::TypeId;

use super::{StorageLayout, StorageType, StorageTypeKind, StorageVariable};

impl Serialize for StorageLayout {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(
            "storage",
            &VariableList {
                variables: &self.items,
                layout: self,
            },
        )?;
        if self.types.is_empty() {
            map.serialize_entry("types", &())?;
        } else {
            map.serialize_entry("types", &TypeTable(self))?;
        }
        map.end()
    }
}

/// The types, sorted by their type identifier.
struct TypeTable<'a>(&'a StorageLayout);

impl Serialize for TypeTable<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let layout = self.0;
        let mut types: Vec<_> = layout.types.values().collect();
        types.sort_unstable_by(|a, b| a.type_identifier.cmp(&b.type_identifier));
        let mut map = serializer.serialize_map(Some(types.len()))?;
        for storage_type in types {
            map.serialize_entry(
                &storage_type.type_identifier,
                &TypeEntry {
                    storage_type,
                    layout,
                },
            )?;
        }
        map.end()
    }
}

/// State variables or struct members, in the order they are laid out.
struct VariableList<'a> {
    variables: &'a [StorageVariable],
    layout: &'a StorageLayout,
}

impl Serialize for VariableList<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.variables.len()))?;
        for variable in self.variables {
            seq.serialize_element(&Variable {
                variable,
                layout: self.layout,
            })?;
        }
        seq.end()
    }
}

struct Variable<'a> {
    variable: &'a StorageVariable,
    layout: &'a StorageLayout,
}

impl Serialize for Variable<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Self { variable, layout } = self;
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("astId", &u64::from(variable.node_id()))?;
        map.serialize_entry("contract", &layout.contract)?;
        map.serialize_entry("label", variable.name())?;
        map.serialize_entry("offset", &variable.offset())?;
        map.serialize_entry("slot", &variable.slot().to_string())?;
        map.serialize_entry("type", type_identifier_of(layout, variable.type_id()))?;
        map.end()
    }
}

struct TypeEntry<'a> {
    storage_type: &'a StorageType,
    layout: &'a StorageLayout,
}

impl Serialize for TypeEntry<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let StorageType {
            label, size, kind, ..
        } = self.storage_type;
        let layout = self.layout;
        let number_of_bytes = size.number_of_bytes().to_string();
        let mut map = serializer.serialize_map(None)?;
        match kind {
            StorageTypeKind::Value => {
                map.serialize_entry("encoding", "inplace")?;
                map.serialize_entry("label", label)?;
                map.serialize_entry("numberOfBytes", &number_of_bytes)?;
            }
            StorageTypeKind::Bytes => {
                map.serialize_entry("encoding", "bytes")?;
                map.serialize_entry("label", label)?;
                map.serialize_entry("numberOfBytes", &number_of_bytes)?;
            }
            StorageTypeKind::DynamicArray { element } => {
                map.serialize_entry("base", type_identifier_of(layout, *element))?;
                map.serialize_entry("encoding", "dynamic_array")?;
                map.serialize_entry("label", label)?;
                map.serialize_entry("numberOfBytes", &number_of_bytes)?;
            }
            StorageTypeKind::FixedSizeArray { element, .. } => {
                map.serialize_entry("base", type_identifier_of(layout, *element))?;
                map.serialize_entry("encoding", "inplace")?;
                map.serialize_entry("label", label)?;
                map.serialize_entry("numberOfBytes", &number_of_bytes)?;
            }
            StorageTypeKind::Mapping { key, value } => {
                map.serialize_entry("encoding", "mapping")?;
                map.serialize_entry("key", type_identifier_of(layout, *key))?;
                map.serialize_entry("label", label)?;
                map.serialize_entry("numberOfBytes", &number_of_bytes)?;
                map.serialize_entry("value", type_identifier_of(layout, *value))?;
            }
            StorageTypeKind::Struct { members } => {
                map.serialize_entry("encoding", "inplace")?;
                map.serialize_entry("label", label)?;
                map.serialize_entry(
                    "members",
                    &VariableList {
                        variables: members,
                        layout,
                    },
                )?;
                map.serialize_entry("numberOfBytes", &number_of_bytes)?;
            }
        }
        map.end()
    }
}

/// The type identifier of `type_id`, which the layout's type table always has.
fn type_identifier_of(layout: &StorageLayout, type_id: TypeId) -> &str {
    layout
        .storage_type(type_id)
        .expect("every type a storage layout refers to is in its table")
        .type_identifier()
}
