//! Tests for the struct recursion flags p3 sets on struct definitions: which
//! references each follows, and the by-value dependencies recorded for p8.

use super::support::{Analyse, Analysis};
use crate::binder::{Definition, StructDefinition};

fn structs(analysis: &Analysis) -> Vec<&StructDefinition> {
    analysis
        .binder()
        .definitions()
        .values()
        .filter_map(|definition| match definition {
            Definition::Struct(definition) => Some(definition),
            _ => None,
        })
        .collect()
}

fn struct_named<'a>(analysis: &'a Analysis, name: &str) -> &'a StructDefinition {
    structs(analysis)
        .into_iter()
        .find(|definition| definition.ir_node.name.unparse() == name)
        .unwrap_or_else(|| panic!("struct `{name}` is declared"))
}

fn assert_recursion(analysis: &Analysis, expected: &[(&str, bool, bool)]) {
    assert_eq!(structs(analysis).len(), expected.len());
    for (name, recursive, recursive_type_graph) in expected {
        let definition = struct_named(analysis, name);
        assert_eq!(definition.is_recursive, *recursive, "{name}");
        assert_eq!(
            definition.has_recursive_type_graph, *recursive_type_graph,
            "{name}"
        );
    }
}

fn analyse(source: &str) -> Analysis {
    Analysis::of_source(source)
        .run(Analyse::Types)
        .expect_no_diagnostics()
}

#[test]
fn member_cycles_and_wrappers_are_recursive() {
    let analysis = analyse(
        "pragma solidity *;
        struct Plain { uint256 a; }
        struct ThroughArray { ThroughArray[] kids; Plain leaf; }
        struct NestingRecursive { uint256 t; ThroughArray s; }
        struct SelfMapping { mapping(uint256 => SelfMapping) m; }",
    );
    assert_recursion(
        &analysis,
        &[
            ("Plain", false, false),
            ("ThroughArray", true, true),
            ("NestingRecursive", true, true),
            ("SelfMapping", true, true),
        ],
    );
}

#[test]
fn function_signatures_only_make_the_type_graph_recursive() {
    let analysis = analyse(
        "pragma solidity *;
        struct Plain { uint256 a; }
        struct Parameter { function (Parameter memory) internal f; }
        struct TupleReturn {
            function () internal returns (uint256, TupleReturn memory, Plain memory) f;
        }
        struct AcyclicReference {
            function (Plain memory) internal returns (Plain memory) f;
        }
        struct MemberCycle { MemberCycle[] kids; }
        struct ReachingMemberCycle { function (MemberCycle memory) internal f; }
        struct ArrayIntoFunctionCycle { FunctionBackToArray[] xs; }
        struct FunctionBackToArray {
            function (ArrayIntoFunctionCycle memory) internal f;
        }
        struct ReachingFunctionCycle { ArrayIntoFunctionCycle inner; }",
    );
    assert_recursion(
        &analysis,
        &[
            ("Plain", false, false),
            ("Parameter", false, true),
            ("TupleReturn", false, true),
            ("AcyclicReference", false, false),
            ("MemberCycle", true, true),
            ("ReachingMemberCycle", false, true),
            ("ArrayIntoFunctionCycle", false, true),
            ("FunctionBackToArray", false, true),
            ("ReachingFunctionCycle", false, true),
        ],
    );
}

#[test]
fn signature_dependencies_stay_tagged_through_nested_types() {
    let analysis = analyse(
        "pragma solidity *;
        struct ArrayParameter { function (ArrayParameter[2][] memory) internal f; }
        struct NestedFunction {
            function (function (NestedFunction memory) internal) internal f;
        }
        struct ContainerOfFunctions {
            mapping(uint256 => function (ContainerOfFunctions memory) internal[]) fs;
        }",
    );
    assert_recursion(
        &analysis,
        &[
            ("ArrayParameter", false, true),
            ("NestedFunction", false, true),
            ("ContainerOfFunctions", false, true),
        ],
    );
}

#[test]
fn duplicate_references_to_acyclic_structs_are_not_recursive() {
    let analysis = analyse(
        "pragma solidity *;
        struct Plain { uint256 a; }
        struct Shared { Plain a; Plain b; }",
    );
    assert_recursion(
        &analysis,
        &[("Plain", false, false), ("Shared", false, false)],
    );
}

#[test]
fn unresolved_members_are_skipped() {
    let analysis = Analysis::of_source(
        "pragma solidity *;
        struct Recursive { Unknown missing; Recursive[] children; }",
    )
    .run(Analyse::Types);
    assert_recursion(&analysis, &[("Recursive", true, true)]);
}

#[test]
fn by_value_dependencies_follow_member_order_and_skip_indirect_references() {
    let analysis = analyse(
        "pragma solidity *;
        struct Leaf { uint256 a; }
        struct Other { uint256 b; }
        struct Holder {
            Other o;
            Leaf[2] l;
            Leaf[] d;
            mapping(uint256 => Leaf) m;
            function (Leaf memory) internal f;
        }",
    );
    let id = |name| struct_named(&analysis, name).ir_node.id();
    assert_eq!(
        struct_named(&analysis, "Holder").by_value_dependencies,
        [id("Other"), id("Leaf")]
    );
    assert!(
        struct_named(&analysis, "Leaf")
            .by_value_dependencies
            .is_empty()
    );
    assert!(
        struct_named(&analysis, "Other")
            .by_value_dependencies
            .is_empty()
    );
}
