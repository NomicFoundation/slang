//! TODO(v2): remove this temporary API once the lexer is fully integrated into the parser, and can be tested through it:

use semver::Version;
use slang_solidity::cst::{Cursor, Node, NonterminalKind, TextRangeExtensions};
use slang_solidity_v2_common::diagnostic::{Diagnostic, Severity, TextRange};

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;

pub struct Error {
    pub message: String,
    pub text_range: TextRange,
}

impl Diagnostic for Error {
    fn text_range(&self) -> TextRange {
        self.text_range.clone()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}

pub struct Comparator<'source> {
    lexer: Lexer<'source>,
    cursor: Cursor,
    errors: Vec<Error>,
}

impl<'source> Comparator<'source> {
    pub fn compare_with_v1_output(
        language_version: Version,
        source: &'source str,
        root_cursor: Cursor,
    ) -> Vec<Error> {
        let mut instance = Self {
            lexer: Lexer::new(
                ContextKind::Solidity,
                source,
                language_version.try_into().unwrap(),
            ),
            cursor: root_cursor,
            errors: vec![],
        };

        instance.check_node();

        while let Some(lexeme) = instance.lexer.next_lexeme() {
            instance.add_error(format!(
                "Lexer v2 produced an extra lexeme at the end of input: {lexeme:?}",
            ));
        }

        instance.errors
    }

    fn check_node(&mut self) {
        match self.cursor.node() {
            Node::Nonterminal(nonterminal) => {
                // TODO(v2): current grammar definition has a bug, where it doesn't include all referenced terminals
                // in every lexical context (e.g. allows Yul nonterminals to reference Solidity terminals, like `StringLiteral`).
                // This is to be fixed in the next PR, but until then, let's "simulate" the switch,
                // and skip the nodes that switch contexts for now:
                match nonterminal.kind {
                    NonterminalKind::Pragma => {
                        self.lexer.switch_context(ContextKind::Pragma);
                        self.lexer.bump(self.cursor.text_range().utf8().len());
                        self.lexer.switch_context(ContextKind::Solidity);
                        return;
                    }
                    NonterminalKind::AssemblyStatement => {
                        self.lexer.switch_context(ContextKind::Yul);
                        self.lexer.bump(self.cursor.text_range().utf8().len());
                        self.lexer.switch_context(ContextKind::Solidity);
                        return;
                    }
                    _ => {}
                }

                if self.cursor.go_to_first_child() {
                    self.check_node();

                    while self.cursor.go_to_next_sibling() {
                        self.check_node();
                    }

                    self.cursor.go_to_parent();
                }
            }

            Node::Terminal(terminal) => match self.lexer.next_lexeme() {
                None => {
                    self.add_error(
                        "V2 lexer stopped producing lexemes, but v1 tree still has more terminals"
                            .to_string(),
                    );
                }
                Some(lexeme) => {
                    let v1_range = self.cursor.text_range().utf8();
                    let v2_range = lexeme.range;

                    let v1_kind = terminal.kind.as_ref();
                    let v2_kind: &'static str = lexeme.kind.into();

                    let v1_output = format!("{v1_kind:?} @ {v1_range:?}");
                    let v2_output = format!("{v2_kind:?} @ {v2_range:?}");

                    if !match (v1_kind, v2_kind) {
                        ("Identifier", v2_kind) if v2_kind.ends_with("_Unreserved") => true,
                        (v1_kind, v2_kind) if format!("{v1_kind}_Reserved") == v2_kind => true,
                        (v1_kind, v2_kind) if format!("{v1_kind}_Unreserved") == v2_kind => true,
                        (v1_kind, v2_kind) if v1_kind == v2_kind => true,
                        _ => false,
                    } {
                        self.add_error(format!("V2 lexer produced a lexeme '{v2_output}', but v1 tree has terminal '{v1_output}'"));
                    }
                }
            },
        }
    }

    fn add_error(&mut self, message: String) {
        self.errors.push(Error {
            message,
            text_range: TextRange::from_bytes_range(
                self.cursor.text_range().start.utf8..self.cursor.text_range().end.utf8,
            ),
        });
    }
}
