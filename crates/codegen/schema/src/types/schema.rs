use std::path::PathBuf;

use indexmap::IndexMap;
use semver::Version;
use serde::{Deserialize, Serialize};

use super::production::ProductionRef;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    pub title: String,
    pub sections: Vec<SchemaSection>,
    pub versions: Vec<Version>,

    pub schema_dir: PathBuf,
    pub productions: IndexMap<String, ProductionRef>,
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
    pub productions: IndexMap<String, ProductionRef>,
}

impl SchemaTopic {
    // TODO(OmarTawfik): This method is definetely used.
    // Need to isolate and report the bug to the rustc team.
    #[allow(dead_code)]
    pub fn productions_file() -> String {
        return "productions.yml".to_owned();
    }

    // TODO(OmarTawfik): This method is definetely used.
    // Need to isolate and report the bug to the rustc team.
    #[allow(dead_code)]
    pub fn notes_file() -> String {
        return "notes.md".to_owned();
    }
}
