use std::collections::{BTreeMap, BTreeSet};
use std::mem::discriminant;
use std::rc::Rc;

use language_v2_definition::model::{
    FragmentItem, Identifier, Item, KeywordItem, KeywordValue, Language, Scanner, TokenItem,
    TriviaItem, VersionSpecifier,
};

use crate::lexer::{Lexeme, LexerModel, LexicalContext};

pub struct LexerModelBuilder {
    fragments: BTreeMap<Identifier, Rc<FragmentItem>>,
}

impl LexerModelBuilder {
    pub fn build(language: &Language) -> LexerModel {
        let instance = Self {
            fragments: Self::collect_fragments(language),
        };

        let contexts = instance.collect_contexts(language);
        let lexeme_kinds = Self::collect_lexeme_kinds(&contexts);

        LexerModel {
            contexts,
            lexeme_kinds,
        }
    }

    fn collect_fragments(language: &Language) -> BTreeMap<Identifier, Rc<FragmentItem>> {
        language
            .items()
            .filter_map(|item| match item {
                Item::Fragment { item } => Some((item.name.clone(), Rc::clone(item))),
                _ => None,
            })
            .collect()
    }

    fn collect_contexts(&self, language: &Language) -> Vec<LexicalContext> {
        let mut non_trivia = BTreeMap::<String, Vec<Lexeme>>::new();

        // TODO(v2):
        // The existing v1 grammar had an inconsistency where it defined all trivia in the 'Default' context,
        // and the v1 parser dealt with it by parsing trivia using that context (regardless of the current one).
        // We should instead remove `Item::trivia` from individual topics, and move it to a separate `Language` field,
        // along with the existing `leading_trivia` and `trailing_trivia` fields.
        // For now, we just collect trivia and duplicate them into all contexts:
        let mut common_trivia = Vec::<Lexeme>::new();

        for topic in language.topics() {
            let context_name = topic.lexical_context.to_string();
            let context = non_trivia.entry(context_name).or_default();

            for item in &topic.items {
                match item {
                    Item::Struct { .. } => {}
                    Item::Enum { .. } => {}
                    Item::Repeated { .. } => {}
                    Item::Separated { .. } => {}
                    Item::Precedence { .. } => {}

                    Item::Trivia { item } => common_trivia.push(self.convert_trivia(item)),
                    Item::Keyword { item } => context.extend(self.convert_keyword(item)),
                    Item::Token { item } => context.extend(self.convert_token(item)),

                    Item::Fragment { .. } => {}
                }
            }
        }

        non_trivia
            .into_iter()
            .map(|(name, mut lexemes)| {
                // Insert common trivia into each context:
                lexemes.extend(common_trivia.iter().cloned());

                LexicalContext { name, lexemes }
            })
            .collect()
    }

    fn collect_lexeme_kinds(contexts: &Vec<LexicalContext>) -> BTreeSet<String> {
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

                        if !reserved.is_always() {
                            kinds.insert(format!("{kind}_Unreserved"));
                        }
                    }
                }
            }
        }

        kinds
    }

    fn convert_trivia(&self, item: &TriviaItem) -> Lexeme {
        Lexeme::Trivia {
            kind: item.name.to_string(),
            regex: self.convert_scanner(&item.scanner),
        }
    }

    fn convert_keyword<'a>(&'a self, item: &'a KeywordItem) -> impl Iterator<Item = Lexeme> + 'a {
        item.definitions.iter().map(|def| Lexeme::Keyword {
            kind: item.name.to_string(),
            regex: self.convert_keyword_value(&def.value),
            reserved: def.reserved.clone().unwrap_or_default(),
        })
    }

    fn convert_token<'a>(&'a self, item: &'a TokenItem) -> impl Iterator<Item = Lexeme> + 'a {
        item.definitions.iter().map(|def| Lexeme::Token {
            kind: item.name.to_string(),
            regex: self.convert_scanner(&def.scanner),
        })
    }

    fn convert_keyword_value(&self, parent: &KeywordValue) -> String {
        match parent {
            KeywordValue::Sequence { values } => values
                .iter()
                .map(|value| self.convert_child_keyword_value(parent, value))
                .collect(),

            KeywordValue::Choice { values } => values
                .iter()
                .map(|value| self.convert_child_keyword_value(parent, value))
                .collect::<Vec<_>>()
                .join("|"),

            KeywordValue::Optional { value } => {
                format!("{}?", self.convert_child_keyword_value(parent, value))
            }

            KeywordValue::Atom { atom } => Self::convert_atom(atom),
        }
    }

    fn convert_child_keyword_value(&self, parent: &KeywordValue, child: &KeywordValue) -> String {
        if discriminant(parent) != discriminant(child)
            && Self::keyword_value_precedence(child) <= Self::keyword_value_precedence(parent)
        {
            format!("({})", self.convert_keyword_value(child))
        } else {
            self.convert_keyword_value(child)
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

    fn convert_scanner(&self, parent: &Scanner) -> String {
        match parent {
            Scanner::Sequence { scanners } => scanners
                .iter()
                .map(|scanner| self.convert_child_scanner(parent, scanner))
                .collect(),

            Scanner::Choice { scanners } => scanners
                .iter()
                .map(|scanner| self.convert_child_scanner(parent, scanner))
                .collect::<Vec<_>>()
                .join("|"),

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
                self.convert_child_scanner(parent, &self.fragments[reference].scanner)
            }
        }
    }

    fn convert_child_scanner(&self, parent: &Scanner, child: &Scanner) -> String {
        if discriminant(parent) != discriminant(child)
            && self.scanner_precedence(child) <= self.scanner_precedence(parent)
        {
            format!("({})", self.convert_scanner(child))
        } else {
            self.convert_scanner(child)
        }
    }

    fn scanner_precedence(&self, scanner: &Scanner) -> u8 {
        match scanner {
            // Binary:
            Scanner::Sequence { .. } | Scanner::Choice { .. } => 1,
            // Postfix:
            Scanner::Optional { .. } | Scanner::ZeroOrMore { .. } | Scanner::OneOrMore { .. } => 2,
            // Primary:
            Scanner::Not { .. } | Scanner::Range { .. } | Scanner::Atom { .. } => 3,
            // Other:
            Scanner::Fragment { reference } => {
                self.scanner_precedence(&self.fragments[reference].scanner)
            }
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
            ' ' | '_' | '-' | ',' | ';' | ':' | '!' | '"' | '/' | '\'' | '&' | '%' | '<' | '='
            | '>' | '~' => c.to_string(),
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
