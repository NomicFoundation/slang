use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};
use slang_solidity_v2_semantic::binder::{self, Resolution};
use slang_solidity_v2_semantic::context::{
    extract_import_paths_from_source_unit, SemanticContext, SemanticFile,
};

use crate::dataset::SolidityProject;
use crate::tests::slang_v2::common::{parse_evm_target, parse_version};

#[derive(Clone)]
pub struct File {
    id: String,
    ir_root: ir::SourceUnit,
    resolved_imports: Map<NodeId, String>,
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

pub struct Input {
    pub(crate) project: &'static SolidityProject,
    pub(crate) files: Vec<File>,
    pub(crate) id_generator: NodeIdGenerator,
}

pub type Output = SemanticContext;

pub fn setup(project: &str) -> Input {
    let payload = super::ir_builder::setup(project);
    let project = payload.project;
    let ir_builder_output = super::ir_builder::test(payload);
    let id_generator = ir_builder_output.id_generator;
    let files = build_files(project, ir_builder_output.ir_source_units);
    Input {
        project,
        files,
        id_generator,
    }
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
                    let resolved_file_id = project
                        .import_resolver
                        .resolve_import(file_id, import_path)
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

pub fn run(input: Input) -> Output {
    test(input)
}

pub fn test(input: Input) -> Output {
    let language_version = parse_version(input.project);
    let evm_target = parse_evm_target(input.project);
    let mut diagnostics = DiagnosticCollection::default();
    let histogram = input.id_generator.histogram();
    let semantic = SemanticContext::build_from(
        language_version,
        evm_target,
        &input.files,
        Some(histogram),
        &mut diagnostics,
    );
    assert!(
        diagnostics.is_empty(),
        "Semantic diagnostics: {diagnostics:?}"
    );
    semantic
}

pub fn count_contracts(output: &Output) -> usize {
    output
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

pub fn count_resolved_references(output: &Output) -> usize {
    let references = output.binder().references();
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
