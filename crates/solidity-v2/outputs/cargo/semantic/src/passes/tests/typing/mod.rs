//! Tests for the typing computed by the resolution pass, split by the
//! typing rule under test. The scaffolding they share lives here.

mod contract_members;
mod conversions;
mod literals;
mod meta_types;
mod overloads;

use ruint::aliases::U256;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdentity};

use super::{Analyse, Analysis, diagnostic_kind, find_function};
use crate::binder::{Binder, Definition};
use crate::types::{FixedSizeArrayType, IntegerType, Type, TypeId, TypeRegistry};

/// Recovers the typing recorded for an expression `node`, resolved to a
/// concrete [`Type`].
fn recover_expression_type(
    node: &ir::Expression,
    binder: &Binder,
    types: &TypeRegistry,
) -> Option<Type> {
    let node_id = node.node_id()?;
    binder
        .node_typing(node_id)
        .as_type_id()
        .map(|type_id| types.get_type_by_id(type_id).clone())
}

/// Collects the recovered type of each expression statement in `body`, in
/// source order.
fn expression_statement_types(
    body: &ir::Block,
    binder: &Binder,
    types: &TypeRegistry,
) -> Vec<Option<Type>> {
    body.statements
        .iter()
        .filter_map(|stmt| match stmt {
            ir::Statement::ExpressionStatement(s) => {
                Some(recover_expression_type(&s.expression, binder, types))
            }
            _ => None,
        })
        .collect()
}

/// Wraps each expression in a no-op expression statement inside the body of an
/// `__test()` function of a synthesized `Test` contract, runs every semantic
/// pass, and returns the typing recorded for each expression (in input order)
/// along with the populated type registry. Non-`Resolved` typings come back
/// as `None`.
///
/// `contract_context` is optional contract-level setup — state variables,
/// nested struct definitions, sibling member functions, etc. — inserted
/// before the `__test()` definition.
fn type_of_expressions(
    language_version: LanguageVersion,
    contract_name: Option<&str>,
    contract_context: Option<&str>,
    expressions: &[&str],
) -> (Vec<Option<Type>>, TypeRegistry) {
    let context_block = contract_context.unwrap_or("");
    let contract_name = contract_name.unwrap_or("Test");
    let expression_statements = expressions
        .iter()
        .map(|expr| format!("{expr};"))
        .collect::<Vec<_>>()
        .join("\n");
    let source = format!(
        r#"
        contract {contract_name} {{
            {context_block}
            function __test() internal {{
                {expression_statements}
            }}
        }}
        "#
    );

    let analysis = Analysis::of_source(&source)
        .version(language_version)
        .run()
        .expect_no_diagnostics();

    let contract = analysis.find_contract(contract_name);
    let function = find_function(&contract.members, "__test").expect("__test function not found");
    let block = function.body.as_ref().expect("__test has a body");

    let typings = expression_statement_types(block, analysis.binder(), analysis.types());

    (typings, analysis.into_types())
}

/// Convenience wrapper for `type_of_expressions` with a single expression and
/// no contract context. Panics if the typing didn't resolve.
fn type_of_expression(expr: &str) -> (Type, TypeRegistry) {
    let (expr_type, types) = try_type_of_expression(expr);
    (
        expr_type.expect("expected resolved type for expression"),
        types,
    )
}

/// Convenience wrapper for `type_of_expressions` with a single expression and
/// no contract context. Returns `None` if the typing didn't resolve.
fn try_type_of_expression(expr: &str) -> (Option<Type>, TypeRegistry) {
    let (typings, types) = type_of_expressions(LanguageVersion::LATEST, None, None, &[expr]);
    let typing = typings.into_iter().next().expect("at least one expression");
    (typing, types)
}

/// Like `type_of_expression`, but with contract-level setup (state variables,
/// member functions, …) inserted before the `__test()` function.
fn type_of_expression_in_context(context: &str, expr: &str) -> (Type, TypeRegistry) {
    let (expr_type, types) = try_type_of_expression_in_context(context, expr);
    (
        expr_type.expect("expected resolved type for expression"),
        types,
    )
}

fn try_type_of_expression_in_context(context: &str, expr: &str) -> (Option<Type>, TypeRegistry) {
    let (typings, types) =
        type_of_expressions(LanguageVersion::LATEST, None, Some(context), &[expr]);
    let typing = typings.into_iter().next().expect("at least one expression");
    (typing, types)
}

fn register_uint_type(types: &mut TypeRegistry, bits: u32) -> TypeId {
    types.register_type(Type::Integer(IntegerType {
        is_signed: false,
        bits,
    }))
}

/// Runs the full pipeline over `source` and returns contract `name`'s folded
/// storage base slot together with the diagnostic emitted, if any. A rejected base
/// slot is reported as a diagnostic and leaves `base_slot` unset.
fn contract_base_slot(source: &str, name: &str) -> (Option<U256>, Option<DiagnosticKind>) {
    let analysis = Analysis::of_source(source).run();
    let binder = analysis.binder();
    let diagnostics = &analysis.diagnostics;
    let contract = analysis.find_contract(name);
    let base_slot = match binder
        .find_definition_by_id(contract.id())
        .expect("contract definition is registered")
    {
        Definition::Contract(contract_definition) => contract_definition.base_slot,
        _ => panic!("expected a contract definition"),
    };
    (base_slot, diagnostic_kind(diagnostics))
}

/// Folds a fixed-size-array length through the real pipeline, returning the
/// computed `FixedSizeArrayType.size` together with the diagnostic emitted, if any.
/// `context` holds any contract-level constants the length references;
/// `array_type` is the variable type (e.g. `"uint256[10 / B]"`). A rejected
/// length reads back as `0`, same as a length that genuinely folds to `0`.
fn folded_array_length(context: &str, array_type: &str) -> (U256, Option<DiagnosticKind>) {
    let source = format!(
        r#"
        contract Test {{
            {context}
            {array_type} sized_array;
        }}
        "#
    );

    let analysis = Analysis::of_source(&source).run();
    let binder = analysis.binder();
    let types = analysis.types();
    let diagnostics = &analysis.diagnostics;

    let contract = analysis.find_contract("Test");
    let state_variable = contract
        .members
        .iter()
        .find_map(|member| match member {
            ir::ContractMember::StateVariableDefinition(state_variable)
                if state_variable.name.unparse() == "sized_array" =>
            {
                Some(state_variable)
            }
            _ => None,
        })
        .expect("`sized_array` state variable not found");

    let type_id = binder
        .node_typing(state_variable.id())
        .as_type_id()
        .expect("state variable has a resolved type");
    let size = match types.get_type_by_id(type_id) {
        Type::FixedSizeArray(FixedSizeArrayType { size, .. }) => *size,
        other => panic!("expected a FixedSizeArray type, got {other:?}"),
    };
    (size, diagnostic_kind(diagnostics))
}

/// Collects the `FunctionCallExpression` of each expression statement in the
/// body of `function` within `contract`, in source order.
fn call_expressions<'a>(
    analysis: &'a Analysis,
    contract: &str,
    function: &str,
) -> Vec<&'a ir::FunctionCallExpression> {
    let c = analysis.find_contract(contract);
    let f = find_function(&c.members, function).expect("function not found");
    let body = f.body.as_ref().expect("function has a body");
    body.statements
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
