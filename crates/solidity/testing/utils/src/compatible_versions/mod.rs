#[cfg(test)]
mod tests;

use std::{collections::BTreeSet, ops::Range, rc::Rc};

use anyhow::{Context, Error, Result};
use semver::{Comparator, Version};
use slang_solidity::generated::{
    cst::{self, Node},
    cst_visitor::{Visitable, Visitor, VisitorEntryResponse},
    kinds::RuleKind,
};

pub fn filter_compatible_versions<'a>(
    versions: &'a Vec<Version>,
    parse_tree: &Rc<cst::Node>,
    source: &str,
) -> Result<BTreeSet<&'a Version>> {
    let mut collector = VersionSpecifierCollector {
        source,
        comparators: vec![],
    };

    parse_tree.visit(&mut collector)?;

    let compatible_versions = versions
        .iter()
        .filter(move |version| {
            collector
                .comparators
                .iter()
                .all(|comparator| comparator.matches(version))
        })
        .collect();

    return Ok(compatible_versions);
}

struct VersionSpecifierCollector<'a> {
    pub source: &'a str,
    pub comparators: Vec<Comparator>,
}

impl<'a> Visitor<Error> for VersionSpecifierCollector<'a> {
    fn enter_rule(
        &mut self,
        kind: RuleKind,
        _range: &Range<usize>,
        _children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        _path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse> {
        if kind != RuleKind::VersionPragmaSpecifier {
            return Ok(VisitorEntryResponse::StepIn);
        }

        let mut parts = String::new();
        self.collect_non_trivia_parts(node, &mut parts)?;

        let comparator = Comparator::parse(&parts)
            .context(format!("Failed to parse specifier: '{parts}'"))
            .context(format!("Failed to extract specifier from node: {node:?}"))?;

        self.comparators.push(comparator);

        return Ok(VisitorEntryResponse::StepOver);
    }
}

impl<'a> VersionSpecifierCollector<'a> {
    fn collect_non_trivia_parts(&mut self, node: &Node, parts: &mut String) -> Result<()> {
        match node {
            Node::Token { range, .. } => {
                let range = range;
                parts.extend(
                    self.source
                        .chars()
                        .skip(range.start)
                        .take(range.end - range.start),
                );
            }
            Node::Rule { children, .. } => {
                for child in children {
                    self.collect_non_trivia_parts(child, parts)?;
                }
            }
        };

        return Ok(());
    }
}
