mod binder;
mod typing;

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdGenerator};
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::context::SemanticFile;

struct TestFile {
    id: String,
    ir_root: ir::SourceUnit,
}

impl SemanticFile for TestFile {
    fn id(&self) -> &str {
        &self.id
    }

    fn ir_root(&self) -> &ir::SourceUnit {
        &self.ir_root
    }

    fn resolved_import_by_node_id(&self, _node_id: NodeId) -> Option<&String> {
        None
    }
}

fn build_file(
    name: &str,
    contents: &str,
    id_generator: &mut NodeIdGenerator,
    language_version: LanguageVersion,
) -> TestFile {
    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(name, contents, language_version);

    assert!(
        diagnostics.is_empty(),
        "Parser diagnostics: {diagnostics:?}"
    );

    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build(name, &source_unit, &contents, id_generator);

    assert!(
        diagnostics.is_empty(),
        "IR builder diagnostics: {diagnostics:?}"
    );

    TestFile {
        id: name.to_string(),
        ir_root,
    }
}
