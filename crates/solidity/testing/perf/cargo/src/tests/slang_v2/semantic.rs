use std::collections::HashMap;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder::{self, Resolution};
use slang_solidity_v2_semantic::context::{
    extract_import_paths_from_source_unit, SemanticContext, SemanticFile,
};

use crate::dataset::SolidityProject;

pub struct File {
    id: String,
    ir_root: ir::SourceUnit,
    resolved_imports: HashMap<NodeId, String>,
}

impl File {
    pub fn add_resolved_import(&mut self, node_id: NodeId, target_file_id: String) {
        self.resolved_imports.insert(node_id, target_file_id);
    }
}

impl SemanticFile for File {
    fn id(&self) -> &str {
        &self.id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, node_id: NodeId) -> Option<&String> {
        self.resolved_imports.get(&node_id)
    }
}

pub fn setup(project: &str) -> (&'static SolidityProject, Vec<File>) {
    let payload = super::ir_builder::setup(project);
    let project = payload.0;
    let ir_source_units = super::ir_builder::test(payload);
    let files = build_files(project, ir_source_units);
    (project, files)
}

pub fn build_files(
    project: &'static SolidityProject,
    ir_source_units: Vec<ir::SourceUnit>,
) -> Vec<File> {
    ir_source_units
        .into_iter()
        .zip(project.sources.keys())
        .map(|(ir_root, file_id)| {
            let import_paths = extract_import_paths_from_source_unit(&ir_root);
            let resolved_imports = import_paths
                .iter()
                .map(|(node_id, import_path)| {
                    // Import paths come from the IR as the unparsed string
                    // literal, including the surrounding quotes; strip them
                    // before delegating to the resolver (which expects the
                    // unquoted path).
                    let stripped = import_path
                        .strip_prefix(|c: char| matches!(c, '"' | '\''))
                        .and_then(|p| p.strip_suffix(|c: char| matches!(c, '"' | '\'')))
                        .expect("import path should be a quoted string literal")
                        .trim();
                    let resolved_file_id = project
                        .import_resolver
                        .resolve_import(file_id, stripped)
                        .expect("files to be resolved");
                    (*node_id, resolved_file_id)
                })
                .collect();
            File {
                id: file_id.to_owned(),
                ir_root,
                resolved_imports,
            }
        })
        .collect()
}

pub fn run(project: &'static SolidityProject, files: Vec<File>) -> SemanticContext {
    test((project, files))
}

pub fn test(
    (project, files): (&'static SolidityProject, Vec<impl SemanticFile>),
) -> SemanticContext {
    let language_version = super::parser::parse_version(project);
    SemanticContext::build_from(language_version, &files)
}

pub fn count_contracts(semantic: &SemanticContext) -> usize {
    semantic
        .binder()
        .definitions()
        .values()
        .filter(|definition| {
            matches!(
                definition,
                binder::Definition::Contract(_)
                    | binder::Definition::Interface(_)
                    | binder::Definition::Library(_)
            )
        })
        .count()
}

pub fn count_resolved_references(semantic: &SemanticContext) -> usize {
    let references = semantic.binder().references();
    // This is a bit stronger than just counting, but it's good to check that these
    // projects are fully resolved
    for reference in references.values() {
        assert!(
            !matches!(reference.resolution, Resolution::Unresolved),
            "unresolved reference at node_id={:?}",
            reference.node_id(),
        );
    }
    references.len()
}
