use std::sync::Arc;

use anyhow::Result;
use semver::Version;
use slang_solidity::bindings::{self, Bindings, Definition};
use slang_solidity::cst::TextIndex;
use slang_solidity::parser::Parser;
use slang_solidity::transform_built_ins_node;

use crate::resolver::TestsPathResolver;

pub fn create_bindings(version: &Version) -> Result<Bindings> {
    let parser = Parser::create(version.clone())?;
    let mut bindings =
        bindings::create_with_resolver(version.clone(), Arc::new(TestsPathResolver {}));

    let built_ins_parse_output = parser.parse(Parser::ROOT_KIND, bindings::get_built_ins(version));
    assert!(
        built_ins_parse_output.is_valid(),
        "built-ins parse without errors"
    );

    let built_ins_cursor = transform_built_ins_node(&built_ins_parse_output.tree())
        .cursor_with_offset(TextIndex::ZERO);

    bindings.add_system_file("built_ins.sol", built_ins_cursor);
    Ok(bindings)
}

pub fn lookup_definition_by_name<'a>(bindings: &'a Bindings, name: &str) -> Option<Definition<'a>> {
    bindings
        .all_definitions()
        .find(|definition| definition.get_cursor().unwrap().node().unparse() == name)
}
