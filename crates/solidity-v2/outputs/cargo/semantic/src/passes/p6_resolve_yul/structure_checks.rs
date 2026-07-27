use std::ops::Range;

use ruint::aliases::U256;
use slang_solidity_v2_common::collections::Set;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::structure::{
    DuplicateYulSwitchCase, YulBreakContinueInForLoopInit, YulBreakContinueInForLoopPost,
    YulBreakContinueOutsideForLoop, YulFunctionInForLoopInit,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::types::literals::{
    value_of_hex_string_literals, value_of_string_literals, yul_literal_value,
};

/// Which clause of a Yul for-loop the traversal is currently inside.
///
/// `break`/`continue` are only allowed in the loop body; a function definition
/// resets the context to [`YulForLoopClause::None`] because a loop does not
/// extend into functions declared inside it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum YulForLoopClause {
    /// Not inside any for-loop clause (or past a function boundary).
    None,
    /// Inside the init block (the first block of the for-loop).
    Init,
    /// Inside the post block (the third block of the for-loop).
    Post,
    /// Inside the body block (the fourth block of the for-loop).
    Body,
}

impl Pass<'_> {
    /// Reports every `case` in a Yul `switch` whose value duplicates an earlier
    /// one. Cases are compared by the 256-bit word each literal evaluates to,
    /// so `0`, `0x0` and `""` all collide.
    pub(super) fn check_duplicate_switch_cases(&mut self, node: &ir::YulSwitchStatement) {
        let mut seen: Set<U256> = Set::default();
        for value_case in node.value_cases.iter() {
            let Some(value) = yul_literal_value(&value_case.value) else {
                continue;
            };
            if !seen.insert(value) {
                let file_id = self
                    .file_node_mapper
                    .file_id_from_node_id(value_case.id())
                    .to_owned();
                self.diagnostics.push(
                    file_id,
                    value_case.range.clone(),
                    DuplicateYulSwitchCase {
                        value: format_yul_literal(&value_case.value),
                    },
                );
            }
        }
    }

    /// Reports a `break`/`continue` keyword that is not directly inside a
    /// for-loop body, according to the current [`YulForLoopClause`].
    pub(super) fn check_break_continue_position(
        &mut self,
        keyword: &str,
        node_id: NodeId,
        range: &Range<usize>,
    ) {
        let kind: Option<DiagnosticKind> = match self.for_loop_clause {
            YulForLoopClause::Body => None,
            YulForLoopClause::None => Some(
                YulBreakContinueOutsideForLoop {
                    keyword: keyword.to_owned(),
                }
                .into(),
            ),
            YulForLoopClause::Init => Some(
                YulBreakContinueInForLoopInit {
                    keyword: keyword.to_owned(),
                }
                .into(),
            ),
            YulForLoopClause::Post => Some(
                YulBreakContinueInForLoopPost {
                    keyword: keyword.to_owned(),
                }
                .into(),
            ),
        };

        if let Some(kind) = kind {
            let file_id = self
                .file_node_mapper
                .file_id_from_node_id(node_id)
                .to_owned();
            self.diagnostics.push(file_id, range.clone(), kind);
        }
    }

    /// Reports a Yul function definition declared directly in a for-loop init
    /// block (i.e. when the current clause is [`YulForLoopClause::Init`]).
    pub(super) fn check_function_definition_position(
        &mut self,
        node_id: NodeId,
        range: &Range<usize>,
    ) {
        if self.for_loop_clause == YulForLoopClause::Init {
            let file_id = self
                .file_node_mapper
                .file_id_from_node_id(node_id)
                .to_owned();
            self.diagnostics
                .push(file_id, range.clone(), YulFunctionInForLoopInit);
        }
    }
}

/// Formats a Yul literal for the duplicate-case message: the original source
/// text for numbers and booleans, and the decoded content (without quotes) for
/// strings.
fn format_yul_literal(literal: &ir::YulLiteral) -> String {
    match literal {
        ir::YulLiteral::TrueKeyword(_)
        | ir::YulLiteral::FalseKeyword(_)
        | ir::YulLiteral::DecimalLiteral(_)
        | ir::YulLiteral::HexLiteral(_) => literal.unparse().to_owned(),
        ir::YulLiteral::StringLiteral(literal) => {
            String::from_utf8_lossy(&value_of_string_literals(std::slice::from_ref(literal)))
                .into_owned()
        }
        ir::YulLiteral::HexStringLiteral(literal) => {
            String::from_utf8_lossy(&value_of_hex_string_literals(std::slice::from_ref(literal)))
                .into_owned()
        }
    }
}
