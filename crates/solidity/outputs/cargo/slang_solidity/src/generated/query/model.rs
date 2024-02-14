// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::fmt;
use std::rc::Rc;

// This crate is copied to another crate, so all imports should be relative
use super::super::kinds::{FieldName, RuleKind, TokenKind};

#[derive(Clone)]
pub struct Query(pub(super) Matcher);

impl Query {
    pub fn parse(text: &str) -> Result<Self, String> {
        Matcher::parse(text).map(Self)
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
pub(super) enum Matcher {
    Binding(Rc<BindingMatcher>),
    Node(Rc<NodeMatcher>),
    Optional(Rc<OptionalMatcher>),
    Alternatives(Rc<AlternativesMatcher>),
    Sequence(Rc<SequenceMatcher>),
    OneOrMore(Rc<OneOrMoreMatcher>),
    Ellipsis,
}

impl Matcher {
    fn parse(text: &str) -> Result<Self, String> {
        super::parser::parse_query(text)
    }
}

impl fmt::Display for Matcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binding(binding) => {
                write!(f, "@{} {}", binding.name, binding.child)
            }
            Self::Node(node) => {
                if let Some(child) = &node.child {
                    write!(f, "[{} {}]", node.node_selector, child)
                } else {
                    write!(f, "[{}]", node.node_selector)
                }
            }
            Self::Optional(optional) => {
                write!(f, "({})?", optional.child)
            }
            Self::Alternatives(alternatives) => {
                let mut done_first = false;
                write!(f, "(")?;
                for a in &alternatives.children {
                    if done_first {
                        write!(f, " | ")?;
                    } else {
                        done_first = true;
                    };
                    a.fmt(f)?;
                }
                write!(f, ")")?;
                Ok(())
            }
            Self::Sequence(sequence) => {
                let mut done_first = false;
                for a in &sequence.children {
                    if done_first {
                        write!(f, " ")?;
                    } else {
                        done_first = true;
                    };
                    a.fmt(f)?;
                }
                Ok(())
            }
            Self::OneOrMore(one_or_more) => {
                write!(f, "({})+", one_or_more.child)
            }
            Self::Ellipsis => write!(f, "..."),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum Kind {
    Rule(RuleKind),
    Token(TokenKind),
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Rule(rule) => write!(f, "{rule}"),
            Kind::Token(token) => write!(f, "{token}"),
        }
    }
}

#[derive(Clone)]
pub(super) enum NodeSelector {
    Anonymous,
    Kind { kind: Kind },
    Text { text: String },
    FieldName { field_name: FieldName },
    FieldNameAndKind { field_name: FieldName, kind: Kind },
    FieldNameAndText { field_name: FieldName, text: String },
}

impl fmt::Display for NodeSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        match self {
            Self::Anonymous => write!(f, "_"),
            Self::Kind { kind } => kind.fmt(f),
            Self::Text { text } => write!(f, "\"{}\"", escape_string(text)),
            Self::FieldName { field_name } => field_name.fmt(f),
            Self::FieldNameAndKind { field_name, kind } => {
                write!(f, "{field_name}; {kind}")
            }
            Self::FieldNameAndText { field_name, text } => {
                write!(f, "{field_name}: \"{}\"", escape_string(text))
            }
        }
    }
}

pub(super) struct BindingMatcher {
    pub name: String,
    pub child: Matcher,
}

pub(super) struct NodeMatcher {
    pub node_selector: NodeSelector,
    pub child: Option<Matcher>,
}

pub(super) struct SequenceMatcher {
    pub children: Vec<Matcher>,
}

pub(super) struct AlternativesMatcher {
    pub children: Vec<Matcher>,
}

pub(super) struct OptionalMatcher {
    pub child: Matcher,
}

pub(super) struct OneOrMoreMatcher {
    pub child: Matcher,
}
