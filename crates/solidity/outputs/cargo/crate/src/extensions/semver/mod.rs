pub mod parser;

use parser::Version;

use crate::generated::utils::LanguageFacts;

pub fn infer_language_version(src: &str) -> Vec<semver::Version> {
    let mut found_ranges = Vec::<parser::Range>::new();
    for line in src.split('\n') {
        if let Some(remaining) = line.strip_prefix("pragma solidity") {
            let version_string = remaining.strip_suffix(";").unwrap_or(remaining).trim();

            if let Ok(r) = parser::parse(version_string) {
                found_ranges.push(r);
            }
        }
    }

    let mut matching_versions = vec![];

    for lang_version in LanguageFacts::ALL_VERSIONS {
        let v: Version = lang_version.into();
        if found_ranges.iter().all(|r| r.matches(&v)) {
            matching_versions.push(lang_version.clone());
        }
    }

    matching_versions.sort();

    matching_versions
}
