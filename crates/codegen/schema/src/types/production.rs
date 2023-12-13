use std::rc::Rc;

use indexmap::IndexMap;
use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::types::parser::Parser;
use crate::types::precedence_parser::PrecedenceParser;
use crate::types::scanner::Scanner;

pub type ProductionRef = Rc<Production>;

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
pub struct Production {
    pub name: String,

    #[serde(default)]
    pub inlined: bool,

    #[serde(flatten)]
    pub definition: ProductionDefinition,
}

impl Production {
    pub fn versions(&self) -> Option<Vec<&Version>> {
        match &self.definition {
            ProductionDefinition::Scanner { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            ProductionDefinition::TriviaParser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            ProductionDefinition::Parser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            ProductionDefinition::PrecedenceParser { version_map, .. } => match version_map {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(tag = "kind")]
pub enum ProductionDefinition {
    Scanner {
        #[serde(flatten)]
        version_map: VersionMap<Scanner>,
    },
    TriviaParser {
        #[serde(flatten)]
        version_map: VersionMap<Parser>,
    },
    Parser {
        #[serde(flatten)]
        version_map: VersionMap<Parser>,
    },
    PrecedenceParser {
        #[serde(flatten)]
        version_map: VersionMap<PrecedenceParser>,
    },
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
