use std::cmp::Ordering;
use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::{ADDRESS_BYTE_SIZE, SELECTOR_SIZE, SLOT_SIZE};
use crate::types::{Type, TypeId, TypeRegistry};

/// Cache of semantic information about contracts and other Solidity elements,
/// derived from the binder and type registry after the semantic passes have
/// run. Built once and stored on the `SemanticContext` so downstream consumers
/// can look up commonly needed analyses without recomputing them.
pub(super) struct SemanticCache {
    /// For each contract definition, the functions visible following the C3
    /// linearisation: overridden functions are dropped and the list is sorted
    /// by name.
    linearised_functions: HashMap<NodeId, Vec<ir::FunctionDefinition>>,

    /// `type_internal_name` result for every registered type.
    type_internal_names: HashMap<TypeId, String>,

    /// `type_canonical_name` result for every registered type. Inner `None`
    /// is a valid outcome (e.g. mappings, recursive structs).
    type_canonical_names: HashMap<TypeId, Option<String>>,

    /// `storage_size_of_type_id` result for every registered type. Inner
    /// `None` signals a type that doesn't live in storage (literals, void,
    /// tuples).
    type_storage_sizes: HashMap<TypeId, Option<usize>>,
}

impl SemanticCache {
    pub(super) fn build_from(binder: &Binder, types: &TypeRegistry) -> Self {
        let mut linearised_functions = HashMap::new();
        for (contract_id, definition) in binder.definitions() {
            if !matches!(definition, Definition::Contract(_)) {
                continue;
            }
            let functions = compute_linearised_functions(binder, types, *contract_id);
            linearised_functions.insert(*contract_id, functions);
        }

        let type_caches = TypeCacheBuilder::new(binder, types).build();

        Self {
            linearised_functions,
            type_internal_names: type_caches.internal_names,
            type_canonical_names: type_caches.canonical_names,
            type_storage_sizes: type_caches.storage_sizes,
        }
    }

    pub(super) fn linearised_functions(
        &self,
        contract_id: NodeId,
    ) -> Option<&[ir::FunctionDefinition]> {
        self.linearised_functions
            .get(&contract_id)
            .map(Vec::as_slice)
    }

    pub(super) fn type_internal_name(&self, type_id: TypeId) -> &str {
        self.type_internal_names
            .get(&type_id)
            .map(String::as_str)
            .expect("type_id is registered in the type registry")
    }

    pub(super) fn type_canonical_name(&self, type_id: TypeId) -> Option<&str> {
        self.type_canonical_names
            .get(&type_id)
            .expect("type_id is registered in the type registry")
            .as_deref()
    }

    pub(super) fn type_storage_size(&self, type_id: TypeId) -> Option<usize> {
        self.type_storage_sizes
            .get(&type_id)
            .copied()
            .expect("type_id is registered in the type registry")
    }
}

fn compute_linearised_functions(
    binder: &Binder,
    types: &TypeRegistry,
    contract_id: NodeId,
) -> Vec<ir::FunctionDefinition> {
    let Some(linearised_bases) = binder.get_linearised_bases(contract_id) else {
        return Vec::new();
    };

    let mut functions: Vec<ir::FunctionDefinition> = Vec::new();
    for base_id in linearised_bases {
        // TODO(validation) SDR[3]: we don't pick up functions defined in
        // interfaces because they should be implemented in inheriting contracts,
        // but this is not yet enforced anywhere.
        let Some(Definition::Contract(base)) = binder.find_definition_by_id(*base_id) else {
            continue;
        };

        for member in &base.ir_node.members {
            let ir::ContractMember::FunctionDefinition(function) = member else {
                continue;
            };
            if !matches!(
                function.kind,
                ir::FunctionKind::Regular | ir::FunctionKind::Fallback | ir::FunctionKind::Receive
            ) {
                continue;
            }
            let overridden = functions
                .iter()
                .any(|existing| function_overrides(binder, types, existing, function));
            if !overridden {
                functions.push(function.clone());
            }
        }
    }

    functions.sort_by(|a, b| match (&a.name, &b.name) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => a.unparse().cmp(&b.unparse()),
    });
    functions
}

