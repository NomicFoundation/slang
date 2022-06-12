use std::fmt::Write;

use inflector::{
    cases::{pascalcase::to_pascal_case, snakecase::to_snake_case},
    Inflector,
};
use proc_macro2::Ident;
use quote::format_ident;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SlangName(String);

impl SlangName {
    pub fn as_str<'a>(&'a self) -> &'a str {
        self.0.as_str()
    }

    pub fn from_string(s: &str) -> Self {
        Self(s.into())
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
        Self({
            let c = result.chars().next().unwrap();
            if !(c.is_alphabetic() || c == '_') {
                println!("INVALID IDENTIFIER: {}", result);
                format!("__{}", result)
            } else {
                result
            }
        })
    }

    pub fn from_terminal_char(char: char) -> Option<Self> {
        DEFAULT_TOKEN_NAMES
            .binary_search_by(|probe| probe.0.cmp(&char))
            .ok()
            .map(|i| Self(format!("{}_char", DEFAULT_TOKEN_NAMES[i].1)))
    }

    pub fn from_prefix_and_index(str: &str, index: usize) -> Self {
        Self(format!("{}{}", str, index))
    }

    pub fn plural(&self) -> Self {
        Self(self.0.to_plural())
    }

    pub fn with_disambiguating_suffix(&self, index: usize) -> Self {
        Self(format!("{}_{}", self.0, index))
    }

    pub fn to_module_name_ident(&self) -> Ident {
        let id = to_snake_case(self.0.as_str());
        if is_reserved_identifier(id.as_str()) {
            format_ident!("r#{}", id)
        } else {
            format_ident!("{}", id)
        }
    }

    pub fn to_type_name_ident(&self) -> Ident {
        if self.0.starts_with('_') {
            format_ident!("{}", self.0.as_str().to_ascii_uppercase())
        } else {
            format_ident!("{}", to_pascal_case(self.0.as_str()))
        }
    }

    pub fn to_enum_tag_ident(&self) -> Ident {
        if self.0.starts_with('_') {
            format_ident!("{}", self.0.as_str().to_ascii_uppercase())
        } else {
            format_ident!("{}", to_pascal_case(self.0.as_str()))
        }
    }

    pub fn to_field_name_ident(&self) -> Ident {
        if self.0.starts_with('_') {
            format_ident!("{}", self.0.as_str().to_ascii_lowercase())
        } else {
            let id = to_snake_case(self.0.as_str());
            if is_reserved_identifier(id.as_str()) {
                format_ident!("r#{}", id)
            } else {
                format_ident!("{}", id)
            }
        }
    }

    pub fn to_parser_name_ident(&self) -> Ident {
        format_ident!("{}_parser", to_snake_case(self.0.as_str()))
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
