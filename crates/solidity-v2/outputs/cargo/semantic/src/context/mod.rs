pub(crate) use contract_data::{ContractData, ContractLinearisations};
pub(crate) use file_node_mapper::FileNodeMapper;
use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::utils::strip_string_literal_quotes;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, Reference};
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_compute_linearisations,
    p5_resolve_references, p6_resolve_yul, p7_code_analysis,
};
use crate::types::{
    ArrayType, ByteArrayType, ContractType, EnumType, FixedPointNumberType, FixedSizeArrayType,
    IntegerType, InterfaceType, LibraryType, MappingType, StructType, TupleType, Type, TypeId,
    TypeRegistry, UserDefinedValueType,
};

mod contract_data;
mod file_node_mapper;

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
        let (node_id, path_string_literal) = match import_clause {
            ir::ImportClause::PathImport(path_import) => {
                (path_import.id(), path_import.path.unparse())
            }
            ir::ImportClause::ImportDeconstruction(import_deconstruction) => (
                import_deconstruction.id(),
                import_deconstruction.path.unparse(),
            ),
        };
        let import_path = strip_string_literal_quotes(path_string_literal).to_owned();
        import_paths.push((node_id, import_path));
    }
    import_paths
}

pub struct SemanticContext {
    binder: Binder,
    types: TypeRegistry,
    file_node_mapper: FileNodeMapper,
    contract_data: ContractData,
}

impl SemanticContext {
    pub fn build_from(
        language_version: LanguageVersion,
        evm_target: EvmTarget,
        files: &[impl SemanticFile],
        diagnostics: &mut DiagnosticCollection,
    ) -> Self {
        let mut binder = Binder::default();
        let mut types = TypeRegistry::new(language_version);
        let file_node_mapper = FileNodeMapper::build_from(files);

        p1_collect_definitions::run(files, &mut binder, diagnostics);
        p2_linearise_contracts::run(files, &mut binder, diagnostics);
        p3_type_definitions::run(files, &mut binder, &mut types, diagnostics);

        let contract_data = p4_compute_linearisations::run(&binder, &types);
        p5_resolve_references::run(files, &mut binder, &mut types);
        p6_resolve_yul::run(&mut binder, &types, &file_node_mapper, diagnostics);
        p7_code_analysis::run(
            &binder,
            language_version,
            evm_target,
            &file_node_mapper,
            diagnostics,
        );

        // Now that all references have been collected and resolved across every
        // pass, finalize the binder by building the definition->references
        // reverse index.
        binder.update_definitions_to_references_index();

        Self {
            binder,
            types,
            file_node_mapper,
            contract_data,
        }
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

    /// Iterates over every contract definition in this compilation unit.
    pub fn all_contracts(&self) -> impl Iterator<Item = &ir::ContractDefinition> + use<'_> {
        self.contract_data.all_contracts()
    }

    pub fn find_contract_by_name<'a, 'b>(
        &'a self,
        name: &'b str,
    ) -> impl Iterator<Item = ir::ContractDefinition> + use<'a>
    where
        'b: 'a,
    {
        self.contract_data.find_contract_by_name(name)
    }
}

impl SemanticContext {
    pub fn file_id_from_node_id(&self, node_id: NodeId) -> &str {
        self.file_node_mapper.file_id_from_node_id(node_id)
    }

    /// Returns the pre-computed list of functions visible in the given
    /// contract's hierarchy (per C3 linearisation), with overrides resolved and
    /// sorted by name. `contract_id` must be a registered contract definition.
    pub fn linearised_functions(&self, contract_id: NodeId) -> &[ir::FunctionDefinition] {
        self.contract_data.linearised_functions(contract_id)
    }

    /// Returns the pre-computed list of state variables visible in the given
    /// contract's hierarchy, in storage-layout order (most-base first, then
    /// each contract's own variables). `contract_id` must be a registered
    /// contract definition.
    pub fn linearised_state_variables(
        &self,
        contract_id: NodeId,
    ) -> &[ir::StateVariableDefinition] {
        self.contract_data.linearised_state_variables(contract_id)
    }

    /// Returns the pre-computed list of errors visible in the given contract's
    /// hierarchy (including base contracts and interfaces, in reverse
    /// linearisation order). `contract_id` must be a registered contract
    /// definition.
    pub fn linearised_errors(&self, contract_id: NodeId) -> &[ir::ErrorDefinition] {
        self.contract_data.linearised_errors(contract_id)
    }

