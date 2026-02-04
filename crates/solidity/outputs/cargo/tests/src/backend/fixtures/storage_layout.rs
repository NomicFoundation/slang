use std::rc::Rc;

use anyhow::Result;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::utils::LanguageFacts;

use crate::compilation::compilation_unit::build_compilation_unit_from_multi_part_file;

// Sample adapted from: https://docs.soliditylang.org/en/v0.8.33/internals/layout_in_storage.html#layout-of-state-variables-in-storage-and-transient-storage
pub const CONTENTS: &str = r#"
// ----- path: main.sol
struct S {
    int32 x;
    bool y;
}
struct T {
    uint256 z;
    uint32 w;
}

contract A {
    uint a;
    uint constant c = 10;
    uint immutable d = 12;
}

contract B {
    uint8[] e;
    mapping(uint => S) f;
    uint16 g;
    uint16 h;
    S s;
    int8 k;
}

contract C is A, B {
    bytes21 l;
    uint8[10] m;
    bytes5[8] n;
    T[2] t;
    bytes5 o;
}
"#;

pub(crate) struct StorageLayout {}

impl StorageLayout {
    pub(crate) fn build_compilation_unit() -> Result<Rc<CompilationUnit>> {
        build_compilation_unit_from_multi_part_file(&LanguageFacts::LATEST_VERSION, CONTENTS)
    }
}
