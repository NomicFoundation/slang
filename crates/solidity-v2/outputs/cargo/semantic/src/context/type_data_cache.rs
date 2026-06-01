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
    size_in_storage: Option<usize>,
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

    pub(super) fn size_in_storage(&self, type_id: TypeId) -> Option<usize> {
        self.get(type_id).size_in_storage
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
        for (type_id, _) in self.types.iter_types() {
            self.compute_type_data(type_id);
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
    #[allow(clippy::too_many_lines)]
    fn compute_type_data(&mut self, type_id: TypeId) {
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
            Type::Literal(kind) => {
                let name = match kind {
                    crate::types::LiteralKind::Integer { value }
                    | crate::types::LiteralKind::HexInteger { value, .. } => {
                        format!("int_const {value}")
                    }
                    crate::types::LiteralKind::Rational { value } => {
                        format!(
                            "rational {numer}/{denom}",
                            numer = value.numer(),
                            denom = value.denom()
                        )
                    }
                    crate::types::LiteralKind::HexString { .. }
                    | crate::types::LiteralKind::String { .. } => "literal_string".to_string(),
                    crate::types::LiteralKind::Address => "address_const".to_string(),
                };
                self.insert(type_id, name, None, None);
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
                self.compute_type_data(element_type);
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
                self.compute_type_data(element_type);
                let element = &self.data[&element_type];
                let element_internal = element.internal_name.clone();
                let element_canonical = element.canonical_name.clone();
                let element_size = element.size_in_storage;
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
                self.compute_type_data(key_type_id);
                self.compute_type_data(value_type_id);
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
                for element_type_id in types {
                    self.compute_type_data(*element_type_id);
                }
                let inner = types
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
                // Insert a placeholder entry first so a recursive
                // `compute_type_data()` (from a self-referential struct member)
                // short-circuits at the top. canonical/storage are still `None`
                // at that point, so the helpers below see `None` and break the
                // cycle.
                let internal = self.definition_canonical_name(definition_id);
                self.insert(type_id, internal, None, None);

                if let Some(member_type_ids) = self.compute_struct_member_type_ids(definition_id) {
                    let canonical = self.struct_canonical_name(&member_type_ids);
                    let storage = self.struct_storage_size(&member_type_ids);
                    let entry = self.data.get_mut(&type_id).unwrap();
                    entry.canonical_name = canonical;
                    entry.size_in_storage = storage;
                }
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
                    self.compute_type_data(target_type_id);
                    let target = &self.data[&target_type_id];
                    (target.canonical_name.clone(), target.size_in_storage)
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
                size_in_storage: storage_size,
            },
        );
    }

    fn struct_canonical_name(&mut self, member_type_ids: &[TypeId]) -> Option<String> {
        let mut fields = Vec::new();
        for member_type_id in member_type_ids {
            // `None` here is either "the member's canonical name itself is
            // None" (mapping, tuple, ...) or "we're in a cycle and the member's
            // canonical entry is still the placeholder". Either way the parent
            // struct has no canonical name.
            let field_canonical = self.data[member_type_id].canonical_name.clone()?;
            fields.push(field_canonical);
        }
        Some(format!("({})", fields.join(",")))
    }

    fn struct_storage_size(&mut self, member_type_ids: &[TypeId]) -> Option<usize> {
        let mut ptr: usize = 0;
        for member_type_id in member_type_ids {
            // Same cycle/no-storage handling as canonical above.
            let member_size = self.data[member_type_id].size_in_storage?;
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

    /// Recurse into a struct members and compute the data of their types,
    /// returning the types. Returns `None` if the type of any member cannot be
    /// resolved.
    fn compute_struct_member_type_ids(&mut self, definition_id: NodeId) -> Option<Vec<TypeId>> {
        let Definition::Struct(struct_) = self
            .binder
            .find_definition_by_id(definition_id)
            .expect("definition in type exists")
        else {
            unreachable!("definition in struct type is not a struct");
        };
        let mut member_type_ids = Vec::new();
        for member in &struct_.ir_node.members {
            let member_type_id = self.binder.node_typing(member.id()).as_type_id()?;
            self.compute_type_data(member_type_id);
            member_type_ids.push(member_type_id);
        }
        Some(member_type_ids)
    }
}
