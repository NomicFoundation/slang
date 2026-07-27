use std::ops::Range;

use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::structure::{
    YulBreakContinueInForLoopInit, YulBreakContinueInForLoopPost, YulBreakContinueOutsideForLoop,
};
use slang_solidity_v2_common::nodes::NodeId;

use super::Pass;

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
}
