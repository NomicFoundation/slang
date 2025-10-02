use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use crate::kinds::{BaseKind, KindTypes, NodeKind};
use crate::query::{CaptureQuantifier, QueryError};
use crate::text_index::TextIndex;

/// The declarative `Query` API is a convenient alternative to the [`Cursor`][`crate::cursor::Cursor`]
/// API for navigating the CST.
///
/// The query engine performs pattern matching, and the execution semantics are closer to
/// unification than to regular expression matching. A query returns all possible matches,
/// not just the longest/shortest/first/last match.
///
/// Please refer to [our documentation](https://nomicfoundation.github.io/slang/latest/user-guide/06-query-language/01-query-syntax/)
/// for detailed information about the query syntax and how to use queries to find matches.
#[derive(Clone, Debug)]
pub struct Query<T: KindTypes> {
    ast_node: ASTNode<T>,
    capture_quantifiers: BTreeMap<String, CaptureQuantifier>,
}

impl<T: KindTypes> Query<T> {
    /// Parses a query string into a query object. It will return a [`QueryError`]
    /// if the query syntax is invalid.
    pub fn create(text: &str) -> Result<Self, QueryError> {
        fn collect_capture_quantifiers<T: KindTypes>(
            ast_node: &ASTNode<T>,
            quantifier: CaptureQuantifier,
            capture_quantifiers: &mut BTreeMap<String, CaptureQuantifier>,
        ) -> Result<(), QueryError> {
            match ast_node {
                ASTNode::Capture(capture) => {
                    // If the capture has already been used, return an error
                    if capture_quantifiers.contains_key(&capture.name) {
                        return Err(QueryError::create(
                            format!("Capture name '{}' used more than once", capture.name),
                            TextIndex::ZERO..TextIndex::ZERO,
                        ));
                    }
                    capture_quantifiers.insert(capture.name.clone(), quantifier);
                    collect_capture_quantifiers(&capture.child, quantifier, capture_quantifiers)?;
                }
                ASTNode::NodeMatch(node_match) => {
                    if let Some(child) = &node_match.child {
                        collect_capture_quantifiers(child, quantifier, capture_quantifiers)?;
                    }
                }
                ASTNode::Optional(optional) => {
                    let quantifier = match quantifier {
                        CaptureQuantifier::One => CaptureQuantifier::ZeroOrOne,
                        CaptureQuantifier::ZeroOrOne => CaptureQuantifier::ZeroOrOne,
                        CaptureQuantifier::OneOrMore => CaptureQuantifier::ZeroOrMore,
                        CaptureQuantifier::ZeroOrMore => CaptureQuantifier::ZeroOrMore,
                    };
                    collect_capture_quantifiers(&optional.child, quantifier, capture_quantifiers)?;
                }
                ASTNode::Alternatives(alternatives) => {
                    let quantifier = match quantifier {
                        CaptureQuantifier::One => CaptureQuantifier::ZeroOrOne,
                        CaptureQuantifier::ZeroOrOne => CaptureQuantifier::ZeroOrOne,
                        CaptureQuantifier::OneOrMore => CaptureQuantifier::ZeroOrMore,
                        CaptureQuantifier::ZeroOrMore => CaptureQuantifier::ZeroOrMore,
                    };
                    for child in &alternatives.children {
                        collect_capture_quantifiers(child, quantifier, capture_quantifiers)?;
                    }
                }
                ASTNode::Sequence(sequence) => {
                    for child in &sequence.children {
                        collect_capture_quantifiers(child, quantifier, capture_quantifiers)?;
                    }
                }
                ASTNode::OneOrMore(one_or_more) => {
                    let quantifier = match quantifier {
                        CaptureQuantifier::One => CaptureQuantifier::OneOrMore,
                        CaptureQuantifier::ZeroOrOne => CaptureQuantifier::ZeroOrMore,
                        CaptureQuantifier::OneOrMore | CaptureQuantifier::ZeroOrMore => {
                            return Err(QueryError::create(
                                "Quantification over quantification is not allowed".to_string(),
                                TextIndex::ZERO..TextIndex::ZERO,
                            ))
                        }
                    };
                    collect_capture_quantifiers(
                        &one_or_more.child,
                        quantifier,
                        capture_quantifiers,
                    )?;
                }
                ASTNode::Adjacency => {}
            }
            Ok(())
        }

        let ast_node = ASTNode::parse(text)?;

        let mut capture_quantifiers = BTreeMap::new();

        collect_capture_quantifiers(&ast_node, CaptureQuantifier::One, &mut capture_quantifiers)?;

        Ok(Self {
            ast_node,
            capture_quantifiers,
        })
    }

