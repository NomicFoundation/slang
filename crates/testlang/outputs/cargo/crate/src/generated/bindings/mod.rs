// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/binding_rules.rs"]
mod binding_rules;

#[path = "generated/built_ins.rs"]
mod built_ins;

use std::sync::Arc;

use metaslang_bindings::{self, PathResolver};
use semver::Version;

use crate::cst::KindTypes;

pub type Bindings = metaslang_bindings::Bindings<KindTypes>;
pub type Definition<'a> = metaslang_bindings::Definition<'a, KindTypes>;
pub type Reference<'a> = metaslang_bindings::Reference<'a, KindTypes>;

pub fn create_with_resolver(
    version: Version,
    resolver: Arc<dyn PathResolver + Sync + Send>,
) -> Bindings {
    Bindings::create(version, binding_rules::BINDING_RULES_SOURCE, resolver)
}

#[cfg(feature = "__private_testing_utils")]
pub fn get_binding_rules() -> &'static str {
    binding_rules::BINDING_RULES_SOURCE
}

pub fn get_built_ins(version: &semver::Version) -> &'static str {
    built_ins::get_contents(version)
}
