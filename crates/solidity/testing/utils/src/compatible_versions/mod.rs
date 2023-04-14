#[cfg(test)]
mod tests;

use std::{collections::BTreeSet, ops::Range, rc::Rc, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use semver::{Comparator, Op, Version};
use slang_solidity::syntax::{
    nodes::{Node, RuleKind},
    visitors::{Visitable, Visitor, VisitorEntryResponse},
};

use crate::node_extensions::NodeExtensions;

pub fn filter_compatible_versions<'a>(
    versions: &'a Vec<Version>,
    parse_tree: &Rc<Node>,
    source: &str,
) -> Result<BTreeSet<&'a Version>> {
    let mut collector = VersionSpecifierCollector {
        source,
        expressions: vec![],
    };

    parse_tree.accept_visitor(&mut collector)?;

    let compatible_versions = versions
        .iter()
        .filter(|version| collector.matches(version))
        .collect();

    return Ok(compatible_versions);
}

struct VersionSpecifierCollector<'a> {
    source: &'a str,
    expressions: Vec<VersionPragmaExpression>,
}

impl<'a> Visitor<Error> for VersionSpecifierCollector<'a> {
    fn enter_rule(
        &mut self,
        kind: RuleKind,
        range: &Range<usize>,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        _path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse> {
        if kind != RuleKind::VersionPragmaExpression {
            return Ok(VisitorEntryResponse::StepIn);
        }

        let expression = match &children[..] {
            [child] => self.extract_expression(child).with_context(|| {
                format!(
                    "Failed to extract pragma at {range:?}: '{value}'",
                    value = child.extract_non_trivia(self.source)
                )
            })?,
            _ => unreachable!("Expected single child: {node:?}"),
        };

        self.expressions.push(expression);

        return Ok(VisitorEntryResponse::StepOver);
    }
}

impl<'a> VersionSpecifierCollector<'a> {
    fn matches(&self, version: &Version) -> bool {
        self.expressions
            .iter()
            .all(|expression| expression.matches(version))
    }

    fn extract_expression(&self, node: &Node) -> Result<VersionPragmaExpression> {
        let (kind, children) = match node {
            Node::Rule { kind, children, .. } => (kind, children),
            _ => panic!("Expected rule: {node:?}"),
        };

        match kind {
            RuleKind::VersionPragmaAlternatives => match &children[..] {
                [left, operator, right] => {
                    assert_eq!(operator.extract_non_trivia(self.source), "||");

                    let left = self.extract_expression(left)?;
                    let right = self.extract_expression(right)?;

                    return Ok(VersionPragmaExpression::Or(Box::new(left), Box::new(right)));
                }
                _ => unreachable!("Expected 3 children: {node:?}"),
            },
            RuleKind::VersionPragmaRange => match &children[..] {
                [left, operator, right] => {
                    assert_eq!(operator.extract_non_trivia(self.source), "-");

                    let mut left = self.extract_expression(left)?.extract_single()?;
                    let mut right = self.extract_expression(right)?.extract_single()?;

                    // Simulate solc bug:
                    // https://github.com/ethereum/solidity/issues/13920
                    left.op = Op::GreaterEq;
                    right.op = Op::LessEq;

                    return Ok(VersionPragmaExpression::And(left, right));
                }
                _ => unreachable!("Expected 3 children: {node:?}"),
            },
            RuleKind::VersionPragmaComparator => {
                let value = node.extract_non_trivia(self.source);
                let comparator = Comparator::from_str(&value)?;

                return Ok(VersionPragmaExpression::Single(comparator));
            }
            RuleKind::VersionPragmaSpecifier => {
                let specifier = node.extract_non_trivia(self.source);
                let comparator = Comparator::from_str(&format!("={specifier}"))?;

                return Ok(VersionPragmaExpression::Single(comparator));
            }
            _ => unreachable!("Unexpected {kind:?}: {children:?}"),
        };
    }
}

#[derive(Debug)]
enum VersionPragmaExpression {
    Or(Box<Self>, Box<Self>),
    And(Comparator, Comparator),
    Single(Comparator),
}

impl VersionPragmaExpression {
    fn matches(&self, version: &Version) -> bool {
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

    fn extract_single(self) -> Result<Comparator> {
        match self {
            Self::Single(comparator) => return Ok(comparator),
            _ => bail!("Expected Single: {self:?}"),
        };
    }
}
