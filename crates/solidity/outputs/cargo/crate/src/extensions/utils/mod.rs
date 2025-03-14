pub mod version;

#[cfg(test)]
pub mod tests;

use semver::Version;

use crate::cst::NonterminalKind;
use crate::utils::LanguageFacts;

/// Parse the version pragmas in the given Solidity source code and return a list of language
/// versions that can fulfill those requirements.
pub fn infer_language_versions(src: &str) -> Vec<Version> {
    let parser = crate::parser::Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
    let output = parser.parse_file_contents(src);

    let mut cursor = output.create_tree_cursor();

    let mut found_ranges = Vec::<version::Range>::new();
    while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionExpressionSets) {
        if let Ok(range) = version::parse_range(cursor.spawn()) {
            found_ranges.push(range);
        }
    }

    if found_ranges.is_empty() {
        return LanguageFacts::ALL_VERSIONS.into();
    }

    let mut matching_versions = vec![];
    for lang_version in LanguageFacts::ALL_VERSIONS {
        if found_ranges.iter().all(|r| r.matches(lang_version)) {
            matching_versions.push(lang_version.clone());
        }
    }

    matching_versions
}
