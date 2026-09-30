//! The `NatSpec` comments recorded while parsing, before they're attached to the CST nodes they
//! document.

use std::ops::Range;

use slang_solidity_v2_cst::structured_cst::natspec::NatSpec;

/// The `NatSpec` comments of a source file that document a token.
///
/// Only the ranges are kept, not the text.
#[derive(Debug, Default)]
pub(crate) struct NatSpecComments {
    comments: Vec<RecordedComment>,
    /// The last comment pushed, until the token it documents is reported.
    ///
    /// A trailing comment, with no token after it, stays here and is dropped.
    pending: Option<Range<usize>>,
}

#[derive(Debug)]
struct RecordedComment {
    range: Range<usize>,
    /// Start of the token the comment documents.
    documented_start: usize,
}

impl NatSpecComments {
    /// Records the `NatSpec` comment at `range`, which may document the next token.
    ///
    /// If another comment is pushed first, it replaces this one, which documents nothing and is dropped.
    ///
    /// Never inlined on purpose: the parser records comments from its token stream, which is
    /// inlined into LALRPOP's parsing loop, and inlining this there slows the whole loop down.
    #[inline(never)]
    pub(crate) fn push(&mut self, range: Range<usize>) {
        self.pending = Some(range);
    }

    /// Reports the token after the last pushed comment, starting at `token_start`, which it
    /// documents.
    ///
    /// Never inlined for the same reason as [`NatSpecComments::push`].
    #[inline(never)]
    pub(crate) fn document(&mut self, token_start: usize) {
        let range = self
            .pending
            .take()
            .expect("a comment to have been pushed before the token it documents");

        debug_assert!(
            self.comments
                .last()
                .is_none_or(|last| last.documented_start <= range.start),
            "comments must be recorded in source order"
        );

        self.comments.push(RecordedComment {
            range,
            documented_start: token_start,
        });
    }

    /// The `NatSpec` comment documenting the node starting at `start`, if any.
    ///
    /// It doesn't matter what kind of node it is.
    #[expect(
        dead_code,
        reason = "PROTOTYPE: used to attach the comments to the CST, in the next rev"
    )]
    pub(crate) fn documenting(&self, start: usize) -> Option<NatSpec> {
        let index = self
            .comments
            .binary_search_by_key(&start, |comment| comment.documented_start)
            .ok()?;

        Some(NatSpec {
            range: self.comments[index].range.clone(),
        })
    }
}
