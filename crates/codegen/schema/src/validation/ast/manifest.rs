use semver::Version;

use crate::{ast_array, ast_optional, ast_value, types, validation::ast::Node, yaml::cst};

pub struct Manifest {
    pub title: Node<String>,
    pub root_production: Node<String>,
    pub sections: Vec<ManifestSection>,
    pub versions: Vec<Node<Version>>,
}

impl Manifest {
    pub fn new(syntax: &cst::NodeRef, value: types::manifest::Manifest) -> Self {
        return Self {
            title: ast_value!(syntax, value, title),
            root_production: ast_value!(syntax, value, root_production),
            sections: ast_array!(syntax, value, sections, ManifestSection),
            versions: ast_array!(syntax, value, versions),
        };
    }
}

pub struct ManifestSection {
    pub title: Node<String>,
    pub topics: Vec<ManifestTopic>,
}

impl ManifestSection {
    pub fn new(syntax: &cst::NodeRef, value: types::manifest::ManifestSection) -> Self {
        return Self {
            title: ast_value!(syntax, value, title),
            topics: ast_array!(syntax, value, topics, ManifestTopic),
        };
    }
}

pub struct ManifestTopic {
    pub title: Node<String>,
    pub definition: Option<Node<String>>,
}

impl ManifestTopic {
    pub fn new(syntax: &cst::NodeRef, value: types::manifest::ManifestTopic) -> Self {
        return Self {
            title: ast_value!(syntax, value, title),
            definition: ast_optional!(syntax, value, definition),
        };
    }
}