fn function_overrides(
    binder: &Binder,
    types: &TypeRegistry,
    function: &ir::FunctionDefinition,
    other: &ir::FunctionDefinition,
) -> bool {
    let name_matches = match (&function.name, &other.name) {
        (None, None) => function.kind == other.kind,
        (Some(name), Some(other_name)) => name.unparse() == other_name.unparse(),
        _ => false,
    };
    if !name_matches {
        return false;
    }
    let type_id = binder.node_typing(function.id()).as_type_id();
    let other_type_id = binder.node_typing(other.id()).as_type_id();
    match (type_id, other_type_id) {
        (Some(type_id), Some(other_type_id)) => {
            types.type_id_is_function_and_overrides(type_id, other_type_id)
        }
        _ => false,
    }
}

/// Builds all three per-type derivation caches in a single structural traversal
/// of the type registry. Sub-type derivations are resolved by looking them up
/// in the (partially built) cache, so each type is visited at most once.
struct TypeCacheBuilder<'a> {
    binder: &'a Binder,
    types: &'a TypeRegistry,
    internal_names: HashMap<TypeId, String>,
    canonical_names: HashMap<TypeId, Option<String>>,
    storage_sizes: HashMap<TypeId, Option<usize>>,
}

struct TypeCaches {
    internal_names: HashMap<TypeId, String>,
    canonical_names: HashMap<TypeId, Option<String>>,
    storage_sizes: HashMap<TypeId, Option<usize>>,
}

impl<'a> TypeCacheBuilder<'a> {
    fn new(binder: &'a Binder, types: &'a TypeRegistry) -> Self {
        Self {
            binder,
            types,
            internal_names: HashMap::new(),
            canonical_names: HashMap::new(),
            storage_sizes: HashMap::new(),
        }
    }

    fn build(mut self) -> TypeCaches {
        let type_ids: Vec<TypeId> = self.types.iter_types().map(|(id, _)| id).collect();
        for type_id in type_ids {
            self.compute(type_id);
        }
        TypeCaches {
            internal_names: self.internal_names,
            canonical_names: self.canonical_names,
            storage_sizes: self.storage_sizes,
        }
    }

    fn definition_canonical_name(&self, definition_id: NodeId) -> String {
        self.binder
            .find_definition_by_id(definition_id)
            .unwrap()
            .identifier()
            .unparse()
            .to_string()
    }

