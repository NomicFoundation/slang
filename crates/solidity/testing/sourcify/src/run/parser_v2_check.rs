use std::collections::HashSet;

use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{
    Cursor, EdgeLabel, NonterminalKind, NonterminalNode, TerminalKind, TextRange,
};
use slang_solidity::diagnostic::{Diagnostic, Severity};

use super::print_errors;
use crate::events::{Events, SingleTestOutcome};
use crate::sourcify::Contract;

// HACK(survey): temporarily replaces the v1/v2 parser comparison with a scan
// for conditionals whose branches are both module import aliases — the
// pattern typed by the module meta-type unification change. Sourcify holds
// only solc-verified sources, so every hit compiled, i.e. both aliases
// resolve to one source unit. Drop this commit once the survey is done.
pub(super) fn run(
    contract: &Contract,
    unit: &CompilationUnit,
    events: &Events,
) -> Option<SingleTestOutcome> {
    let mut test_outcome = SingleTestOutcome::Passed;

    for file in unit.files() {
        let hits = scan_alias_conditionals(file.create_tree_cursor());
        if !hits.is_empty() {
            print_errors(
                contract,
                events,
                file.id(),
                &hits,
                slang_solidity::diagnostic::render,
            );
            test_outcome = SingleTestOutcome::Failed;
        }
    }

    Some(test_outcome)
}

struct AliasConditionalHit(Cursor);

impl Diagnostic for AliasConditionalHit {
    fn text_range(&self) -> TextRange {
        self.0.text_range()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        format!(
            "ALIAS_CONDITIONAL_HIT `{expression}`",
            expression = self.0.node().unparse()
        )
    }
}

fn scan_alias_conditionals(cursor: Cursor) -> Vec<AliasConditionalHit> {
    let mut aliases = HashSet::new();
    let mut import_cursor = cursor.clone();
    while import_cursor.go_to_next_nonterminal_with_kinds(&[
        NonterminalKind::PathImport,
        NonterminalKind::NamedImport,
    ]) {
        // `ImportDeconstructionSymbol` aliases are excluded: they alias
        // individual symbols, not module types.
        let import = import_cursor.node().into_nonterminal().unwrap();
        for edge in &import.children {
            let Some(alias) = edge
                .node
                .as_nonterminal_with_kind(NonterminalKind::ImportAlias)
            else {
                continue;
            };
            for alias_edge in &alias.children {
                if let Some(identifier) = alias_edge
                    .node
                    .as_terminal_with_kind(TerminalKind::Identifier)
                {
                    aliases.insert(identifier.text.clone());
                }
            }
        }
    }

    if aliases.is_empty() {
        return Vec::new();
    }

    let mut hits = Vec::new();
    let mut conditional_cursor = cursor;
    while conditional_cursor
        .go_to_next_nonterminal_with_kind(NonterminalKind::ConditionalExpression)
    {
        let conditional = conditional_cursor.node().into_nonterminal().unwrap();
        let branch_is_alias = |label: EdgeLabel| {
            branch_identifier(&conditional, label)
                .is_some_and(|identifier| aliases.contains(&identifier))
        };
        if branch_is_alias(EdgeLabel::TrueExpression) && branch_is_alias(EdgeLabel::FalseExpression)
        {
            hits.push(AliasConditionalHit(conditional_cursor.clone()));
        }
    }

    hits
}

/// The text of the branch expression when it is a bare identifier.
fn branch_identifier(conditional: &NonterminalNode, label: EdgeLabel) -> Option<String> {
    let branch = conditional
        .children
        .iter()
        .find(|edge| edge.label == label)?;
    let expression = branch.node.as_nonterminal()?;
    let variant = expression
        .children
        .iter()
        .find(|edge| edge.label == EdgeLabel::Variant)?;
    Some(
        variant
            .node
            .as_terminal_with_kind(TerminalKind::Identifier)?
            .text
            .clone(),
    )
}

#[cfg(test)]
mod tests {
    use semver::Version;
    use slang_solidity::parser::Parser;

    use super::scan_alias_conditionals;

    fn scan(source: &str) -> Vec<String> {
        let parser = Parser::create(Version::new(0, 8, 0)).unwrap();
        let output = parser.parse_file_contents(source);
        assert!(output.is_valid(), "test source must parse: {source}");
        scan_alias_conditionals(output.create_tree_cursor())
            .iter()
            .map(|hit| hit.0.node().unparse())
            .collect()
    }

    #[test]
    fn detects_conditional_between_path_import_aliases() {
        let hits = scan(
            r#"
            import "./lib.sol" as A;
            import "./lib.sol" as B;
            function pick() pure returns (uint256) {
                return (true ? A : B).K;
            }
            "#,
        );
        assert_eq!(hits, ["true ? A : B"]);
    }

    #[test]
    fn detects_conditional_between_named_import_aliases() {
        let hits = scan(
            r#"
            import * as A from "./lib.sol";
            import * as B from "./lib.sol";
            function pick() pure returns (uint256) {
                return (false ? A : B).K;
            }
            "#,
        );
        assert_eq!(hits, ["false ? A : B"]);
    }

    #[test]
    fn ignores_conditional_between_plain_identifiers() {
        let hits = scan(
            r#"
            import "./lib.sol" as A;
            function pick(uint256 x, uint256 y) pure returns (uint256) {
                return true ? x : y;
            }
            "#,
        );
        assert_eq!(hits, Vec::<String>::new());
    }

    #[test]
    fn ignores_deconstruction_symbol_aliases() {
        let hits = scan(
            r#"
            import {K as A} from "./lib.sol";
            import {L as B} from "./lib.sol";
            function pick() pure returns (uint256) {
                return true ? A : B;
            }
            "#,
        );
        assert_eq!(hits, Vec::<String>::new());
    }
}
