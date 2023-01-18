use std::collections::HashMap;

use semver::Version;
use serde::{Deserialize, Serialize};

use crate::types::productions::ProductionRef;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Grammar {
    pub title: String,
    pub sections: Vec<GrammarSection>,
    pub versions: Vec<Version>,
    pub productions: HashMap<String, ProductionRef>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GrammarSection {
    pub title: String,
    pub topics: Vec<GrammarTopic>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GrammarTopic {
    pub title: String,
    pub productions: Vec<ProductionRef>,
}
