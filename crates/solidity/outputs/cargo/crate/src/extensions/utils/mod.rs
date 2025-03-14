pub mod version;

#[cfg(test)]
pub mod tests;

use semver::Version;

use crate::cst::NonterminalKind;
use crate::utils::LanguageFacts;

/// Parse the version pragmas in the given Solidity source code and return a list of language
/// versions that can fulfill those requirements.
pub fn infer_language_versions(src: &str) -> impl Iterator<Item = &'static Version> {
    let parser = crate::parser::Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
    let output = parser.parse_file_contents(src);

    let mut cursor = output.create_tree_cursor();

    let mut found_ranges = Vec::<version::Range>::new();
    while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionExpressionSets) {
        if let Ok(range) = version::parse_range(cursor.spawn()) {
            found_ranges.push(range);
        }
    }

    VersionIterator::new(found_ranges)
}

struct VersionIterator {
    valid_ranges: Vec<version::Range>,
    version_index: usize,
}

impl VersionIterator {
    fn new(valid_ranges: Vec<version::Range>) -> VersionIterator {
        VersionIterator {
            valid_ranges,
            version_index: 0,
        }
    }

    fn next_version(&mut self) -> Option<&'static Version> {
        let result = if self.version_index < LanguageFacts::ALL_VERSIONS.len() {
            Some(&LanguageFacts::ALL_VERSIONS[self.version_index])
        } else {
            None
        };

        self.version_index += 1;

        result
    }

    fn is_valid(&self, version: &Version) -> bool {
        if self.valid_ranges.is_empty() {
            true
        } else {
            self.valid_ranges.iter().all(|r| r.matches(version))
        }
    }
}

impl Iterator for VersionIterator {
    type Item = &'static Version;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(version) = self.next_version() {
            if self.is_valid(version) {
                return Some(version);
            }
        }

        None
    }
}