    /// The abstract syntax tree (AST) representation of the query.
    pub(crate) fn ast_node(&self) -> &ASTNode<T> {
        &self.ast_node
    }

    /// A map of capture names to their quantifiers, used to define how many times a capture can occur.
    pub fn capture_quantifiers(&self) -> &BTreeMap<String, CaptureQuantifier> {
        &self.capture_quantifiers
    }
}

impl<T: KindTypes> fmt::Display for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast_node.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub enum ASTNode<T: KindTypes> {
    Capture(Rc<CaptureASTNode<T>>),
    NodeMatch(Rc<NodeMatchASTNode<T>>),
    Optional(Rc<OptionalASTNode<T>>),
    Alternatives(Rc<AlternativesASTNode<T>>),
    Sequence(Rc<SequenceASTNode<T>>),
    OneOrMore(Rc<OneOrMoreASTNode<T>>),
    Adjacency,
}

impl<T: KindTypes> ASTNode<T> {
    fn parse(text: &str) -> Result<Self, QueryError> {
        super::parser::parse_query(text)
    }
}

impl<T: KindTypes> fmt::Display for ASTNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capture(capture) => {
                write!(f, "@{} {}", capture.name, capture.child)
            }
            Self::NodeMatch(node) => {
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
                    }
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
                    }
                    a.fmt(f)?;
                }
                Ok(())
            }
            Self::OneOrMore(one_or_more) => {
                write!(f, "({})+", one_or_more.child)
            }
            Self::Adjacency => write!(f, "."),
        }
    }
}

#[derive(Clone, Debug)]
pub enum NodeSelector<T: KindTypes> {
    Anonymous,
    NodeKind {
        node_kind: NodeKind<T>,
    },
    NodeText {
        node_text: String,
    },
    EdgeLabel {
        edge_label: T::EdgeLabel,
    },
    EdgeLabelAndNodeKind {
        edge_label: T::EdgeLabel,
        node_kind: NodeKind<T>,
    },
    EdgeLabelAndNodeText {
        edge_label: T::EdgeLabel,
        node_text: String,
    },
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
            Self::NodeKind { node_kind } => node_kind.fmt(f),
            Self::NodeText { node_text } => write!(f, "\"{}\"", escape_string(node_text)),
            Self::EdgeLabel { edge_label } => {
                write!(f, "{}", edge_label.as_static_str())
            }
            Self::EdgeLabelAndNodeKind {
                edge_label,
                node_kind,
            } => {
                write!(f, "{}; {node_kind}", edge_label.as_static_str())
            }
            Self::EdgeLabelAndNodeText {
                edge_label,
                node_text,
            } => {
                write!(
                    f,
                    "{}: \"{}\"",
                    edge_label.as_static_str(),
                    escape_string(node_text)
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct CaptureASTNode<T: KindTypes> {
    pub name: String,
    pub child: ASTNode<T>,
}

#[derive(Debug)]
pub struct NodeMatchASTNode<T: KindTypes> {
    pub node_selector: NodeSelector<T>,
    pub child: Option<ASTNode<T>>,
}

#[derive(Debug)]
pub struct SequenceASTNode<T: KindTypes> {
    pub children: Vec<ASTNode<T>>,
    // By default sequences can match any number of nodes at the beginning and
    // end of it. Setting this value to true prevents it and instead forces
    // strict adjacency at the edges.
    pub adjacent: bool,
}

#[derive(Debug)]
pub struct AlternativesASTNode<T: KindTypes> {
    pub children: Vec<ASTNode<T>>,
}

#[derive(Debug)]
pub struct OptionalASTNode<T: KindTypes> {
    pub child: ASTNode<T>,
}

#[derive(Debug)]
pub struct OneOrMoreASTNode<T: KindTypes> {
    pub child: ASTNode<T>,
}
