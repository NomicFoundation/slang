use std::collections::HashMap;

use slang_solidity_v2_ir::ir::{self, NodeId};
use slang_solidity_v2_semantic::binder;
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

    fn resolved_import_by_node_id(&self, node_id: ir::NodeId) -> Option<&String> {
        self.resolved_imports.get(&node_id)
    }
}

pub fn setup(project: &str) -> (&'static SolidityProject, Vec<File>) {
    let (project, sources) = super::ir_builder::setup(project);
    let ir_source_units = super::ir_builder::test(project, sources);
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
                .filter_map(|(node_id, import_path)| {
                    let resolved_file_id = project
                        .import_resolver
                        .resolve_import(file_id, import_path)?;
                    Some((*node_id, resolved_file_id))
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

pub fn run(project: &'static SolidityProject, files: Vec<File>) {
    test(project, files);
}

pub fn test(project: &'static SolidityProject, files: Vec<impl SemanticFile>) -> SemanticContext {
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
