use semver::Version;

use crate::{types, validation::ast::Node, yaml::cst};

pub struct Manifest {
    pub title: Node<String>,
    pub root_production: Node<String>,
    pub sections: Vec<ManifestSection>,
    pub versions: Vec<Node<Version>>,
}

impl Manifest {
    pub fn new(syntax: &cst::NodeRef, value: types::manifest::Manifest) -> Self {
        return Self {
            title: Node::new(syntax.get("title"), value.title),
            root_production: Node::new(syntax.get("rootProduction"), value.root_production),
            sections: syntax
                .get("sections")
                .zip(value.sections, ManifestSection::new),
            versions: syntax.get("versions").zip(value.versions, Node::new),
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
            title: Node::new(syntax.get("title"), value.title),
            topics: syntax.get("topics").zip(value.topics, ManifestTopic::new),
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
            title: Node::new(syntax.get("title"), value.title),
            definition: value.definition.and_then(|definition| {
                return Some(Node::new(syntax.get("definition"), definition));
            }),
        };
    }
}
