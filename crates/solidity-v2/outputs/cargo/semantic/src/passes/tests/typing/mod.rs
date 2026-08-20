//! Tests for the typing computed by the resolution pass, split by the
//! typing rule under test. The scaffolding they share lives here.

mod contract_members;
mod conversions;
mod literals;
mod meta_types;
mod overloads;

use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::{self, NodeIdentity};

use super::{Analyse, Analysis, diagnostic_kind, diagnostic_kinds, find_function};
use crate::binder::Binder;
use crate::types::{Type, TypeRegistry};

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

/// Configures the typing of one or more expressions. Each is wrapped in a
/// no-op expression statement inside the body of a `__test()` function of a
/// synthesized `Test` contract, so a typing comes back for every one of them,
/// in the order they were given. Defaults to the latest language version and to no
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

    /// Runs the passes over the synthesized source, without asserting on the
    /// diagnostics.
    fn run(&self) -> Analysis {
        let expression_statements = self
            .expressions
            .iter()
            .map(|expr| format!("{expr};"))
            .collect::<Vec<_>>()
            .join("\n");
        let source = format!(
            r#"
            contract Test {{
                {members}
                function __test() internal {{
                    {expression_statements}
                }}
            }}
            "#,
            members = self.members.unwrap_or(""),
        );

        Analysis::of_source(&source)
            .version(self.language_version)
            .run(Analyse::References)
    }

    /// The typing of every expression, in the order they were given, together
    /// with the registry the passes populated. An expression whose typing
    /// isn't `Resolved` comes back as `None`.
    fn into_types(self) -> (Vec<Option<Type>>, TypeRegistry) {
        let analysis = self.run().expect_no_diagnostics();
        let typings = expression_statement_types(
            analysis.function_body("Test", "__test"),
            analysis.binder(),
            analysis.types(),
        );

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

    /// The diagnostic the passes reported for the expressions, if any. Unlike
    /// the typing terminals this one doesn't assert the passes were quiet:
    /// being diagnosed is the point. Panics if more than one was reported.
    fn into_diagnostic(self) -> Option<DiagnosticKind> {
        diagnostic_kind(&self.run().diagnostics)
    }
}
