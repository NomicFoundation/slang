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
    #[allow(dead_code)]
    pub fn name(&self) -> &String {
        match self {
            Self::Scanner { name, .. }
            | Self::TriviaParser { name, .. }
            | Self::Parser { name, .. }
            | Self::PrecedenceParser { name, .. } => name,
        }
    }

    #[allow(dead_code)]
    pub fn versions(&self) -> Option<Vec<Version>> {
        match self {
            Production::Scanner { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().cloned().collect()),
            },
            Production::TriviaParser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().cloned().collect()),
            },
            Production::Parser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().cloned().collect()),
            },
            Production::PrecedenceParser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().cloned().collect()),
            },
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum VersionMap<T> {
    Unversioned(Rc<T>),
    Versioned(IndexMap<Version, Rc<T>>),
}

impl<T> VersionMap<T> {
    #[allow(dead_code)]
    pub fn get_for_version(&self, version: &Version) -> Option<Rc<T>> {
        match self {
            VersionMap::Unversioned(t) => Some(t.clone()),
            VersionMap::Versioned(versions) => versions
                .iter()
                .filter(|(v, _)| *v <= version)
                .last()
                .map(|(_, t)| t.clone()),
        }
    }
}
