#[cfg(test)]
mod tests;

use std::{rc::Rc, str::FromStr};

use anyhow::{bail, ensure, Context, Error, Result};
use semver::{Comparator, Op, Version};
use slang_solidity::{
    language::Language,
    syntax::{
        nodes::{Node, RuleKind, RuleNode, TextRange},
        parser::ProductionKind,
        visitors::{Visitable, Visitor, VisitorEntryResponse},
    },
};

use crate::node_extensions::NodeExtensions;

pub fn extract_version_pragmas(
    source: &str,
    latest_version: &Version,
) -> Result<Vec<VersionPragma>> {
    let output =
        Language::new(latest_version.to_owned())?.parse(ProductionKind::SourceUnit, source)?;
    let parse_tree = output.parse_tree();
    let mut collector = PragmaCollector { pragmas: vec![] };
    parse_tree.accept_visitor(&mut collector)?;
    return Ok(collector.pragmas);
}

struct PragmaCollector {
    pragmas: Vec<VersionPragma>,
}

impl Visitor<Error> for PragmaCollector {
    fn enter_rule(
        &mut self,
        node: &Rc<RuleNode>,
        _path: &Vec<Rc<RuleNode>>,
        range: &TextRange,
    ) -> Result<VisitorEntryResponse> {
        if node.kind != RuleKind::VersionPragmaExpressionList {
            return Ok(VisitorEntryResponse::StepIn);
        }

        for child in &node.children {
            let pragma = self.extract_pragma(child).with_context(|| {
                format!(
                    "Failed to extract pragma at {range:?}: '{value}'",
                    value = child.extract_non_trivia()
                )
            });

            self.pragmas.push(pragma?);
        }

        return Ok(VisitorEntryResponse::StepOver);
    }
}

impl PragmaCollector {
    fn extract_pragma(&self, node: &Node) -> Result<VersionPragma> {
        let (kind, children) = match node {
            Node::Rule(rule_node) => (rule_node.kind, rule_node.children.clone()),
            _ => bail!("Expected rule: {node:?}"),
        };

        let children: Vec<_> = children
            .into_iter()
            .filter(|child| !child.is_trivia())
            .collect();

        match kind {
            RuleKind::VersionPragmaAlternatives => match &children[..] {
                [left, operator, right] => {
                    ensure!(operator.extract_non_trivia() == "||");

                    let left = self.extract_pragma(left)?;
                    let right = self.extract_pragma(right)?;

                    return Ok(VersionPragma::or(left, right));
                }
                _ => bail!("Expected 3 children: {node:?}"),
            },
            RuleKind::VersionPragmaRange => match &children[..] {
                [start, operator, end] => {
                    ensure!(operator.extract_non_trivia() == "-");

                    let mut start = self.extract_pragma(start)?.comparator()?;
                    let mut end = self.extract_pragma(end)?.comparator()?;

                    // Simulate solc bug:
                    // https://github.com/ethereum/solidity/issues/13920
                    start.op = Op::GreaterEq;
                    end.op = Op::LessEq;

                    return Ok(VersionPragma::and(
                        VersionPragma::single(start),
                        VersionPragma::single(end),
                    ));
                }
                _ => bail!("Expected 3 children: {node:?}"),
            },
            RuleKind::VersionPragmaComparator => {
                let value = node.extract_non_trivia();
                let comparator = Comparator::from_str(&value)?;

                return Ok(VersionPragma::single(comparator));
            }
            RuleKind::VersionPragmaSpecifier => {
                let specifier = node.extract_non_trivia();
                let comparator = Comparator::from_str(&format!("={specifier}"))?;

                return Ok(VersionPragma::single(comparator));
            }
            _ => bail!("Unexpected {kind:?}: {children:?}"),
        };
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
