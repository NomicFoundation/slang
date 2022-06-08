pub trait NamedCharacter {
    fn raw_name(&self) -> Option<String>;
    fn slang_name(&self) -> Option<String>;
}

impl NamedCharacter for char {
    fn raw_name(&self) -> Option<String> {
        DEFAULT_TOKEN_NAMES
            .binary_search_by(|probe| probe.0.cmp(self))
            .ok()
            .map(|i| format!("{}", DEFAULT_TOKEN_NAMES[i].1))
    }
    fn slang_name(&self) -> Option<String> {
        self.raw_name().map(|n| format!("{}_char", n))
    }
}

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
