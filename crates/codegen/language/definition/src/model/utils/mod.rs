use crate::model::{self, Item};
use semver::Version;
use std::collections::BTreeSet;

mod identifier;
mod version_specifier;

pub use identifier::*;
pub use version_specifier::*;

/// Collects all versions that changes the language grammar in a breaking way.
pub fn collect_breaking_versions(lang: &model::Language) -> BTreeSet<Version> {
    let first = lang.versions.first().cloned().unwrap();
    let mut res = BTreeSet::from_iter([first]);

    let mut add_spec = |spec: &Option<model::VersionSpecifier>| {
        let Some(spec) = spec else {
            return;
        };

        match spec.clone() {
            model::VersionSpecifier::Never => (),
            model::VersionSpecifier::From { from } => {
                res.insert(from);
            }
            model::VersionSpecifier::Till { till } => {
                res.insert(till);
            }
            model::VersionSpecifier::Range { from, till } => {
                res.insert(from);
                res.insert(till);
            }
        }
    };

    for item in lang
        .sections
        .iter()
        .flat_map(|s| &s.topics)
        .flat_map(|t| &t.items)
    {
        match item.as_ref() {
            Item::Struct { item } => {
                add_spec(&item.enabled);
                for field in item.fields.values() {
                    match field {
                        model::Field::Required { .. } => (),
                        model::Field::Optional { enabled, .. } => add_spec(enabled),
                    }
                }
            }
            Item::Enum { item } => {
                add_spec(&item.enabled);
                for variant in &item.variants {
                    add_spec(&variant.enabled);
                }
            }
            Item::Repeated { item } => add_spec(&item.enabled),
            Item::Separated { item } => add_spec(&item.enabled),
            Item::Precedence { item } => add_spec(&item.enabled),
            Item::Keyword { item } => {
                for definition in &item.definitions {
                    add_spec(&definition.enabled);
                    add_spec(&definition.reserved);
                }
            }
            Item::Token { item } => {
                for definition in &item.definitions {
                    add_spec(&definition.enabled);
                }
            }
            Item::Fragment { item } => add_spec(&item.enabled),
            Item::Trivia { .. } => {}
        }
    }

    res
}
