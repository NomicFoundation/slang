//! Selecting an overload from the call's arguments, and the operand typing
//! that follows from the selection.

use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdentity};

use super::{Analysis, expression};
use crate::binder::Typing;
use crate::types::{FunctionType, IntegerType, Type, TypeId, UserMetaType};

/// Collects the `FunctionCallExpression` of each expression statement in the
/// body of `function` within `contract`, in source order.
fn call_expressions<'a>(
    analysis: &'a Analysis,
    contract: &str,
    function: &str,
) -> Vec<&'a ir::FunctionCallExpression> {
    analysis
        .function_body(contract, function)
        .statements
        .iter()
        .filter_map(|stmt| match stmt {
            ir::Statement::ExpressionStatement(s) => match &s.expression {
                ir::Expression::FunctionCallExpression(call) => Some(call),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

#[test]
fn test_overload_resolution_unsigned_to_signed_argument_is_version_gated() {
    // End-to-end: an overload taking `int16` is only reachable from a `uint8`
    // argument before 0.8.1, where `uint8` still implicitly converts to `int16`.
    let setup = "
        uint8 u;
        function pick(int16 a) internal pure returns (uint8) { a; return 1; }
        function pick(string memory a) internal pure returns (uint16) { a; return 2; }
    ";

    // 0.8.0: `uint8` -> `int16` is allowed, so the `int16` overload matches.
    let (typing, _) = expression("pick(u)")
        .with_members(setup)
        .version(LanguageVersion::V0_8_0)
        .into_type();
    assert_eq!(
        typing,
        Some(Type::Integer(IntegerType {
            is_signed: false,
            bits: 8,
        })),
    );

    // 0.8.1: `uint8` -> `int16` is rejected, so neither overload matches.
    let (typing, _) = expression("pick(u)")
        .with_members(setup)
        .version(LanguageVersion::V0_8_1)
        .into_type();
    assert_eq!(typing, None);
}

#[test]
fn test_overload_resolution_widens_byte_array_argument() {
    let setup = "
        function pick(bytes32 a) internal pure returns (uint8) { a; return 1; }
        function pick(string memory a) internal pure returns (uint16) { a; return 2; }
    ";
    let (type_, _) = expression("pick(bytes20(0))")
        .with_members(setup)
        .into_resolved_type();
    assert_eq!(
        type_,
        Type::Integer(IntegerType {
            is_signed: false,
            bits: 8,
        })
    );
}

#[test]
fn test_overload_resolution_rejects_byte_array_narrowing() {
    let setup = "
        function pick(bytes20 a) internal pure returns (uint8) { a; return 1; }
        function pick(string memory a) internal pure returns (uint16) { a; return 2; }
    ";
    let (type_, _) = expression("pick(bytes32(0))")
        .with_members(setup)
        .into_type();
    // Neither overload matches: `bytes32` does not convert to `bytes20` nor
    // to `string`. The call is unresolved.
    assert_eq!(type_, None);
}

#[test]
fn test_meta_type_argument_does_not_match_overloads() {
    // Passing a type name as an argument must not match any overload
    // candidate during disambiguation.
    let context = r#"
        function f(uint x) internal pure returns (bool) { return x > 0; }
        function f(bool x) internal pure returns (uint) { return x ? 1 : 0; }
    "#;
    let (type_, _) = expression("f(uint)").with_members(context).into_type();
    assert_eq!(type_, None);
}

#[test]
fn test_overloaded_call_operand_narrows_to_selected_overload() {
    // When an overloaded callee resolves to a single overload through the
    // call's arguments, the operand's typing is narrowed from the whole
    // candidate set (`Undetermined`) down to the selected overload — for
    // positional and named argument calls alike.
    let source = r#"
        contract C {
            function f(uint x) internal pure {}
            function f(bool b) internal pure {}
            function g() internal pure {
                f(1);
                f({b: true});
            }
        }
    "#;

    let analysis = Analysis::of_source(source).run().expect_no_diagnostics();
    let binder = analysis.binder();
    let types = analysis.types();

    let calls = call_expressions(&analysis, "C", "g");
    assert_eq!(calls.len(), 2);

    // Recovers the single parameter type of the operand's (now resolved)
    // function type, failing if the operand is still an ambiguous candidate set.
    let sole_parameter_type = |call: &ir::FunctionCallExpression| -> TypeId {
        let node_id = call.operand.node_id().expect("operand has a node id");
        let type_id = match binder.node_typing(node_id) {
            Typing::Resolved(type_id) => type_id,
            other => panic!("operand should be narrowed to a single overload, got {other:?}"),
        };
        let Type::Function(FunctionType {
            parameter_types, ..
        }) = types.get_type_by_id(type_id)
        else {
            panic!("operand should type as a function");
        };
        assert_eq!(
            parameter_types.len(),
            1,
            "each overload takes one parameter"
        );
        parameter_types[0]
    };

    // `f(1)`: the literal only converts to `uint`, selecting `f(uint)`.
    assert_eq!(sole_parameter_type(calls[0]), types.uint256());
    // `f({b: true})`: the named argument selects `f(bool)`.
    assert_eq!(sole_parameter_type(calls[1]), types.boolean());
}

#[test]
fn test_overloaded_declaration_via_type_name_operand_narrows() {
    // Even though calling an overload through a contract *type name* is invalid,
    // the operand still disambiguates to the overload matching the arguments:
    // its typing is narrowed to that specific (non-callable) declaration rather
    // than left as the ambiguous candidate set.
    let source = r#"
        contract A {
            function f() external {}
            function f(uint x) external {}
        }
        contract B {
            function g() internal {
                A.f();
                A.f(1);
            }
        }
    "#;

    let analysis = Analysis::of_source(source).run();
    let binder = analysis.binder();
    let types = analysis.types();
    let diagnostics = &analysis.diagnostics;

    // Both calls are invalid: external functions aren't callable via the type name.
    assert_eq!(
        diagnostics.iter().count(),
        2,
        "both calls via the contract type name should be rejected"
    );

    let calls = call_expressions(&analysis, "B", "g");
    assert_eq!(calls.len(), 2);

    // The operand narrows to the user meta type of the selected overload's
    // function definition (a non-callable declaration), not the candidate set.
    let selected_definition = |call: &ir::FunctionCallExpression| {
        let node_id = call.operand.node_id().expect("operand has a node id");
        match binder.node_typing(node_id) {
            Typing::Resolved(type_id) => match types.get_type_by_id(type_id) {
                Type::UserMetaType(UserMetaType { definition_id }) => *definition_id,
                other => panic!("operand should be a declaration meta type, got {other:?}"),
            },
            other => panic!("operand should be narrowed to a single overload, got {other:?}"),
        }
    };

    // The definition's own typing tells us which overload was picked.
    let parameter_count = |definition_id| match binder
        .node_typing(definition_id)
        .as_type_id()
        .map(|id| types.get_type_by_id(id))
    {
        Some(Type::Function(FunctionType {
            parameter_types, ..
        })) => parameter_types.len(),
        other => panic!("definition should type as a function, got {other:?}"),
    };

    let first = selected_definition(calls[0]);
    let second = selected_definition(calls[1]);
    assert_ne!(
        first, second,
        "the two calls should disambiguate to different overloads"
    );
    // `A.f()` selects the parameterless overload, `A.f(1)` the one-parameter one.
    assert_eq!(parameter_count(first), 0);
    assert_eq!(parameter_count(second), 1);
}
