use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition};
use crate::context::{ADDRESS_BYTE_SIZE, SELECTOR_SIZE, SLOT_SIZE};
use crate::types::{Type, TypeId, TypeRegistry};

/// Pre-computed derivations for a single registered type.
struct TypeData {
    /// `type_internal_name` result.
    internal_name: String,

    /// `type_canonical_name` result. `None` is a valid outcome (e.g. mappings,
    /// recursive structs).
    canonical_name: Option<String>,

    /// `storage_size_of_type_id` result. `None` signals a type that doesn't
    /// live in storage (literals, void, tuples).
    storage_size: Option<usize>,
}

/// Cache of type-derivation results, computed once at the end of the semantic
/// passes. Every `TypeId` registered in the `TypeRegistry` has an entry.
pub(super) struct TypeDataCache {
    data: HashMap<TypeId, TypeData>,
}

impl TypeDataCache {
    pub(super) fn build_from(binder: &Binder, types: &TypeRegistry) -> Self {
        TypeDataCacheBuilder::new(binder, types).build()
    }

    fn get(&self, type_id: TypeId) -> &TypeData {
        self.data
            .get(&type_id)
            .expect("type_id is registered in the type registry")
    }

    pub(super) fn internal_name(&self, type_id: TypeId) -> &str {
        &self.get(type_id).internal_name
    }

    pub(super) fn canonical_name(&self, type_id: TypeId) -> Option<&str> {
        self.get(type_id).canonical_name.as_deref()
    }

    pub(super) fn storage_size(&self, type_id: TypeId) -> Option<usize> {
        self.get(type_id).storage_size
    }
}

/// Builds the per-type derivation cache in a single structural traversal of
/// the type registry. Sub-type derivations are resolved by looking them up in
/// the (partially built) cache, so each type is visited at most once.
struct TypeDataCacheBuilder<'a> {
    binder: &'a Binder,
    types: &'a TypeRegistry,
    data: HashMap<TypeId, TypeData>,
}

impl<'a> TypeDataCacheBuilder<'a> {
    fn new(binder: &'a Binder, types: &'a TypeRegistry) -> Self {
        Self {
            binder,
            types,
            data: HashMap::new(),
        }
    }

    fn build(mut self) -> TypeDataCache {
        let type_ids: Vec<TypeId> = self.types.iter_types().map(|(id, _)| id).collect();
        for type_id in type_ids {
            self.compute(type_id);
        }
        TypeDataCache { data: self.data }
    }

    fn definition_canonical_name(&self, definition_id: NodeId) -> String {
        self.binder
            .find_definition_by_id(definition_id)
            .unwrap()
            .identifier()
            .unparse()
            .to_string()
    }

