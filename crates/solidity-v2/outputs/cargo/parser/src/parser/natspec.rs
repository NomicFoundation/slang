//! The `NatSpec` comments of a source file
//!
//! The [`NonTriviaTokens`] feeds the parser its tokens, reading each comment's lexemes with a
//! [`CommentReader`], and pushes the comment on the [`NatSpecStack`] with the token after it, which
//! it documents. The parser then pops it from the stack as it reduces the node starting with that
//! token.

use std::cell::RefCell;
use std::ops::Range;

use slang_solidity_v2_cst::structured_cst::natspec::NatSpec;

use crate::lexer::{Lexeme, LexemeKind, Lexer};

/// The stack of `NatSpec` comments of a source file that document a token, pushed and not popped
/// yet, in source order.
///
/// It's shared between [`NonTriviaTokens`] and the grammar actions:
///
/// - [`NonTriviaTokens`] pushes each comment with the start of the non-trivia token after it,
///   which it documents (see [`NatSpecStack::attach_natspec_to`]). A comment followed by another
///   comment, or by no token at all, documents nothing, and is dropped without being pushed.
/// - Each documentable node pops, as it's reduced, the comment documenting the token it starts with
///   (see [`NatSpecStack::pop_for_token_at_range`]). The comments documenting a token inside the
///   node are dropped then: that token doesn't start a documentable node, or its node would have
///   popped them already.
///
/// The comments documenting a token outside all documentable nodes are never popped.
#[derive(Debug, Default)]
pub(crate) struct NatSpecStack {
    comments: RefCell<Vec<DocumentingComment>>,
}

/// A `NatSpec` comment pushed on the [`NatSpecStack`], with the token it documents.
#[derive(Debug)]
struct DocumentingComment {
    natspec: NatSpec,
    /// Start of the token the comment documents: the first non-trivia token after it.
    documented_token_start: usize,
}

impl NatSpecStack {
    /// Pushes `natspec`, documenting the token starting at `token_start`.
    ///
    /// Never inlined on purpose: the parser pushes comments from [`NonTriviaTokens`], which is
    /// inlined into LALRPOP's parsing loop, and inlining this there slows the whole loop down.
    #[inline(never)]
    pub(crate) fn attach_natspec_to(&self, natspec: NatSpec, token_start: usize) {
        let mut comments = self.comments.borrow_mut();
        debug_assert!(
            comments
                .last()
                .is_none_or(|last| last.documented_token_start <= natspec.range.start),
            "comments must be pushed in source order"
        );

        comments.push(DocumentingComment {
            natspec,
            documented_token_start: token_start,
        });
    }

    /// Pops the `NatSpec` comment documenting the node being reduced, which spans from `start` to
    /// `end`, if any.
    ///
    /// Must be called as each documentable node is reduced: the comments inside the node that are
    /// still pushed are dropped, since they document something that isn't documentable.
    pub(crate) fn pop_for_token_at_range(&self, start: usize, end: usize) -> Option<NatSpec> {
        let mut comments = self.comments.borrow_mut();
        if comments.is_empty() {
            // Quick exit, helps to inline this function but not the slow path of `pop_documenting`.
            return None;
        }
        pop_documenting(&mut comments, start, end)
    }
}

/// See [`NatSpecStack::pop_for_token_at_range`]. From the top of the stack, the pushed comments
/// are:
///
/// - The comment documenting the parser's lookahead token, which the lexer may have read after
///   the node. It documents a later node, so it's kept.
/// - The comments inside the node. The documentable nodes inside it were reduced before and popped
///   theirs, so these document something that isn't documentable, and are dropped.
/// - The comment documenting the token the node starts with, which is popped.
/// - The comments before the node, documenting a node containing it that isn't reduced yet.
///
/// Never inlined on purpose: it only runs when comments are pushed, and inlining it into the
/// grammar actions would grow LALRPOP's reduction code, which is part of the parsing loop.
#[inline(never)]
fn pop_documenting(
    comments: &mut Vec<DocumentingComment>,
    start: usize,
    end: usize,
) -> Option<NatSpec> {
    let lookahead = comments.pop_if(|comment| comment.documented_token_start >= end);
    debug_assert!(
        comments
            .last()
            .is_none_or(|comment| comment.documented_token_start < end),
        "only the lookahead token can be documented after the node"
    );

    // Drop the comments inside the node, documenting tokens that don't start a documentable node.
    while comments
        .pop_if(|comment| comment.documented_token_start > start)
        .is_some()
    {}

    let natspec = comments
        .pop_if(|comment| comment.documented_token_start == start)
        .map(|comment| comment.natspec);

    if let Some(lookahead) = lookahead {
        comments.push(lookahead);
    }

    natspec
}

