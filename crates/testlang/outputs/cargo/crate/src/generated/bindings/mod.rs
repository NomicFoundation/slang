// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/binding_rules.rs"]
mod binding_rules;

#[path = "generated/built_ins.rs"]
mod built_ins;

use std::rc::Rc;

use semver::Version;

use crate::cst::KindTypes;

pub type BindingGraph = metaslang_bindings::BindingGraph<KindTypes>;
pub type Definition<'a> = metaslang_bindings::Definition<'a, KindTypes>;
pub type Reference<'a> = metaslang_bindings::Reference<'a, KindTypes>;
pub type BindingLocation = metaslang_bindings::BindingLocation<KindTypes>;
pub type UserFileLocation = metaslang_bindings::UserFileLocation<KindTypes>;

pub use metaslang_bindings::{BuiltInLocation, PathResolver};

pub fn create_with_resolver(
    version: Version,
    resolver: Rc<dyn PathResolver<KindTypes>>,
) -> BindingGraph {
    BindingGraph::create(version, binding_rules::BINDING_RULES_SOURCE, resolver)
}

#[cfg(feature = "__private_testing_utils")]
pub fn get_binding_rules() -> &'static str {
    binding_rules::BINDING_RULES_SOURCE
}

pub fn get_built_ins(version: &semver::Version) -> &'static str {
    built_ins::get_contents(version)
}
