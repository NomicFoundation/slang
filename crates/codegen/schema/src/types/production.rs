use std::rc::Rc;

use indexmap::IndexMap;
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

use super::{parser::Parser, precedence_parser::PrecedenceParser, scanner::Scanner};

pub type ProductionRef = Rc<Production>;

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(tag = "kind")]
pub enum Production {
    Scanner {
        name: String,
        #[serde(flatten)]
        version_map: VersionMap<Scanner>,
    },
    TriviaParser {
        name: String,
        #[serde(flatten)]
        version_map: VersionMap<Parser>,
    },
    Parser {
        name: String,
        #[serde(flatten)]
        version_map: VersionMap<Parser>,
    },
    PrecedenceParser {
        name: String,
        #[serde(flatten)]
        version_map: VersionMap<PrecedenceParser>,
    },
}

impl Production {
    pub fn name(&self) -> &String {
        match self {
            Self::Scanner { name, .. }
            | Self::TriviaParser { name, .. }
            | Self::Parser { name, .. }
            | Self::PrecedenceParser { name, .. } => name,
        }
    }

    pub fn versions(&self) -> Option<Vec<&Version>> {
        match self {
            Production::Scanner { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            Production::TriviaParser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            Production::Parser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            Production::PrecedenceParser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum VersionMap<T> {
    Unversioned(Rc<T>),
    Versioned(IndexMap<Version, Option<Rc<T>>>),
}

impl<T> VersionMap<T> {
    pub fn get_for_version(&self, version: &Version) -> Option<Rc<T>> {
        match self {
            VersionMap::Unversioned(t) => Some(t.clone()),
            VersionMap::Versioned(versions) => versions
                .keys()
                .rev()
                .find(|v| *v <= version)
                .and_then(|v| versions.get(v).unwrap().clone()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Reference {
    #[schemars(title = "Production Reference")]
    pub reference: String,
}
