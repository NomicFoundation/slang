// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{NAPINodeExtensions, NonterminalNode, TerminalNode};
use crate::napi_interface::{
    NonterminalKind, RustEdge, RustNode, RustNonterminalNode, TerminalKind,
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonterminalKind::SourceUnit => selector.source_unit()?,
        NonterminalKind::Tree => selector.tree()?,
        NonterminalKind::TreeNode => selector.tree_node()?,
        NonterminalKind::AdditionExpression => selector.addition_expression()?,
        NonterminalKind::NegationExpression => selector.negation_expression()?,
        NonterminalKind::MemberAccessExpression => selector.member_access_expression()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}
impl Selector {
    fn source_unit(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::SourceUnitMembers)
        })?)])
    }
}

impl Selector {
    fn tree(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::TreeKeyword))?),
            self.try_select(|node| node.is_terminal_with_kind(TerminalKind::Identifier))?,
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::TreeNode))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn tree_node(&mut self) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::OpenBracket))?),
            Some(
                self.select(|node| {
                    node.is_nonterminal_with_kind(NonterminalKind::TreeNodeChildren)
                })?,
            ),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn addition_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Plus))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn negation_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_terminal_with_kind(TerminalKind::Bang))?),
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
        ])
    }
}

impl Selector {
    fn member_access_expression(
        &mut self,
    ) -> Result<Vec<Option<Either<NonterminalNode, TerminalNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_nonterminal_with_kind(NonterminalKind::Expression))?),
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Either<NonterminalNode, TerminalNode>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonterminalKind::SourceUnitMember => selector.source_unit_member()?,
        NonterminalKind::TreeNodeChild => selector.tree_node_child()?,
        NonterminalKind::Expression => selector.expression()?,
        NonterminalKind::Literal => selector.literal()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_member(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::Tree,
                NonterminalKind::Expression,
                NonterminalKind::SeparatedIdentifiers,
                NonterminalKind::Literal,
            ])
        })
    }
}

impl Selector {
    fn tree_node_child(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kind(NonterminalKind::TreeNode)
                || node.is_terminal_with_kind(TerminalKind::DelimitedIdentifier)
        })
    }
}

impl Selector {
    fn expression(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
        self.select(|node| {
            node.is_nonterminal_with_kinds(&[
                NonterminalKind::AdditionExpression,
                NonterminalKind::NegationExpression,
                NonterminalKind::MemberAccessExpression,
            ]) || node
                .is_terminal_with_kinds(&[TerminalKind::StringLiteral, TerminalKind::Identifier])
        })
    }
}

impl Selector {
    fn literal(&mut self) -> Result<Either<NonterminalNode, TerminalNode>> {
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonterminalKind::SourceUnitMembers => selector.source_unit_members()?,
        NonterminalKind::TreeNodeChildren => selector.tree_node_children()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_members(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) = self
            .try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::SourceUnitMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn tree_node_children(&mut self) -> Result<Vec<Either<NonterminalNode, TerminalNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_nonterminal_with_kind(NonterminalKind::TreeNodeChild))?
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
    #[napi(ts_arg_type = "cst.NonterminalNode")] node: &NonterminalNode,
) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        NonterminalKind::SeparatedIdentifiers => selector.separated_identifiers()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn separated_identifiers(&mut self) -> Result<Vec<Vec<Either<NonterminalNode, TerminalNode>>>> {
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

    fn select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Either<NonterminalNode, TerminalNode>> {
        match self.try_select(filter)? {
            Some(node) => Ok(node),
            None => Error::MissingChild(self.index).into(),
        }
    }

    fn try_select(
        &mut self,
        filter: impl FnOnce(&RustNode) -> bool,
    ) -> Result<Option<Either<NonterminalNode, TerminalNode>>> {
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
    #[error("Unexpected parent node with NonterminalKind '{0}'.")]
    UnexpectedParent(NonterminalKind),

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
