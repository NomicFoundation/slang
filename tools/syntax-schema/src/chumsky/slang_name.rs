use std::{cell::Cell, fmt::Write};

use inflector::Inflector;
use proc_macro2::Ident;
use quote::format_ident;

use crate::schema::ExpressionConfig;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SlangName {
    Anonymous,
    String {
        text: String, // stored in snake case
        plural: bool,
        negated: bool,
        suffix: usize,
    },
    Numbered {
        number: usize,
        plural: bool,
        suffix: usize,
    },
    Positional {
        position: usize,
    },
}

impl SlangName {
    pub fn anonymous() -> Self {
        Self::Anonymous
    }

    pub fn from_string(s: &str) -> Self {
        let text: String = s.into();
        let text = text.to_snake_case();
        Self::String {
            text,
            plural: false,
            negated: false,
            suffix: 0,
        }
    }

    pub fn from_terminal(string: &str) -> Self {
        let mut result = String::new();
        let mut last_was_named = None;
        for c in string.chars() {
            if let Some(name) = DEFAULT_TOKEN_NAMES
                .binary_search_by(|probe| probe.0.cmp(&c))
                .ok()
                .map(|i| DEFAULT_TOKEN_NAMES[i].1)
            {
                if last_was_named != None {
                    write!(&mut result, "_").unwrap();
                }
                write!(&mut result, "{}", name).unwrap();
                last_was_named = Some(true)
            } else {
                if last_was_named == Some(true) {
                    write!(&mut result, "_").unwrap();
                }
                write!(&mut result, "{}", c).unwrap();
                last_was_named = Some(false)
            }
        }
        Self::String {
            text: {
                let c = result.chars().next().unwrap();
                if !(c.is_alphabetic() || c == '_') {
                    println!("INVALID IDENTIFIER: {}", result);
                    format!("__{}", result)
                } else {
                    result
                }
                .to_snake_case()
            },
            plural: false,
            negated: false,
            suffix: 0,
        }
    }

    pub fn from_terminal_char(char: char) -> Self {
        DEFAULT_TOKEN_NAMES
            .binary_search_by(|probe| probe.0.cmp(&char))
            .ok()
            .map(|i| Self::String {
                text: format!("{}_char", DEFAULT_TOKEN_NAMES[i].1),
                plural: false,
                negated: false,
                suffix: 0,
            })
            .unwrap_or_else(|| Self::Anonymous)
    }

    pub fn self_or_numbered(self, index: &mut Cell<usize>) -> Self {
        if self == Self::Anonymous {
            let i = index.get();
            index.set(i + 1);
            Self::Numbered {
                number: i,
                plural: false,
                suffix: 0,
            }
        } else {
            self
        }
    }

    pub fn self_or_positional(self, i: usize) -> Self {
        if self == Self::Anonymous {
            Self::Positional { position: i }
        } else {
            self
        }
    }

