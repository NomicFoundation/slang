// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{NAPINodeExtensions, NonTerminalNode, TerminalNode};
use crate::napi_interface::{
    NonTerminalKind, RustEdge, RustNode, RustNonTerminalNode, TerminalKind,
};

//
// Sequences:
//

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node | null>",
    catch_unwind
)]
pub fn select_sequence(
    #[napi(ts_arg_type = "cst.NonTerminalNode")] node: &NonTerminalNode,
) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonTerminalKind::SourceUnit => selector.source_unit()?,
        NonTerminalKind::Tree => selector.tree()?,
        NonTerminalKind::TreeNode => selector.tree_node()?,
        NonTerminalKind::AdditionExpression => selector.addition_expression()?,
        NonTerminalKind::NegationExpression => selector.negation_expression()?,
        NonTerminalKind::MemberAccessExpression => selector.member_access_expression()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}
impl Selector {
    fn source_unit(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::SourceUnitMembers)
        })?)])
    }
}

impl Selector {
    fn tree(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::TreeKeyword))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TreeNode))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn tree_node(&mut self) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBracket))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonTerminalKind::TreeNodeChildren)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn addition_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Plus))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn negation_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Bang))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn member_access_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonTerminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonTerminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Period))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?),
        ])
    }
}
//
// Choices:
//

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.NonTerminalNode")] node: &NonTerminalNode,
) -> Result<Either<NonTerminalNode, TerminalNode>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonTerminalKind::SourceUnitMember => selector.source_unit_member()?,
        NonTerminalKind::TreeNodeChild => selector.tree_node_child()?,
        NonTerminalKind::Expression => selector.expression()?,
        NonTerminalKind::Literal => selector.literal()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_member(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::Tree,
                NonTerminalKind::Expression,
                NonTerminalKind::SeparatedIdentifiers,
                NonTerminalKind::Literal,
            ])
        })
    }
}

impl Selector {
    fn tree_node_child(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonTerminalKind::TreeNode)
                || node.is_terminal_with_kind(TerminalKind::DelimitedIdentifier)
        })
    }
}

impl Selector {
    fn expression(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonTerminalKind::AdditionExpression,
                NonTerminalKind::NegationExpression,
                NonTerminalKind::MemberAccessExpression,
            ]) || node
                .is_terminal_with_kinds(&[TerminalKind::StringLiteral, TerminalKind::Identifier])
        })
    }
}

impl Selector {
    fn literal(&mut self) -> Result<Either<NonTerminalNode, TerminalNode>> {
        self.select(|node| node.is_terminal_with_kind(TerminalKind::StringLiteral))
    }
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
    #[napi(ts_arg_type = "cst.NonTerminalNode")] node: &NonTerminalNode,
) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonTerminalKind::SourceUnitMembers => selector.source_unit_members()?,
        NonTerminalKind::TreeNodeChildren => selector.tree_node_children()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_members(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::SourceUnitMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn tree_node_children(&mut self) -> Result<Vec<Either<NonTerminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonTerminalKind::TreeNodeChild))?
        {
            items.push(item);
        }

        Ok(items)
    }
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
    #[napi(ts_arg_type = "cst.NonTerminalNode")] node: &NonTerminalNode,
) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonTerminalKind::SeparatedIdentifiers => selector.separated_identifiers()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn separated_identifiers(&mut self) -> Result<Vec<Vec<Either<NonTerminalNode, TerminalNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Period))?
            {
                separators.push(separator);

                separated.push(
                    self.select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
                );
            }
        }

        Ok(vec![separated, separators])
    }
}

//
// Common:
//

struct Selector {
    node: Rc<RustNonTerminalNode>,
    index: usize,
}

impl Selector {
    fn new(node: &NonTerminalNode) -> Self {
        Self {
            node: Rc::clone(&node.0),
            index: 0,
        }
    }

    fn select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Either<NonTerminalNode, TerminalNode>> {
        match self.try_select(filter)? {
            Some(node) => Ok(node),
            None => Error::MissingChild(self.index).into(),
        }
    }

    fn try_select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Option<Either<NonTerminalNode, TerminalNode>>> {
        while let Some(child) = self.node.children.get(self.index) {
            match child {
                node if node.is_trivia() => {
                    // skip trivia, since it's not part of the AST
                    self.index += 1;
                    continue;
                }
                RustEdge {
                    node: RustNode::Terminal(terminal),
                    ..
                } if matches!(terminal.kind, TerminalKind::SKIPPED) => {
                    return Error::SkippedTerminal(self.index).into();
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
    #[error("Unexpected parent node with NonTerminalKind '{0}'.")]
    UnexpectedParent(NonTerminalKind),

    // Should not theoretically happen, since we're only called from our own generated AST types.
    #[error("Unexpected trailing children at index '{0}'.")]
    UnexpectedTrailing(usize),

    // Should not theoretically happen, unless AST error recovery was changed.
    #[error("Missing child node at index '{0}'.")]
    MissingChild(usize),

    // Can happen if the user decided to use an incorrect/incomplete CST node.
    #[error("Unexpected SKIPPED terminal at index '{0}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet.")]
    SkippedTerminal(usize),
}

impl<T> From<Error> for Result<T> {
    fn from(error: Error) -> Self {
        Err(napi::Error::from_reason(error.to_string()))
    }
}
