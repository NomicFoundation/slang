use std::fmt;
use std::rc::Rc;

// This crate is copied to another crate, so all imports should be relative
use crate::ModuleInputs;

#[derive(Clone)]
pub struct Query<T: ModuleInputs>(pub(super) Matcher<T>);

impl<T: ModuleInputs> Query<T> {
    pub fn parse(text: &str) -> Result<Self, String> {
        Matcher::parse(text).map(Self)
    }
}

impl<T: ModuleInputs> fmt::Display for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
pub(super) enum Matcher<T: ModuleInputs> {
    Binding(Rc<BindingMatcher<T>>),
    Node(Rc<NodeMatcher<T>>),
    Optional(Rc<OptionalMatcher<T>>),
    Alternatives(Rc<AlternativesMatcher<T>>),
    Sequence(Rc<SequenceMatcher<T>>),
    OneOrMore(Rc<OneOrMoreMatcher<T>>),
    Ellipsis,
}

impl<T: ModuleInputs> Matcher<T> {
    fn parse(text: &str) -> Result<Self, String> {
        super::parser::parse_query(text)
    }
}

impl<T: ModuleInputs> fmt::Display for Matcher<T> {
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
pub(super) enum Kind<T: ModuleInputs> {
    Rule(T::NonTerminalKind),
    Token(T::TerminalKind),
}

impl<T: ModuleInputs> fmt::Display for Kind<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rule(rule) => write!(f, "{rule}"),
            Self::Token(token) => write!(f, "{token}"),
        }
    }
}

#[derive(Clone)]
pub(super) enum NodeSelector<T: ModuleInputs> {
    Anonymous,
    Kind { kind: Kind<T> },
    Text { text: String },
    Label { label: T::LabelKind },
    LabelAndKind { label: T::LabelKind, kind: Kind<T> },
    LabelAndText { label: T::LabelKind, text: String },
}

impl<T: ModuleInputs> fmt::Display for NodeSelector<T> {
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
            Self::Label { label } => label.fmt(f),
            Self::LabelAndKind { label, kind } => {
                write!(f, "{label}; {kind}")
            }
            Self::LabelAndText { label, text } => {
                write!(f, "{label}: \"{}\"", escape_string(text))
            }
        }
    }
}

pub(super) struct BindingMatcher<T: ModuleInputs> {
    pub name: String,
    pub child: Matcher<T>,
}

pub(super) struct NodeMatcher<T: ModuleInputs> {
    pub node_selector: NodeSelector<T>,
    pub child: Option<Matcher<T>>,
}

pub(super) struct SequenceMatcher<T: ModuleInputs> {
    pub children: Vec<Matcher<T>>,
}

pub(super) struct AlternativesMatcher<T: ModuleInputs> {
    pub children: Vec<Matcher<T>>,
}

pub(super) struct OptionalMatcher<T: ModuleInputs> {
    pub child: Matcher<T>,
}

pub(super) struct OneOrMoreMatcher<T: ModuleInputs> {
    pub child: Matcher<T>,
}
