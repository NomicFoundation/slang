use std::rc::Rc;

use indexmap::IndexMap;
use semver::Version;

use crate::{types, yaml::cst};

use super::{node::Node, parser::Parser, precedence_parser::PrecedenceParser, scanner::Scanner};

pub type ProductionRef = std::rc::Rc<Production>;

pub enum Production {
    Scanner {
        name: Node<String>,
        version_map: Node<VersionMap<Scanner>>,
    },
    TriviaParser {
        name: Node<String>,
        version_map: Node<VersionMap<Parser>>,
    },
    Parser {
        name: Node<String>,
        version_map: Node<VersionMap<Parser>>,
    },
    PrecedenceParser {
        name: Node<String>,
        version_map: Node<VersionMap<PrecedenceParser>>,
    },
}

impl Production {
    pub fn new(cst_node: &cst::NodeRef, value: types::production::ProductionRef) -> ProductionRef {
        match value.as_ref() {
            types::production::Production::Scanner {
                name,
                version_map: versioning,
            } => Rc::new(Self::Scanner {
                name: Node::new(cst_node.value_of_field("name"), name.to_owned()),
                version_map: VersionMap::new(cst_node, versioning.to_owned()),
            }),
            types::production::Production::TriviaParser {
                name,
                version_map: versioning,
            } => Rc::new(Self::TriviaParser {
                name: Node::new(cst_node.value_of_field("name"), name.to_owned()),
                version_map: VersionMap::new(cst_node, versioning.to_owned()),
            }),
            types::production::Production::Parser {
                name,
                version_map: versioning,
            } => Rc::new(Self::Parser {
                name: Node::new(cst_node.value_of_field("name"), name.to_owned()),
                version_map: VersionMap::new(cst_node, versioning.to_owned()),
            }),
            types::production::Production::PrecedenceParser {
                name,
                version_map: versioning,
            } => Rc::new(Self::PrecedenceParser {
                name: Node::new(cst_node.value_of_field("name"), name.to_owned()),
                version_map: VersionMap::new(cst_node, versioning.to_owned()),
            }),
        }
    }

    pub fn name_node(&self) -> &Node<String> {
        match self {
            Self::Scanner { name, .. }
            | Self::TriviaParser { name, .. }
            | Self::Parser { name, .. }
            | Self::PrecedenceParser { name, .. } => name,
        }
    }

    pub fn version_map_cst_node(&self) -> cst::NodeRef {
        match self {
            Production::Scanner { version_map, .. } => version_map.cst_node.clone(),
            Production::TriviaParser { version_map, .. } => version_map.cst_node.clone(),
            Production::Parser { version_map, .. } => version_map.cst_node.clone(),
            Production::PrecedenceParser { version_map, .. } => version_map.cst_node.clone(),
        }
    }

    pub fn versions(&self) -> Option<Vec<&Node<Version>>> {
        match self {
            Production::Scanner { version_map, .. } => match version_map.value {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            Production::TriviaParser { version_map, .. } => match version_map.value {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            Production::Parser { version_map, .. } => match version_map.value {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
            Production::PrecedenceParser { version_map, .. } => match version_map.value {
                VersionMap::Unversioned(_) => None,
                VersionMap::Versioned(ref map) => Some(map.keys().collect()),
            },
        }
    }
}

pub trait ConcreteAbstractPair {
    type AbstractType;
    fn new(cst_node: &cst::NodeRef, value: Rc<Self::AbstractType>) -> Rc<Self>;
}

pub enum VersionMap<T: ConcreteAbstractPair> {
    Unversioned(Rc<T>),
    Versioned(IndexMap<Node<Version>, Option<Rc<T>>>),
}

impl<T: ConcreteAbstractPair> VersionMap<T> {
    pub fn new(
        cst_node: &cst::NodeRef,
        value: types::production::VersionMap<T::AbstractType>,
    ) -> Node<Self> {
        match value {
            types::production::VersionMap::Unversioned(value) => {
                let cst_node = &cst_node.value_of_field("unversioned");
                let value = Self::Unversioned(T::new(&cst_node, value));

                return Node::new(cst_node, value);
            }

            types::production::VersionMap::Versioned(value) => {
                let cst_node = &cst_node.value_of_field("versioned");
                let value = Self::Versioned(
                    value
                        .into_iter()
                        .map(|(version, expression)| {
                            let field = cst_node.field(&version.to_string());
                            (
                                Node::new(&field.key, version),
                                expression.map(|expression| T::new(&field.value, expression)),
                            )
                        })
                        .collect(),
                );

                return Node::new(cst_node, value);
            }
        };
    }
}

pub struct Reference {
    pub reference: Node<String>,
}

impl Reference {
    pub fn new(cst_node: &cst::NodeRef, value: types::production::Reference) -> Node<Self> {
        let cst_node = cst_node.field("reference");
        Node::new(
            &cst_node.key,
            Self {
                reference: Node::new(&cst_node.value, value.reference),
            },
        )
    }
}
