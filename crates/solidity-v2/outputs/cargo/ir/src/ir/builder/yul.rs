use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::structure::{
    DuplicateSwitchDefaultCase, TrailingSwitchValueCase,
};
use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use crate::ir::builder::{CstToIrBuilder, Source};
use crate::ir::nodes as output;

impl<S: Source> CstToIrBuilder<'_, S> {
    pub(super) fn build_yul_switch_statement(
        &mut self,
        source: &input::YulSwitchStatement,
    ) -> output::YulSwitchStatement {
        let id = self.next_id(output::NodeKind::YulSwitchStatement);
        let range = source.calculate_text_range().unwrap_or_default();
        let expression = self.build_yul_expression(&source.expression);

        let mut value_cases: Vec<output::YulValueCase> = Vec::new();
        let mut default_case: Option<output::YulDefaultCase> = None;
        for case in &source.cases.elements {
            match case {
                input::YulSwitchCase::YulValueCase(value_case) => {
                    if default_case.is_some() {
                        self.report(&value_case.case_keyword, TrailingSwitchValueCase);
                    }
                    value_cases.push(self.build_yul_value_case(value_case));
                }
                input::YulSwitchCase::YulDefaultCase(default) => {
                    if default_case.is_some() {
                        self.report(&default.default_keyword, DuplicateSwitchDefaultCase);
                    } else {
                        default_case = Some(self.build_yul_default_case(default));
                    }
                }
            }
        }

        Arc::new(output::YulSwitchStatementStruct {
            id,
            range,
            expression,
            value_cases: value_cases.into(),
            default_case,
        })
    }
}