    pub fn self_or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        if self == Self::Anonymous {
            f()
        } else {
            self
        }
    }

    pub fn pluralize(self) -> Self {
        match self {
            SlangName::Anonymous => self,
            SlangName::Positional { .. } => panic!("Cannot pluralize a positional name"),
            SlangName::String { plural: true, .. } | SlangName::Numbered { plural: true, .. } => {
                panic!("Cannot pluralize a plural name")
            }
            SlangName::String {
                text,
                suffix,
                negated,
                ..
            } => Self::String {
                text,
                suffix,
                negated,
                plural: true,
            },
            SlangName::Numbered { number, suffix, .. } => Self::Numbered {
                number,
                suffix,
                plural: true,
            },
        }
    }

    pub fn negate(self) -> Self {
        match self {
            SlangName::Anonymous | SlangName::Numbered { .. } => self,
            SlangName::Positional { .. } => panic!("Cannot negate a positional name"),
            SlangName::String { negated: true, .. } => panic!("Cannot negate a negated name"),
            SlangName::String {
                text,
                suffix,
                plural,
                ..
            } => Self::String {
                text,
                suffix,
                negated: true,
                plural,
            },
        }
    }

    pub fn with_disambiguating_suffix(self, suffix: usize) -> Self {
        match self {
            SlangName::Anonymous => panic!("Cannot disambiguate an anonymous name"),
            SlangName::Positional { .. } => panic!("Cannot disambiguate an positional name"),
            SlangName::String {
                text,
                plural,
                negated,
                ..
            } => Self::String {
                text,
                negated,
                plural,
                suffix: suffix + 1,
            },
            SlangName::Numbered { number, plural, .. } => Self::Numbered {
                number,
                suffix: suffix + 1,
                plural,
            },
        }
    }

    fn to_snake_case_string(&self) -> String {
        match self {
            SlangName::Anonymous => panic!("Cannot use anonymous names"),
            SlangName::Positional { position } => format!("_{}", position),
            SlangName::String {
                text,
                plural,
                negated,
                suffix,
            } => {
                let text = if *negated {
                    format!("not_{}", text)
                } else {
                    text.clone()
                };
                let text = if *plural { text.to_plural() } else { text };
                if 0 < *suffix {
                    format!("{}_{}", text, suffix)
                } else {
                    text
                }
            }
            SlangName::Numbered {
                number,
                plural,
                suffix,
            } => {
                let text = format!("_t{}", number);
                let text = if *plural { format!("{}s", text) } else { text };
                if 0 < *suffix {
                    format!("{}_{}", text, suffix)
                } else {
                    text
                }
            }
        }
    }

    fn to_pascal_case_string(&self) -> String {
        match self {
            SlangName::Anonymous => panic!("Cannot use anonymous names"),
            SlangName::Positional { position } => format!("_{}", position),
            SlangName::String {
                text,
                plural,
                negated,
                suffix,
            } => {
                let text = if *negated {
                    format!("not_{}", text)
                } else {
                    text.clone()
                };
                let text = if *plural { text.to_plural() } else { text };
                let text = text.to_pascal_case();
                if 0 < *suffix {
                    format!("{}_{}", text, suffix)
                } else {
                    text
                }
            }
            SlangName::Numbered {
                number,
                plural,
                suffix,
            } => {
                let text = format!("_T{}", number);
                let text = if *plural { format!("{}s", text) } else { text };
                if 0 < *suffix {
                    format!("{}_{}", text, suffix)
                } else {
                    text
                }
            }
        }
    }

    pub fn to_module_name_ident(&self) -> Ident {
        format_ident!("{}", self.to_snake_case_string())
    }

    pub fn to_type_name_ident(&self) -> Ident {
        format_ident!("{}", self.to_pascal_case_string())
    }

    pub fn to_enum_tag_ident(&self) -> Ident {
        format_ident!("{}", self.to_pascal_case_string())
    }

    pub fn to_field_name_ident(&self) -> Ident {
        let text = self.to_snake_case_string();
        if is_reserved_identifier(&text) {
            format_ident!("r#{}", text)
        } else {
            format_ident!("{}", text)
        }
    }

    pub fn to_parser_name_ident(&self) -> Ident {
        format_ident!("{}_parser", self.to_snake_case_string())
    }

    pub fn to_unmapped_parser_name_ident(&self) -> Ident {
        format_ident!("{}_parser_unmapped", self.to_snake_case_string())
    }
}

impl ExpressionConfig {
    pub fn slang_name(&self) -> SlangName {
        if let Some(name) = &self.name {
            SlangName::from_string(name)
        } else {
            SlangName::anonymous()
        }
    }
}

fn is_reserved_identifier(s: &str) -> bool {
    RUST_RESERVED_IDENTIFIERS.binary_search(&s).is_ok()
}

const RUST_RESERVED_IDENTIFIERS: &[&str] = &[
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

const DEFAULT_TOKEN_NAMES: &[(char, &str)] = &[
    ('\t', "tab"),
    ('\n', "lf"),
    ('\r', "cr"),
    (' ', "space"),
    ('!', "bang"),
    ('"', "double_quote"),
    ('#', "hash"),
    ('$', "dollar"),
    ('%', "percent"),
    ('&', "ampersand"),
    ('\'', "quote"),
    ('(', "open_paren"),
    (')', "close_paren"),
    ('*', "star"),
    ('+', "plus"),
    (',', "comma"),
    ('-', "minus"),
    ('.', "period"),
    ('/', "slash"),
    ('0', "zero"),
    ('1', "one"),
    ('2', "two"),
    ('3', "three"),
    ('4', "four"),
    ('5', "five"),
    ('6', "six"),
    ('7', "seven"),
    ('8', "eight"),
    ('9', "nine"),
    (':', "colon"),
    (';', "semicolon"),
    ('<', "less"),
    ('=', "equal"),
    ('>', "greater"),
    ('?', "question"),
    ('@', "at"),
    ('[', "open_bracket"),
    ('\\', "backslash"),
    (']', "close_bracket"),
    ('^', "caret"),
    ('_', "underscore"),
    ('`', "backquote"),
    ('{', "open_brace"),
    ('|', "bar"),
    ('}', "close_brace"),
    ('~', "tilde"),
    ('«', "open_double_angle"),
    ('¬', "not"),
    ('»', "close_double_angle"),
    ('…', "ellipsis"),
];
