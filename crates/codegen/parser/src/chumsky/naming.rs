use std::fmt::Write;

use inflector::Inflector;

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
            format!("u{}Char", code)
        })
        .to_pascal_case()
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
    .to_pascal_case()
}

const DEFAULT_TOKEN_NAMES: &[(char, &str)] = &[
    ('\t', "tab"),
    ('\n', "linefeed"),
    ('\r', "carriage_return"),
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
    ('`', "backquote"),
    ('{', "open_brace"),
    ('|', "pipe"),
    ('}', "close_brace"),
    ('~', "tilde"),
    ('«', "open_double_angle"),
    ('¬', "not"),
    ('»', "close_double_angle"),
    ('…', "ellipsis"),
];
