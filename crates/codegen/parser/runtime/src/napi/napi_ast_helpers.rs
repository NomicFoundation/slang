use std::rc::Rc;

use napi::{Env, JsObject};

use crate::kinds::{RuleKind, TokenKind};
use crate::napi::napi_cst::{RuleNode, ToJS};
use crate::napi::{RustNamedNode, RustNode, RustRuleNode};

pub struct Helper {
    env: Env,
    node: Rc<RustRuleNode>,
    index: usize,
}

impl Helper {
    pub fn new(node: &RuleNode, kind: RuleKind, env: Env) -> Self {
        assert_eq!(
            node.kind(),
            kind,
            "Expected a parent RuleNode of kind '{kind:?}'."
        );

        Self {
            env,
            node: node.0.clone(),
            index: 0,
        }
    }

    pub fn pick(&mut self, filter: impl FnOnce(&RustNode) -> bool) -> JsObject {
        self.try_pick(filter).unwrap_or_else(|| {
            panic!("Unexpected child at index '{index}'.", index = self.index);
        })
    }

    pub fn try_pick(&mut self, filter: impl FnOnce(&RustNode) -> bool) -> Option<JsObject> {
        loop {
            match self.node.children.get(self.index)? {
                RustNamedNode {
                    name: _,
                    node: RustNode::Rule(rule),
                } if matches!(
                    rule.kind,
                    RuleKind::LeadingTrivia | RuleKind::TrailingTrivia
                ) =>
                {
                    // skip trivia, since it's not part of the AST
                    self.index += 1;
                    continue;
                }
                RustNamedNode {
                    name: _,
                    node: RustNode::Token(token),
                } if matches!(token.kind, TokenKind::SKIPPED) => {
                    panic!(
                        "Unexpected SKIPPED token at index '{index}'.",
                        index = self.index,
                    );
                }
                node if filter(node) => {
                    self.index += 1;
                    return Some(node.to_js(&self.env));
                }
                _ => {
                    return None;
                }
            }
        }
    }
}

impl Drop for Helper {
    fn drop(&mut self) {
        assert!(
            self.try_pick(|_| true).is_none(),
            "Unexpected trailing children at index '{index}'.",
            index = self.index - 1,
        );
    }
}
