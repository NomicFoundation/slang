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
pub use storage_layout::{
    StorageLayoutBuilder, StorageMember, StoragePosition, StorageSize, StorageTypeKind,
    StorageTypeLayout, StorageTypeTable,
};
pub use types::{AbiNameError, AbiTypeSpelling};

use crate::binder::{Binder, BinderCapacities, Definition, Reference};
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_compute_linearisations,
    p5_resolve_references, p6_resolve_yul, p7_contract_properties, p8_code_analysis,
};
use crate::types::TypeRegistry;

mod contract_data;
pub(crate) mod dispatch;
mod file_node_mapper;
mod storage_layout;
mod types;

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
}
