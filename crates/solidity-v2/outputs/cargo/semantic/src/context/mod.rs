use std::fmt::Write;
use std::ops::Range;

pub use contract_data::ContractReference;
pub(crate) use contract_data::{ContractData, ContractLinearisations};
pub use dispatch::VirtualTarget;
pub(crate) use file_node_mapper::FileNodeMapper;
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::utils::strings::strip_string_literal_quotes;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;
pub(crate) use storage_layout::StorageAnalyzer;
pub use storage_layout::{
    StorageLayoutBuilder, StorageMember, StoragePosition, StorageSize, StorageTypeKind,
    StorageTypeLayout,
};

use crate::binder::{Binder, BinderCapacities, Definition, Reference};
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_compute_linearisations,
    p5_resolve_references, p6_resolve_yul, p7_contract_properties, p8_code_analysis,
};
use crate::types::{
    ArraySliceType, ArrayType, ByteArrayType, ContractType, DataLocation, EnumType, ErrorType,
    EventType, FixedPointNumberType, FixedSizeArrayType, FunctionType, FunctionTypeMutability,
    IntegerType, InterfaceType, LibraryType, MappingType, MetaType, StructType, TupleType, Type,
    TypeId, TypeRegistry, UserDefinedValueType, UserMetaType,
};

mod contract_data;
pub(crate) mod dispatch;
mod file_node_mapper;
mod storage_layout;

/// Trait for files that can be used as input to the semantic analysis passes.
pub trait SemanticFile {
    /// Returns the file identifier.
    fn id(&self) -> &FileId;

    /// Returns the root IR node of the file.
    fn ir_root(&self) -> &ir::SourceUnit;

    /// Returns the resolved import target file ID for the given import node, if resolved.
    fn resolved_import_by_node_id(&self, node_id: NodeId) -> Option<&FileId>;
}

/// One import declared by a source unit, as collected by
/// [`extract_imports_from_source_unit`].
#[derive(Clone, Debug)]
pub struct SourceUnitImport {
    /// The IR node declaring the import.
    pub node_id: NodeId,
    /// The path the import names, with the surrounding quotes stripped.
    pub path: String,
    /// The range of the path literal in the file's source text.
    pub range: Range<usize>,
}

