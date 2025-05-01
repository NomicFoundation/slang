use std::rc::Rc;

#[path = "../../generated/extensions/built_ins.rs"]
mod built_ins;

use built_ins::define_built_ins;
use metaslang_bindings::{FileGraphBuilder, GraphHandle, ScopeBuilder, ScopeGraphBuilder};
use metaslang_cst::text_index::TextIndex;
use semver::Version;

use crate::bindings::{BindingGraphBuilder, BindingGraphInitializationError};
use crate::cst::{KindTypes, Node, NonterminalKind};

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::unnecessary_wraps)]
pub fn add_built_ins(
    builder: &mut BindingGraphBuilder,
    version: Version,
) -> Result<(), BindingGraphInitializationError> {
    let empty_node = Node::nonterminal(NonterminalKind::SourceUnit, Vec::new())
        .into_nonterminal()
        .unwrap();
    let empty_cursor = Rc::clone(&empty_node).create_cursor(TextIndex::ZERO);
    let mut file_builder = builder.build_built_ins_file("built_ins.sol", empty_cursor);
    let root_node = file_builder.root_node();

    // __SLANG_SOLIDITY_BUILT_INS_SCOPE_GUARD__ keep in sync with binding rules
    let mut globals = SolidityScopeBuilder::new(&mut file_builder, "@@built-ins@@", root_node);

    define_built_ins(&mut file_builder, &mut globals, &version);

    Ok(())
}

struct SolidityScopeBuilder {
    scope_builder: ScopeGraphBuilder,
}

impl SolidityScopeBuilder {
    fn new(
        builder: &mut FileGraphBuilder<'_, KindTypes>,
        guard_symbol: &str,
        root_node: GraphHandle,
    ) -> Self {
        let scope_builder = ScopeGraphBuilder::new(builder, guard_symbol, root_node, None);
        Self { scope_builder }
    }
}

impl ScopeBuilder<KindTypes> for SolidityScopeBuilder {
    fn new_context(
        &self,
        builder: &mut metaslang_bindings::FileGraphBuilder<'_, KindTypes>,
        guard_symbol: &str,
    ) -> Self {
        let scope_builder =
            ScopeGraphBuilder::new_context(&self.scope_builder, builder, guard_symbol);
        Self { scope_builder }
    }

    fn define_function(
        &mut self,
        builder: &mut metaslang_bindings::FileGraphBuilder<'_, KindTypes>,
        identifier: &str,
        _parameters: &[&str],
        return_type: Option<&str>,
    ) {
        self.scope_builder
            .define_function(builder, identifier, return_type);
    }

    fn define_field(
        &mut self,
        builder: &mut metaslang_bindings::FileGraphBuilder<'_, KindTypes>,
        identifier: &str,
        field_type: &str,
    ) {
        self.scope_builder
            .define_field(builder, identifier, field_type);
    }

    fn define_type(
        &mut self,
        builder: &mut metaslang_bindings::FileGraphBuilder<'_, KindTypes>,
        identifier: &str,
    ) -> Self {
        let scope_builder = self.scope_builder.define_type(builder, identifier);
        Self { scope_builder }
    }
}
