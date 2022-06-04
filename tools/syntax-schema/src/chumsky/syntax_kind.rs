use std::collections::BTreeSet;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SyntaxKindKind {
    Punctuation,
    Token,
    Keyword,
    StructuredToken,
    Production,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyntaxKindName {
    kind: SyntaxKindKind,
    name: String,
}

impl SyntaxKindName {
    pub fn production(name: String) -> Self {
        Self {
            kind: SyntaxKindKind::Production,
            name: format!("P_{}", name.to_case(Case::ScreamingSnake)),
        }
    }

    pub fn structured_token(name: String) -> Self {
        Self {
            kind: SyntaxKindKind::StructuredToken,
            name: format!("S_{}", name.to_case(Case::ScreamingSnake)),
        }
    }

    pub fn token(name: String) -> Self {
        Self {
            kind: SyntaxKindKind::Token,
            name: format!("T_{}", name.to_case(Case::ScreamingSnake)),
        }
    }

    pub fn terminal(name: String) -> Self {
        let char_names: Vec<Option<&str>> = name.chars().map(default_name_of_character).collect();

        if char_names.iter().any(Option::is_none) {
            let mut result = vec!["K".to_owned()];

            // Escape characters not valid in identifiers
            // Q: do we need to handle LC/UC distinctions?
            let mut need_underscore = true;
            let mut last_was_lowercase = false;
            for c in name.chars() {
                if c.is_ascii_alphanumeric() {
                    need_underscore |= last_was_lowercase && c.is_uppercase();
                    if need_underscore {
                        result.push("_".to_owned());
                    }
                    result.push(c.to_uppercase().to_string());
                    last_was_lowercase = c.is_lowercase();
                    need_underscore = false;
                } else if let Some(name) = default_name_of_character(c) {
                    result.push("_".to_owned());
                    result.push(name.to_owned());
                    need_underscore = true;
                    last_was_lowercase = false;
                } else {
                    result.push("_".to_owned());
                    // NO, doesn't work, but for now panic is ok
                    result.push(c.escape_unicode().to_string());
                    need_underscore = true;
                    last_was_lowercase = false;
                }
            }

            Self {
                kind: SyntaxKindKind::Keyword,
                name: result.join(""),
            }
        } else {
            Self {
                kind: SyntaxKindKind::Punctuation,
                name: char_names
                    .into_iter()
                    .map(Option::unwrap)
                    .collect::<Vec<_>>()
                    .join("_"),
            }
        }
    }

    pub fn is_token(&self) -> bool {
        self.kind == SyntaxKindKind::Keyword
            || self.kind == SyntaxKindKind::Token
            || self.kind == SyntaxKindKind::Punctuation
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn qualified_name(&self) -> String {
        format!(
            "{}Kind::{}",
            if self.is_token() { "Token" } else { "Rule" },
            self.name
        )
    }
}

fn default_name_of_character(c: char) -> Option<&'static str> {
    DEFAULT_TOKEN_NAMES
        .binary_search_by(|probe| probe.0.cmp(&c))
        .ok()
        .map(|i| DEFAULT_TOKEN_NAMES[i].1)
}

const DEFAULT_TOKEN_NAMES: &[(char, &str)] = &[
    ('\t', "TAB"),
    ('\n', "LF"),
    ('\r', "CR"),
    (' ', "SPACE"),
    ('!', "BANG"),
    ('"', "DOUBLE_QUOTE"),
    ('#', "HASH"),
    ('$', "DOLLAR"),
    ('%', "PERCENT"),
    ('&', "AMPERSAND"),
    ('\'', "QUOTE"),
    ('(', "OPEN_PAREN"),
    (')', "CLOSE_PAREN"),
    ('*', "STAR"),
    ('+', "PLUS"),
    (',', "COMMA"),
    ('-', "MINUS"),
    ('.', "PERIOD"),
    ('/', "SLASH"),
    (':', "COLON"),
    (';', "SEMICOLON"),
    ('<', "LESS"),
    ('=', "EQUAL"),
    ('>', "GREATER"),
    ('?', "QUESTION"),
    ('@', "AT"),
    ('[', "OPEN_BRACKET"),
    ('\\', "BACKSLASH"),
    (']', "CLOSE_BRACKET"),
    ('^', "CARET"),
    ('_', "UNDERSCORE"),
    ('`', "BACKQUOTE"),
    ('{', "OPEN_BRACE"),
    ('|', "BAR"),
    ('}', "CLOSE_BRACE"),
    ('~', "TILDE"),
    ('«', "OPEN_DOUBLE_ANGLE"),
    ('¬', "NOT"),
    ('»', "CLOSE_DOUBLE_ANGLE"),
    ('…', "ELLIPSIS"),
];

impl Grammar {
    pub(super) fn generate_syntax_kinds(&self) -> TokenStream {
        let mut syntax_kinds = BTreeSet::new();
        for p in self.productions.values().flatten() {
            p.collect_syntax_kind_names(self, &mut syntax_kinds)
        }

        let mut syntax_kinds: Vec<SyntaxKindName> = syntax_kinds.into_iter().collect();
        syntax_kinds.sort();

        let rule_kinds = syntax_kinds
            .iter()
            .filter(|n| !n.is_token())
            .map(|s| format_ident!("{}", s.name()));

        let token_kinds = syntax_kinds
            .iter()
            .filter(|n| n.is_token())
            .map(|s| format_ident!("{}", s.name()));

        quote!(
            pub enum RuleOrToken {
                Rule {
                    kind: RuleKind,
                    children: Vec<RuleOrToken>,
                },
                Token {
                    kind: TokenKind,
                    span: Range<usize>,
                },
            }

            fn rule(kind: RuleKind, children: Vec<RuleOrToken>) -> RuleOrToken {
                RuleOrToken::Rule { kind, children }
            }

            fn token(kind: TokenKind, span: Range<usize>) -> RuleOrToken {
                RuleOrToken::Token { kind, span }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[allow(non_camel_case_types)]
            pub enum RuleKind {
                #(#rule_kinds),*
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[allow(non_camel_case_types)]
            pub enum TokenKind {
                #(#token_kinds),*
            }
        )
    }
}

impl Production {
    fn syntax_kind_name(&self) -> SyntaxKindName {
        if self.is_token {
            let expr = self.expression_to_generate();
            if expr.config.preserve_token_structure.unwrap_or_default() {
                SyntaxKindName::structured_token(self.name.clone())
            } else {
                SyntaxKindName::token(self.name.clone())
            }
        } else {
            SyntaxKindName::production(self.name.clone())
        }
    }

    fn collect_syntax_kind_names(&self, grammar: &Grammar, accum: &mut BTreeSet<SyntaxKindName>) {
        let expr = self.expression_to_generate();
        if !self.is_token || expr.config.preserve_token_structure.unwrap_or_default() {
            accum.insert(self.syntax_kind_name());
            expr.collect_syntax_kinds(grammar, accum);
        }
    }
}

impl Expression {
    fn collect_syntax_kinds(&self, grammar: &Grammar, accum: &mut BTreeSet<SyntaxKindName>) {
        match &self.ebnf {
            EBNF::Repeat(EBNFRepeat { expr, .. }) => expr.collect_syntax_kinds(grammar, accum),
            EBNF::Choice(exprs) | EBNF::Sequence(exprs) => {
                for e in exprs {
                    e.collect_syntax_kinds(grammar, accum)
                }
            }
            EBNF::Difference(EBNFDifference { minuend, .. }) => {
                minuend.collect_syntax_kinds(grammar, accum);
            }
            EBNF::Terminal(string) => {
                accum.insert(SyntaxKindName::terminal(string.clone()));
            }
            EBNF::Reference(name) => {
                if let Some(production) = grammar.get_production(name) {
                    if production.is_token {
                        accum.insert(production.syntax_kind_name());
                    }
                }
            }
            EBNF::End | EBNF::Not(_) | EBNF::Range(_) => {}
        };
    }
}
