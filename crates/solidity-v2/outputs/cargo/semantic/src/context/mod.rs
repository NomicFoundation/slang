use std::collections::HashSet;
use std::rc::Rc;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, Reference, Scope};
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_resolve_references,
};
use crate::types::{Type, TypeId, TypeRegistry};

/// Trait for files that can be used as input to the semantic analysis passes.
pub trait SemanticFile {
    /// Returns the file identifier.
    fn id(&self) -> &str;

    /// Returns the root IR node of the file.
    fn ir_root(&self) -> &ir::SourceUnit;

    /// Returns the resolved import target file ID for the given import node, if resolved.
    fn resolved_import_by_node_id(&self, node_id: NodeId) -> Option<&String>;
}

pub fn extract_import_paths_from_source_unit(
    source_unit: &ir::SourceUnit,
) -> Vec<(NodeId, String)> {
    let mut import_paths = Vec::new();

    for member in &source_unit.members {
        let ir::SourceUnitMember::ImportClause(import_clause) = member else {
            continue;
        };
        let (node_id, path) = match import_clause {
            ir::ImportClause::PathImport(path_import) => {
                (path_import.id(), path_import.path.unparse().to_owned())
            }
            ir::ImportClause::ImportDeconstruction(import_deconstruction) => (
                import_deconstruction.id(),
                import_deconstruction.path.unparse().to_owned(),
            ),
        };
        import_paths.push((node_id, path));
    }
    import_paths
}

pub struct SemanticContext {
    binder: Binder,
    types: TypeRegistry,
}

impl SemanticContext {
    pub fn build_from(language_version: LanguageVersion, files: &[impl SemanticFile]) -> Self {
        let mut binder = Binder::default();
        let mut types = TypeRegistry::default();

        p1_collect_definitions::run(files, &mut binder);
        p2_linearise_contracts::run(files, &mut binder);
        p3_type_definitions::run(files, &mut binder, &mut types);
        p4_resolve_references::run(files, &mut binder, &mut types, language_version);

        Self { binder, types }
    }

    // TODO: this should not be public
    pub fn binder(&self) -> &Binder {
        &self.binder
    }

    // TODO: this should not be public
    pub fn types(&self) -> &TypeRegistry {
        &self.types
    }

    pub fn all_definitions(&self) -> impl Iterator<Item = &Definition> + use<'_> {
        self.binder.definitions().values()
    }

    pub fn all_references(&self) -> impl Iterator<Item = &Reference> + use<'_> {
        self.binder.references().values()
    }

    pub fn find_contract_by_name(&self, name: &str) -> Option<ir::ContractDefinition> {
        self.binder().definitions().values().find_map(|definition| {
            let Definition::Contract(contract) = definition else {
                return None;
            };
            if definition.identifier().unparse() == name {
                Some(Rc::clone(&contract.ir_node))
            } else {
                None
            }
        })
    }
}

impl SemanticContext {
    pub fn node_id_to_file_id(&self, node_id: NodeId) -> Option<String> {
        let scope_id = self.binder().scope_id_for_node_id(node_id)?;
        let Scope::File(file_scope) = self.binder().get_scope_by_id(scope_id) else {
            return None;
        };
        Some(file_scope.file_id.clone())
    }

    pub fn resolve_reference_identifier_to_definition_id(&self, node_id: NodeId) -> Option<NodeId> {
        let reference = self
            .binder()
            .find_reference_by_identifier_node_id(node_id)?;
        self.binder()
            .follow_symbol_aliases(&reference.resolution)
            .as_definition_id()
    }

    pub fn resolve_reference_identifier_to_immediate_definition_id(
        &self,
        node_id: NodeId,
    ) -> Option<NodeId> {
        let reference = self
            .binder()
            .find_reference_by_identifier_node_id(node_id)?;
        reference.resolution.as_definition_id()
    }

    pub(crate) fn definition_canonical_name(&self, definition_id: NodeId) -> String {
        self.binder
            .find_definition_by_id(definition_id)
            .unwrap()
            .identifier()
            .unparse()
            .to_string()
    }

