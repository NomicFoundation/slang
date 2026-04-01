use slang_solidity_v2_common::versions::LanguageVersion;

use crate::binder::Binder;
use crate::compilation::file::File;
use crate::ir::NodeId;
use crate::passes::{
    p1_collect_definitions, p2_linearise_contracts, p3_type_definitions, p4_resolve_references,
};
use crate::types::TypeRegistry;

pub struct SemanticContext {
    binder: Binder,
    types: TypeRegistry,
}

impl SemanticContext {
    pub fn build_from(language_version: LanguageVersion, files: &[File]) -> Self {
        let mut binder = Binder::new();
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

impl SemanticContext {
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
}
