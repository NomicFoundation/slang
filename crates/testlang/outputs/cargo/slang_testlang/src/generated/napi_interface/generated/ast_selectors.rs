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
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        RuleKind::SourceUnit => selector.source_unit()?,
        RuleKind::Tree => selector.tree()?,
        RuleKind::TreeNode => selector.tree_node()?,
        RuleKind::AdditionExpression => selector.addition_expression()?,
        RuleKind::NegationExpression => selector.negation_expression()?,
        RuleKind::MemberAccessExpression => selector.member_access_expression()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}
impl Selector {
    fn source_unit(&mut self) -> Result<Vec<Option<Either<RuleNode, TokenNode>>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_rule_with_kind(RuleKind::SourceUnitMembers)
        })?)])
    }
}

impl Selector {
    fn tree(&mut self) -> Result<Vec<Option<Either<RuleNode, TokenNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::TreeKeyword))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TreeNode))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn tree_node(&mut self) -> Result<Vec<Option<Either<RuleNode, TokenNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBracket))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TreeNodeChildren))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBracket))?),
        ])
    }
}

impl Selector {
    fn addition_expression(&mut self) -> Result<Vec<Option<Either<RuleNode, TokenNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Plus))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn negation_expression(&mut self) -> Result<Vec<Option<Either<RuleNode, TokenNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Bang))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
        ])
    }
}

impl Selector {
    fn member_access_expression(&mut self) -> Result<Vec<Option<Either<RuleNode, TokenNode>>>> {
        Ok(vec![
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::Expression))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Period))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?),
        ])
    }
}
//
// Choices:
//

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
) -> Result<Either<RuleNode, TokenNode>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        RuleKind::SourceUnitMember => selector.source_unit_member()?,
        RuleKind::TreeNodeChild => selector.tree_node_child()?,
        RuleKind::Expression => selector.expression()?,
        RuleKind::Literal => selector.literal()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_member(&mut self) -> Result<Either<RuleNode, TokenNode>> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::Tree,
                RuleKind::Expression,
                RuleKind::SeparatedIdentifiers,
                RuleKind::Literal,
            ])
        })
    }
}

impl Selector {
    fn tree_node_child(&mut self) -> Result<Either<RuleNode, TokenNode>> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::TreeNode)
                || node.is_token_with_kind(TokenKind::DelimitedIdentifier)
        })
    }
}

impl Selector {
    fn expression(&mut self) -> Result<Either<RuleNode, TokenNode>> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::AdditionExpression,
                RuleKind::NegationExpression,
                RuleKind::MemberAccessExpression,
            ]) || node.is_token_with_kinds(&[TokenKind::StringLiteral, TokenKind::Identifier])
        })
    }
}

impl Selector {
    fn literal(&mut self) -> Result<Either<RuleNode, TokenNode>> {
        self.select(|node| node.is_token_with_kind(TokenKind::StringLiteral))
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
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
) -> Result<Vec<Either<RuleNode, TokenNode>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        RuleKind::SourceUnitMembers => selector.source_unit_members()?,
        RuleKind::TreeNodeChildren => selector.tree_node_children()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_members(&mut self) -> Result<Vec<Either<RuleNode, TokenNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::SourceUnitMember))?
        {
            items.push(item);
        }

        Ok(items)
    }
}

impl Selector {
    fn tree_node_children(&mut self) -> Result<Vec<Either<RuleNode, TokenNode>>> {
        let mut items = vec![];

        while let Some(item) =
            self.try_select(|node| node.is_rule_with_kind(RuleKind::TreeNodeChild))?
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
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
) -> Result<Vec<Vec<Either<RuleNode, TokenNode>>>> {
    let mut selector = Selector::new(node);

    let result = match node.kind() {
        RuleKind::SeparatedIdentifiers => selector.separated_identifiers()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn separated_identifiers(&mut self) -> Result<Vec<Vec<Either<RuleNode, TokenNode>>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        if let Some(first) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?
        {
            separated.push(first);

            while let Some(separator) =
                self.try_select(|node| node.is_token_with_kind(TokenKind::Period))?
            {
                separators.push(separator);

                separated.push(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?);
            }
        }

        Ok(vec![separated, separators])
    }
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