    pub fn type_internal_name(&self, type_id: TypeId) -> String {
        match self.types.get_type_by_id(type_id) {
            Type::Address { .. } => "address".to_string(),
            Type::Array { element_type, .. } => {
                format!(
                    "{element}[]",
                    element = self.type_internal_name(*element_type)
                )
            }
            Type::Boolean => "bool".to_string(),
            Type::ByteArray { width } => format!("bytes{width}"),
            Type::Bytes { .. } => "bytes".to_string(),
            Type::FixedPointNumber {
                signed,
                bits,
                precision_bits,
            } => format!(
                "{prefix}{bits}x{precision_bits}",
                prefix = if *signed { "fixed" } else { "ufixed" },
            ),
            Type::FixedSizeArray {
                element_type, size, ..
            } => {
                format!(
                    "{element}[{size}]",
                    element = self.type_internal_name(*element_type),
                )
            }
            Type::Function(_) => "function".to_string(),
            Type::Integer { signed, bits } => format!(
                "{prefix}{bits}",
                prefix = if *signed { "int" } else { "uint" }
            ),
            Type::Literal(_) => "literal".to_string(),
            Type::Mapping {
                key_type_id,
                value_type_id,
            } => format!(
                "mapping({key_type} => {value_type})",
                key_type = self.type_internal_name(*key_type_id),
                value_type = self.type_internal_name(*value_type_id)
            ),
            Type::String { .. } => "string".to_string(),
            Type::Tuple { types } => format!(
                "({types})",
                types = types
                    .iter()
                    .map(|type_id| self.type_internal_name(*type_id))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Type::Contract { definition_id }
            | Type::Enum { definition_id }
            | Type::Interface { definition_id }
            | Type::Struct { definition_id, .. }
            | Type::UserDefinedValue { definition_id } => {
                self.definition_canonical_name(*definition_id)
            }
            Type::Void => "void".to_string(),
        }
    }

    pub fn type_canonical_name(&self, type_id: TypeId) -> Option<String> {
        self.type_canonical_name_impl(type_id, &mut HashSet::new())
    }

    fn type_canonical_name_impl(
        &self,
        type_id: TypeId,
        visited_structs: &mut HashSet<NodeId>,
    ) -> Option<String> {
        match self.types.get_type_by_id(type_id) {
            Type::Address { .. }
            | Type::Boolean
            | Type::ByteArray { .. }
            | Type::Bytes { .. }
            | Type::FixedPointNumber { .. }
            | Type::Function(_)
            | Type::Integer { .. }
            | Type::String { .. } => Some(self.type_internal_name(type_id)),

            Type::Array { element_type, .. } => self
                .type_canonical_name_impl(*element_type, visited_structs)
                .map(|element| format!("{element}[]",)),
            Type::FixedSizeArray {
                element_type, size, ..
            } => self
                .type_canonical_name_impl(*element_type, visited_structs)
                .map(|element| format!("{element}[{size}]",)),

            Type::Contract { .. } | Type::Interface { .. } => Some("address".to_string()),

            Type::Enum { .. } => Some("uint8".to_string()),

            Type::Struct { definition_id, .. } => {
                // Recursive structs are not valid Solidity, but guard against cycles
                // to avoid unbounded recursion if malformed types reach this point.
                // TODO(validation): The recursion should be detected in the
                // `type_definition` pass.
                if !visited_structs.insert(*definition_id) {
                    return None;
                }
                let Definition::Struct(struct_) = self
                    .binder
                    .find_definition_by_id(*definition_id)
                    .expect("definition in type exists")
                else {
                    unreachable!("definition in struct type is not a struct");
                };
                let mut fields = Vec::new();
                for member in &struct_.ir_node.members {
                    let member_type_id = self.binder.node_typing(member.id()).as_type_id()?;
                    let field_type =
                        self.type_canonical_name_impl(member_type_id, visited_structs)?;
                    fields.push(field_type);
                }
                visited_structs.remove(definition_id);
                Some(format!("({fields})", fields = fields.join(",")))
            }

            Type::UserDefinedValue { definition_id } => {
                let Definition::UserDefinedValueType(udvt) = self
                    .binder
                    .find_definition_by_id(*definition_id)
                    .expect("definition in type exists")
                else {
                    unreachable!("definition in user defined value type is not a UDVT");
                };
                udvt.target_type_id
                    .and_then(|type_id| self.type_canonical_name_impl(type_id, visited_structs))
            }

            Type::Literal(_) | Type::Mapping { .. } | Type::Tuple { .. } | Type::Void => None,
        }
    }

    pub const SLOT_SIZE: usize = 32;
    pub(crate) const ADDRESS_BYTE_SIZE: usize = 20;
    pub(crate) const SELECTOR_SIZE: usize = 4;

    pub fn storage_size_of_type_id(&self, type_id: TypeId) -> Option<usize> {
        self.storage_size_of_type_id_impl(type_id, &mut HashSet::new())
    }

    fn storage_size_of_type_id_impl(
        &self,
        type_id: TypeId,
        visited_structs: &mut HashSet<NodeId>,
    ) -> Option<usize> {
        match self.types.get_type_by_id(type_id) {
            Type::Address { .. } | Type::Contract { .. } | Type::Interface { .. } => {
                Some(Self::ADDRESS_BYTE_SIZE)
            }
            Type::Boolean => Some(1),
            Type::FixedPointNumber { bits, .. } | Type::Integer { bits, .. } => {
                Some((bits.div_ceil(8)).try_into().unwrap())
            }
            Type::ByteArray { width } => Some((*width).try_into().unwrap()),
            Type::Enum { .. } => Some(1),
            Type::Bytes { .. } | Type::String { .. } => Some(Self::SLOT_SIZE),
            Type::Mapping { .. } => Some(Self::SLOT_SIZE),

            Type::Array { .. } => Some(Self::SLOT_SIZE),
            Type::FixedSizeArray {
                element_type, size, ..
            } => {
                let element_size =
                    self.storage_size_of_type_id_impl(*element_type, visited_structs)?;
                if element_size > Self::SLOT_SIZE {
                    let slots_per_element = element_size.div_ceil(Self::SLOT_SIZE);
                    Some(slots_per_element * size * Self::SLOT_SIZE)
                } else {
                    let elements_per_slot = Self::SLOT_SIZE / element_size;
                    let num_slots = size.div_ceil(elements_per_slot);
                    Some(num_slots * Self::SLOT_SIZE)
                }
            }

            Type::Function(function_type) => {
                if function_type.is_externally_visible() {
                    Some(Self::ADDRESS_BYTE_SIZE + Self::SELECTOR_SIZE)
                } else {
                    // NOTE: an internal function ref type is 8 bytes long, it's
                    // opaque and its meaning not documented
                    Some(8)
                }
            }
            Type::Struct { definition_id, .. } => {
                // Recursive structs are not valid Solidity, but guard against cycles
                // to avoid unbounded recursion if malformed types reach this point.
                // TODO(validation): The recursion should be detected in the
                // `type_definition` pass.
                if !visited_structs.insert(*definition_id) {
                    return None;
                }
                let Definition::Struct(struct_definition) =
                    self.binder.find_definition_by_id(*definition_id)?
                else {
                    return None;
                };
                let mut ptr: usize = 0;
                for member in &struct_definition.ir_node.members {
                    let member_type_id = self.binder.node_typing(member.id()).as_type_id()?;
                    let member_size =
                        self.storage_size_of_type_id_impl(member_type_id, visited_structs)?;
                    let remaining_bytes = Self::SLOT_SIZE - (ptr % Self::SLOT_SIZE);
                    if remaining_bytes < Self::SLOT_SIZE && member_size >= remaining_bytes {
                        ptr += remaining_bytes;
                    }
                    ptr += member_size;
                }
                visited_structs.remove(definition_id);
                // round up the final allocation to a full slot, because the
                // next variable needs to start at the next slot anyway
                ptr = ptr.div_ceil(Self::SLOT_SIZE) * Self::SLOT_SIZE;
                Some(ptr)
            }
            Type::UserDefinedValue { definition_id } => {
                let Definition::UserDefinedValueType(user_defined_value) =
                    self.binder.find_definition_by_id(*definition_id)?
                else {
                    return None;
                };
                self.storage_size_of_type_id_impl(
                    user_defined_value.target_type_id?,
                    visited_structs,
                )
            }

            Type::Literal(_) => None,
            Type::Tuple { .. } => None,
            Type::Void => None,
        }
    }
}
