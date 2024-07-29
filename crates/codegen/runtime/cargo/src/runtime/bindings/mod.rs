#[path = "generated/binding_rules.rs"]
mod binding_rules;

use metaslang_bindings;
use semver::Version;

use crate::cst::KindTypes;

pub type Bindings = metaslang_bindings::Bindings<KindTypes>;
pub type Handle<'a> = metaslang_bindings::Handle<'a, KindTypes>;

pub fn create(version: Version) -> Bindings {
    Bindings::create(version, binding_rules::BINDING_RULES_SOURCE)
}

#[cfg(feature = "__private_testing_utils")]
pub fn get_binding_rules() -> &'static str {
    binding_rules::BINDING_RULES_SOURCE
}