    /// Populates the cache for `type_id`. Sub-type derivations are resolved by
    /// ensuring the sub-type is computed (recursive call), then looking up the
    /// cached values — there are no separate recursive paths per derivation.
    ///
    /// Cycle detection (only relevant for self-referential structs, which are
    /// invalid Solidity but guarded against defensively): a placeholder entry
    /// (with `internal_name` set and `canonical_name`/`storage_size` left as
    /// `None`) is inserted at the top of the `Struct` arm before recursing
    /// into members, so a recursive `compute(self_type)` short-circuits via
    /// the `data.contains_key` check. The cycled struct's canonical/storage
    /// are still `None` at that point, so the sub-type lookup inside
    /// `struct_canonical_name` / `struct_storage_size` returns `None`, which
    /// propagates up.
    fn compute(&mut self, type_id: TypeId) {
        if self.data.contains_key(&type_id) {
            return;
        }
        match self.types.get_type_by_id(type_id) {
            Type::Address { .. } => self.insert(
                type_id,
                "address".to_string(),
                Some("address".to_string()),
                Some(ADDRESS_BYTE_SIZE),
            ),
            Type::Boolean => self.insert(
                type_id,
                "bool".to_string(),
                Some("bool".to_string()),
                Some(1),
            ),
            Type::ByteArray { width } => {
                let width = *width;
                let name = format!("bytes{width}");
                let storage = width.try_into().unwrap();
                self.insert(type_id, name.clone(), Some(name), Some(storage));
            }
            Type::Bytes { .. } => self.insert(
                type_id,
                "bytes".to_string(),
                Some("bytes".to_string()),
                Some(SLOT_SIZE),
            ),
            Type::FixedPointNumber {
                signed,
                bits,
                precision_bits,
            } => {
                let (signed, bits, precision_bits) = (*signed, *bits, *precision_bits);
                let name = format!(
                    "{prefix}{bits}x{precision_bits}",
                    prefix = if signed { "fixed" } else { "ufixed" }
                );
                let storage = (bits.div_ceil(8)).try_into().unwrap();
                self.insert(type_id, name.clone(), Some(name), Some(storage));
            }
            Type::Function(function_type) => {
                let externally_visible = function_type.is_externally_visible();
                // NOTE: an internal function ref type is 8 bytes long, it's
                // opaque and its meaning not documented
                let storage = if externally_visible {
                    ADDRESS_BYTE_SIZE + SELECTOR_SIZE
                } else {
                    8
                };
                self.insert(
                    type_id,
                    "function".to_string(),
                    Some("function".to_string()),
                    Some(storage),
                );
            }
            Type::Integer { signed, bits } => {
                let (signed, bits) = (*signed, *bits);
                let name = format!(
                    "{prefix}{bits}",
                    prefix = if signed { "int" } else { "uint" }
                );
                let storage = (bits.div_ceil(8)).try_into().unwrap();
                self.insert(type_id, name.clone(), Some(name), Some(storage));
            }
            Type::Literal(_) => {
                self.insert(type_id, "literal".to_string(), None, None);
            }
            Type::String { .. } => self.insert(
                type_id,
                "string".to_string(),
                Some("string".to_string()),
                Some(SLOT_SIZE),
            ),
            Type::Void => {
                self.insert(type_id, "void".to_string(), None, None);
            }

            Type::Array { element_type, .. } => {
                let element_type = *element_type;
                self.compute(element_type);
                let element = &self.data[&element_type];
                let element_internal = element.internal_name.clone();
                let element_canonical = element.canonical_name.clone();
                self.insert(
                    type_id,
                    format!("{element_internal}[]"),
                    element_canonical.map(|c| format!("{c}[]")),
                    Some(SLOT_SIZE),
                );
            }
            Type::FixedSizeArray {
                element_type, size, ..
            } => {
                let element_type = *element_type;
                let size = *size;
                self.compute(element_type);
                let element = &self.data[&element_type];
                let element_internal = element.internal_name.clone();
                let element_canonical = element.canonical_name.clone();
                let element_size = element.storage_size;
                let storage = element_size.map(|es| {
                    if es > SLOT_SIZE {
                        let slots_per_element = es.div_ceil(SLOT_SIZE);
                        slots_per_element * size * SLOT_SIZE
                    } else {
                        let elements_per_slot = SLOT_SIZE / es;
                        let num_slots = size.div_ceil(elements_per_slot);
                        num_slots * SLOT_SIZE
                    }
                });
                self.insert(
                    type_id,
                    format!("{element_internal}[{size}]"),
                    element_canonical.map(|c| format!("{c}[{size}]")),
                    storage,
                );
            }
            Type::Mapping {
                key_type_id,
                value_type_id,
            } => {
                let key_type_id = *key_type_id;
                let value_type_id = *value_type_id;
                self.compute(key_type_id);
                self.compute(value_type_id);
                let key_internal = self.data[&key_type_id].internal_name.clone();
                let value_internal = self.data[&value_type_id].internal_name.clone();
                self.insert(
                    type_id,
                    format!("mapping({key_internal} => {value_internal})"),
                    None,
                    Some(SLOT_SIZE),
                );
            }
            Type::Tuple { types } => {
                // Bounded by the tuple's arity (small in practice); we need an
                // owned copy so we can iterate while making `&mut self` calls.
                let element_type_ids: Vec<TypeId> = types.clone();
                for element_type_id in &element_type_ids {
                    self.compute(*element_type_id);
                }
                let inner = element_type_ids
                    .iter()
                    .map(|element_type_id| self.data[element_type_id].internal_name.clone())
                    .collect::<Vec<_>>()
                    .join(",");
                self.insert(type_id, format!("({inner})"), None, None);
            }
            Type::Contract { definition_id } | Type::Interface { definition_id } => {
                let definition_id = *definition_id;
                let name = self.definition_canonical_name(definition_id);
                self.insert(
                    type_id,
                    name,
                    Some("address".to_string()),
                    Some(ADDRESS_BYTE_SIZE),
                );
            }
            Type::Enum { definition_id } => {
                let definition_id = *definition_id;
                let name = self.definition_canonical_name(definition_id);
                self.insert(type_id, name, Some("uint8".to_string()), Some(1));
            }
            Type::Struct { definition_id, .. } => {
                let definition_id = *definition_id;
                // Insert a placeholder entry first so a recursive `compute()`
                // (from a self-referential struct member) short-circuits at
                // the top. canonical/storage are still `None` at that point,
                // so the helpers below see `None` and break the cycle.
                let internal = self.definition_canonical_name(definition_id);
                self.insert(type_id, internal, None, None);

                let canonical = self.struct_canonical_name(definition_id);
                let storage = self.struct_storage_size(definition_id);
                let entry = self.data.get_mut(&type_id).unwrap();
                entry.canonical_name = canonical;
                entry.storage_size = storage;
            }
            Type::UserDefinedValue { definition_id } => {
                let definition_id = *definition_id;
                let internal = self.definition_canonical_name(definition_id);
                let target_type_id = {
                    let Definition::UserDefinedValueType(udvt) = self
                        .binder
                        .find_definition_by_id(definition_id)
                        .expect("definition in type exists")
                    else {
                        unreachable!("definition in user defined value type is not a UDVT");
                    };
                    udvt.target_type_id
                };
                let (canonical, storage) = if let Some(target_type_id) = target_type_id {
                    self.compute(target_type_id);
                    let target = &self.data[&target_type_id];
                    (target.canonical_name.clone(), target.storage_size)
                } else {
                    (None, None)
                };
                self.insert(type_id, internal, canonical, storage);
            }
        }
    }

