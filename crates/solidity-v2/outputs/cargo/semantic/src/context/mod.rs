use std::sync::Arc;

use file_node_mapper::FileNodeMapper;
use semantic_cache::SemanticCache;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::utils::strip_string_literal_quotes;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, Reference};
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_resolve_references,
};
use crate::types::{TypeId, TypeRegistry};

mod file_node_mapper;
mod semantic_cache;

pub const SLOT_SIZE: usize = 32;
pub(crate) const ADDRESS_BYTE_SIZE: usize = 20;
pub(crate) const SELECTOR_SIZE: usize = 4;

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
    cache: SemanticCache,
}

impl SemanticContext {
    pub fn build_from(language_version: LanguageVersion, files: &[impl SemanticFile]) -> Self {
        let mut binder = Binder::default();
        let mut types = TypeRegistry::default();

        p1_collect_definitions::run(files, &mut binder);
        p2_linearise_contracts::run(files, &mut binder);
        p3_type_definitions::run(files, &mut binder, &mut types, language_version);
        p4_resolve_references::run(files, &mut binder, &mut types, language_version);

        let file_node_mapper = FileNodeMapper::build_from(files);
        let cache = SemanticCache::build_from(&binder, &types);

        Self {
            binder,
            types,
            file_node_mapper,
            cache,
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

    pub fn find_contract_by_name(&self, name: &str) -> Option<ir::ContractDefinition> {
        self.binder().definitions().values().find_map(|definition| {
            let Definition::Contract(contract) = definition else {
                return None;
            };
            if definition.identifier().unparse() == name {
                Some(Arc::clone(&contract.ir_node))
            } else {
                None
            }
        })
    }
}

impl SemanticContext {
    pub fn file_id_from_node_id(&self, node_id: NodeId) -> &str {
        self.file_node_mapper.file_id_from_node_id(node_id)
    }

    /// Returns the pre-computed list of functions visible in the given
    /// contract's hierarchy (per C3 linearisation), with overrides resolved and
    /// sorted by name. Returns `None` if `contract_id` is not a contract
    /// definition (an entry exists for every contract, even when empty).
    pub fn linearised_functions(&self, contract_id: NodeId) -> Option<&[ir::FunctionDefinition]> {
        self.cache.linearised_functions(contract_id)
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

    /// Returns the internal (debug-ish) name for a type. The cache is built
    /// for every `TypeId` registered in the type registry, so this never fails
    /// for a `TypeId` obtained from this context.
    pub fn type_internal_name(&self, type_id: TypeId) -> &str {
        self.cache.type_internal_name(type_id)
    }

    /// Returns the canonical (ABI-encoded) name for a type, or `None` if the
    /// type has no canonical form (mappings, tuples, recursive structs, ...).
    pub fn type_canonical_name(&self, type_id: TypeId) -> Option<&str> {
        self.cache.type_canonical_name(type_id)
    }

    /// Returns the storage byte size for a type, or `None` if the type doesn't
    /// live in storage (literals, tuples, void).
    pub fn storage_size_of_type_id(&self, type_id: TypeId) -> Option<usize> {
        self.cache.type_storage_size(type_id)
    }
}
