#[path = "generated/binding_rules.rs"]
mod binding_rules;

use metaslang_bindings;
use semver::Version;

use crate::cst::KindTypes;

pub type BindingsError = metaslang_bindings::BindingsError;
pub type Bindings = metaslang_bindings::Bindings<KindTypes>;
pub type Handle<'a> = metaslang_bindings::Handle<'a, KindTypes>;

pub fn create(version: Version) -> Bindings {
    Bindings::create(version, binding_rules::BINDING_RULES_SOURCE)
}