    /// Returns the pre-computed list of events visible in the given contract's
    /// hierarchy (including base contracts and interfaces, in reverse
    /// linearisation order). `contract_id` must be a registered contract
    /// definition.
    pub fn linearised_events(&self, contract_id: NodeId) -> &[ir::EventDefinition] {
        self.contract_data.linearised_events(contract_id)
    }

    pub fn resolve_reference_identifier_to_definition_id(&self, node_id: NodeId) -> Option<NodeId> {
        let reference = self
            .binder()
            .find_reference_by_identifier_node_id(node_id)?;
        self.binder()
            .follow_symbol_aliases(reference.resolution.clone())
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
            Type::Address(_) => "address".to_string(),
            Type::Array(ArrayType { element_type, .. }) => {
                format!(
                    "{element}[]",
                    element = self.type_internal_name(*element_type)
                )
            }
            Type::Boolean => "bool".to_string(),
            Type::ByteArray(ByteArrayType { width }) => format!("bytes{width}"),
            Type::Bytes(_) => "bytes".to_string(),
            Type::FixedPointNumber(FixedPointNumberType {
                is_signed,
                bits,
                decimal_places,
            }) => format!(
                "{prefix}{bits}x{decimal_places}",
                prefix = if *is_signed { "fixed" } else { "ufixed" },
            ),
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
                format!(
                    "{element}[{size}]",
                    element = self.type_internal_name(*element_type),
                )
            }
            Type::Function(_) => "function".to_string(),
            Type::Integer(IntegerType { is_signed, bits }) => format!(
                "{prefix}{bits}",
                prefix = if *is_signed { "int" } else { "uint" }
            ),
            Type::Literal(_) => "literal".to_string(),
            Type::Mapping(MappingType {
                key_type_id,
                value_type_id,
            }) => format!(
                "mapping({key_type} => {value_type})",
                key_type = self.type_internal_name(*key_type_id),
                value_type = self.type_internal_name(*value_type_id)
            ),
            Type::String(_) => "string".to_string(),
            Type::Tuple(TupleType { types }) => format!(
                "({types})",
                types = types
                    .iter()
                    .map(|type_id| self.type_internal_name(*type_id))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Type::Contract(ContractType { definition_id })
            | Type::Enum(EnumType { definition_id })
            | Type::Interface(InterfaceType { definition_id })
            | Type::Library(LibraryType { definition_id })
            | Type::Struct(StructType { definition_id, .. })
            | Type::UserDefinedValue(UserDefinedValueType { definition_id }) => {
                self.definition_canonical_name(*definition_id)
            }
            Type::Void => "void".to_string(),
        }
    }

    pub const SLOT_SIZE: usize = 32;
    pub(crate) const ADDRESS_BYTE_SIZE: usize = 20;
    pub(crate) const SELECTOR_SIZE: usize = 4;

    pub fn storage_size_of_type_id(&self, type_id: TypeId) -> Option<usize> {
        self.storage_size_of_type_id_impl(type_id, &mut Set::default())
    }

    fn storage_size_of_type_id_impl(
        &self,
        type_id: TypeId,
        visited_structs: &mut Set<NodeId>,
    ) -> Option<usize> {
        match self.types.get_type_by_id(type_id) {
            Type::Address(_) | Type::Contract(_) | Type::Interface(_) => {
                Some(Self::ADDRESS_BYTE_SIZE)
            }
            Type::Boolean => Some(1),
            Type::FixedPointNumber(FixedPointNumberType { bits, .. })
            | Type::Integer(IntegerType { bits, .. }) => {
                Some((bits.div_ceil(8)).try_into().unwrap())
            }
            Type::ByteArray(ByteArrayType { width }) => Some((*width).try_into().unwrap()),
            Type::Enum(_) => Some(1),
            Type::Bytes(_) | Type::String(_) => Some(Self::SLOT_SIZE),
            Type::Mapping(_) => Some(Self::SLOT_SIZE),

            Type::Array(_) => Some(Self::SLOT_SIZE),
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
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
            Type::Struct(StructType { definition_id, .. }) => {
                // Recursive structs are not valid Solidity, but guard against cycles
                // to avoid unbounded recursion if malformed types reach this point.
                // TODO(validation) SDR[19]: The recursion should be detected in the
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
                    if remaining_bytes < Self::SLOT_SIZE && member_size > remaining_bytes {
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
            Type::UserDefinedValue(UserDefinedValueType { definition_id }) => {
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

            Type::Library(_) | Type::Literal(_) | Type::Tuple(_) | Type::Void => None,
        }
    }
}
