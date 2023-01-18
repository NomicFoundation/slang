use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::types::productions::ProductionRef;

pub type ManifestFile = Manifest;

pub type TopicFile = Vec<ProductionRef>;

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Manifest {
    pub title: String,
    pub root_production: String,
    pub sections: Vec<ManifestSection>,
    #[schemars(with = "Vec<String>")]
    pub versions: Vec<Version>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ManifestSection {
    pub title: String,
    pub topics: Vec<ManifestTopic>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ManifestTopic {
    pub title: String,
    pub definition: Option<String>,
}
