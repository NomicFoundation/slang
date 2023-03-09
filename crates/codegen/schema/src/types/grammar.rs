use std::{collections::HashMap, path::PathBuf};

use indexmap::IndexMap;
use semver::Version;
use serde::{Deserialize, Serialize};

use super::production::ProductionRef;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Grammar {
    pub title: String,
    pub sections: Vec<GrammarSection>,
    pub versions: Vec<Version>,

    pub manifest_dir: PathBuf,
    pub productions: HashMap<String, ProductionRef>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GrammarSection {
    pub title: String,
    pub path: String,
    pub topics: Vec<GrammarTopic>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GrammarTopic {
    pub title: String,
    pub path: String,
    pub productions: IndexMap<String, ProductionRef>,
}

impl GrammarTopic {
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
