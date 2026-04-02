use std::collections::{BTreeMap, BTreeSet};
use std::mem::discriminant;

use indexmap::IndexMap;
use language_v2_definition::model::{
    Identifier, Item, KeywordItem, KeywordValue, Language, LexicalContext as LanguageContext,
    Scanner, TokenItem, TriviaItem, VersionSpecifier,
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

        let lexeme_kinds = Self::collect_lexeme_kinds(&contexts);

        LexerModel {
            contexts,
            lexeme_kinds,
        }
    }

    fn collect_lexeme_kinds(contexts: &[LexicalContext]) -> BTreeSet<String> {
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
}

struct LexicalContextBuilder {
    fragments: BTreeMap<Identifier, Scanner>,

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
        let regex = self.convert_scanner(&scanner, false);
        self.subpatterns.insert(name.clone(), regex);
    }

    fn convert_trivia(&mut self, item: &TriviaItem) -> Lexeme {
        Lexeme::Trivia {
            kind: item.name.to_string(),
            regex: self.convert_scanner(&item.scanner, false),
        }
    }

    fn convert_keyword(item: &KeywordItem, identifier: Option<&Identifier>) -> Lexeme {
        Lexeme::Keyword {
            kind: item.name.to_string(),
            identifier: identifier.map(|id| id.to_string()),
            regex: Self::convert_keyword_value(&item.value),
            reserved: item.reserved.clone().unwrap_or_default(),
        }
    }

    fn convert_token(&mut self, item: &TokenItem) -> Lexeme {
        let not_followed_by = item
            .not_followed_by
            .as_ref()
            .map(|scanner| self.convert_scanner(scanner, true));

        Lexeme::Token {
            kind: item.name.to_string(),
            regex: self.convert_scanner(&item.scanner, false),
            not_followed_by,
        }
    }

    fn convert_keyword_value(parent: &KeywordValue) -> String {
        match parent {
            KeywordValue::Sequence { values } => values
                .iter()
                .map(|value| Self::convert_child_keyword_value(parent, value))
                .collect(),

            KeywordValue::Choice { values } => values
                .iter()
                .map(|value| Self::convert_child_keyword_value(parent, value))
                .collect::<Vec<_>>()
                .join("|"),

            KeywordValue::Optional { value } => {
                format!("{}?", Self::convert_child_keyword_value(parent, value))
            }

            KeywordValue::Atom { atom } => Self::convert_atom(atom),
        }
    }

    fn convert_child_keyword_value(parent: &KeywordValue, child: &KeywordValue) -> String {
        if discriminant(parent) != discriminant(child)
            && Self::keyword_value_precedence(child) <= Self::keyword_value_precedence(parent)
        {
            format!("({})", Self::convert_keyword_value(child))
        } else {
            Self::convert_keyword_value(child)
        }
    }

    fn keyword_value_precedence(value: &KeywordValue) -> u8 {
        match value {
            // Binary:
            KeywordValue::Sequence { .. } | KeywordValue::Choice { .. } => 1,
            // Postfix:
            KeywordValue::Optional { .. } => 2,
            // Primary:
            KeywordValue::Atom { .. } => 3,
        }
    }

    fn convert_scanner(&mut self, parent: &Scanner, inline_fragment: bool) -> String {
        match parent {
            Scanner::Sequence { scanners } => {
                let mut result = String::new();
                for scanner in scanners {
                    result.push_str(&self.convert_child_scanner(parent, scanner, inline_fragment));
                }
                result
            }

            Scanner::Choice { scanners } => {
                let mut parts = Vec::new();
                for scanner in scanners {
                    parts.push(self.convert_child_scanner(parent, scanner, inline_fragment));
                }
                parts.join("|")
            }

            Scanner::Optional { scanner } => {
                format!(
                    "{}?",
                    self.convert_child_scanner(parent, scanner, inline_fragment)
                )
            }

            Scanner::ZeroOrMore { scanner } => {
                format!(
                    "{}*",
                    self.convert_child_scanner(parent, scanner, inline_fragment)
                )
            }

            Scanner::OneOrMore { scanner } => {
                format!(
                    "{}+",
                    self.convert_child_scanner(parent, scanner, inline_fragment)
                )
            }

            Scanner::Not { chars } => {
                format!(
                    "[^{}]",
                    chars
                        .iter()
                        .map(|c| Self::convert_char(*c))
                        .collect::<String>()
                )
            }

            Scanner::Range {
                inclusive_start,
                inclusive_end,
            } => format!(
                "[{}-{}]",
                Self::convert_char(*inclusive_start),
                Self::convert_char(*inclusive_end)
            ),

            Scanner::Atom { atom } => Self::convert_atom(atom),

            Scanner::Fragment { reference } => {
                if inline_fragment {
                    let scanner = self.fragments[reference].clone();
                    self.convert_child_scanner(parent, &scanner, inline_fragment)
                } else {
                    self.add_subpattern(reference);
                    format!("(?&{reference})")
                }
            }
        }
    }

    fn convert_child_scanner(
        &mut self,
        parent: &Scanner,
        child: &Scanner,
        inline_fragment: bool,
    ) -> String {
        let needs_parens = discriminant(parent) != discriminant(child)
            && Self::scanner_precedence(child) <= Self::scanner_precedence(parent);
        if needs_parens {
            let inner = self.convert_scanner(child, inline_fragment);
            format!("({inner})")
        } else {
            self.convert_scanner(child, inline_fragment)
        }
    }

    fn scanner_precedence(scanner: &Scanner) -> u8 {
        match scanner {
            // Binary:
            Scanner::Sequence { .. } | Scanner::Choice { .. } => 1,
            // Postfix:
            Scanner::Optional { .. } | Scanner::ZeroOrMore { .. } | Scanner::OneOrMore { .. } => 2,
            // Primary:
            Scanner::Not { .. }
            | Scanner::Range { .. }
            | Scanner::Atom { .. }
            | Scanner::Fragment { .. } => 3,
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
