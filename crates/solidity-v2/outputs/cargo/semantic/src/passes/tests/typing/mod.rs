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

/// Wraps `members` in the synthesized `Test` contract every source in this
/// module is built around, so the tests agree on what surrounds the code under
/// test.
fn test_contract(members: &str) -> String {
    format!(
        r#"
        contract Test {{
            {members}
        }}
        "#
    )
}

/// Configures the typing of one or more expressions. Each is wrapped in a
/// no-op expression statement inside the body of a `__test()` function of a
/// [`test_contract`], so a typing comes back for every one of them, in the
/// order they were given. Defaults to the latest language version and to no
/// contract members beyond `__test()` itself.
///
/// Reach for [`expression`] instead of [`expressions`] when a single one will
/// do, which is the common case.
struct ExpressionTyping<'a> {
    expressions: Vec<&'a str>,
    members: Option<&'a str>,
    language_version: LanguageVersion,
}

/// Starts configuring the typing of a single `expr`.
fn expression(expr: &str) -> ExpressionTyping<'_> {
    expressions(&[expr])
}

/// Starts configuring the typing of several expressions, which share one
/// `__test()` body and hence one scope.
fn expressions<'a>(expressions: &[&'a str]) -> ExpressionTyping<'a> {
    ExpressionTyping {
        expressions: expressions.to_vec(),
        members: None,
        language_version: LanguageVersion::LATEST,
    }
}

impl<'a> ExpressionTyping<'a> {
    /// Contract-level setup the expressions resolve against: state variables,
    /// nested struct definitions, sibling member functions, etc. It is
    /// inserted ahead of the `__test()` definition.
    fn with_members(mut self, members: &'a str) -> Self {
        self.members = Some(members);
        self
    }

    fn version(mut self, language_version: LanguageVersion) -> Self {
        self.language_version = language_version;
        self
    }

    /// The typing of every expression, in the order they were given, together
    /// with the registry the passes populated. An expression whose typing
    /// isn't `Resolved` comes back as `None`.
    fn into_types(self) -> (Vec<Option<Type>>, TypeRegistry) {
        let expression_statements = self
            .expressions
            .iter()
            .map(|expr| format!("{expr};"))
            .collect::<Vec<_>>()
            .join("\n");
        let source = test_contract(&format!(
            r#"
            {members}
            function __test() internal {{
                {expression_statements}
            }}
            "#,
            members = self.members.unwrap_or(""),
        ));

        let analysis = Analysis::of_source(&source)
            .version(self.language_version)
            .run()
            .expect_no_diagnostics();

        let contract = analysis.find_contract("Test");
        let function =
            find_function(&contract.members, "__test").expect("__test function not found");
        let block = function.body.as_ref().expect("__test has a body");

        let typings = expression_statement_types(block, analysis.binder(), analysis.types());

        (typings, analysis.into_type_registry())
    }

    /// [`Self::into_types`] for a lone expression. Panics unless exactly one
    /// was given, so a caller can't quietly assert on the first of several.
    fn into_type(self) -> (Option<Type>, TypeRegistry) {
        assert_eq!(
            1,
            self.expressions.len(),
            "`into_type` needs exactly one expression"
        );
        let (typings, types) = self.into_types();
        let typing = typings.into_iter().next().expect("one expression");
        (typing, types)
    }

    /// [`Self::into_type`], panicking if the expression didn't resolve to a type.
    fn into_resolved_type(self) -> (Type, TypeRegistry) {
        let (typing, types) = self.into_type();
        (
            typing.expect("expected resolved type for expression"),
            types,
        )
    }
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
/// `members` holds any contract-level constants the length references;
/// `array_type` is the variable type (e.g. `"uint256[10 / B]"`). A rejected
/// length reads back as `0`, same as a length that genuinely folds to `0`.
fn folded_array_length(members: &str, array_type: &str) -> (U256, Option<DiagnosticKind>) {
    let source = test_contract(&format!("{members}\n{array_type} sized_array;"));

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
