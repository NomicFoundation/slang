use std::sync::Arc;

use anyhow::Result;
use semver::Version;
use slang_solidity::bindings::{self, Bindings};
use slang_solidity::parser::Parser;

use crate::resolver::TestsPathResolver;

pub fn create_bindings(version: &Version) -> Result<Bindings> {
    let parser = Parser::new(version.clone())?;
    let mut bindings =
        bindings::create_with_resolver(version.clone(), Arc::new(TestsPathResolver {}));

    let built_ins_parse_output = parser.parse(Parser::ROOT_KIND, bindings::get_built_ins(version));
    assert!(
        built_ins_parse_output.is_valid(),
        "built-ins parse without errors"
    );
    bindings.add_system_file("built_ins.sol", built_ins_parse_output.create_tree_cursor());
    Ok(bindings)
}