/// Collects the imports declared in a source unit.
pub fn extract_imports_from_source_unit(source_unit: &ir::SourceUnit) -> Vec<SourceUnitImport> {
    let mut import_paths = Vec::new();

    for member in source_unit.members.iter() {
        let ir::SourceUnitMember::ImportClause(import_clause) = member else {
            continue;
        };
        let (node_id, path) = match import_clause {
            ir::ImportClause::PathImport(path_import) => (path_import.id(), &path_import.path),
            ir::ImportClause::ImportDeconstruction(import_deconstruction) => {
                (import_deconstruction.id(), &import_deconstruction.path)
            }
        };
        import_paths.push(SourceUnitImport {
            node_id,
            path: strip_string_literal_quotes(path.unparse()).to_owned(),
            range: path.range.clone(),
        });
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
        node_kinds: Option<&ir::NodeKindHistogram>,
        diagnostics: &mut DiagnosticCollection,
    ) -> Self {
        // When the IR node-kind histogram is available, pre-size the dominant
        // binder maps from it to avoid grow/rehash churn while populating them.
        let mut binder = match node_kinds {
            Some(node_kinds) => Binder::with_capacity(BinderCapacities::from(node_kinds)),
            None => Binder::default(),
        };
        let mut types = TypeRegistry::new(language_version);
        let file_node_mapper = FileNodeMapper::build_from(files);

        p1_collect_definitions::run(files, &mut binder, language_version, diagnostics);
        p2_linearise_contracts::run(files, &mut binder, diagnostics);
        p3_type_definitions::run(
            files,
            &mut binder,
            language_version,
            &mut types,
            &file_node_mapper,
            diagnostics,
        );

        let mut contract_data = p4_compute_linearisations::run(
            &binder,
            &types,
            language_version,
            &file_node_mapper,
            diagnostics,
        );
        p5_resolve_references::run(
            files,
            &mut binder,
            &mut types,
            &file_node_mapper,
            diagnostics,
        );
        p6_resolve_yul::run(
            &mut binder,
            language_version,
            evm_target,
            &types,
            &file_node_mapper,
            diagnostics,
        );
        p7_contract_properties::run(&binder, &mut contract_data, &types);
        p8_code_analysis::run(
            &binder,
            &contract_data,
            language_version,
            evm_target,
            &file_node_mapper,
            &types,
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
    pub fn file_id_from_node_id(&self, node_id: NodeId) -> &FileId {
        self.file_node_mapper.file_id_from_node_id(node_id)
    }

    /// Returns the pre-computed list of functions visible in the given
    /// contract's or interface's hierarchy (per C3 linearisation, interface
    /// bases included), with overrides resolved and sorted by name, led by the
    /// nameless fallback and then receive. A function overridden by a public
    /// state variable's getter is dropped from the list. An interface function
    /// no contract implements stays, so only an abstract contract or an
    /// interface lists one. `contract_id` must be a registered contract or
    /// interface definition.
    pub fn linearised_functions(&self, contract_id: NodeId) -> &[ir::FunctionDefinition] {
        self.contract_data.linearised_functions(contract_id)
    }

    /// For each contract, the contracts that its creation code embeds through
    /// `new`, `type(...).creationCode` or `type(...).runtimeCode`. Keyed by
    /// definition id and then by dependency id, mapped to the first expression
    /// embedding them. Contracts without dependencies have no entry, and
    /// libraries never have one.
    pub fn creation_bytecode_dependencies(
        &self,
    ) -> &SortedMap<NodeId, SortedMap<NodeId, ContractReference>> {
        self.contract_data.creation_bytecode_dependencies()
    }

    /// The same for the deployed code.
    pub fn deployed_bytecode_dependencies(
        &self,
    ) -> &SortedMap<NodeId, SortedMap<NodeId, ContractReference>> {
        self.contract_data.deployed_bytecode_dependencies()
    }

    /// The creation and deployed dependencies combined into one map, built on
    /// each call. When both embed the same contract, the creation expression is
    /// the one recorded.
    pub fn contract_dependencies(&self) -> SortedMap<NodeId, SortedMap<NodeId, ContractReference>> {
        self.contract_data.contract_dependencies()
    }

    /// The errors the given contract's or library's code can revert with,
    /// through `revert E(...)` or a call `E(...)`, from its creation code or
    /// any code its deployed entry points reach, each once, in a stable order.
    /// They can be declared anywhere, eg. in a library or at file level. Empty
    /// for an interface or a definition that reaches none.
    pub fn used_errors(&self, definition_id: NodeId) -> &[ir::ErrorDefinition] {
        self.contract_data.used_errors(definition_id)
    }

    /// The same for the events the given contract's or library's code can
    /// emit.
    pub fn used_events(&self, definition_id: NodeId) -> &[ir::EventDefinition] {
        self.contract_data.used_events(definition_id)
    }

    /// Returns the pre-computed list of state variables visible in the given
    /// contract's hierarchy, in storage-layout order (most-base first, then
    /// each contract's own variables), empty for an interface. `contract_id`
    /// must be a registered contract or interface definition.
    pub fn linearised_state_variables(
        &self,
        contract_id: NodeId,
    ) -> &[ir::StateVariableDefinition] {
        self.contract_data.linearised_state_variables(contract_id)
    }

    /// Returns the pre-computed list of errors visible in the given contract's
    /// or interface's hierarchy (including base contracts and interfaces, in
    /// reverse linearisation order). `contract_id` must be a registered contract
    /// or interface definition.
    pub fn linearised_errors(&self, contract_id: NodeId) -> &[ir::ErrorDefinition] {
        self.contract_data.linearised_errors(contract_id)
    }

    /// Returns the pre-computed list of events visible in the given contract's
    /// or interface's hierarchy (including base contracts and interfaces, in
    /// reverse linearisation order). `contract_id` must be a registered contract
    /// or interface definition.
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

    /// Qualifies a nested definition with its enclosing scope, as solc's
    /// `canonicalName` does: `L.S`, `C.E`.
    fn write_definition_canonical_name(&self, definition_id: NodeId, out: &mut String) {
        if let Some(enclosing) = self.binder.enclosing_definition_node_id(definition_id) {
            self.write_definition_canonical_name(enclosing, out);
            out.push('.');
        }
        out.push_str(
            self.binder
                .find_definition_by_id(definition_id)
                .unwrap()
                .identifier()
                .unparse(),
        );
    }

    pub fn type_internal_name(&self, type_id: TypeId) -> String {
        let mut name = String::new();
        self.write_type_internal_name(type_id, &mut name);
        name
    }

    fn write_type_internal_name(&self, type_id: TypeId, out: &mut String) {
        match self.types.get_type_by_id(type_id) {
            Type::Address(_) => out.push_str("address"),
            Type::Array(ArrayType { element_type, .. }) => {
                self.write_type_internal_name(*element_type, out);
                out.push_str("[]");
            }
            Type::ArraySlice(ArraySliceType { array_type_id }) => {
                self.write_type_internal_name(*array_type_id, out);
                out.push_str(" slice");
            }
            Type::Boolean => out.push_str("bool"),
            Type::ByteArray(ByteArrayType { width }) => write!(out, "bytes{width}").unwrap(),
            Type::Bytes(_) => out.push_str("bytes"),
            Type::FixedPointNumber(FixedPointNumberType {
                is_signed,
                bits,
                decimal_places,
            }) => write!(
                out,
                "{prefix}{bits}x{decimal_places}",
                prefix = if *is_signed { "fixed" } else { "ufixed" },
            )
            .unwrap(),
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
                self.write_type_internal_name(*element_type, out);
                write!(out, "[{size}]").unwrap();
            }
            Type::Function(_) => out.push_str("function"),
            Type::Integer(IntegerType { is_signed, bits }) => write!(
                out,
                "{prefix}{bits}",
                prefix = if *is_signed { "int" } else { "uint" }
            )
            .unwrap(),
            Type::Literal(_) => out.push_str("literal"),
            Type::Mapping(MappingType {
                key_type_id,
                value_type_id,
            }) => {
                out.push_str("mapping(");
                self.write_type_internal_name(*key_type_id, out);
                out.push_str(" => ");
                self.write_type_internal_name(*value_type_id, out);
                out.push(')');
            }
            Type::String(_) => out.push_str("string"),
            Type::Tuple(TupleType { types }) => {
                out.push('(');
                for (index, type_id) in types.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    self.write_type_internal_name(*type_id, out);
                }
                out.push(')');
            }
            Type::Contract(ContractType { definition_id })
            | Type::Enum(EnumType { definition_id })
            | Type::Interface(InterfaceType { definition_id })
            | Type::Library(LibraryType { definition_id })
            | Type::Struct(StructType { definition_id, .. })
            | Type::UserDefinedValue(UserDefinedValueType { definition_id }) => {
                self.write_definition_canonical_name(*definition_id, out);
            }
            Type::Error(ErrorType { definition_id }) => {
                out.push_str("error(");
                self.write_definition_canonical_name(*definition_id, out);
                out.push(')');
            }
            Type::Event(EventType { definition_id }) => {
                out.push_str("event(");
                self.write_definition_canonical_name(*definition_id, out);
                out.push(')');
            }
            // Meta-types print in solc's `type(T)` notation.
            Type::MetaType(MetaType { type_id }) => {
                out.push_str("type(");
                self.write_type_internal_name(*type_id, out);
                out.push(')');
            }
            Type::UserMetaType(UserMetaType { definition_id }) => {
                out.push_str("type(");
                self.write_definition_canonical_name(*definition_id, out);
                out.push(')');
            }
        }
    }

    /// The type as solc's `Type::toString(true)` spells it, which is the JSON ABI `internalType`
    /// and the storage layout label: a kind prefix on user-defined types (`struct C.S`,
    /// `enum C.E`, `contract I`), `address payable`, and function types with their parameters,
    /// mutability, `external` and returns; never a data location. Everything else spells as
    /// [`Self::type_internal_name`].
    pub fn type_abi_internal_name(&self, type_id: TypeId) -> String {
        // Sized on the ABI benchmarks: most names fit without the buffer regrowing.
        let mut name = String::with_capacity(32);
        self.write_type_abi_internal_name(type_id, &mut name);
        name
    }

    fn write_type_abi_internal_name(&self, type_id: TypeId, out: &mut String) {
        match self.types.get_type_by_id(type_id) {
            Type::Address(address) if address.is_payable => out.push_str("address payable"),
            Type::Array(ArrayType { element_type, .. }) => {
                self.write_type_abi_internal_name(*element_type, out);
                out.push_str("[]");
            }
            Type::Contract(ContractType { definition_id })
            | Type::Interface(InterfaceType { definition_id }) => {
                out.push_str("contract ");
                self.write_definition_canonical_name(*definition_id, out);
            }
            Type::Enum(EnumType { definition_id }) => {
                out.push_str("enum ");
                self.write_definition_canonical_name(*definition_id, out);
            }
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
                self.write_type_abi_internal_name(*element_type, out);
                write!(out, "[{size}]").unwrap();
            }
            Type::Function(function_type) => {
                self.write_function_type_abi_internal_name(function_type, out);
            }
            Type::Mapping(MappingType {
                key_type_id,
                value_type_id,
            }) => {
                out.push_str("mapping(");
                self.write_type_abi_internal_name(*key_type_id, out);
                out.push_str(" => ");
                self.write_type_abi_internal_name(*value_type_id, out);
                out.push(')');
            }
            Type::Struct(StructType { definition_id, .. }) => {
                out.push_str("struct ");
                self.write_definition_canonical_name(*definition_id, out);
            }
            _ => self.write_type_internal_name(type_id, out),
        }
    }

    fn write_type_abi_internal_names(&self, type_ids: &[TypeId], out: &mut String) {
        for (index, type_id) in type_ids.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            self.write_type_abi_internal_name(*type_id, out);
        }
    }

    /// `function (T1,T2) [pure|view|payable] [external] [returns (R1,R2)]`: `nonpayable` and
    /// `internal` are implied by their absence, as in solc.
    fn write_function_type_abi_internal_name(
        &self,
        function_type: &FunctionType,
        out: &mut String,
    ) {
        out.push_str("function (");
        self.write_type_abi_internal_names(&function_type.parameter_types, out);
        out.push(')');
        out.push_str(match function_type.mutability {
            FunctionTypeMutability::Pure => " pure",
            FunctionTypeMutability::View => " view",
            FunctionTypeMutability::Payable => " payable",
            FunctionTypeMutability::NonPayable => "",
        });
        if function_type.is_externally_visible() {
            out.push_str(" external");
        }
        if let Type::Tuple(TupleType { types }) =
            self.types.get_type_by_id(function_type.return_type)
        {
            // An empty tuple is not printed
            if !types.is_empty() {
                out.push_str(" returns (");
                self.write_type_abi_internal_names(types, out);
                out.push(')');
            }
        } else {
            out.push_str(" returns (");
            self.write_type_abi_internal_name(function_type.return_type, out);
            out.push(')');
        }
    }

    pub fn type_library_name(&self, type_id: TypeId) -> Option<String> {
        let mut name = self.type_library_element_name(type_id)?;
        if self.types.get_type_by_id(type_id).data_location() == Some(DataLocation::Storage) {
            name.push_str(" storage");
        }
        Some(name)
    }

    /// A user-defined value type is spelled as the type it wraps, under an
    /// array as well as on its own. A mapping keeps the wrapper's name
    /// (matching solc, which has no ABI spelling for one).
    fn type_library_element_name(&self, type_id: TypeId) -> Option<String> {
        let name = match self.types.get_type_by_id(type_id) {
            Type::UserDefinedValue(UserDefinedValueType { definition_id }) => self
                .type_internal_name(
                    self.binder
                        .user_defined_value_target_type_id(*definition_id)?,
                ),
            Type::Array(ArrayType { element_type, .. }) => {
                format!(
                    "{element}[]",
                    element = self.type_library_element_name(*element_type)?
                )
            }
            Type::FixedSizeArray(FixedSizeArrayType {
                element_type, size, ..
            }) => {
                format!(
                    "{element}[{size}]",
                    element = self.type_library_element_name(*element_type)?
                )
            }
            _ => self.type_internal_name(type_id),
        };
        Some(name)
    }

    pub fn storage_size_of_type_id(&self, type_id: TypeId) -> Option<StorageSize> {
        self.storage_analyzer().storage_size_of_type_id(type_id)
    }

    /// The type table of a storage layout: `roots` (the types of its state
    /// variables) and every type those refer to, each with `describe` applied
    /// to how it is laid out. `None` when one of them cannot be stored or
    /// overflows storage.
    pub fn storage_type_table<T>(
        &self,
        roots: impl IntoIterator<Item = TypeId>,
        describe: impl FnMut(TypeId, StorageTypeLayout) -> T,
    ) -> Option<SortedMap<TypeId, T>> {
        self.storage_analyzer().storage_type_table(roots, describe)
    }

    fn storage_analyzer(&self) -> StorageAnalyzer<'_> {
        StorageAnalyzer::new(&self.binder, &self.types)
    }
}
