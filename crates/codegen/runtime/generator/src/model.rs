use std::collections::BTreeSet;
use std::rc::Rc;

use codegen_language_definition::model::{BuiltInLabel, Language};
use semver::Version;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::ast::AstModel;
use crate::kinds::KindsModel;
use crate::parser::ParserModel;

#[derive(Default, Serialize)]
pub struct RuntimeModel {
    /// Defines the `Language::SUPPORTED_VERSIONS` field.
    all_versions: BTreeSet<Version>,
    parser: ParserModel,
    ast: AstModel,
    kinds: KindsModel,
    builtin_labels: Vec<String>,
}

impl RuntimeModel {
    pub fn from_language(language: &Rc<Language>) -> Self {
        Self {
            all_versions: language.versions.iter().cloned().collect(),
            ast: AstModel::create(language),
            parser: ParserModel::from_language(language),
            kinds: KindsModel::create(language),
            builtin_labels: builtin_labels(),
        }
    }

    pub fn for_stubs() -> Self {
        Self {
            builtin_labels: builtin_labels(),
            ..Default::default()
        }
    }
}

fn builtin_labels() -> Vec<String> {
    BuiltInLabel::iter()
        .map(|label| label.as_ref().to_owned())
        .collect()
}
