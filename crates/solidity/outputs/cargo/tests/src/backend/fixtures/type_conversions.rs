use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::utils::LanguageFacts;

use crate::compilation::compilation_unit::build_compilation_unit_from_multi_part_file;

pub const CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract TypeConversions {
    function convert(uint256 value) internal pure {
        // Array-type cast: the call operand is an `IndexAccessExpression` (`uint256[]`).
        uint256[](value);
        // Elementary-type cast: the call operand is an `ElementaryType`.
        uint256(value);
        // Plain function call: the call operand is an `Identifier` naming a function.
        identity(value);
    }

    function identity(uint256 value) internal pure returns (uint256) {
        return value;
    }
}
"#;

pub(crate) struct TypeConversions;

impl TypeConversions {
    pub(crate) fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
        build_compilation_unit_from_multi_part_file(&LanguageFacts::LATEST_VERSION, CONTENTS)
    }
}
