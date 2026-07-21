//! Tests that a public state variable's getter overrides a same-named function
//! inherited from a base contract, so `linearised_functions` drops it.

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::NodeIdGenerator;

use super::build_file;
use crate::context::SemanticContext;

/// Builds a `SemanticContext` over `source`, asserting no diagnostics.
fn build_context(source: &str) -> SemanticContext {
    let mut id_generator = NodeIdGenerator::default();
    let file = build_file(
        "test.sol".into(),
        source,
        &mut id_generator,
        LanguageVersion::LATEST,
    );
    let files = vec![file];
    let mut diagnostics = DiagnosticCollection::default();
    let context = SemanticContext::build_from(
        LanguageVersion::LATEST,
        EvmTarget::LATEST,
        &files,
        None,
        &mut diagnostics,
    );
    assert!(
        diagnostics.is_empty(),
        "Semantic diagnostics: {diagnostics:?}"
    );
    context
}

/// The names of the named functions in `contract`'s linearised function list.
/// The list is already sorted by name.
fn linearised_function_names(context: &SemanticContext, contract: &str) -> Vec<String> {
    let definition = context
        .find_contract_by_name(contract)
        .next()
        .unwrap_or_else(|| panic!("contract `{contract}` not found"));
    context
        .linearised_functions(definition.id())
        .iter()
        .filter_map(|function| function.name.as_ref().map(|name| name.unparse().to_owned()))
        .collect()
}

#[test]
fn getter_shadows_overridden_function() {
    let context = build_context(
        r#"
        contract Base {
            function x() external virtual returns (uint256) { return 0; }
        }
        contract D is Base {
            uint256 public override x;
        }
        "#,
    );

    // The getter serves D's dispatch slot, so `Base.x` is dropped from D.
    assert!(!linearised_function_names(&context, "D").contains(&"x".to_owned()));
    // Base's own list still has the function.
    assert!(linearised_function_names(&context, "Base").contains(&"x".to_owned()));
}

#[test]
fn getter_shadows_from_middle_of_hierarchy() {
    let context = build_context(
        r#"
        abstract contract A {
            function x() external virtual returns (uint256);
        }
        contract B is A {
            uint256 public override x;
        }
        contract D is B {}
        "#,
    );

    // The getter overriding in B shadows `A.x` for both B and its descendant D.
    assert!(!linearised_function_names(&context, "B").contains(&"x".to_owned()));
    assert!(!linearised_function_names(&context, "D").contains(&"x".to_owned()));
}

#[test]
fn parameterized_getter_shadows_overridden_function() {
    let context = build_context(
        r#"
        abstract contract Base {
            function m(uint256) external virtual returns (uint256);
        }
        contract D is Base {
            mapping(uint256 => uint256) public override m;
        }
        "#,
    );

    // The mapping's getter has the function type `function(uint256) returns
    // (uint256)`, matching `Base.m`, so the function is dropped.
    assert!(!linearised_function_names(&context, "D").contains(&"m".to_owned()));
}

#[test]
fn differently_named_variable_does_not_shadow() {
    let context = build_context(
        r#"
        contract Base {
            function x() external virtual returns (uint256) { return 0; }
        }
        contract D is Base {
            uint256 public y;
        }
        "#,
    );

    // `y`'s getter has a different name, so `Base.x` stays in D's list.
    assert!(linearised_function_names(&context, "D").contains(&"x".to_owned()));
}
