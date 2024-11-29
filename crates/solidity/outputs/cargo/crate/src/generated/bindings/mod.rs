// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/binding_rules.rs"]
mod binding_rules;

#[path = "generated/built_ins.rs"]
pub mod built_ins;

use std::rc::Rc;

use semver::Version;

use crate::cst::KindTypes;

pub type BindingGraph = metaslang_bindings::BindingGraph<KindTypes>;
pub type Definition<'a> = metaslang_bindings::Definition<'a, KindTypes>;
pub type Reference<'a> = metaslang_bindings::Reference<'a, KindTypes>;
pub type BindingLocation = metaslang_bindings::BindingLocation<KindTypes>;
pub type UserFileLocation = metaslang_bindings::UserFileLocation<KindTypes>;

pub use metaslang_bindings::{BuiltInLocation, PathResolver};

use crate::parser::ParserInitializationError;

#[derive(thiserror::Error, Debug)]
pub enum BindingGraphInitializationError {
    #[error(transparent)]
    ParserInitialization(#[from] ParserInitializationError),
}

pub fn create_with_resolver(
    version: Version,
    resolver: Rc<dyn PathResolver<KindTypes>>,
) -> Result<BindingGraph, ParserInitializationError> {
    let mut binding_graph = BindingGraph::create(
        version.clone(),
        binding_rules::USER_BINDING_RULES_SOURCE,
        binding_rules::SYSTEM_BINDING_RULES_SOURCE,
        resolver,
    );

    crate::extensions::bindings::add_built_ins(&mut binding_graph, version)?;

    Ok(binding_graph)
}

#[cfg(feature = "__private_testing_utils")]
pub fn get_user_binding_rules() -> &'static str {
    binding_rules::USER_BINDING_RULES_SOURCE
}

#[cfg(feature = "__private_testing_utils")]
pub fn get_system_binding_rules() -> &'static str {
    binding_rules::SYSTEM_BINDING_RULES_SOURCE
}
