// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)] // large match statements for all non-terminals
#![allow(clippy::unnecessary_wraps)] // using `Result` for all functions for error handling

use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{NAPINodeExtensions, NonterminalNode, TerminalNode};
use crate::napi_interface::{EdgeLabel, NonterminalKind, RustEdge, RustNode, RustNonterminalNode};

//
// Sequences:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node | null>",
    catch_unwind
)]
pub fn select_sequence(
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
    unreachable!("Invoking AST selectors in stubs: {node:#?}")
} //
  // Choices:
  //

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Either<NonterminalNode, TerminalNode>> {
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
    unreachable!("Invoking AST selectors in stubs: {node:#?}")
}

//
// Common:
//

struct Selector {
    node: Rc<RustNonterminalNode>,
    index: usize,
}

impl Selector {
    fn new(node: &NonterminalNode) -> Self {
        Self {
            node: Rc::clone(&node.0),
            index: 0,
        }
    }

    fn select(&mut self, target_label: EdgeLabel) -> Result<Either<NonterminalNode, TerminalNode>> {
        match self.try_select(target_label) {
            Some(node) => Ok(node),
            None => Error::MissingChild(target_label).into(),
        }
    }

    fn try_select(
        &mut self,
        target_label: EdgeLabel,
    ) -> Option<Either<NonterminalNode, TerminalNode>> {
        let (label, node) = self.current()?;

        if label == target_label {
            self.index += 1;
            Some(node.clone().into_js_either_node())
        } else {
            None
        }
    }

    fn current(&mut self) -> Option<(EdgeLabel, RustNode)> {
        loop {
            let RustEdge { label, node } = self.node.children.get(self.index)?;

            match label {
                // Skip unlabeled nodes:
                | None
                // Skip trivia:
                | Some(EdgeLabel::LeadingTrivia | EdgeLabel::TrailingTrivia) => {
                    self.index += 1;
                    continue;
                }
                // Otherwise, return the edge:
                Some(other_label) => {
                    return Some((*other_label, node.clone()));
                }
            }
        }
    }

    fn finalize(mut self) -> Result<()> {
        match self.current() {
            Some((label, _)) => Error::UnrecognizedChild(label).into(),
            _ => Ok(()),
        }
    }
}

type Result<T> = std::result::Result<T, napi::Error>;

#[derive(Debug, thiserror::Error)]
enum Error {
    // Should not theoretically happen, since we're only called from our own generated AST types.
    #[error("Unexpected parent node with NonterminalKind '{0}'.")]
    UnexpectedParent(NonterminalKind),

    #[error("Unrecognized child with label '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    UnrecognizedChild(EdgeLabel),

    #[error("Missing child with label '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    MissingChild(EdgeLabel),
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(napi::Error::from_reason(error.to_string()))
    }
}
