use std::rc::Rc;

use crate::cst::{RuleNode, TokenNode};
pub use crate::kinds::{RuleKind, TokenKind};

// TODO: xpath?

#[macro_export]
macro_rules! token {
    (@tail $result:tt first $token:ident) => {
        $result.token($crate::kinds::TokenKind::$token, $crate::tree_path::Qualifier::First)
    };
    (@tail $result:tt last $token:ident) => {
        $result.token($crate::kinds::TokenKind::$token, $crate::tree_path::Qualifier::Last)
    };
    (@tail $result:tt $token:ident) => {
        $result.token($crate::kinds::TokenKind::$token, $crate::tree_path::Qualifier::All)
    };
    (@tail $result:tt first $rule:ident $($rest:ident)+) => {
        token!(@tail ($result.rule($crate::kinds::RuleKind::$rule, $crate::tree_path::Qualifier::First)) $($rest)+)
    };
    (@tail $result:tt last $rule:ident $($rest:ident)+) => {
        token!(@tail ($result.rule($crate::kinds::RuleKind::$rule, $crate::tree_path::Qualifier::Last)) $($rest)+)
    };
    (@tail $result:tt $rule:ident $($rest:ident)+) => {
        token!(@tail ($result.rule($crate::kinds::RuleKind::$rule, $crate::tree_path::Qualifier::All)) $($rest)+)
    };
    ($($rest:ident)+) => {
        token!(@tail ($crate::tree_path::RuleTreePath::default()) $($rest)+)
    }
}

#[macro_export]
macro_rules! rule {
    (@tail $result:tt ) => { $result };
    (@tail $result:tt first $rule:ident $($rest:ident)*) => {
        rule!(@tail ($result.rule($crate::kinds::RuleKind::$rule, $crate::tree_path::Qualifier::First)) $($rest)*)
    };
    (@tail $result:tt last $rule:ident $($rest:ident)*) => {
        rule!(@tail ($result.rule($crate::kinds::RuleKind::$rule, $crate::tree_path::Qualifier::Last)) $($rest)*)
    };
    (@tail $result:tt $rule:ident $($rest:ident)*) => {
        rule!(@tail ($result.rule($crate::kinds::RuleKind::$rule, $crate::tree_path::Qualifier::All)) $($rest)*)
    };
    ($($rest:ident)*) => {
        rule!(@tail ($crate::tree_path::RuleTreePath::default()) $($rest)*)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Qualifier {
    First,
    Last,
    All,
}

#[derive(Default, Clone)]
pub struct RuleTreePath {
    steps: Vec<(RuleKind, Qualifier)>,
}

impl From<RuleKind> for RuleTreePath {
    fn from(kind: RuleKind) -> Self {
        Self::default().rule(kind, Qualifier::All)
    }
}

impl RuleTreePath {
    pub fn head_rule_kind(&self) -> Option<RuleKind> {
        self.steps.first().map(|(kind, _)| *kind)
    }

    pub fn as_node_tree_path(self) -> NodeTreePath {
        NodeTreePath::Rule(self)
    }

    pub fn rule(mut self, kind: RuleKind, qualifier: Qualifier) -> Self {
        self.steps.push((kind, qualifier));
        self
    }

    pub fn token(self, kind: TokenKind, qualifier: Qualifier) -> TokenTreePath {
        TokenTreePath {
            rule_path: self,
            kind,
            qualifier,
        }
    }

    // TODO: when this is generated, use a chained iterator
    pub fn resolve(&self, root: Rc<RuleNode>) -> Vec<Rc<RuleNode>> {
        let mut result = vec![root];
        for (kind, qualifier) in &self.steps {
            result = match qualifier {
                Qualifier::Last => result
                    .iter()
                    .flat_map(|node| {
                        node.children
                            .iter()
                            .rev()
                            .filter_map(|child| child.as_rule_with_kind(&[*kind]).cloned())
                            .take(1)
                    })
                    .collect(),
                Qualifier::First => result
                    .iter()
                    .flat_map(|node| {
                        node.children
                            .iter()
                            .filter_map(|child| child.as_rule_with_kind(&[*kind]).cloned())
                            .take(1)
                    })
                    .collect(),
                Qualifier::All => result
                    .iter()
                    .flat_map(|node| {
                        node.children
                            .iter()
                            .filter_map(|child| child.as_rule_with_kind(&[*kind]).cloned())
                    })
                    .collect(),
            }
        }
        result
    }
}

#[derive(Clone)]
pub struct TokenTreePath {
    rule_path: RuleTreePath,
    kind: TokenKind,
    qualifier: Qualifier,
}

impl From<TokenKind> for TokenTreePath {
    fn from(kind: TokenKind) -> Self {
        RuleTreePath::default().token(kind, Qualifier::All)
    }
}

impl TokenTreePath {
    pub fn as_node_tree_path(self) -> NodeTreePath {
        NodeTreePath::Token(self)
    }

    // TODO: when this is generated, use a chained iterator
    pub fn resolve(&self, root: Rc<RuleNode>) -> Vec<Rc<TokenNode>> {
        let rule_nodes = self.rule_path.resolve(root);
        match self.qualifier {
            Qualifier::Last => rule_nodes
                .iter()
                .flat_map(|node| {
                    node.children
                        .iter()
                        .rev()
                        .filter_map(|child| child.as_token_with_kind(&[self.kind]).cloned())
                        .take(1)
                })
                .collect(),
            Qualifier::First => rule_nodes
                .iter()
                .flat_map(|node| {
                    node.children
                        .iter()
                        .filter_map(|child| child.as_token_with_kind(&[self.kind]).cloned())
                        .take(1)
                })
                .collect(),
            Qualifier::All => rule_nodes
                .iter()
                .flat_map(|node| {
                    node.children
                        .iter()
                        .filter_map(|child| child.as_token_with_kind(&[self.kind]).cloned())
                })
                .collect(),
        }
    }
}

#[derive(Clone)]
pub enum NodeTreePath {
    Rule(RuleTreePath),
    Token(TokenTreePath),
}
