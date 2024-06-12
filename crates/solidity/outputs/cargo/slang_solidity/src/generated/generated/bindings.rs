// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use semver::Version;

use crate::bindings_support::Bindings;

#[allow(clippy::needless_pass_by_value)]
pub fn create_for(version: Version) -> Bindings {
    _ = version;
    unreachable!("Language does not define binding rules");
}
mod supress_dependencies {
    use stack_graphs as _;
}
