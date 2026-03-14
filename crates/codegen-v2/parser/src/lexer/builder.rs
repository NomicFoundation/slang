use std::collections::{BTreeMap, BTreeSet};
use std::mem::discriminant;
use std::rc::Rc;

use indexmap::IndexMap;
use language_v2_definition::model::{
    FragmentItem, Identifier, Item, KeywordItem, KeywordValue, Language,
    LexicalContext as LanguageContext, Scanner, TokenItem, TriviaItem, VersionSpecifier,
};

use crate::lexer::{Lexeme, LexerModel, LexicalContext};

pub struct LexerModelBuilder;

impl LexerModelBuilder {
    pub fn build(language: &Language) -> LexerModel {
        // TODO(v2):
        // The existing v1 grammar had an inconsistency where it defined all trivia in the 'Default' context,
        // and the v1 parser dealt with it by parsing trivia using that context (regardless of the current one).
        // We should instead remove `Item::trivia` from individual topics, and move it to a separate `Language` field,
        // along with the existing `leading_trivia` and `trailing_trivia` fields.
        // For now, we just collect trivia and duplicate them into all contexts:
        let mut common_trivia = Vec::<Rc<TriviaItem>>::new();
        let mut all_fragments = BTreeMap::<Identifier, Rc<FragmentItem>>::new();

        for item in language.items() {
            match item {
                Item::Trivia { item } => {
                    common_trivia.push(Rc::clone(item));
                }
                Item::Fragment { item } => {
                    all_fragments.insert(item.name.clone(), Rc::clone(item));
                }
                _ => {}
            }
        }

        let contexts: Vec<LexicalContext> = language
            .contexts
            .iter()
            .map(|language_context| {
                LexicalContextBuilder::build(
                    common_trivia.clone(),
                    all_fragments.clone(),
                    language_context,
                )
            })
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
    common_trivia: Vec<Rc<TriviaItem>>,
    all_fragments: BTreeMap<Identifier, Rc<FragmentItem>>,

    lexemes: Vec<Lexeme>,
    subpatterns: IndexMap<Identifier, String>,
}

impl LexicalContextBuilder {
    fn build(
        common_trivia: Vec<Rc<TriviaItem>>,
        all_fragments: BTreeMap<Identifier, Rc<FragmentItem>>,
        language_context: &LanguageContext,
    ) -> LexicalContext {
        let mut instance = Self {
            common_trivia,
            all_fragments,

            lexemes: Vec::new(),
            subpatterns: IndexMap::new(),
        };

        for item in language_context.items() {
            match item {
                Item::Struct { .. }
                | Item::Enum { .. }
                | Item::Repeated { .. }
                | Item::Separated { .. }
                | Item::Precedence { .. }
                | Item::Trivia { .. } => {}

                Item::Keyword { item } => {
                    let lexemes =
                        Self::convert_keyword(item, language_context.identifier_token.as_ref());
                    instance.lexemes.extend(lexemes);
                }
                Item::Token { item } => {
                    let lexemes = instance.convert_token(item);
                    instance.lexemes.extend(lexemes);
                }
                Item::Fragment { item } => {
                    instance.add_subpattern(&item.name);
                }
            }
        }

        for i in 0..instance.common_trivia.len() {
            let item = Rc::clone(&instance.common_trivia[i]);
            let lexeme = instance.convert_trivia(&item);
            instance.lexemes.push(lexeme);
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
            return;
        }

        let item = Rc::clone(&self.all_fragments[name]);
        let regex = self.convert_scanner(&item.scanner);
        self.subpatterns.insert(name.clone(), regex);
    }

    fn convert_trivia(&mut self, item: &TriviaItem) -> Lexeme {
        Lexeme::Trivia {
            kind: item.name.to_string(),
            regex: self.convert_scanner(&item.scanner),
        }
    }

    fn convert_keyword(item: &KeywordItem, identifier: Option<&Identifier>) -> Vec<Lexeme> {
        let mut result = Vec::new();

        for definition in &item.definitions {
            result.push(Lexeme::Keyword {
                kind: item.name.to_string(),
                identifier: identifier.map(|id| id.to_string()),
                regex: Self::convert_keyword_value(&definition.value),
                reserved: definition.reserved.clone().unwrap_or_default(),
            });
        }

        result
    }

    fn convert_token(&mut self, item: &TokenItem) -> Vec<Lexeme> {
        let mut result = Vec::new();

        for definition in &item.definitions {
            result.push(Lexeme::Token {
                kind: item.name.to_string(),
                regex: self.convert_scanner(&definition.scanner),
            });
        }

        result
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

    fn convert_scanner(&mut self, parent: &Scanner) -> String {
        match parent {
            Scanner::Sequence { scanners } => {
                let mut result = String::new();
                for scanner in scanners {
                    result.push_str(&self.convert_child_scanner(parent, scanner));
                }
                result
            }

            Scanner::Choice { scanners } => {
                let mut parts = Vec::new();
                for scanner in scanners {
                    parts.push(self.convert_child_scanner(parent, scanner));
                }
                parts.join("|")
            }

            Scanner::Optional { scanner } => {
                format!("{}?", self.convert_child_scanner(parent, scanner))
            }

            Scanner::ZeroOrMore { scanner } => {
                format!("{}*", self.convert_child_scanner(parent, scanner))
            }

            Scanner::OneOrMore { scanner } => {
                format!("{}+", self.convert_child_scanner(parent, scanner))
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
                self.add_subpattern(reference);
                format!("(?&{reference})")
            }
        }
    }

    fn convert_child_scanner(&mut self, parent: &Scanner, child: &Scanner) -> String {
        let needs_parens = discriminant(parent) != discriminant(child)
            && Self::scanner_precedence(child) <= Self::scanner_precedence(parent);
        if needs_parens {
            let inner = self.convert_scanner(child);
            format!("({inner})")
        } else {
            self.convert_scanner(child)
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
