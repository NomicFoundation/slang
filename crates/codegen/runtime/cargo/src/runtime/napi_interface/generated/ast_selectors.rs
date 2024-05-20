// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use napi::bindgen_prelude::Either3;
use napi_derive::napi;

use crate::napi_interface::cst::{InvalidNode, NAPINodeExtensions, RuleNode, TokenNode};
#[allow(unused_imports)] // testlang does not use it
use crate::napi_interface::TokenKind;
use crate::napi_interface::{RuleKind, RustNode, RustRuleNode};

// NOTE: We cannot use it in `#[napi]`-decorated functions, because it doesn't have type information.
type EitherNode = Either3<RuleNode, TokenNode, InvalidNode>;

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
) -> Result<Vec<Option<Either3<RuleNode, TokenNode, InvalidNode>>>> {
    unreachable!("Invoking AST selectors in stubs: {node:#?}")
} //
  // Choices:
  //

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
) -> Result<Either3<RuleNode, TokenNode, InvalidNode>> {
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
) -> Result<Vec<Either3<RuleNode, TokenNode, InvalidNode>>> {
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
) -> Result<Vec<Vec<Either3<RuleNode, TokenNode, InvalidNode>>>> {
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

    fn select(&mut self, filter: impl FnOnce(&RustNode) -> bool) -> Result<EitherNode> {
        match self.try_select(filter)? {
            Some(node) => Ok(node),
            None => Error::MissingChild(self.index).into(),
        }
    }

    fn try_select(&mut self, filter: impl FnOnce(&RustNode) -> bool) -> Result<Option<EitherNode>> {
        while let Some(child) = self.node.children.get(self.index) {
            match &**child {
                node if node.is_trivia() => {
                    // skip trivia, since it's not part of the AST
                    self.index += 1;
                    continue;
                }
                RustNode::Invalid(_) => {
                    return Error::InvalidNode(self.index).into();
                }
                node if filter(node) => {
                    self.index += 1;
                    return Ok(Some(node.clone().into_js_either_node()));
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
    #[error("Unexpected invalid node at index '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    InvalidNode(usize),
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(napi::Error::from_reason(error.to_string()))
    }
}