/// Reads a `NatSpec` comment from its lexemes, up to the token it documents.
#[derive(Debug)]
struct CommentReader {
    range: Range<usize>,
    state: ReaderState,
}

/// Where a [`CommentReader`] is in the comment, which decides what the next lexeme does to it.
#[derive(Clone, Copy, Debug)]
enum ReaderState {
    /// Right after a `///` line, before its line break.
    /// 
    /// Only `///` comments can be in this state.
    AfterLine,
    /// After the line break of a `///` line, and maybe some indentation: another `///` line
    /// continues the comment.
    /// 
    /// Only `///` comments can be in this state.
    AfterLineBreak,
    /// The comment is complete: either a `/** ... */` one, or `///` lines followed by something
    /// else than another `///` line.
    Complete,
}

impl CommentReader {
    fn new(lexeme: &Lexeme) -> Self {
        debug_assert!(
            lexeme.kind.is_natspec_comment(),
            "a comment starts with a NatSpec lexeme"
        );

        let state = match lexeme.kind {
            LexemeKind::SingleLineNatSpecComment
            | LexemeKind::PragmaSingleLineNatSpecComment
            | LexemeKind::YulSingleLineNatSpecComment => ReaderState::AfterLine,
            _ => ReaderState::Complete,
        };
        Self {
            range: lexeme.range.clone(),
            state,
        }
    }

    /// Updates the comment with the trivia `lexeme` read after it.
    ///
    /// The lexer reads each `///` line on its own, so consecutive ones, only separated by a line
    /// break and indentation, are merged into a single comment, which includes the line break
    /// after its last line. Any other `NatSpec` comment replaces this one, which is dropped
    /// without documenting anything.
    fn read(&mut self, lexeme: &Lexeme) {
        use ReaderState::{AfterLine, AfterLineBreak, Complete};

        debug_assert!(
            lexeme.kind.is_trivia(),
            "only trivia is read into a comment"
        );

        match (&lexeme.kind, self.state) {
            (
                LexemeKind::SingleLineNatSpecComment
                | LexemeKind::PragmaSingleLineNatSpecComment
                | LexemeKind::YulSingleLineNatSpecComment,
                AfterLineBreak,
            ) => {
                // Line comments separated by a line break get merged
                self.range.end = lexeme.range.end;
                self.state = AfterLine;
            }
            // Either a multi-line comment, or a single line that is `Complete` start a new comment
            (kind, _) if kind.is_natspec_comment() => *self = Self::new(lexeme),

            (
                LexemeKind::EndOfLine | LexemeKind::PragmaEndOfLine | LexemeKind::YulEndOfLine,
                AfterLine,
            ) => {
                // An end of line after a single line comment is part of the comment, and expects another line
                self.range.end = lexeme.range.end;
                self.state = AfterLineBreak;
            }
            // Whitespaces get ignored
            (
                LexemeKind::Whitespace | LexemeKind::PragmaWhitespace | LexemeKind::YulWhitespace,
                _,
            ) => {}
            // A blank line, or a regular comment complete any kind of comment
            (_, _) => self.state = Complete,
        }
    }
}

/// A `NonTriviaTokens` filters trivia tokens out of a lexer, collecting NatSpec comments into a 
/// `NatSpecStack` whenever they are followed by a non trivia token.
pub(crate) struct NonTriviaTokens<'source, 'natspec> {
    lexer: Lexer<'source>,
    natspec_stack: &'natspec NatSpecStack,
}

