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
    #[allow(dead_code)]
    enclosing_type: Option<String>,
}

impl SolidityScopeBuilder {
    fn new(
        builder: &mut FileGraphBuilder<'_, KindTypes>,
        guard_symbol: &str,
        root_node: GraphHandle,
    ) -> Self {
        let scope_builder = ScopeGraphBuilder::new(builder, guard_symbol, root_node, None);
        Self {
            scope_builder,
            enclosing_type: None,
        }
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
        Self {
            scope_builder,
            enclosing_type: None,
        }
    }

    fn define_function(
        &mut self,
        builder: &mut metaslang_bindings::FileGraphBuilder<'_, KindTypes>,
        identifier: &str,
        #[allow(unused_variables)] parameters: &[&str],
        return_type: Option<&str>,
    ) {
        #[cfg(feature = "__private_backend_api")]
        let tag = function_built_in_tag(identifier, parameters).map(|tag| tag as u32);
        #[cfg(not(feature = "__private_backend_api"))]
        let tag = None;
        self.scope_builder
            .define_function(builder, identifier, return_type, tag);
    }

    fn define_field(
        &mut self,
        builder: &mut metaslang_bindings::FileGraphBuilder<'_, KindTypes>,
        identifier: &str,
        field_type: &str,
    ) {
        #[cfg(feature = "__private_backend_api")]
        let tag = field_built_in_tag(self.enclosing_type.as_deref(), identifier, field_type)
            .map(|tag| tag as u32);
        #[cfg(not(feature = "__private_backend_api"))]
        let tag = None;
        self.scope_builder
            .define_field(builder, identifier, field_type, tag);
    }

    fn define_type(
        &mut self,
        builder: &mut metaslang_bindings::FileGraphBuilder<'_, KindTypes>,
        identifier: &str,
    ) -> Self {
        #[cfg(feature = "__private_backend_api")]
        let tag = type_built_in_tag(identifier).map(|tag| tag as u32);
        #[cfg(not(feature = "__private_backend_api"))]
        let tag = None;
        let scope_builder = self.scope_builder.define_type(builder, identifier, tag);
        Self {
            scope_builder,
            enclosing_type: Some(identifier.to_owned()),
        }
    }
}

#[cfg(feature = "__private_backend_api")]
use super::built_ins::BuiltInTag;

#[cfg(feature = "__private_backend_api")]
fn function_built_in_tag(identifier: &str, parameters: &[&str]) -> Option<BuiltInTag> {
    match identifier {
        "require" => {
            if parameters.len() == 1 {
                Some(BuiltInTag::RequireBool)
            } else if parameters.len() == 2 {
                if parameters[1].starts_with("string") {
                    Some(BuiltInTag::RequireBoolString)
                } else {
                    Some(BuiltInTag::RequireBoolError)
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(feature = "__private_backend_api")]
fn field_built_in_tag(
    enclosing_type: Option<&str>,
    identifier: &str,
    _field_type: &str,
) -> Option<BuiltInTag> {
    match enclosing_type {
        None => None,
        Some("%MessageType") => match identifier {
            "sender" => Some(BuiltInTag::MsgSender),
            _ => None,
        },
        Some(_) => None,
    }
}

#[cfg(feature = "__private_backend_api")]
fn type_built_in_tag(_identifier: &str) -> Option<BuiltInTag> {
    None
}