    fn insert(
        &mut self,
        type_id: TypeId,
        internal_name: String,
        canonical_name: Option<String>,
        storage_size: Option<usize>,
    ) {
        self.data.insert(
            type_id,
            TypeData {
                internal_name,
                canonical_name,
                storage_size,
            },
        );
    }

    fn struct_canonical_name(&mut self, definition_id: NodeId) -> Option<String> {
        let member_node_ids = self.collect_struct_member_node_ids(definition_id);
        let mut fields = Vec::new();
        for member_id in member_node_ids {
            let member_type_id = self.binder.node_typing(member_id).as_type_id()?;
            self.compute(member_type_id);
            // `None` here is either "the member's canonical name itself is
            // None" (mapping, tuple, ...) or "we're in a cycle and the member's
            // canonical entry is still the placeholder". Either way the parent
            // struct has no canonical name.
            let field_canonical = self.data[&member_type_id].canonical_name.clone()?;
            fields.push(field_canonical);
        }
        Some(format!("({})", fields.join(",")))
    }

    fn struct_storage_size(&mut self, definition_id: NodeId) -> Option<usize> {
        let member_node_ids = self.collect_struct_member_node_ids(definition_id);
        let mut ptr: usize = 0;
        for member_id in member_node_ids {
            let member_type_id = self.binder.node_typing(member_id).as_type_id()?;
            self.compute(member_type_id);
            // Same cycle/no-storage handling as canonical above.
            let member_size = self.data[&member_type_id].storage_size?;
            let remaining_bytes = SLOT_SIZE - (ptr % SLOT_SIZE);
            if remaining_bytes < SLOT_SIZE && member_size >= remaining_bytes {
                ptr += remaining_bytes;
            }
            ptr += member_size;
        }
        // round up the final allocation to a full slot, because the next variable
        // needs to start at the next slot anyway
        Some(ptr.div_ceil(SLOT_SIZE) * SLOT_SIZE)
    }

    fn collect_struct_member_node_ids(&self, definition_id: NodeId) -> Vec<NodeId> {
        let Definition::Struct(struct_) = self
            .binder
            .find_definition_by_id(definition_id)
            .expect("definition in type exists")
        else {
            unreachable!("definition in struct type is not a struct");
        };
        struct_.ir_node.members.iter().map(|m| m.id()).collect()
    }
}
