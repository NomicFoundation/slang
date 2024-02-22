// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

use napi::{Env, JsObject};
use napi_derive::napi;

use crate::napi_interface::cst::{RuleNode, ToJS};
use crate::napi_interface::{RuleKind, RustNamedNode, RustNode, RustRuleNode, TokenKind};

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
    env: Env,
) -> Result<Vec<Option<JsObject>>> {
    let mut selector = Selector::new(node, env);

    let result = match node.kind() {
        RuleKind::SourceUnit => selector.source_unit()?,
        RuleKind::Tree => selector.tree()?,
        RuleKind::TreeNode => selector.tree_node()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![Some(self.select(|node| {
            node.is_rule_with_kind(RuleKind::SourceUnitMembers)
        })?)])
    }
}

impl Selector {
    fn tree(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::TreeKeyword))?),
            self.try_select(|node| node.is_token_with_kind(TokenKind::Identifier))?,
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TreeNode))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::Semicolon))?),
        ])
    }
}

impl Selector {
    fn tree_node(&mut self) -> Result<Vec<Option<JsObject>>> {
        Ok(vec![
            Some(self.select(|node| node.is_token_with_kind(TokenKind::OpenBracket))?),
            Some(self.select(|node| node.is_rule_with_kind(RuleKind::TreeNodeChildren))?),
            Some(self.select(|node| node.is_token_with_kind(TokenKind::CloseBracket))?),
        ])
    }
}

//
// Choices:
//

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn select_choice(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    env: Env,
) -> Result<JsObject> {
    let mut selector = Selector::new(node, env);

    let result = match node.kind() {
        RuleKind::SourceUnitMember => selector.source_unit_member()?,
        RuleKind::TreeNodeChild => selector.tree_node_child()?,
        RuleKind::Literal => selector.literal()?,
        _ => {
            return Error::UnexpectedParent(node.kind()).into();
        }
    };

    selector.finalize()?;
    Ok(result)
}

impl Selector {
    fn source_unit_member(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kinds(&[
                RuleKind::Tree,
                RuleKind::SeparatedIdentifiers,
                RuleKind::Literal,
            ])
        })
    }
}

impl Selector {
    fn tree_node_child(&mut self) -> Result<JsObject> {
        self.select(|node| {
            node.is_rule_with_kind(RuleKind::TreeNode)
                || node.is_token_with_kind(TokenKind::DelimitedIdentifier)
        })
    }
}

impl Selector {
    fn literal(&mut self) -> Result<JsObject> {
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
    env: Env,
) -> Result<Vec<JsObject>> {
    let mut selector = Selector::new(node, env);

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
    fn source_unit_members(&mut self) -> Result<Vec<JsObject>> {
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
    fn tree_node_children(&mut self) -> Result<Vec<JsObject>> {
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
    env: Env,
) -> Result<Vec<Vec<JsObject>>> {
    let mut selector = Selector::new(node, env);

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
    fn separated_identifiers(&mut self) -> Result<Vec<Vec<JsObject>>> {
        let mut separated = vec![];
        let mut separators = vec![];

        separated.push(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?);

        while let Some(separator) =
            self.try_select(|node| node.is_token_with_kind(TokenKind::Period))?
        {
            separators.push(separator);

            separated.push(self.select(|node| node.is_token_with_kind(TokenKind::Identifier))?);
        }

        Ok(vec![separated, separators])
    }
}

//
// Common:
//

struct Selector {
    env: Env,
    node: Rc<RustRuleNode>,
    index: usize,
}

impl Selector {
    fn new(node: &RuleNode, env: Env) -> Self {
        Self {
            env,
            node: node.0.clone(),
            index: 0,
        }
    }

    fn select(&mut self, filter: impl FnOnce(&RustNode) -> bool) -> Result<JsObject> {
        match self.try_select(filter)? {
            Some(node) => Ok(node),
            None => Error::MissingChild(self.index).into(),
        }
    }

    fn try_select(&mut self, filter: impl FnOnce(&RustNode) -> bool) -> Result<Option<JsObject>> {
        while let Some(child) = self.node.children.get(self.index) {
            match child {
                RustNamedNode {
                    name: _,
                    node: RustNode::Rule(rule),
                } if rule.kind.is_trivia() => {
                    // skip trivia, since it's not part of the AST
                    self.index += 1;
                    continue;
                }
                RustNamedNode {
                    name: _,
                    node: RustNode::Token(token),
                } if matches!(token.kind, TokenKind::SKIPPED) => {
                    return Error::SkippedToken(self.index).into();
                }
                node if filter(node) => {
                    self.index += 1;
                    return Ok(Some(node.to_js(&self.env)));
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
