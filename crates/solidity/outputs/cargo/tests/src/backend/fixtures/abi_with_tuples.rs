use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::utils::LanguageFacts;

use crate::compilation::compilation_unit::build_compilation_unit_from_multi_part_file;

pub const CONTENTS: &str = r#"
contract Test {
    struct S { uint a; uint[] b; T[] c; }
    struct T { uint x; uint y; }
    function f(S memory, T memory, uint) public pure {}
    function g() public pure returns (S memory, T memory, uint) {}
}
"#;

pub(crate) struct AbiWithTuples;

impl AbiWithTuples {
    pub(crate) fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
        build_compilation_unit_from_multi_part_file(&LanguageFacts::LATEST_VERSION, CONTENTS)
    }
}
