//! Various Slang language utilities.

#[path = "language_facts.generated.rs"]
mod language_facts;

#[path = "versions.generated.rs"]
pub(crate) mod versions;

pub use language_facts::LanguageFacts;

mod version;

#[cfg(test)]
mod tests;

use semver::Version;

use crate::cst::NonterminalKind;

/// Parse the version pragmas in the given Solidity source code and return a list of language
/// versions that can fulfill those requirements.
pub(crate) fn infer_language_versions(src: &str) -> impl Iterator<Item = &'static Version> {
    let parser = crate::parser::Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
    let output = parser.parse_file_contents(src);

    let mut cursor = output.create_tree_cursor();

    let mut found_ranges = Vec::<version::Range>::new();
    while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionExpressionSets) {
        if let Ok(range) = version::parse_range(cursor.spawn()) {
            found_ranges.push(range);
        }
    }

    LanguageFacts::ALL_VERSIONS.iter().filter(move |version| {
        if found_ranges.is_empty() {
            true
        } else {
            found_ranges.iter().all(|range| range.matches(version))
        }
    })
}
