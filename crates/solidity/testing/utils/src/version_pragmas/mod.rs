#[cfg(test)]
mod tests;

use std::str::FromStr;

use anyhow::{bail, ensure, Context, Result};
use semver::{Comparator, Op, Version};
use slang_solidity::{
    cst::Node,
    kinds::{ProductionKind, RuleKind, TokenKind},
    language::Language,
};

use crate::node_extensions::NodeExtensions;

pub fn extract_version_pragmas(
    source: &str,
    latest_version: &Version,
) -> Result<Vec<VersionPragma>> {
    let language = &Language::new(latest_version.to_owned())?;
    let output = language.parse(ProductionKind::SourceUnit, source);

    let mut cursor = output.parse_tree().cursor();
    let mut pragmas = vec![];
    while let Some(rule_node) = cursor.find_rule_with_kind(&[RuleKind::VersionPragmaExpression]) {
        pragmas.push(
            extract_pragma(&Node::Rule(rule_node.clone())).with_context(|| {
                format!(
                    "Failed to extract pragma at {range:?}: '{value}'",
                    range = cursor.text_range(),
                    value = rule_node.extract_non_trivia()
                )
            })?,
        );
        cursor.go_to_next_non_descendent();
    }

    return Ok(pragmas);
}

fn extract_pragma(expression_node: &Node) -> Result<VersionPragma> {
    let expression_rule = match expression_node {
        Node::Rule(rule) => rule,
        _ => bail!("Expected rule: {expression_node:?}"),
    };

    ensure!(
        expression_rule.kind == RuleKind::VersionPragmaExpression,
        "Expected VersionPragmaExpression: {expression_rule:?}",
    );

    let inner_expression = match &expression_rule.children[..] {
        [child] => match child {
            Node::Rule(rule) => rule,
            _ => bail!("Expected rule: {child:?}"),
        },
        _ => unreachable!("Expected single child: {expression_rule:?}"),
    };

    let inner_children: Vec<_> = inner_expression
        .children
        .iter()
        .filter(|child| !child.is_trivia())
        .collect();

    match inner_expression.kind {
        RuleKind::VersionPragmaBinaryExpression => match &inner_children[..] {
            [left, operator, right] => {
                let operator_kind = match operator {
                    Node::Token(token) => token.kind,
                    _ => bail!("Expected rule: {operator:?}"),
                };

                match operator_kind {
                    TokenKind::BarBar => {
                        let left = extract_pragma(left)?;
                        let right = extract_pragma(right)?;

                        return Ok(VersionPragma::or(left, right));
                    }
                    TokenKind::Minus => {
                        let mut left = extract_pragma(left)?.comparator()?;
                        let mut right = extract_pragma(right)?.comparator()?;

                        // Simulate solc bug:
                        // https://github.com/ethereum/solidity/issues/13920
                        left.op = Op::GreaterEq;
                        right.op = Op::LessEq;

                        return Ok(VersionPragma::and(
                            VersionPragma::single(left),
                            VersionPragma::single(right),
                        ));
                    }

                    _ => bail!("Unexpected operator: {operator:?}"),
                };
            }

            _ => bail!("Expected 3 children: {inner_expression:?}"),
        },
        RuleKind::VersionPragmaUnaryExpression => {
            let value = inner_expression.extract_non_trivia();
            let comparator = Comparator::from_str(&value)?;

            return Ok(VersionPragma::single(comparator));
        }
        RuleKind::VersionPragmaSpecifier => {
            let specifier = inner_expression.extract_non_trivia();
            let comparator = Comparator::from_str(&format!("={specifier}"))?;

            return Ok(VersionPragma::single(comparator));
        }
        _ => bail!("Unexpected inner expression: {inner_expression:?}"),
    };
}

#[derive(Debug)]
pub enum VersionPragma {
    Or(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Single(Comparator),
}

impl VersionPragma {
    pub fn or(left: Self, right: Self) -> Self {
        return Self::Or(Box::new(left), Box::new(right));
    }

    pub fn and(left: Self, right: Self) -> Self {
        return Self::And(Box::new(left), Box::new(right));
    }

    pub fn single(comparator: Comparator) -> Self {
        return Self::Single(comparator);
    }

    pub fn matches(&self, version: &Version) -> bool {
        match self {
            Self::Or(left, right) => {
                return left.matches(version) || right.matches(version);
            }
            Self::And(left, right) => {
                return left.matches(version) && right.matches(version);
            }
            Self::Single(comparator) => {
                return comparator.matches(version);
            }
        };
    }

    fn comparator(self) -> Result<Comparator> {
        match self {
            Self::Single(comparator) => return Ok(comparator),
            _ => bail!("Expected Single Comparator: {self:?}"),
        };
    }
}