impl Iterator for NonTriviaTokens<'_, '_> {
    // This is the format expected by LALRPOP
    type Item = Result<(usize, LexemeKind, usize), ()>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(lexeme) = self.lexer.next_lexeme() {
            // NatSpec reading is guarded by a cheap kind check
            if lexeme.kind.is_natspec_comment() {
                // Reads the rest of the comment, and returns the token after it
                let lexeme = self.read_natspec_comment(lexeme)?;
                return Some(Ok((lexeme.range.start, lexeme.kind, lexeme.range.end)));
            }
            if lexeme.kind.is_trivia() {
                continue;
            }
            return Some(Ok((lexeme.range.start, lexeme.kind, lexeme.range.end)));
        }
        None
    }
}

impl<'source, 'natspec> NonTriviaTokens<'source, 'natspec> {
    pub(crate) fn new(lexer: Lexer<'source>, natspec_stack: &'natspec NatSpecStack) -> Self {
        Self {
            lexer,
            natspec_stack,
        }
    }

    /// Reads the `NatSpec` comment starting with `first`, and the trivia after it, up to the token
    /// it documents, which is returned. A comment followed by no token is dropped.
    ///
    /// Never inlined on purpose: it only runs for `NatSpec` comments, and inlining it into
    /// [`NonTriviaTokens::next`], which is inlined into LALRPOP's parsing loop, slows the whole
    /// loop down.
    #[inline(never)]
    fn read_natspec_comment(&mut self, first: Lexeme) -> Option<Lexeme> {
        let mut comment = CommentReader::new(&first);
        while let Some(lexeme) = self.lexer.next_lexeme() {
            if !lexeme.kind.is_trivia() {
                self.natspec_stack.attach_natspec_to(
                    NatSpec {
                        range: comment.range,
                    },
                    lexeme.range.start,
                );
                return Some(lexeme);
            }
            comment.read(&lexeme);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use slang_solidity_v2_cst::structured_cst::natspec::NatSpec;

    use super::NatSpecStack;

    fn push(stack: &NatSpecStack, range: Range<usize>, token_start: usize) {
        stack.attach_natspec_to(NatSpec { range }, token_start);
    }

    fn pop(stack: &NatSpecStack, start: usize, end: usize) -> Option<Range<usize>> {
        stack
            .pop_for_token_at_range(start, end)
            .map(|natspec| natspec.range)
    }

    #[test]
    fn pops_the_comment_documenting_the_node_start() {
        let stack = NatSpecStack::default();
        push(&stack, 0..5, 6);

        assert_eq!(pop(&stack, 6, 20), Some(0..5));
        assert_eq!(pop(&stack, 6, 20), None);
    }

    #[test]
    fn keeps_the_comments_before_and_after_the_node() {
        let stack = NatSpecStack::default();
        // Documents the node containing the one being reduced
        push(&stack, 0..5, 6);
        // Documents the node being reduced
        push(&stack, 10..15, 16);
        // Documents the lookahead token, after the node
        push(&stack, 30..35, 36);

        assert_eq!(pop(&stack, 16, 25), Some(10..15));
        assert_eq!(pop(&stack, 36, 50), Some(30..35));
        assert_eq!(pop(&stack, 6, 60), Some(0..5));
    }

    #[test]
    fn drops_the_comments_inside_the_node() {
        let stack = NatSpecStack::default();
        push(&stack, 0..5, 6);
        // Documents a token inside the node that doesn't start a documentable node
        push(&stack, 10..15, 16);

        assert_eq!(pop(&stack, 6, 20), Some(0..5));
        assert_eq!(pop(&stack, 16, 18), None);

        // Even when the node itself isn't documented
        let stack = NatSpecStack::default();
        push(&stack, 10..15, 16);

        assert_eq!(pop(&stack, 6, 20), None);
        assert_eq!(pop(&stack, 16, 18), None);
    }
}
