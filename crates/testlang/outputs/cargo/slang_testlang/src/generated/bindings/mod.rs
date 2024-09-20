// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/binding_rules.rs"]
mod binding_rules;

#[path = "generated/built_ins.rs"]
mod built_ins;

use std::sync::Arc;

use metaslang_bindings::{self, PathResolver};

use crate::cst::KindTypes;
use crate::language::Language;

pub type Bindings = metaslang_bindings::Bindings<KindTypes>;
pub type Definition<'a> = metaslang_bindings::Definition<'a, KindTypes>;
pub type Reference<'a> = metaslang_bindings::Reference<'a, KindTypes>;

pub fn create_with_resolver(
    language: &Language,
    resolver: Arc<dyn PathResolver + Sync + Send>,
) -> Bindings {
    let version = language.version.clone();
    let built_ins_parse_output =
        language.parse(Language::ROOT_KIND, built_ins::get_contents(&version));
    assert!(
        built_ins_parse_output.is_valid(),
        "built-ins parse without errors"
    );

    let mut bindings = Bindings::create(version, binding_rules::BINDING_RULES_SOURCE, resolver);
    bindings.add_built_ins(built_ins_parse_output.create_tree_cursor());
    bindings
}

#[cfg(feature = "__private_testing_utils")]
pub fn get_binding_rules() -> &'static str {
    binding_rules::BINDING_RULES_SOURCE
}

#[cfg(feature = "__private_testing_utils")]
pub fn get_built_ins(version: &semver::Version) -> &'static str {
    built_ins::get_contents(version)
}
