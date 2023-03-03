use semver::Version;

use crate::{types, yaml::cst};

use super::node::Node;

pub struct Manifest {
    pub title: Node<String>,
    pub root_production: Node<String>,
    pub sections: Vec<ManifestSection>,
    pub versions: Vec<Node<Version>>,
}

impl Manifest {
    pub fn new(cst_node: &cst::NodeRef, value: types::manifest::Manifest) -> Self {
        return Self {
            title: Node::new(cst_node.value_of_field("title"), value.title),
            root_production: Node::new(
                cst_node.value_of_field("rootProduction"),
                value.root_production,
            ),
            sections: cst_node
                .value_of_field("sections")
                .zip(value.sections, ManifestSection::new),
            versions: cst_node
                .value_of_field("versions")
                .zip(value.versions, Node::new),
        };
    }
}

pub struct ManifestSection {
    pub title: Node<String>,
    pub topics: Vec<ManifestTopic>,
}

impl ManifestSection {
    pub fn new(cst_node: &cst::NodeRef, value: types::manifest::ManifestSection) -> Self {
        return Self {
            title: Node::new(cst_node.value_of_field("title"), value.title),
            topics: cst_node
                .value_of_field("topics")
                .zip(value.topics, ManifestTopic::new),
        };
    }
}

pub struct ManifestTopic {
    pub title: Node<String>,
    pub definition: Option<Node<String>>,
}

impl ManifestTopic {
    pub fn new(cst_node: &cst::NodeRef, value: types::manifest::ManifestTopic) -> Self {
        return Self {
            title: Node::new(cst_node.value_of_field("title"), value.title),
            definition: value.definition.and_then(|definition| {
                return Some(Node::new(cst_node.value_of_field("definition"), definition));
            }),
        };
    }
}
