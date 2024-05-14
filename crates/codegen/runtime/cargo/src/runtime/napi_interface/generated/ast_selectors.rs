// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{RuleNode, TokenNode};
use crate::napi_interface::{RuleKind, RustLabeledNode, RustNode, RustRuleNode, TokenKind};

//
// Sequences:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node | null>",
    catch_unwind
)]
pub fn select_sequence(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
) -> Result<Vec<Option<Either<RuleNode, TokenNode>>>> {
    unreachable!("Invoking AST selectors in stubs: {node:#?}")
} //
  // Choices:
  //

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
) -> Result<Either<RuleNode, TokenNode>> {
    unreachable!("Invoking AST selectors in stubs: {node:#?}")
}

//
// Repeated:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node>",
    catch_unwind
)]
pub fn select_repeated(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
) -> Result<Vec<Either<RuleNode, TokenNode>>> {
    unreachable!("Invoking AST selectors in stubs: {node:#?}")
}

//
// Separated:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "[Array<cst.Node>, Array<cst.Node>]",
    catch_unwind
)]
pub fn select_separated(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
) -> Result<Vec<Vec<Either<RuleNode, TokenNode>>>> {
    unreachable!("Invoking AST selectors in stubs: {node:#?}")
}

//
// Common:
//

struct Selector {
    node: Rc<RustRuleNode>,
    index: usize,
}

impl Selector {
    fn new(node: &RuleNode) -> Self {
        Self {
            node: Rc::clone(&node.0),
            index: 0,
        }
    }

    fn select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Either<RuleNode, TokenNode>> {
        match self.try_select(filter)? {
            Some(node) => Ok(node),
            None => Error::MissingChild(self.index).into(),
        }
    }

    fn try_select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Option<Either<RuleNode, TokenNode>>> {
        while let Some(child) = self.node.children.get(self.index) {
            match child {
                node if node.is_trivia() => {
                    // skip trivia, since it's not part of the AST
                    self.index += 1;
                    continue;
                }
                RustLabeledNode {
                    label: _,
                    node: RustNode::Token(token),
                } if matches!(token.kind, TokenKind::SKIPPED) => {
                    return Error::SkippedToken(self.index).into();
                }
                labeled if filter(labeled) => {
                    self.index += 1;
                    return Ok(Some(labeled.node.clone().into_js_either_node()));
                }
                _ => {
                    break;
                }
            }
        }

        Ok(None)
    }

    fn finalize(mut self) -> Result<()> {
        if self.try_select(|_| true)?.is_some() {
            return Error::UnexpectedTrailing(self.index - 1).into();
        }

        Ok(())
    }
}

type Result<T> = std::result::Result<T, napi::Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    // Should not theoretically happen, since we're only called from our own generated AST types.
    #[error("Unexpected parent node with RuleKind '{0}'.")]
    UnexpectedParent(RuleKind),

    // Should not theoretically happen, since we're only called from our own generated AST types.
    #[error("Unexpected trailing children at index '{0}'.")]
    UnexpectedTrailing(usize),

    // Should not theoretically happen, unless AST error recovery was changed.
    #[error("Missing child node at index '{0}'.")]
    MissingChild(usize),

    // Can happen if the user decided to use an incorrect/incomplete CST node.
    #[error("Unexpected SKIPPED token at index '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    SkippedToken(usize),
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(napi::Error::from_reason(error.to_string()))
    }
}
