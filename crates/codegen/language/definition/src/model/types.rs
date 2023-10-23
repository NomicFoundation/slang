pub use self::wrapper::*;

/// We want to generate all Spanned types in a single module.
/// Unfortunately, module-level (inner) attribute macros are not supported yet:
/// This is why we put the attribute on a wrapper module containing all of them, then re-export its inner contents.
/// More information: https://github.com/rust-lang/rust/issues/54726
#[codegen_language_internal_macros::derive_internals]
mod wrapper {
    use crate::Identifier;
    use indexmap::{IndexMap, IndexSet};
    use semver::Version;
    use serde::Serialize;
    use std::{path::PathBuf, rc::Rc};
    use strum_macros::{Display, EnumDiscriminants, EnumIter};

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct Language {
        pub name: Identifier,

        pub root_item: Identifier,

        pub leading_trivia: TriviaParser,
        pub trailing_trivia: TriviaParser,

        pub versions: IndexSet<Version>,

        pub sections: Vec<Section>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct Section {
        pub title: String,
        pub topics: Vec<Topic>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct Topic {
        pub title: String,
        pub notes_file: Option<PathBuf>,
        pub lexical_context: Option<Identifier>,

        pub items: Vec<Rc<Item>>,
    }

    #[derive(Debug, Eq, PartialEq, EnumDiscriminants, Serialize)]
    #[strum_discriminants(name(ItemKind))]
    #[strum_discriminants(derive(Display, EnumIter))]
    pub enum Item {
        Struct { item: StructItem },
        Enum { item: EnumItem },

        Repeated { item: RepeatedItem },
        Separated { item: SeparatedItem },

        Precedence { item: PrecedenceItem },

        Keyword { item: KeywordItem },
        Token { item: TokenItem },

        Fragment { item: FragmentItem },
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct StructItem {
        pub name: Identifier,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub fields: IndexMap<Identifier, Field>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct EnumItem {
        pub name: Identifier,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub variants: Vec<Variant>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct RepeatedItem {
        pub name: Identifier,
        pub repeated: Identifier,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub allow_empty: Option<bool>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct SeparatedItem {
        pub name: Identifier,
        pub separated: Identifier,
        pub separator: Identifier,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub allow_empty: Option<bool>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct PrecedenceItem {
        pub name: Identifier,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub operators: Vec<Operator>,
        pub primary_expressions: Vec<Variant>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct Operator {
        pub expression_name: Identifier,
        pub model: OperatorModel,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub fields: IndexMap<Identifier, Field>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub enum OperatorModel {
        Prefix,
        Postfix,
        BinaryLeftAssociative,
        BinaryRightAssociative,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct Variant {
        pub name: Identifier,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub fields: IndexMap<Identifier, Field>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub enum Field {
        Required {
            reference: FieldReference,
        },
        Optional {
            reference: FieldReference,

            enabled_in: Option<Version>,
            disabled_in: Option<Version>,
        },
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub enum FieldReference {
        NonTerminal { item: Identifier },
        Terminal { items: IndexSet<Identifier> },
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub enum TriviaParser {
        Sequence { parsers: Vec<TriviaParser> },
        Choice { parsers: Vec<TriviaParser> },

        ZeroOrMore { parser: Box<TriviaParser> },
        Optional { parser: Box<TriviaParser> },

        Token { token: Identifier },
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct KeywordItem {
        pub name: Identifier,
        pub identifier: Identifier,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub reserved_in: Option<Version>,
        pub unreserved_in: Option<Version>,

        pub value: KeywordValue,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub enum KeywordValue {
        Sequence {
            values: Vec<KeywordValue>,
        },
        Optional {
            value: Box<KeywordValue>,
        },
        Range {
            inclusive_start: usize,
            inclusive_end: usize,
            increment: usize,
        },
        Atom {
            atom: String,
        },
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct TokenItem {
        pub name: Identifier,

        pub is_terminator: Option<bool>,
        pub is_open_delimiter_for: Option<Identifier>,
        pub is_close_delimiter_for: Option<Identifier>,

        pub definitions: Vec<TokenDefinition>,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct TokenDefinition {
        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub scanner: Scanner,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct FragmentItem {
        pub name: Identifier,

        pub enabled_in: Option<Version>,
        pub disabled_in: Option<Version>,

        pub scanner: Scanner,
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub enum Scanner {
        Sequence {
            scanners: Vec<Scanner>,
        },
        Choice {
            scanners: Vec<Scanner>,
        },
        Optional {
            scanner: Box<Scanner>,
        },
        ZeroOrMore {
            scanner: Box<Scanner>,
        },
        OneOrMore {
            scanner: Box<Scanner>,
        },
        Not {
            chars: IndexSet<char>,
        },
        Range {
            inclusive_start: char,
            inclusive_end: char,
        },
        Atom {
            atom: String,
        },
        TrailingContext {
            scanner: Box<Scanner>,
            not_followed_by: Box<Scanner>,
        },
        Fragment {
            reference: Identifier,
        },
    }
}