    /// Populates all three caches for `type_id`. Sub-type derivations are
    /// resolved by ensuring the sub-type is computed (recursive call), then
    /// looking up the cached values — there are no separate recursive paths
    /// per derivation.
    ///
    /// Cycle detection (only relevant for self-referential structs, which are
    /// invalid Solidity but guarded against defensively): `internal_name` is
    /// inserted at the top of the `Struct` arm before recursing into members,
    /// so a recursive `compute(self_type)` short-circuits via the
    /// `internal_names.contains_key` check. The cycled struct's `canonical_name`
    /// / `storage_size` aren't yet cached at that point, so the sub-type lookup
    /// inside `struct_canonical_name` / `struct_storage_size` returns `None`,
    /// which propagates up.
    fn compute(&mut self, type_id: TypeId) {
        if self.internal_names.contains_key(&type_id) {
            return;
        }
        match self.types.get_type_by_id(type_id) {
            Type::Address { .. } => self.insert_all(
                type_id,
                "address".to_string(),
                Some("address".to_string()),
                Some(ADDRESS_BYTE_SIZE),
            ),
            Type::Boolean => self.insert_all(
                type_id,
                "bool".to_string(),
                Some("bool".to_string()),
                Some(1),
            ),
            Type::ByteArray { width } => {
                let width = *width;
                let name = format!("bytes{width}");
                let storage = width.try_into().unwrap();
                self.insert_all(type_id, name.clone(), Some(name), Some(storage));
            }
            Type::Bytes { .. } => self.insert_all(
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
                self.insert_all(type_id, name.clone(), Some(name), Some(storage));
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
                self.insert_all(
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
                self.insert_all(type_id, name.clone(), Some(name), Some(storage));
            }
            Type::Literal(_) => {
                self.insert_all(type_id, "literal".to_string(), None, None);
            }
            Type::String { .. } => self.insert_all(
                type_id,
                "string".to_string(),
                Some("string".to_string()),
                Some(SLOT_SIZE),
            ),
            Type::Void => {
                self.insert_all(type_id, "void".to_string(), None, None);
            }

            Type::Array { element_type, .. } => {
                let element_type = *element_type;
                self.compute(element_type);
                let element_internal = self.internal_names[&element_type].clone();
                let element_canonical = self.canonical_names[&element_type].clone();
                self.insert_all(
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
                let element_internal = self.internal_names[&element_type].clone();
                let element_canonical = self.canonical_names[&element_type].clone();
                let element_size = self.storage_sizes[&element_type];
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
                self.insert_all(
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
                let key_internal = self.internal_names[&key_type_id].clone();
                let value_internal = self.internal_names[&value_type_id].clone();
                self.insert_all(
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
                    .map(|element_type_id| self.internal_names[element_type_id].clone())
                    .collect::<Vec<_>>()
                    .join(",");
                self.insert_all(type_id, format!("({inner})"), None, None);
            }
            Type::Contract { definition_id } | Type::Interface { definition_id } => {
                let definition_id = *definition_id;
                let name = self.definition_canonical_name(definition_id);
                self.insert_all(
                    type_id,
                    name,
                    Some("address".to_string()),
                    Some(ADDRESS_BYTE_SIZE),
                );
            }
            Type::Enum { definition_id } => {
                let definition_id = *definition_id;
                let name = self.definition_canonical_name(definition_id);
                self.insert_all(type_id, name, Some("uint8".to_string()), Some(1));
            }
            Type::Struct { definition_id, .. } => {
                let definition_id = *definition_id;
                // Insert internal_name first so a recursive `compute()` (from a
                // self-referential struct member) short-circuits at the top.
                // canonical/storage are not yet cached at that point, so the
                // helpers below see `None` and break the cycle.
                let internal = self.definition_canonical_name(definition_id);
                self.internal_names.insert(type_id, internal);

                let canonical = self.struct_canonical_name(definition_id);
                let storage = self.struct_storage_size(definition_id);
                self.canonical_names.insert(type_id, canonical);
                self.storage_sizes.insert(type_id, storage);
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
                    (
                        self.canonical_names[&target_type_id].clone(),
                        self.storage_sizes[&target_type_id],
                    )
                } else {
                    (None, None)
                };
                self.insert_all(type_id, internal, canonical, storage);
            }
        }
    }

    fn insert_all(
        &mut self,
        type_id: TypeId,
        internal: String,
        canonical: Option<String>,
        storage: Option<usize>,
    ) {
        self.internal_names.insert(type_id, internal);
        self.canonical_names.insert(type_id, canonical);
        self.storage_sizes.insert(type_id, storage);
    }

    fn struct_canonical_name(&mut self, definition_id: NodeId) -> Option<String> {
        let member_node_ids = self.collect_struct_member_node_ids(definition_id);
        let mut fields = Vec::new();
        for member_id in member_node_ids {
            let member_type_id = self.binder.node_typing(member_id).as_type_id()?;
            self.compute(member_type_id);
            // `None` here is either "the member's canonical name itself is
            // None" (mapping, tuple, ...) or "we're in a cycle and the member's
            // canonical entry hasn't been inserted yet". Either way the parent
            // struct has no canonical name.
            let field_canonical = self
                .canonical_names
                .get(&member_type_id)
                .cloned()
                .flatten()?;
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
            let member_size = self
                .storage_sizes
                .get(&member_type_id)
                .copied()
                .flatten()?;
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
