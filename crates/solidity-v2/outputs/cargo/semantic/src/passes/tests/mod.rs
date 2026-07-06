mod binder;
mod typing;

use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::context::SemanticFile;

struct TestFile {
    id: FileId,
    ir_root: ir::SourceUnit,
}

impl SemanticFile for TestFile {
    fn id(&self) -> &FileId {
        &self.id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, _node_id: NodeId) -> Option<&FileId> {
        None
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
    } = ir::build(&file_id, &source_unit, &contents, id_generator);

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    TestFile {
        id: file_id,
        ir_root,
    }
}
