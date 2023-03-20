use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

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
    pub path: String,
    pub topics: Vec<ManifestTopic>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ManifestTopic {
    pub title: String,
    pub path: String,
}
