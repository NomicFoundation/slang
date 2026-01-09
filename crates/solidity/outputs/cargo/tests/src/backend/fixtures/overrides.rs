use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use slang_solidity::utils::LanguageFacts;

pub const MAIN_ID: &str = "main.sol";
pub const MAIN_SOL_CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract Base
{
    function in_base() internal pure {}
    function override_me() virtual external view {}
}

contract Middle is Base {
    function in_middle() external pure {}
    function override_me() virtual override public view {}
}

contract Inherited is Middle
{
    function override_me() override public pure {}
}

"#;

pub(crate) struct Overrides {}

impl Overrides {
    pub(crate) fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
        let mut builder = CompilationBuilder::create(LanguageFacts::LATEST_VERSION, Self {})?;

        builder.add_file(MAIN_ID)?;

        Ok(Rc::new(builder.build()))
    }
}

impl CompilationBuilderConfig for Overrides {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        match file_id {
            MAIN_ID => Ok(Some(MAIN_SOL_CONTENTS.to_owned())),
            _ => Ok(None),
        }
    }

    fn resolve_import(
        &mut self,
        _source_file_id: &str,
        _import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        Ok(None)
    }
}
