use std::{cell::Cell, fmt::Write};

use inflector::Inflector;
use proc_macro2::Ident;
use quote::format_ident;

pub fn name_of_terminal_char(char: char) -> String {
    DEFAULT_TOKEN_NAMES
        .binary_search_by(|probe| probe.0.cmp(&char))
        .ok()
        .map(|i| format!("{}", DEFAULT_TOKEN_NAMES[i].1))
        .or_else(|| {
            unicode_names2::name(char).map(|n| format!("{}", n.to_string().to_snake_case()))
        })
        .unwrap_or_else(|| {
            let code: u32 = char.into();
            format!("u{}_char", code)
        })
}

pub fn name_of_terminal_string(string: &str) -> String {
    if string.chars().count() == 1 {
        return name_of_terminal_char(string.chars().next().unwrap());
    }
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

    let c = result.chars().next().unwrap();
    if !(c.is_alphabetic() || c == '_') {
        println!("INVALID IDENTIFIER: {}", result);
        format!("__{}", result)
    } else {
        result
    }
    .to_snake_case()
}

pub fn anonymous_filter(cell: &Cell<usize>) -> String {
    let count = cell.get();
    cell.set(count + 1);
    format!("filter@{}", count)
}

pub fn anonymous_terminals(cell: &Cell<usize>) -> String {
    let count = cell.get();
    cell.set(count + 1);
    format!("terminal@{}", count)
}

pub fn anonymous_choice(cell: &Cell<usize>) -> String {
    let count = cell.get();
    cell.set(count + 1);
    format!("choices@{}", count)
}

pub fn anonymous_sequence(cell: &Cell<usize>) -> String {
    let count = cell.get();
    cell.set(count + 1);
    format!("sequence@{}", count)
}

pub fn is_anonymous(str: &str) -> bool {
    str.contains('@')
}

pub fn pluralize(str: &str) -> String {
    format!("{}_repeated", str)
    // str.to_plural()
}

pub fn with_disambiguating_suffix(str: &str, suffix: usize) -> String {
    format!("{}_{}_", str, suffix)
}

fn without_disambiguating_suffix(str: &str) -> String {
    if str.ends_with('_') {
        str.trim_end_matches(|c: char| c.is_numeric() || c == '_')
            .to_string()
    } else {
        str.to_string()
    }
}

fn without_anonymous_marker(str: &str) -> String {
    str.replace('@', "_")
}

fn to_pascal_case_string(str: &str) -> String {
    str.to_pascal_case()
}

pub fn to_module_name_ident(str: &str) -> Ident {
    let str = without_disambiguating_suffix(str);
    let str = without_anonymous_marker(&str);
    if is_reserved_identifier(&str) {
        format_ident!("r#{}", str)
    } else {
        format_ident!("{}", str)
    }
}

pub fn to_field_name_ident(str: &str) -> Ident {
    let str = without_anonymous_marker(str);
    if is_reserved_identifier(&str) {
        format_ident!("r#{}", str)
    } else {
        format_ident!("{}", str)
    }
}

pub fn to_parser_name_ident(str: &str) -> Ident {
    let str = without_disambiguating_suffix(str);
    let str = without_anonymous_marker(&str);
    format_ident!("{}_parser", str)
}

pub fn to_type_name_ident(str: &str) -> Ident {
    let str = without_disambiguating_suffix(str);
    let str = without_anonymous_marker(&str);
    format_ident!("{}", to_pascal_case_string(&str))
}

pub fn to_enum_tag_ident(str: &str) -> Ident {
    let str = without_disambiguating_suffix(str);
    let str = without_anonymous_marker(&str);
    format_ident!("{}", to_pascal_case_string(&str))
}

pub fn to_kind_ident(module_str: &str, type_str: &str) -> Ident {
    let module_str = without_disambiguating_suffix(module_str);
    let type_str = without_disambiguating_suffix(type_str);
    let type_str = without_anonymous_marker(&type_str);
    if module_str == type_str {
        format_ident!("{}", to_pascal_case_string(&module_str))
    } else {
        println!("{} != {}", module_str, type_str);
        format_ident!(
            "{}{}",
            to_pascal_case_string(&module_str),
            to_pascal_case_string(&type_str)
        )
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
    ('[', "open_bracket"),
    ('\\', "backslash"),
    (']', "close_bracket"),
    ('^', "caret"),
    ('_', "underscore"),
    ('`', "backtick"),
    ('{', "open_brace"),
    ('|', "pipe"),
    ('}', "close_brace"),
    ('~', "tilde"),
    ('«', "open_double_angle"),
    ('¬', "not"),
    ('»', "close_double_angle"),
    ('…', "ellipsis"),
];
