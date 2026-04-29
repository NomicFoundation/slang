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

fn build_file(name: &str, contents: &str, id_generator: &mut NodeIdGenerator) -> TestFile {
    let ParseOutput {
        source_unit,
        errors,
    } = Parser::parse(contents, LanguageVersion::V0_8_30);

    assert!(errors.is_empty(), "Parser errors: {errors:?}");

    let ir_root = ir::build(&source_unit, &contents, id_generator);
    TestFile {
        id: name.to_string(),
        ir_root,
    }
}
