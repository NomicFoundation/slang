use std::sync::Arc;

use anyhow::Result;
use semver::Version;
use slang_solidity::bindings::{self, add_built_ins, Bindings, Definition};

use crate::resolver::TestsPathResolver;

pub fn create_bindings(version: &Version) -> Result<Bindings> {
    let mut bindings =
        bindings::create_with_resolver(version.clone(), Arc::new(TestsPathResolver {}));
    add_built_ins(&mut bindings, version)?;

    Ok(bindings)
}

pub fn lookup_definition_by_name<'a>(bindings: &'a Bindings, name: &str) -> Option<Definition<'a>> {
    bindings
        .all_definitions()
        .find(|definition| definition.get_cursor().unwrap().node().unparse() == name)
}
