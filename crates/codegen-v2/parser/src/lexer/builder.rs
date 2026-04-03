use std::collections::{BTreeMap, BTreeSet};
use std::mem::discriminant;

use indexmap::IndexMap;
use language_v2_definition::model::{
    Identifier, Item, KeywordItem, KeywordScanner, Language, LexicalContext as LanguageContext,
    TokenItem, TokenScanner, TriviaItem, VersionSpecifier,
};

use crate::lexer::{Lexeme, LexerModel, LexicalContext};

pub struct LexerModelBuilder;

impl LexerModelBuilder {
    pub fn build(language: &Language) -> LexerModel {
        let contexts: Vec<LexicalContext> = language
            .contexts
            .iter()
            .map(LexicalContextBuilder::build)
            .collect();

        let all_lexeme_kinds = Self::collect_all_lexeme_kinds(&contexts);
        let trivia_lexeme_kinds = Self::collect_trivia_lexeme_kinds(&contexts);

        LexerModel {
            contexts,
            all_lexeme_kinds,
            trivia_lexeme_kinds,
        }
    }

    fn collect_all_lexeme_kinds(contexts: &[LexicalContext]) -> BTreeSet<String> {
        let mut kinds = BTreeSet::new();

        for context in contexts {
            for lexeme in &context.lexemes {
                match lexeme {
                    Lexeme::Trivia { kind, .. } => {
                        kinds.insert(kind.clone());
                    }
                    Lexeme::Token { kind, .. } => {
                        kinds.insert(kind.clone());
                    }
                    Lexeme::Keyword { kind, reserved, .. } => {
                        if match reserved {
                            VersionSpecifier::Always => true,
                            VersionSpecifier::Never => false,
                            VersionSpecifier::From { .. } => true,
                            VersionSpecifier::Till { .. } => true,
                            VersionSpecifier::Range { .. } => true,
                        } {
                            kinds.insert(format!("{kind}_Reserved"));
                        }

                        if match reserved {
                            VersionSpecifier::Always => false,
                            VersionSpecifier::Never => true,
                            VersionSpecifier::From { .. } => true,
                            VersionSpecifier::Till { .. } => true,
                            VersionSpecifier::Range { .. } => true,
                        } {
                            kinds.insert(format!("{kind}_Unreserved"));
                        }
                    }
                }
            }
        }

        kinds
    }

    fn collect_trivia_lexeme_kinds(contexts: &[LexicalContext]) -> BTreeSet<String> {
        let mut kinds = BTreeSet::new();

        for context in contexts {
            for lexeme in &context.lexemes {
                if let Lexeme::Trivia { kind, .. } = lexeme {
                    kinds.insert(kind.clone());
                }
            }
        }

        kinds
    }
}

struct LexicalContextBuilder {
    fragments: BTreeMap<Identifier, TokenScanner>,

    lexemes: Vec<Lexeme>,
    subpatterns: IndexMap<Identifier, String>,
}

impl LexicalContextBuilder {
    fn build(language_context: &LanguageContext) -> LexicalContext {
        let fragments = language_context
            .items()
            .filter_map(|item| {
                if let Item::Fragment { item } = item {
                    Some((item.name.clone(), item.scanner.clone()))
                } else {
                    None
                }
            })
            .collect();

        let mut instance = Self {
            fragments,

            lexemes: Vec::new(),
            subpatterns: IndexMap::new(),
        };

        for item in language_context.items() {
            match item {
                Item::Struct { .. }
                | Item::Enum { .. }
                | Item::Repeated { .. }
                | Item::Separated { .. }
                | Item::Precedence { .. } => {}

                Item::Trivia { item } => {
                    let lexeme = instance.convert_trivia(item);
                    instance.lexemes.push(lexeme);
                }
                Item::Keyword { item } => {
                    let lexeme =
                        Self::convert_keyword(item, language_context.identifier_token.as_ref());
                    instance.lexemes.push(lexeme);
                }
                Item::Token { item } => {
                    let lexeme = instance.convert_token(item);
                    instance.lexemes.push(lexeme);
                }

                Item::Fragment { .. } => {}
            }
        }

        let Self {
            lexemes,
            subpatterns,
            ..
        } = instance;

        LexicalContext {
            name: language_context.name.to_string(),
            lexemes,
            subpatterns,
        }
    }

    fn add_subpattern(&mut self, name: &Identifier) {
        if self.subpatterns.contains_key(name) {
            return; // Already added before.
        }

        let scanner = self.fragments[name].clone();
        let regex = self.convert_token_scanner(&scanner, false);
        self.subpatterns.insert(name.clone(), regex);
    }

    fn convert_trivia(&mut self, item: &TriviaItem) -> Lexeme {
        Lexeme::Trivia {
            kind: item.name.to_string(),
            regex: self.convert_token_scanner(&item.scanner, false),
        }
    }

    fn convert_keyword(item: &KeywordItem, identifier: Option<&Identifier>) -> Lexeme {
        Lexeme::Keyword {
            kind: item.name.to_string(),
            identifier: identifier.map(|id| id.to_string()),
            regex: Self::convert_keyword_scanner(&item.scanner),
            reserved: item.reserved.clone().unwrap_or_default(),
        }
    }

    fn convert_token(&mut self, item: &TokenItem) -> Lexeme {
        let not_followed_by = item
            .not_followed_by
            .as_ref()
            .map(|scanner| self.convert_token_scanner(scanner, true));

        Lexeme::Token {
            kind: item.name.to_string(),
            regex: self.convert_token_scanner(&item.scanner, false),
            not_followed_by,
        }
    }

