use super::fixtures;
use crate::ast::{FunctionDefinition, FunctionKind};
use crate::define_fixture;

define_fixture!(
    SegmentFunctions,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

library L {
    function run() external pure returns (uint) {
        function() internal pure returns (uint) p = g;
        return p();
    }
    function g() internal pure returns (uint) { return 1; }
}

contract C {
    uint value;
    function(uint) internal pure returns (uint) p;
    constructor() {
        function() internal pure returns (uint) l = local;
        value = l();
        p = stored;
    }
    function run() public view returns (uint) { return p(1); }
    function local() internal pure returns (uint) { return 1; }
    function stored(uint x) internal pure returns (uint) { return x; }
}
"#,
);

/// Names each function, and the nameless constructor by its kind.
fn names(functions: &[FunctionDefinition]) -> Vec<String> {
    functions
        .iter()
        .map(|function| match function.name() {
            Some(name) => name.name().to_owned(),
            None if matches!(function.kind(), FunctionKind::Constructor) => {
                "constructor".to_owned()
            }
            None => unreachable!("the fixture's only nameless function is a constructor"),
        })
        .collect()
}

#[test]
fn test_contract_segment_functions() {
    let unit = SegmentFunctions::build_compilation_unit();
    let contract = unit
        .find_contract_by_name("C")
        .next()
        .expect("can find contract");

    assert_eq!(names(&contract.creation_functions()), ["constructor", "local"]);
    assert_eq!(names(&contract.creation_pointer_targets()), ["local"]);
    // Only `stored` has the signature of `p(1)`.
    assert_eq!(names(&contract.runtime_functions()), ["run", "stored"]);
    assert_eq!(names(&contract.runtime_pointer_targets()), ["stored"]);
}

#[test]
fn test_library_segment_functions() {
    let unit = SegmentFunctions::build_compilation_unit();
    let library = fixtures::find_library(&unit, "L");

    assert_eq!(names(&library.runtime_functions()), ["run", "g"]);
    assert_eq!(names(&library.runtime_pointer_targets()), ["g"]);
}
