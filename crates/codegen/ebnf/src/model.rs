use language_definition::model::Identifier;

/// A [`Entry`] holds all definitions under the same name.
/// Some grammar items can produce more than one definition each (tokens, keywords, operators).
///
/// Additionally, it computes the EBNF ID from the name, accourding to the following rules:
///
/// - For nonterminals, we use the original name in `PascalCase`.
/// - For terminals, we use the name in `SCREAMING_SNAKE_CASE`.
/// - For fragments, we add `«guillemets»` around the name.
#[derive(derive_new::new)]
pub struct Entry {
    pub name: Identifier,
    pub ebnf_id: String,

    pub section_index: usize,
    pub topic_index: usize,

    pub definitions: Vec<Definition>,
}

/// Each EBNF [`Definition`] starts with zero or more lines of doc comments,
/// then its name, an equal `=` sign, and a value, which is one of two kinds:
///
/// ```ebnf
/// (* A sequence of values, split into multiple lines: *)
/// Name = Value1
///        Value2
///        Value3;
///
/// (* A choice of values, split into multiple lines, and separated by a bar `|`: *)
/// Name = Value1
///      | Value2
///      | Value3;
/// ```
#[derive(derive_new::new)]
pub struct Definition {
    pub leading_comments: Vec<String>,
    pub values: Vec<Value>,
    pub kind: DefinitionKind,
}

pub enum DefinitionKind {
    Sequence,
    Choice,
}

/// A [`Value`] is a single EBNF [`Expression`], followed by an optional trailing comment (at the end of the line).
///
/// ```ebnf
/// Name = Value; (* optional trailing comment for Value *)
/// ```
#[derive(derive_new::new)]
pub struct Value {
    pub expression: Expression,
    pub trailing_comment: Option<String>,
}

/// An [`Expression`] is a recursive structure that serializes to a single EBNF snippet without any comments.
///
/// ```ebnf
/// Foo = Bar (Baz | "c")+;
///       ^^^^^^^^^^^^^^^^
/// ```
#[derive(derive_new::new)]
pub enum Expression {
    Sequence {
        expressions: Vec<Self>,
    },
    Choice {
        expressions: Vec<Self>,
    },
    Optional {
        expression: Box<Self>,
    },
    ZeroOrMore {
        expression: Box<Self>,
    },
    OneOrMore {
        expression: Box<Self>,
    },
    Not {
        expression: Box<Self>,
    },
    Range {
        inclusive_start: Box<Self>,
        inclusive_end: Box<Self>,
    },
    Atom {
        atom: String,
    },
    NegativeLookAhead {
        expression: Box<Self>,
    },
    Reference {
        leading_comment: Option<String>,
        reference: Identifier,
    },
}

impl Expression {
    pub fn precedence(&self) -> u8 {
        // We are specifying precedence "groups" instead of a flat list.
        // This separates members of the same precedence, like both "a b (c | d)" and "a | b | (c d)".
        match self {
            // Binary
            Self::Choice { .. } | Self::Sequence { .. } => 1,

            // Prefix
            Self::Not { .. } => 2,

            // Postfix
            Self::OneOrMore { .. } | Self::Optional { .. } | Self::ZeroOrMore { .. } => 3,

            // Primary
            Self::Range { .. }
            | Self::Atom { .. }
            | Self::NegativeLookAhead { .. }
            | Self::Reference { .. } => 4,
        }
    }
}