    fn convert_keyword_scanner(parent: &KeywordScanner) -> String {
        match parent {
            KeywordScanner::Sequence { scanners } => scanners
                .iter()
                .map(|scanner| Self::convert_child_keyword_scanner(parent, scanner))
                .collect(),

            KeywordScanner::Choice { scanners } => scanners
                .iter()
                .map(|scanner| Self::convert_child_keyword_scanner(parent, scanner))
                .collect::<Vec<_>>()
                .join("|"),

            KeywordScanner::Optional { scanner } => {
                format!("{}?", Self::convert_child_keyword_scanner(parent, scanner))
            }

            KeywordScanner::Atom { atom } => Self::convert_atom(atom),
        }
    }

    fn convert_child_keyword_scanner(parent: &KeywordScanner, child: &KeywordScanner) -> String {
        if discriminant(parent) != discriminant(child)
            && Self::keyword_scanner_precedence(child) <= Self::keyword_scanner_precedence(parent)
        {
            format!("({})", Self::convert_keyword_scanner(child))
        } else {
            Self::convert_keyword_scanner(child)
        }
    }

    fn keyword_scanner_precedence(scanner: &KeywordScanner) -> u8 {
        match scanner {
            // Binary:
            KeywordScanner::Sequence { .. } | KeywordScanner::Choice { .. } => 1,
            // Postfix:
            KeywordScanner::Optional { .. } => 2,
            // Primary:
            KeywordScanner::Atom { .. } => 3,
        }
    }

    fn convert_token_scanner(&mut self, parent: &TokenScanner, inline_fragment: bool) -> String {
        match parent {
            TokenScanner::Sequence { scanners } => {
                let mut result = String::new();
                for scanner in scanners {
                    result.push_str(&self.convert_child_token_scanner(
                        parent,
                        scanner,
                        inline_fragment,
                    ));
                }
                result
            }

            TokenScanner::Choice { scanners } => {
                let mut parts = Vec::new();
                for scanner in scanners {
                    parts.push(self.convert_child_token_scanner(parent, scanner, inline_fragment));
                }
                parts.join("|")
            }

            TokenScanner::Optional { scanner } => {
                format!(
                    "{}?",
                    self.convert_child_token_scanner(parent, scanner, inline_fragment)
                )
            }

            TokenScanner::ZeroOrMore { scanner } => {
                format!(
                    "{}*",
                    self.convert_child_token_scanner(parent, scanner, inline_fragment)
                )
            }

            TokenScanner::OneOrMore { scanner } => {
                format!(
                    "{}+",
                    self.convert_child_token_scanner(parent, scanner, inline_fragment)
                )
            }

            TokenScanner::Not { chars } => {
                format!(
                    "[^{}]",
                    chars
                        .iter()
                        .map(|c| Self::convert_char(*c))
                        .collect::<String>()
                )
            }

            TokenScanner::Range {
                inclusive_start,
                inclusive_end,
            } => format!(
                "[{}-{}]",
                Self::convert_char(*inclusive_start),
                Self::convert_char(*inclusive_end)
            ),

            TokenScanner::Atom { atom } => Self::convert_atom(atom),

            TokenScanner::Fragment { reference } => {
                if inline_fragment {
                    let scanner = self.fragments[reference].clone();
                    self.convert_child_token_scanner(parent, &scanner, inline_fragment)
                } else {
                    self.add_subpattern(reference);
                    format!("(?&{reference})")
                }
            }
        }
    }

    fn convert_child_token_scanner(
        &mut self,
        parent: &TokenScanner,
        child: &TokenScanner,
        inline_fragment: bool,
    ) -> String {
        let needs_parens = discriminant(parent) != discriminant(child)
            && Self::token_scanner_precedence(child) <= Self::token_scanner_precedence(parent);
        if needs_parens {
            let inner = self.convert_token_scanner(child, inline_fragment);
            format!("({inner})")
        } else {
            self.convert_token_scanner(child, inline_fragment)
        }
    }

    fn token_scanner_precedence(scanner: &TokenScanner) -> u8 {
        match scanner {
            // Binary:
            TokenScanner::Sequence { .. } | TokenScanner::Choice { .. } => 1,
            // Postfix:
            TokenScanner::Optional { .. }
            | TokenScanner::ZeroOrMore { .. }
            | TokenScanner::OneOrMore { .. } => 2,
            // Primary:
            TokenScanner::Not { .. }
            | TokenScanner::Range { .. }
            | TokenScanner::Atom { .. }
            | TokenScanner::Fragment { .. } => 3,
        }
    }

    fn convert_atom(atom: &str) -> String {
        atom.chars().map(Self::convert_char).collect()
    }

    fn convert_char(c: char) -> String {
        match c {
            // use alphanumerics as-is:
            'a'..='z' | 'A'..='Z' | '0'..='9' => c.to_string(),
            // use punctuation as-is:
            ' ' | '_' | '-' | ',' | ';' | ':' | '!' | '"' | '#' | '/' | '\'' | '&' | '%' | '<'
            | '=' | '>' | '~' => c.to_string(),
            // escape regex control characters:
            '^' | '$' | '|' | '?' | '.' | '(' | ')' | '[' | ']' | '{' | '}' | '*' | '\\' | '+' => {
                format!("\\{c}")
            }
            // escape breaks:
            '\t' => "\\t".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            _ => {
                panic!("Unsupported character in lexer char: '{c}'");
            }
        }
    }
}
