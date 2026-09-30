//! Exercises `is_recursive` on the struct definition AST nodes.

use super::fixtures;
use crate::define_fixture;

define_fixture!(
    Shapes,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

struct ThroughArray { ThroughArray[] kids; }
struct NestingRecursive { uint256 t; ThroughArray s; }
struct NestingNesting { NestingRecursive deeper; }
struct ThroughMapping { mapping(uint256 => ThroughArray) m; }
struct Plain { uint256 a; }
struct ThroughFunction { function (ThroughFunction memory) internal f; }
struct Multi { function () internal returns (Multi memory, uint256) f; }
struct ReachingThroughFunction { uint256 a; function (ThroughArray memory) internal f; }
struct OnFunctionCycleWithRecursive { function (ArrayRecursiveOnFunctionCycle memory) internal f; }
struct ArrayRecursiveOnFunctionCycle {
    ArrayRecursiveOnFunctionCycle[] qs;
    function (OnFunctionCycleWithRecursive memory) internal g;
}
struct ByValueOfArrayRecursive { ArrayOfByValue b; uint256 tail; }
struct ArrayOfByValue { ByValueOfArrayRecursive[] items; }
struct SelfMapping { mapping(uint256 => SelfMapping) m; }
struct FixedUnderDynamic { FixedUnderDynamic[2][] x; }
struct ArrayIntoFunctionCycle { FunctionBackToArray[] xs; }
struct FunctionBackToArray { function (ArrayIntoFunctionCycle memory) internal f; }
struct ReachingFunctionCycle { ArrayIntoFunctionCycle inner; }
"#,
);

fn is_recursive(struct_name: &str) -> bool {
    let unit = Shapes::build_compilation_unit();
    fixtures::find_struct(&unit, struct_name).is_recursive()
}

#[test]
fn test_a_struct_on_no_cycle_is_not_recursive() {
    assert!(!is_recursive("Plain"));
    assert!(!is_recursive("ReachingThroughFunction"));
    assert!(!is_recursive("ReachingFunctionCycle"));
}

#[test]
fn test_a_struct_on_a_cycle_of_arrays_or_mappings_is_recursive() {
    assert!(is_recursive("ThroughArray"));
    assert!(is_recursive("SelfMapping"));
    assert!(is_recursive("FixedUnderDynamic"));
    assert!(is_recursive("ByValueOfArrayRecursive"));
    assert!(is_recursive("ArrayOfByValue"));
}

#[test]
fn test_a_struct_nesting_a_recursive_struct_is_recursive() {
    assert!(is_recursive("NestingRecursive"));
    assert!(is_recursive("NestingNesting"));
    assert!(is_recursive("ThroughMapping"));
}

#[test]
fn test_a_struct_on_a_cycle_closed_by_a_function_type_is_recursive() {
    assert!(is_recursive("ThroughFunction"));
    assert!(is_recursive("Multi"));
    assert!(is_recursive("ArrayIntoFunctionCycle"));
    assert!(is_recursive("FunctionBackToArray"));
}

#[test]
fn test_a_function_cycle_through_a_recursive_struct_marks_only_that_struct() {
    assert!(is_recursive("ArrayRecursiveOnFunctionCycle"));
    assert!(!is_recursive("OnFunctionCycleWithRecursive"));
}
