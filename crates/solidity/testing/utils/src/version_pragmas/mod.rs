#[cfg(test)]
mod tests;

use std::str::FromStr;

use anyhow::{bail, Context, Result};
use semver::{Comparator, Op, Version};
use slang_solidity::cst::Node;
use slang_solidity::kinds::{RuleKind, TokenKind};
use slang_solidity::language::Language;

use crate::node_extensions::NodeExtensions;

pub fn extract_version_pragmas(
    source: &str,
    latest_version: &Version,
) -> Result<Vec<VersionPragma>> {
    let language = &Language::new(latest_version.to_owned())?;
    let output = language.parse(RuleKind::SourceUnit, source);

    let mut pragmas = vec![];
    let mut cursor = output.create_tree_cursor();

    while !cursor.is_completed() {
        let node = &cursor.node();

        if matches!(node, Node::Rule(rule) if rule.kind == RuleKind::VersionPragmaExpression) {
            pragmas.push(extract_pragma(node).with_context(|| {
                format!(
                    "Failed to extract pragma at {range:?}: '{value}'",
                    range = cursor.text_range(),
                    value = node.extract_non_trivia()
                )
            })?);

            // Skip other VersionPragmaExpression nodes under the same node:
            cursor.go_to_next_non_descendent();
        } else {
            cursor.go_to_next_rule();
        }
    }

    Ok(pragmas)
}

fn extract_pragma(node: &Node) -> Result<VersionPragma> {
    match node {
        Node::Rule(rule) => {
            let children = rule
                .children
                .iter()
                .filter(|child| !child.is_trivia())
                .collect::<Vec<_>>();

            match (rule.kind, &children[..]) {
                (RuleKind::VersionPragmaExpression, [inner]) => extract_pragma(inner),

                (RuleKind::VersionPragmaOrExpression, [left, op, right])
                    if op.is_token_with_kind(TokenKind::BarBar) =>
                {
                    let left = extract_pragma(left)?;
                    let right = extract_pragma(right)?;

                    Ok(VersionPragma::or(left, right))
                }

                (RuleKind::VersionPragmaRangeExpression, [left, op, right])
                    if op.is_token_with_kind(TokenKind::Minus) =>
                {
                    let mut left = extract_pragma(left)?.into_comparator()?;
                    let mut right = extract_pragma(right)?.into_comparator()?;

                    // Simulate solc bug:
                    // https://github.com/ethereum/solidity/issues/13920
                    left.op = Op::GreaterEq;
                    right.op = Op::LessEq;

                    Ok(VersionPragma::and(
                        VersionPragma::single(left),
                        VersionPragma::single(right),
                    ))
                }

                (RuleKind::VersionPragmaPrefixExpression, _) => {
                    let value = rule.extract_non_trivia();
                    let comparator = Comparator::from_str(&value)?;

                    Ok(VersionPragma::single(comparator))
                }

                (RuleKind::VersionPragmaSpecifier, _) => {
                    let value = rule.extract_non_trivia();
                    let comparator = Comparator::from_str(&format!("={value}"))?;

                    Ok(VersionPragma::single(comparator))
                }

                (kind, children) => {
                    bail!("Unexpected rule kind '{kind}' and children: {children:#?}")
                }
            }
        }

        Node::Token(token) => match token.kind {
            TokenKind::AsciiStringLiteral => {
                let value = token.text.trim_matches('"').trim_matches('\'');
                let comparator = Comparator::from_str(&format!("={value}"))?;

                Ok(VersionPragma::single(comparator))
            }

            _ => bail!("Unexpected token: {token:#?}"),
        },
    }
}

#[derive(Debug)]
pub enum VersionPragma {
    Or(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Single(Comparator),
}

impl VersionPragma {
    pub fn or(left: Self, right: Self) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }

    pub fn and(left: Self, right: Self) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }

    pub fn single(comparator: Comparator) -> Self {
        Self::Single(comparator)
    }

    pub fn matches(&self, version: &Version) -> bool {
        match self {
            Self::Or(left, right) => left.matches(version) || right.matches(version),
            Self::And(left, right) => left.matches(version) && right.matches(version),
            Self::Single(comparator) => comparator.matches(version),
        }
    }

    fn into_comparator(self) -> Result<Comparator> {
        match self {
            Self::Single(comparator) => Ok(comparator),
            _ => bail!("Expected Single Comparator: {self:?}"),
        }
    }
}
