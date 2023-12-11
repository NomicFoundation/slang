use std::path::PathBuf;
use std::rc::Rc;

use indexmap::{IndexMap, IndexSet};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::types::ProductionRef;

pub type LanguageDefinitionRef = Rc<LanguageDefinition>;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LanguageDefinition {
    pub title: String,
    pub sections: Vec<LanguageSection>,
    pub versions: Vec<Version>,

    pub root_production: String,
    pub productions: IndexMap<String, ProductionRef>,

    pub language_dir: PathBuf,
}

impl LanguageDefinition {
    pub fn required_productions(&self) -> IndexSet<&str> {
        IndexSet::from([&self.root_production, "LeadingTrivia", "TrailingTrivia"])
    }
}

impl LanguageDefinition {
    pub const MANIFEST_FILE_NAME: &'static str = "manifest.yml";
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LanguageSection {
    pub title: String,
    pub path: String,
    pub topics: Vec<LanguageTopic>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LanguageTopic {
    pub title: String,
    pub path: String,
    pub productions: Vec<ProductionRef>,
}

impl LanguageTopic {
    pub const PRODUCTIONS_FILE_NAME: &'static str = "productions.yml";

    pub const NOTES_FILE_NAME: &'static str = "notes.md";
}
