use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::structure::{
    CatchClauseKind, DuplicateCatchClause, InvalidCatchClauseName,
};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use crate::ir::builder::{CstToIrBuilder, Source};
use crate::ir::nodes as output;

impl<S: Source> CstToIrBuilder<'_, S> {
    pub(super) fn build_try_statement(
        &mut self,
        source: &input::TryStatement,
    ) -> output::TryStatement {
        let id = self.next_id(output::NodeKind::TryStatement);
        let range = source.calculate_text_range().unwrap_or_default();
        let expression = self.build_expression(&source.expression);
        let returns = source
            .returns
            .as_ref()
            .map(|value| self.build_returns_declaration(value));
        let body = self.build_block(&source.body);
        let catch_clauses = self.build_catch_clauses(&source.catch_clauses);

        Arc::new(output::TryStatementStruct {
            id,
            range,
            expression,
            returns,
            body,
            catch_clauses,
        })
    }

    pub(super) fn build_catch_clauses(
        &mut self,
        source: &input::CatchClauses,
    ) -> output::CatchClauses {
        if source.elements.is_empty() {
            return Arc::default();
        }

        let mut clauses = Vec::with_capacity(source.elements.len());

        // A `try` statement's catch clauses must each carry a valid selector
        // name (`Error`, `Panic`, or none for a low-level clause), and it may
        // declare at most one clause of each kind. Flag invalid names, and any
        // additional clause of a kind already seen.
        let mut seen_error = false;
        let mut seen_panic = false;
        let mut seen_low_level = false;
        for item in &source.elements {
            let Some(clause) = self.build_catch_clause(item) else {
                continue;
            };

            let (kind, seen) = match &clause.kind {
                output::CatchClauseKind::Error => (CatchClauseKind::Error, &mut seen_error),
                output::CatchClauseKind::Panic => (CatchClauseKind::Panic, &mut seen_panic),
                output::CatchClauseKind::LowLevel => {
                    (CatchClauseKind::LowLevel, &mut seen_low_level)
                }
            };
            if *seen {
                self.diagnostics.push(
                    self.file_id.to_owned(),
                    item.calculate_text_range().unwrap_or_default(),
                    DuplicateCatchClause { kind },
                );
            }
            *seen = true;

            clauses.push(clause);
        }
        Arc::from(clauses)
    }

    fn build_catch_clause(&mut self, source: &input::CatchClause) -> Option<output::CatchClause> {
        let id = self.next_id(output::NodeKind::CatchClause);
        let range = source.calculate_text_range().unwrap_or_default();
        let kind = self.build_catch_clause_kind(source)?;
        let parameters = source
            .error
            .as_ref()
            .map(|error| self.build_parameters_declaration(&error.parameters));
        let body = self.build_block(&source.body);

        Some(Arc::new(output::CatchClauseStruct {
            id,
            range,
            kind,
            parameters,
            body,
        }))
    }

    fn build_catch_clause_kind(
        &mut self,
        source: &input::CatchClause,
    ) -> Option<output::CatchClauseKind> {
        let Some(error) = &source.error else {
            return Some(output::CatchClauseKind::LowLevel);
        };

        let panic_allowed = self.language_version >= LanguageVersion::V0_8_1;
        let kind = match error
            .name
            .as_ref()
            .map(|name| self.source.text(name.range.clone()))
        {
            Some("Error") => output::CatchClauseKind::Error,
            Some("Panic") if panic_allowed => output::CatchClauseKind::Panic,
            Some(_) => {
                self.diagnostics.push(
                    self.file_id.to_owned(),
                    source.calculate_text_range().unwrap(),
                    InvalidCatchClauseName { panic_allowed },
                );
                return None;
            }
            None => output::CatchClauseKind::LowLevel,
        };
        Some(kind)
    }
}
