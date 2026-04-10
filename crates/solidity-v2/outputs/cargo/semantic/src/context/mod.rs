use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeId};

use crate::binder::Binder;
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_resolve_references,
};
use crate::types::TypeRegistry;

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
}
