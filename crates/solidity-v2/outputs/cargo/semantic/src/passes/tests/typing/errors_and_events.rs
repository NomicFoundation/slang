//! Typing of error instantiations and event invocations. Both build a value
//! of their own type rather than of the meta-type of the declaration they name,
//! which is what lets a member reached through one fail to resolve.

use slang_solidity_v2_common::diagnostics::kinds::resolution::MemberNotFound;

use super::{Analyse, Analysis, expression, expression_statement_types};
use crate::binder::Definition;
use crate::types::{ErrorType, EventType, Type};

#[test]
fn test_error_instantiation_is_error_typed() {
    let (type_, _types) = expression("E(1)")
        .with_members("error E(uint256 amount);")
        .into_resolved_type();

    assert!(
        matches!(type_, Type::Error(ErrorType { .. })),
        "expected `E(1)` to be typed as an error, got {type_:?}",
    );
}

#[test]
fn test_event_invocation_is_event_typed() {
    let (type_, _types) = expression("E(1)")
        .with_members("event E(uint256 amount);")
        .into_resolved_type();

    assert!(
        matches!(type_, Type::Event(EventType { .. })),
        "expected `E(1)` to be typed as an event, got {type_:?}",
    );
}

#[test]
fn test_error_instantiation_with_named_arguments_is_error_typed() {
    // The named-argument form goes through its own typing path, and has to
    // agree with the positional one above.
    let (type_, _types) = expression("E({amount: 1})")
        .with_members("error E(uint256 amount);")
        .into_resolved_type();

    assert!(
        matches!(type_, Type::Error(ErrorType { .. })),
        "expected `E({{amount: 1}})` to be typed as an error, got {type_:?}",
    );
}

#[test]
fn test_event_invocation_with_named_arguments_is_event_typed() {
    let (type_, _types) = expression("E({amount: 1})")
        .with_members("event E(uint256 amount);")
        .into_resolved_type();

    assert!(
        matches!(type_, Type::Event(EventType { .. })),
        "expected `E({{amount: 1}})` to be typed as an event, got {type_:?}",
    );
}

#[test]
fn test_error_type_carries_its_declaration() {
    let source = r#"
        contract Test {
            error E(uint256 amount);

            function probe() internal pure {
                E(1);
            }
        }
        "#;

    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let typings = expression_statement_types(
        analysis.function_body("Test", "probe"),
        analysis.binder(),
        analysis.types(),
    );

    let [Some(Type::Error(ErrorType { definition_id }))] = typings.as_slice() else {
        panic!("expected `E(1)` to be typed as an error, got {typings:?}");
    };

    let definition = analysis.binder().find_definition_by_id(*definition_id);
    let Some(Definition::Error(error)) = definition else {
        panic!("expected the type to carry an error definition, got {definition:?}");
    };
    assert_eq!("E", error.ir_node.name.unparse());
}

#[test]
fn test_event_type_carries_its_declaration() {
    let source = r#"
        contract Test {
            event E(uint256 amount);

            function probe() internal pure {
                E(1);
            }
        }
        "#;

    let analysis = Analysis::of_source(source)
        .run(Analyse::References)
        .expect_no_diagnostics();
    let typings = expression_statement_types(
        analysis.function_body("Test", "probe"),
        analysis.binder(),
        analysis.types(),
    );

    let [Some(Type::Event(EventType { definition_id }))] = typings.as_slice() else {
        panic!("expected `E(1)` to be typed as an event, got {typings:?}");
    };

    let definition = analysis.binder().find_definition_by_id(*definition_id);
    let Some(Definition::Event(event)) = definition else {
        panic!("expected the type to carry an event definition, got {definition:?}");
    };
    assert_eq!("E", event.ir_node.name.unparse());
}

#[test]
fn test_no_members_on_an_error_instantiation() {
    // Members belong to the declaration, not to an instantiation of it.
    // Before these types existed the operand was `Unresolved` and the
    // member-not-found check was suppressed.
    assert_eq!(
        (
            None,
            Some(
                MemberNotFound {
                    name: "amount".to_owned()
                }
                .into()
            )
        ),
        expression("E(1).amount")
            .with_members("error E(uint256 amount);")
            .into_type_and_diagnostic(),
    );
}

#[test]
fn test_no_members_on_an_event_invocation() {
    assert_eq!(
        (
            None,
            Some(
                MemberNotFound {
                    name: "amount".to_owned()
                }
                .into()
            )
        ),
        expression("E(1).amount")
            .with_members("event E(uint256 amount);")
            .into_type_and_diagnostic(),
    );
}
