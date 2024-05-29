use std::collections::BTreeSet;
use std::rc::Rc;

use codegen_language_definition::model::Language;
use semver::Version;
use serde::Serialize;

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
}

impl RuntimeModel {
    pub fn from_language(language: &Rc<Language>) -> Self {
        Self {
            all_versions: language.versions.iter().cloned().collect(),
            ast: AstModel::create(language),
            parser: ParserModel::from_language(language),
            kinds: KindsModel::create(language),
        }
    }
}
