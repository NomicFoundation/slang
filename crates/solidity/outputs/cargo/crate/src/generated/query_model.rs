// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use super::kinds::{FieldName, RuleKind, TokenKind};

#[derive(Clone)]
pub enum Query {
    Binding(Rc<BindingMatch>),
    Node(Rc<NodeMatch>),
    Optional(Rc<OptionalMatch>),
    Alternatives(Rc<AlternativesMatch>),
    Sequence(Rc<SequenceMatch>),
    OneOrMore(Rc<OneOrMoreMatch>),
    Ellipsis,
}

impl Query {
    pub fn parse(text: &str) -> Result<Self, String> {
        super::query_parser::parse_query(text)
    }

    pub fn unparse(&self) -> String {
        match self {
            Self::Binding(binding) => {
                format!("@{} {}", binding.name, binding.child.unparse())
            }
            Self::Node(node) => {
                if let Some(child) = &node.child {
                    format!("[{} {}]", node.id.unparse(), child.unparse())
                } else {
                    format!("[{}]", node.id.unparse())
                }
            }
            Self::Optional(optional) => {
                format!("({})?", optional.child.unparse())
            }
            Self::Alternatives(alternatives) => {
                format!(
                    "({})",
                    alternatives
                        .children
                        .iter()
                        .map(|c| c.unparse())
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
            }
            Self::Sequence(sequence) => sequence
                .children
                .iter()
                .map(|c| c.unparse())
                .collect::<Vec<_>>()
                .join(" "),
            Self::OneOrMore(one_or_more) => {
                format!("({})+", one_or_more.child.unparse())
            }
            Self::Ellipsis => "...".to_string(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    Rule(RuleKind),
    Token(TokenKind),
}

impl Kind {
    fn unparse(self) -> String {
        match self {
            Kind::Rule(rule) => rule.to_string(),
            Kind::Token(token) => token.to_string(),
        }
    }
}

#[derive(Clone)]
pub enum NodeId {
    Anonymous,
    Kind { kind: Kind },
    String { string: String },
    Field { field: FieldName },
    FieldAndKind { field: FieldName, kind: Kind },
    FieldAndString { field: FieldName, string: String },
}

impl NodeId {
    fn unparse(&self) -> String {
        match self {
            NodeId::Anonymous => "_".to_string(),
            NodeId::Kind { kind } => kind.unparse(),
            NodeId::String { string } => format!("\"{}\"", Self::escape_string(string)),
            NodeId::Field { field } => field.to_string(),
            NodeId::FieldAndKind { field, kind } => {
                format!("{field}; {}", kind.unparse())
            }
            NodeId::FieldAndString { field, string } => {
                format!("{field}: \"{}\"", Self::escape_string(string))
            }
        }
    }

    fn escape_string(string: &str) -> String {
        string
            .chars()
            .map(|c| match c {
                '"' => "\\\"".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                '\u{08}' => "\\b".to_string(),
                '\u{0c}' => "\\f".to_string(),
                _ if c.is_ascii_graphic() => c.to_string(),
                _ => format!("\\u{{{:x}}}", c as u32),
            })
            .collect::<String>()
    }
}

pub struct BindingMatch {
    pub name: String,
    pub child: Query,
}

pub struct NodeMatch {
    pub id: NodeId,
    pub child: Option<Query>,
}

pub struct SequenceMatch {
    pub children: Vec<Query>,
}

pub struct AlternativesMatch {
    pub children: Vec<Query>,
}

pub struct OptionalMatch {
    pub child: Query,
}

pub struct OneOrMoreMatch {
    pub child: Query,
}
