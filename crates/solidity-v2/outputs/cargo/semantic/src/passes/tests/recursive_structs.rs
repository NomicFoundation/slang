//! Tests for the struct recursion kinds p3 sets on struct definitions: which
//! references each follows, and the by-value dependencies recorded for p8.

use super::support::{Analyse, Analysis};
use crate::binder::{Definition, RecursionKind, StructDefinition};

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

fn assert_recursion(analysis: &Analysis, expected: &[(&str, RecursionKind)]) {
    assert_eq!(structs(analysis).len(), expected.len());
    for (name, recursive) in expected {
        let definition = struct_named(analysis, name);
        assert_eq!(definition.recursive.as_ref(), Some(recursive), "{name}");
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
            ("Plain", RecursionKind::NonRecursive),
            ("ThroughArray", RecursionKind::ByReference),
            ("NestingRecursive", RecursionKind::ByReference),
            ("SelfMapping", RecursionKind::ByReference),
        ],
    );
}

#[test]
fn by_value_cycles_and_wrappers_are_by_value() {
    let analysis = analyse(
        "pragma solidity *;
        struct ThroughFixedArray { ThroughFixedArray[2] kids; }
        struct Holding { ThroughFixedArray inner; }
        struct Referring { ThroughFixedArray[] inner; }",
    );
    assert_recursion(
        &analysis,
        &[
            ("ThroughFixedArray", RecursionKind::ByValue),
            ("Holding", RecursionKind::ByValue),
            ("Referring", RecursionKind::ByReference),
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
            ("Plain", RecursionKind::NonRecursive),
            ("Parameter", RecursionKind::ByFunctionType),
            ("TupleReturn", RecursionKind::ByFunctionType),
            ("AcyclicReference", RecursionKind::NonRecursive),
            ("MemberCycle", RecursionKind::ByReference),
            ("ReachingMemberCycle", RecursionKind::ByFunctionType),
            ("ArrayIntoFunctionCycle", RecursionKind::ByFunctionType),
            ("FunctionBackToArray", RecursionKind::ByFunctionType),
            ("ReachingFunctionCycle", RecursionKind::ByFunctionType),
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
            ("ArrayParameter", RecursionKind::ByFunctionType),
            ("NestedFunction", RecursionKind::ByFunctionType),
            ("ContainerOfFunctions", RecursionKind::ByFunctionType),
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
        &[
            ("Plain", RecursionKind::NonRecursive),
            ("Shared", RecursionKind::NonRecursive),
        ],
    );
}

#[test]
fn unresolved_members_are_skipped() {
    let analysis = Analysis::of_source(
        "pragma solidity *;
        struct Recursive { Unknown missing; Recursive[] children; }",
    )
    .run(Analyse::Types);
    assert_recursion(&analysis, &[("Recursive", RecursionKind::ByReference)]);
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
