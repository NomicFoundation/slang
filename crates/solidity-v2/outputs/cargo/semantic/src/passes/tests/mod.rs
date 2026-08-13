mod alias_following;
mod binder;
mod contract_dependencies;
mod getter_overrides;
mod typing;
mod user_defined_operator_functions;

use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::context::{SemanticFile, extract_import_paths_from_source_unit};

struct TestFile {
    id: FileId,
    ir_root: ir::SourceUnit,
    /// Empty unless the file was built by [`build_files`], which is the only
    /// helper that knows about sibling files to resolve imports against.
    resolved_imports: Map<NodeId, FileId>,
}

impl SemanticFile for TestFile {
    fn id(&self) -> &FileId {
        &self.id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, node_id: NodeId) -> Option<&FileId> {
        self.resolved_imports.get(&node_id)
    }
}

fn build_file(
    file_id: FileId,
    contents: &str,
    id_generator: &mut NodeIdGenerator,
    language_version: LanguageVersion,
) -> TestFile {
    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&file_id, contents, language_version);

    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build(
        &file_id,
        &source_unit,
        &contents,
        language_version,
        id_generator,
    );

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    TestFile {
        id: file_id,
        ir_root,
        resolved_imports: Map::default(),
    }
}

/// Builds several files that may import each other, given as `(file name,
/// contents)` pairs. Import paths name the files verbatim, so a source
/// importing `"b.sol"` resolves to the entry named `b.sol`; a path naming no
/// entry stays unresolved, as it would for a missing file.
///
/// The files are built in the order given, which is the order the binder sees
/// their nodes in, and hence the order a file-scope lookup resolves them in.
fn build_files(sources: &[(&str, &str)], language_version: LanguageVersion) -> Vec<TestFile> {
    let mut id_generator = NodeIdGenerator::default();

    sources
        .iter()
        .map(|(name, contents)| {
            let mut file = build_file(
                (*name).into(),
                contents,
                &mut id_generator,
                language_version,
            );

            file.resolved_imports = extract_import_paths_from_source_unit(&file.ir_root)
                .into_iter()
                .filter(|(_, path)| sources.iter().any(|(name, _)| name == path))
                .map(|(node_id, path)| (node_id, path.as_str().into()))
                .collect();

            file
        })
        .collect()
}
