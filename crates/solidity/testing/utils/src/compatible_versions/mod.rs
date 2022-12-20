#[cfg(test)]
mod tests;

use std::rc::Rc;

use anyhow::{Context, Error, Result};
use semver::{Comparator, Version};
use solidity_rust_lib::generated::{
    cst::{self, Node, Visitable, Visitor, VisitorEntryResponse},
    kinds::RuleKind,
};

pub fn filter_compatible_versions<'a>(
    versions: &'a [Version],
    parse_tree: &Rc<cst::Node>,
    source: &str,
) -> Result<impl Iterator<Item = &'a Version>> {
    let mut collector = VersionSpecifierCollector {
        source,
        comparators: vec![],
    };

    parse_tree.visit(&mut collector)?;

    let compatible_versions = versions.iter().filter(move |version| {
        collector
            .comparators
            .iter()
            .all(|comparator| comparator.matches(version))
    });

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
        _children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        _path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse> {
        if kind != RuleKind::VersionPragmaSpecifier {
            return Ok(VisitorEntryResponse::StepIn);
        }

        let mut specifier = String::new();
        self.collect_non_trivia_parts(node, &mut specifier)?;

        let comparator = Comparator::parse(&specifier)?;
        self.comparators.push(comparator);

        return Ok(VisitorEntryResponse::StepOver);
    }
}

impl<'a> VersionSpecifierCollector<'a> {
    fn collect_non_trivia_parts(&mut self, node: &Node, specifier: &mut String) -> Result<()> {
        match node {
            Node::Token { lex_node, .. } => {
                let range = lex_node.range();
                let part = &self.source.get(range.to_owned()).context(format!(
                    "Failed to extract source part at range '{range:?}'"
                ))?;

                specifier.push_str(part);
            }
            Node::Rule { children, .. } | Node::Group { children, .. } => {
                for child in children {
                    self.collect_non_trivia_parts(child, specifier)?;
                }
            }
        };

        return Ok(());
    }
}
