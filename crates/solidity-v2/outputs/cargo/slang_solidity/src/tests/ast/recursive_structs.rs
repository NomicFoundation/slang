//! Exercises both recursion accessors on struct definition AST nodes.

use super::fixtures;
use crate::define_fixture;

define_fixture!(
    Shapes,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

struct Plain { uint256 a; }
struct ThroughArray { ThroughArray[] kids; }
struct ThroughFunction { function (ThroughFunction memory) internal f; }
"#,
);

#[test]
fn test_struct_recursion_accessors() {
    let unit = Shapes::build_compilation_unit();
    for (name, recursive, recursive_type_graph) in [
        ("Plain", false, false),
        ("ThroughArray", true, true),
        ("ThroughFunction", false, true),
    ] {
        let definition = fixtures::find_struct(&unit, name);
        assert_eq!(definition.is_recursive(), recursive, "{name}");
        assert_eq!(
            definition.has_recursive_type_graph(),
            recursive_type_graph,
            "{name}"
        );
    }
}
