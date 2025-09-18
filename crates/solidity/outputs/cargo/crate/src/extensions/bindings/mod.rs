use std::rc::Rc;

#[path = "built_ins.generated.rs"]
mod built_ins;

use built_ins::define_built_ins;
use metaslang_bindings::ScopeGraphBuilder;
use metaslang_cst::text_index::TextIndex;
use semver::Version;

use crate::bindings::BindingGraphBuilder;
use crate::cst::{Node, NonterminalKind};

#[allow(clippy::needless_pass_by_value)]
pub fn add_built_ins(builder: &mut BindingGraphBuilder, version: Version) {
    let empty_node = Node::nonterminal(NonterminalKind::SourceUnit, Vec::new())
        .into_nonterminal()
        .unwrap();
    let empty_cursor = Rc::clone(&empty_node).create_cursor(TextIndex::ZERO);

    let mut file_builder = builder.build_built_ins_file("built_ins.sol", empty_cursor);

    let root_node = file_builder.root_node();
    // __SLANG_SOLIDITY_BUILT_INS_SCOPE_GUARD__ keep in sync with binding rules
    let mut globals = ScopeGraphBuilder::new(&mut file_builder, "@@built-ins@@", root_node, None);

    define_built_ins(&mut file_builder, &mut globals, &version);
}
