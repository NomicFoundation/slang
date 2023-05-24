use std::{path::PathBuf, rc::Rc};

use indexmap::{IndexMap, IndexSet};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::types::ProductionRef;

#[allow(dead_code)]
pub type SchemaRef = Rc<Schema>;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    pub title: String,
    pub sections: Vec<SchemaSection>,
    pub versions: Vec<Version>,

    pub root_production: String,
    pub productions: IndexMap<String, ProductionRef>,

    pub schema_dir: PathBuf,
}

impl Schema {
    #[allow(dead_code)]
    pub fn required_productions(&self) -> IndexSet<&str> {
        return IndexSet::from([&self.root_production, "LeadingTrivia", "TrailingTrivia"]);
    }
}

impl Schema {
    #[allow(dead_code)]
    pub const MANIFEST_FILE_NAME: &'static str = "manifest.yml";
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SchemaSection {
    pub title: String,
    pub path: String,
    pub topics: Vec<SchemaTopic>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SchemaTopic {
    pub title: String,
    pub path: String,
    pub productions: Vec<ProductionRef>,
}

impl SchemaTopic {
    #[allow(dead_code)]
    pub const PRODUCTIONS_FILE_NAME: &'static str = "productions.yml";

    #[allow(dead_code)]
    pub const NOTES_FILE_NAME: &'static str = "notes.md";
}
