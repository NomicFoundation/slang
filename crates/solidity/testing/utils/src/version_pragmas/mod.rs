#[cfg(test)]
mod tests;

use std::{rc::Rc, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use semver::{Comparator, Op, Version};
use slang_solidity::{
    language::Language,
    syntax::{
        nodes::{Node, RuleKind, TextRange},
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

    let parse_tree = if let Some(parse_tree) = output.parse_tree() {
        parse_tree
    } else {
        bail!("Failed to extract a parse tree.");
    };

    let mut collector = PragmaCollector {
        source,
        pragmas: vec![],
    };

    parse_tree.accept_visitor(&mut collector)?;

    return Ok(collector.pragmas);
}

struct PragmaCollector<'context> {
    source: &'context str,
    pragmas: Vec<VersionPragma>,
}

impl<'context> Visitor<Error> for PragmaCollector<'context> {
    fn enter_rule(
        &mut self,
        kind: RuleKind,
        range: &TextRange,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        _path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse> {
        if kind != RuleKind::VersionPragmaExpression {
            return Ok(VisitorEntryResponse::StepIn);
        }

        let pragma = match &children[..] {
            [child] => self.extract_pragma(child).with_context(|| {
                format!(
                    "Failed to extract pragma at {range:?}: '{value}'",
                    range = range.start.byte..range.end.byte,
                    value = child.extract_non_trivia(self.source)
                )
            })?,
            _ => unreachable!("Expected single child: {node:?}"),
        };

        self.pragmas.push(pragma);

        return Ok(VisitorEntryResponse::StepOver);
    }
}

impl<'context> PragmaCollector<'context> {
    fn extract_pragma(&self, node: &Node) -> Result<VersionPragma> {
        let (kind, children) = match node {
            Node::Rule { kind, children, .. } => (kind, children),
            _ => panic!("Expected rule: {node:?}"),
        };

        match kind {
            RuleKind::VersionPragmaAlternatives => match &children[..] {
                [left, operator, right] => {
                    assert_eq!(operator.extract_non_trivia(self.source), "||");

                    let left = self.extract_pragma(left)?;
                    let right = self.extract_pragma(right)?;

                    return Ok(VersionPragma::Or(Box::new(left), Box::new(right)));
                }
                _ => unreachable!("Expected 3 children: {node:?}"),
            },
            RuleKind::VersionPragmaRange => match &children[..] {
                [left, operator, right] => {
                    assert_eq!(operator.extract_non_trivia(self.source), "-");

                    let mut left = self.extract_pragma(left)?.extract_single()?;
                    let mut right = self.extract_pragma(right)?.extract_single()?;

                    // Simulate solc bug:
                    // https://github.com/ethereum/solidity/issues/13920
                    left.op = Op::GreaterEq;
                    right.op = Op::LessEq;

                    return Ok(VersionPragma::And(left, right));
                }
                _ => unreachable!("Expected 3 children: {node:?}"),
            },
            RuleKind::VersionPragmaComparator => {
                let value = node.extract_non_trivia(self.source);
                let comparator = Comparator::from_str(&value)?;

                return Ok(VersionPragma::Single(comparator));
            }
            RuleKind::VersionPragmaSpecifier => {
                let specifier = node.extract_non_trivia(self.source);
                let comparator = Comparator::from_str(&format!("={specifier}"))?;

                return Ok(VersionPragma::Single(comparator));
            }
            _ => unreachable!("Unexpected {kind:?}: {children:?}"),
        };
    }
}

#[derive(Debug)]
pub enum VersionPragma {
    Or(Box<Self>, Box<Self>),
    And(Comparator, Comparator),
    Single(Comparator),
}

impl VersionPragma {
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

    fn extract_single(self) -> Result<Comparator> {
        match self {
            Self::Single(comparator) => return Ok(comparator),
            _ => bail!("Expected Single: {self:?}"),
        };
    }
}
