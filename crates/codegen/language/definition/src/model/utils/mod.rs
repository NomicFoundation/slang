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

    fn add_spec(res: &mut BTreeSet<Version>, spec: &Option<model::VersionSpecifier>) {
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
    }

    for item in lang
        .sections
        .iter()
        .flat_map(|s| &s.topics)
        .flat_map(|t| &t.items)
    {
        match item.as_ref() {
            Item::Struct { item } => {
                add_spec(&mut res, &item.enabled);
                for field in &item.fields {
                    match field.1 {
                        model::Field::Required { .. } => (),
                        model::Field::Optional { enabled, .. } => add_spec(&mut res, enabled),
                    }
                }
            }
            Item::Enum { item } => {
                add_spec(&mut res, &item.enabled);
                for variant in &item.variants {
                    add_spec(&mut res, &variant.enabled);
                }
            }
            Item::Repeated { item } => add_spec(&mut res, &item.enabled),
            Item::Separated { item } => add_spec(&mut res, &item.enabled),
            Item::Precedence { item } => add_spec(&mut res, &item.enabled),
            Item::Keyword { item } => {
                for definition in &item.definitions {
                    add_spec(&mut res, &definition.enabled);
                }
            }
            Item::Token { item } => {
                for definition in &item.definitions {
                    add_spec(&mut res, &definition.enabled);
                }
            }
            Item::Fragment { item } => add_spec(&mut res, &item.enabled),
            Item::Trivia { .. } => {}
        }
    }

    res
}
