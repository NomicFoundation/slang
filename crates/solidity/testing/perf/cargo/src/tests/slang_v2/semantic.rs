use std::collections::HashMap;
use std::rc::Rc;

use slang_solidity_v2_ir::interner::Interner;
use slang_solidity_v2_ir::ir::{self, NodeId};
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::{
    extract_import_paths_from_source_unit, FileId, SemanticContext, SemanticFile,
};

use crate::dataset::SolidityProject;

pub struct File {
    id: FileId,
    ir_root: ir::SourceUnit,
    resolved_imports: HashMap<NodeId, FileId>,
}

impl File {
    pub fn add_resolved_import(&mut self, node_id: NodeId, target_file_id: FileId) {
        self.resolved_imports.insert(node_id, target_file_id);
    }
}

impl SemanticFile for File {
    fn file_id(&self) -> FileId {
        self.id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, node_id: ir::NodeId) -> Option<FileId> {
        self.resolved_imports.get(&node_id).copied()
    }
}

pub fn setup(project: &str) -> (&'static SolidityProject, Interner, Vec<File>) {
    let (project, sources) = super::ir_builder::setup(project);
    let (mut interner, ir_source_units) = super::ir_builder::test(project, sources);
    let files = build_files(project, ir_source_units, &mut interner);
    (project, interner, files)
}

pub fn build_files(
    project: &'static SolidityProject,
    ir_source_units: Vec<ir::SourceUnit>,
    interner: &mut Interner,
) -> Vec<File> {
    ir_source_units
        .into_iter()
        .zip(project.sources.keys())
        .map(|(ir_root, source_id)| {
            let file_id = interner.intern(source_id);
            let import_paths = extract_import_paths_from_source_unit(&ir_root, interner);
            let resolved_imports = import_paths
                .iter()
                .filter_map(|(node_id, import_path)| {
                    let resolved_file_id = project
                        .import_resolver
                        .resolve_import(source_id, import_path)?;
                    Some((*node_id, interner.intern(&resolved_file_id)))
                })
                .collect();
            File {
                id: file_id,
                ir_root,
                resolved_imports,
            }
        })
        .collect()
}

pub fn run(
    project: &'static SolidityProject,
    interner: Interner,
    files: Vec<File>,
) -> SemanticContext {
    test(project, interner, files)
}

pub fn test(
    project: &'static SolidityProject,
    interner: Interner,
    files: Vec<impl SemanticFile>,
) -> SemanticContext {
    let language_version = super::parser::parse_version(project);
    SemanticContext::build_from(language_version, &files, &Rc::new(interner))
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
