//! `StructDefinition::is_recursive`: which structs refer to themselves through
//! their members' types.

use slang_solidity_v2_common::collections::Map;

use crate::ast::Definition;
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
struct ReachingMulti { Multi m; }
struct ReachingThroughFunction { uint256 a; function (ThroughArray memory) internal f; }
struct FunctionOverArray { function (ThroughArray[] memory) internal f; }
struct FunctionOverMapping { function (mapping(uint256 => ThroughArray) storage) internal f; }
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

/// The fixture's structs by name, each answering `is_recursive()`.
fn classified() -> Map<String, bool> {
    let unit = Shapes::build_compilation_unit();
    unit.all_definitions()
        .filter_map(|definition| match definition {
            Definition::Struct(structure) => Some((
                structure.name().name().to_string(),
                structure.is_recursive(),
            )),
            _ => None,
        })
        .collect()
}

#[test]
fn test_a_struct_referring_to_itself_through_any_member_type_is_recursive() {
    let structs = classified();
    assert!(structs["ThroughArray"]);
    assert!(structs["SelfMapping"]);
    assert!(structs["FixedUnderDynamic"]);
    assert!(structs["ByValueOfArrayRecursive"]);
    assert!(structs["ArrayOfByValue"]);
    assert!(structs["ThroughFunction"]);
    assert!(structs["Multi"]);
    assert!(structs["ArrayIntoFunctionCycle"]);
    assert!(structs["FunctionBackToArray"]);
    assert!(structs["OnFunctionCycleWithRecursive"]);
    assert!(structs["ArrayRecursiveOnFunctionCycle"]);
}

#[test]
fn test_a_struct_only_reaching_a_recursive_one_is_not_recursive() {
    let structs = classified();
    assert!(!structs["Plain"]);
    assert!(!structs["NestingRecursive"]);
    assert!(!structs["NestingNesting"]);
    assert!(!structs["ThroughMapping"]);
    assert!(!structs["ReachingMulti"]);
    assert!(!structs["ReachingThroughFunction"]);
    assert!(!structs["FunctionOverArray"]);
    assert!(!structs["FunctionOverMapping"]);
    assert!(!structs["ReachingFunctionCycle"]);
}
