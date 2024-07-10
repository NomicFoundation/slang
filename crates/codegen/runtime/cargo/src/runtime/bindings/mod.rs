#[path = "generated/binding_rules.rs"]
mod binding_rules;

use std::sync::Arc;

use metaslang_bindings::{self, DefaultPathResolver, PathResolver};
use semver::Version;

use crate::cst::KindTypes;

pub type Bindings = metaslang_bindings::Bindings<KindTypes>;
pub type Handle<'a> = metaslang_bindings::Handle<'a, KindTypes>;

pub fn create(version: Version) -> Bindings {
    create_with_resolver(version, Arc::new(DefaultPathResolver {}))
}

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
