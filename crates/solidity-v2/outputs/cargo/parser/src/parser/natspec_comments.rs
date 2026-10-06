//! The `NatSpec` comments read by the lexer, until the parser attaches them to the CST nodes they
//! document.

use std::cell::RefCell;
use std::ops::Range;

use slang_solidity_v2_cst::structured_cst::natspec::NatSpec;

/// The `NatSpec` comments of a source file that document a token, recorded and not taken yet, in
/// source order.
///
/// They're shared, through [`NatSpecComments::split`], between the token stream, which records
/// them, and the grammar actions, which take them as they reduce the documentable nodes.
#[derive(Debug, Default)]
pub(crate) struct NatSpecComments {
    comments: RefCell<Vec<RecordedComment>>,
}

#[derive(Debug)]
struct RecordedComment {
    range: Range<usize>,
    /// Start of the token the comment documents.
    documented_start: usize,
}

impl NatSpecComments {
    /// Splits the comments into the side recording them, and the side taking them.
    pub(crate) fn split(&self) -> (NatSpecProducer<'_>, NatSpecConsumer<'_>) {
        (
            NatSpecProducer {
                comments: &self.comments,
                pending: None,
            },
            NatSpecConsumer {
                comments: &self.comments,
            },
        )
    }
}

/// Records the `NatSpec` comments read by the lexer, with the token each one documents.
#[derive(Debug)]
pub(crate) struct NatSpecProducer<'a> {
    comments: &'a RefCell<Vec<RecordedComment>>,
    /// The last comment pushed, until the token it documents is reported.
    ///
    /// A trailing comment, with no token after it, stays here and is dropped.
    pending: Option<Range<usize>>,
}

impl NatSpecProducer<'_> {
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
    /// Never inlined for the same reason as [`NatSpecProducer::push`].
    #[inline(never)]
    pub(crate) fn document(&mut self, token_start: usize) {
        let range = self
            .pending
            .take()
            .expect("a comment to have been pushed before the token it documents");

        let mut comments = self.comments.borrow_mut();
        debug_assert!(
            comments
                .last()
                .is_none_or(|last| last.documented_start <= range.start),
            "comments must be recorded in source order"
        );

        comments.push(RecordedComment {
            range,
            documented_start: token_start,
        });
    }
}

/// Takes the `NatSpec` comments recorded by the [`NatSpecProducer`], as the parser reduces the
/// nodes they document.
#[derive(Debug)]
pub(crate) struct NatSpecConsumer<'a> {
    comments: &'a RefCell<Vec<RecordedComment>>,
}

impl NatSpecConsumer<'_> {
    /// Takes the `NatSpec` comment documenting the node being reduced, which spans from `start` to
    /// `end`, if any.
    ///
    /// Must be called as each documentable node is reduced: the comments inside the node that are
    /// still recorded are dropped, since they document something that isn't documentable.
    pub(crate) fn take(&self, start: usize, end: usize) -> Option<NatSpec> {
        let mut comments = self.comments.borrow_mut();
        if comments.is_empty() {
            // Quick exit, helps to inline this function but not the slow path of `take_documenting`.
            return None;
        }
        take_documenting(&mut comments, start, end)
    }
}

/// See [`NatSpecConsumer::take`]. From the top of the stack, the recorded comments are:
///
/// - The comment documenting the parser's lookahead token, which the lexer may have read after
///   the node. It documents a later node, so it's kept.
/// - The comments inside the node. The documentable nodes inside it were reduced before and took
///   theirs, so these document something that isn't documentable, and are dropped.
/// - The comment documenting the token the node starts with, which is taken.
/// - The comments before the node, documenting a node containing it that isn't reduced yet.
///
/// Never inlined on purpose: it only runs when comments are recorded, and inlining it into the
/// grammar actions would grow LALRPOP's reduction code, which is part of the parsing loop.
#[inline(never)]
fn take_documenting(
    comments: &mut Vec<RecordedComment>,
    start: usize,
    end: usize,
) -> Option<NatSpec> {
    let lookahead = comments.pop_if(|comment| comment.documented_start >= end);
    debug_assert!(
        comments
            .last()
            .is_none_or(|comment| comment.documented_start < end),
        "only the lookahead token can be documented after the node"
    );

    // Drop the comments inside the node.
    while comments
        .pop_if(|comment| comment.documented_start > start)
        .is_some()
    {}

    let natspec = comments
        .pop_if(|comment| comment.documented_start == start)
        .map(|comment| NatSpec {
            range: comment.range,
        });

    if let Some(lookahead) = lookahead {
        comments.push(lookahead);
    }

    natspec
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::{NatSpecComments, NatSpecConsumer, NatSpecProducer};

    /// Records a comment at `range`, documenting the token at `token_start`.
    fn record(producer: &mut NatSpecProducer<'_>, range: Range<usize>, token_start: usize) {
        producer.push(range);
        producer.document(token_start);
    }

    fn take(consumer: &NatSpecConsumer<'_>, start: usize, end: usize) -> Option<Range<usize>> {
        consumer.take(start, end).map(|natspec| natspec.range)
    }

    #[test]
    fn takes_the_comment_documenting_the_node_start() {
        let comments = NatSpecComments::default();
        let (mut producer, consumer) = comments.split();
        record(&mut producer, 0..5, 6);

        assert_eq!(take(&consumer, 6, 20), Some(0..5));
        assert_eq!(take(&consumer, 6, 20), None);
    }

    #[test]
    fn keeps_the_comments_before_and_after_the_node() {
        let comments = NatSpecComments::default();
        let (mut producer, consumer) = comments.split();
        // Documents the node containing the one being reduced
        record(&mut producer, 0..5, 6);
        // Documents the node being reduced
        record(&mut producer, 10..15, 16);
        // Documents the lookahead token, after the node
        record(&mut producer, 30..35, 36);

        assert_eq!(take(&consumer, 16, 25), Some(10..15));
        assert_eq!(take(&consumer, 36, 50), Some(30..35));
        assert_eq!(take(&consumer, 6, 60), Some(0..5));
    }

    #[test]
    fn drops_the_comments_inside_the_node() {
        let comments = NatSpecComments::default();
        let (mut producer, consumer) = comments.split();
        record(&mut producer, 0..5, 6);
        // Documents a token inside the node that doesn't start a documentable node
        record(&mut producer, 10..15, 16);

        assert_eq!(take(&consumer, 6, 20), Some(0..5));
        assert_eq!(take(&consumer, 16, 18), None);

        // Even when the node itself isn't documented
        let comments = NatSpecComments::default();
        let (mut producer, consumer) = comments.split();
        record(&mut producer, 10..15, 16);

        assert_eq!(take(&consumer, 6, 20), None);
        assert_eq!(take(&consumer, 16, 18), None);
    }

    #[test]
    fn drops_a_trailing_comment() {
        let comments = NatSpecComments::default();
        let (mut producer, consumer) = comments.split();
        producer.push(0..5);

        assert_eq!(take(&consumer, 0, 10), None);
    }
}
