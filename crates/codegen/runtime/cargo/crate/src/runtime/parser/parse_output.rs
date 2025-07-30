use std::rc::Rc;

use crate::cst::{Cursor, NonterminalNode, TextIndex};
use crate::parser::ParseError;

/// The result of parsing source code using [`Parser`][`crate::parser::Parser`].
///
/// ```
/// # use slang_solidity::parser::Parser;
/// # use slang_solidity::utils::LanguageFacts;
/// #
/// # let source = "contract Foo {}";
/// # let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
/// // Initialize a parser and get some source code to parse...
/// let output = parser.parse_file_contents(&source);
///
/// for err in output.errors() {
///     // Handle any parse errors
/// }
///
/// // Get a cursor at the root of the parse tree. You can use this to navigate the tree.
/// let mut cursor = output.create_tree_cursor();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ParseOutput {
    pub(crate) tree: Rc<NonterminalNode>,
    pub(crate) errors: Vec<ParseError>,
}

impl ParseOutput {
    /// Returns the root node of the parse tree.
    pub fn tree(&self) -> &Rc<NonterminalNode> {
        &self.tree
    }

    /// Returns a list of all the parsing errors that were found when parsing the given
    /// source code.
    pub fn errors(&self) -> &Vec<ParseError> {
        &self.errors
    }

    /// Returns `true` if no parsing errors were found.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// Creates a cursor that starts at the root of the parse tree.
    pub fn create_tree_cursor(&self) -> Cursor {
        Rc::clone(&self.tree).create_cursor(TextIndex::ZERO)
    }
}
