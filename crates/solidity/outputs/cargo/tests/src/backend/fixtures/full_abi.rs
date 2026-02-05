use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::utils::LanguageFacts;

use crate::compilation::compilation_unit::build_compilation_unit_from_multi_part_file;

pub const CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8;

interface Base {
    error SomethingWrong(string);
    event BaseEvent(uint a, string m) anonymous;
}

contract Test is Base {
    bytes32 public b;
    constructor() { b = hex"12345678901234567890123456789012"; }
    event Event(uint indexed a, bytes32 b);
    error InsufficientBalance(uint256 available, uint256 required);
    function foo(uint a) public { emit Event(a, b); }
    receive() external payable { }
    fallback() external { }
}
"#;

pub(crate) struct FullAbi;

impl FullAbi {
    pub(crate) fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
        build_compilation_unit_from_multi_part_file(&LanguageFacts::LATEST_VERSION, CONTENTS)
    }
}
