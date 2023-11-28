pub use self::wrapper::*;
pub use indexmap::{IndexMap, IndexSet};

/// We want to generate all Spanned types in a single module.
/// Unfortunately, module-level (inner) attribute macros are not supported yet:
/// This is why we put the attribute on a wrapper module containing all of them, then re-export its inner contents.
/// More information: https://github.com/rust-lang/rust/issues/54726
#[codegen_language_internal_macros::derive_internals]
mod wrapper {
    use super::{IndexMap, IndexSet};

    use crate::model::Identifier;
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

        Trivia { item: TriviaItem },
        Keyword { item: KeywordItem },
        Token { item: TokenItem },

        Fragment { item: FragmentItem },
    }

    impl Item {
        pub fn name(&self) -> &Identifier {
            match self {
                Item::Struct { item } => &item.name,
                Item::Enum { item } => &item.name,
                Item::Repeated { item } => &item.name,
                Item::Separated { item } => &item.name,
                Item::Precedence { item } => &item.name,
                Item::Trivia { item } => &item.name,
                Item::Keyword { item } => &item.name,
                Item::Token { item } => &item.name,
                Item::Fragment { item } => &item.name,
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct StructItem {
        pub name: Identifier,

        pub enabled: Option<VersionSpecifier>,

        pub error_recovery: Option<FieldsErrorRecovery>,
        pub fields: IndexMap<Identifier, Field>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct EnumItem {
        pub name: Identifier,

        pub enabled: Option<VersionSpecifier>,

        pub variants: Vec<EnumVariant>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct EnumVariant {
        pub name: Identifier,

        pub enabled: Option<VersionSpecifier>,

        pub reference: Identifier,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct RepeatedItem {
        pub name: Identifier,
        pub repeated: Identifier,

        pub enabled: Option<VersionSpecifier>,

        pub allow_empty: Option<bool>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct SeparatedItem {
        pub name: Identifier,
        pub separated: Identifier,
        pub separator: Identifier,

        pub enabled: Option<VersionSpecifier>,

        pub allow_empty: Option<bool>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct PrecedenceItem {
        pub name: Identifier,

        pub enabled: Option<VersionSpecifier>,

        pub precedence_expressions: Vec<PrecedenceExpression>,
        pub primary_expressions: Vec<PrimaryExpression>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct PrecedenceExpression {
        pub name: Identifier,
        // TODO(#638): Remove this, once we adapt the DSL v1 codegen model to the new v2 definition.
        pub rule_name: Identifier,

        pub operators: Vec<PrecedenceOperator>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct PrecedenceOperator {
        pub model: OperatorModel,

        pub enabled: Option<VersionSpecifier>,

        pub error_recovery: Option<FieldsErrorRecovery>,
        pub fields: IndexMap<Identifier, Field>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
    pub enum OperatorModel {
        Prefix,
        Postfix,
        BinaryLeftAssociative,
        BinaryRightAssociative,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct PrimaryExpression {
        pub expression: Identifier,

        pub enabled: Option<VersionSpecifier>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct FieldsErrorRecovery {
        pub terminator: Option<Identifier>,
        pub delimiters: Option<FieldDelimiters>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct FieldDelimiters {
        pub open: Identifier,
        pub close: Identifier,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum Field {
        Required {
            kind: FieldKind,
        },
        Optional {
            kind: FieldKind,

            enabled: Option<VersionSpecifier>,
        },
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum FieldKind {
        NonTerminal { item: Identifier },
        Terminal { items: IndexSet<Identifier> },
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum TriviaParser {
        Sequence { parsers: Vec<TriviaParser> },
        Choice { parsers: Vec<TriviaParser> },

        Optional { parser: Box<TriviaParser> },
        OneOrMore { parser: Box<TriviaParser> },
        ZeroOrMore { parser: Box<TriviaParser> },

        Trivia { trivia: Identifier },
    }

    #[derive(Debug, Eq, PartialEq, Serialize)]
    pub struct TriviaItem {
        pub name: Identifier,

        pub scanner: Scanner,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct KeywordItem {
        pub name: Identifier,
        pub identifier: Identifier,

        pub definitions: Vec<KeywordDefinition>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct KeywordDefinition {
        pub enabled: Option<VersionSpecifier>,
        pub reserved: Option<VersionSpecifier>,

        pub value: KeywordValue,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum KeywordValue {
        Sequence { values: Vec<KeywordValue> },
        Optional { value: Box<KeywordValue> },
        Choice { values: Vec<KeywordValue> },
        Atom { atom: String },
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct TokenItem {
        pub name: Identifier,

        pub definitions: Vec<TokenDefinition>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct TokenDefinition {
        pub enabled: Option<VersionSpecifier>,

        pub scanner: Scanner,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub struct FragmentItem {
        pub name: Identifier,

        pub enabled: Option<VersionSpecifier>,

        pub scanner: Scanner,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
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

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    pub enum VersionSpecifier {
        Never,
        From { from: Version },
        Till { till: Version },
        Range { from: Version, till: Version },
    }
}
