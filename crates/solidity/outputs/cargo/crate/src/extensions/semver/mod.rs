pub mod parser;

use metaslang_cst::text_index::TextIndex;
use parser::Version;

use crate::cst::NonterminalKind;
use crate::generated::utils::LanguageFacts;

pub fn infer_language_version(src: &str) -> Vec<semver::Version> {
    let parser = crate::parser::Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
    let output = parser.parse_nonterminal(NonterminalKind::SourceUnit, src);

    let mut cursor = output.tree.create_cursor(TextIndex::ZERO);

    let mut found_ranges = Vec::<parser::Range>::new();
    while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::VersionPragma) {
        let pragma_text = cursor.node().unparse();
        let pragma_text = pragma_text.strip_prefix("solidity").unwrap_or(&pragma_text).trim();

        if let Ok(r) = parser::parse(pragma_text) {
            found_ranges.push(r);
        }
    }

    let mut matching_versions = vec![];
    for lang_version in LanguageFacts::ALL_VERSIONS {
        let v: Version = lang_version.into();
        if found_ranges.iter().all(|r| r.matches(&v)) {
            matching_versions.push(lang_version.clone());
        }
    }

    matching_versions
}
