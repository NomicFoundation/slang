use std::fmt;
use std::rc::Rc;

use crate::KindTypes;

#[derive(Clone)]
pub struct Query<T: KindTypes>(pub(super) Matcher<T>);

impl<T: KindTypes> Query<T> {
    pub fn parse(text: &str) -> Result<Self, String> {
        Matcher::parse(text).map(Self)
    }
}

impl<T: KindTypes> fmt::Display for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
pub(super) enum Matcher<T: KindTypes> {
    Binding(Rc<BindingMatcher<T>>),
    Node(Rc<NodeMatcher<T>>),
    Optional(Rc<OptionalMatcher<T>>),
    Alternatives(Rc<AlternativesMatcher<T>>),
    Sequence(Rc<SequenceMatcher<T>>),
    OneOrMore(Rc<OneOrMoreMatcher<T>>),
    Ellipsis,
}

impl<T: KindTypes> Matcher<T> {
    fn parse(text: &str) -> Result<Self, String> {
        super::parser::parse_query(text)
    }
}

impl<T: KindTypes> fmt::Display for Matcher<T> {
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
pub(super) enum Kind<T: KindTypes> {
    Terminal(T::NonTerminalKind),
    NonTerminal(T::TerminalKind),
}

impl<T: KindTypes> fmt::Display for Kind<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Terminal(kind) => write!(f, "{kind}"),
            Self::NonTerminal(kind) => write!(f, "{kind}"),
        }
    }
}

#[derive(Clone)]
pub(super) enum NodeSelector<T: KindTypes> {
    Anonymous,
    Kind { kind: Kind<T> },
    Text { text: String },
    Label { label: T::EdgeLabel },
    LabelAndKind { label: T::EdgeLabel, kind: Kind<T> },
    LabelAndText { label: T::EdgeLabel, text: String },
}

impl<T: KindTypes> fmt::Display for NodeSelector<T> {
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
            Self::Label { label: edge } => edge.fmt(f),
            Self::LabelAndKind { label: edge, kind } => {
                write!(f, "{edge}; {kind}")
            }
            Self::LabelAndText { label: edge, text } => {
                write!(f, "{edge}: \"{}\"", escape_string(text))
            }
        }
    }
}

pub(super) struct BindingMatcher<T: KindTypes> {
    pub name: String,
    pub child: Matcher<T>,
}

pub(super) struct NodeMatcher<T: KindTypes> {
    pub node_selector: NodeSelector<T>,
    pub child: Option<Matcher<T>>,
}

pub(super) struct SequenceMatcher<T: KindTypes> {
    pub children: Vec<Matcher<T>>,
}

pub(super) struct AlternativesMatcher<T: KindTypes> {
    pub children: Vec<Matcher<T>>,
}

pub(super) struct OptionalMatcher<T: KindTypes> {
    pub child: Matcher<T>,
}

pub(super) struct OneOrMoreMatcher<T: KindTypes> {
    pub child: Matcher<T>,
}
