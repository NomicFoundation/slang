use crate::{types, yaml};

use super::{
    manifest::Manifest,
    node::Node,
    production::{Production, ProductionRef},
};

pub type PathBufRef = std::rc::Rc<std::path::PathBuf>;

pub struct File<T> {
    pub path: PathBufRef,
    pub ast: Node<T>,
}

pub type ManifestFile = File<Manifest>;

impl ManifestFile {
    pub fn new(file: &yaml::files::File<types::manifest::ManifestFile>) -> Self {
        let yaml::files::File {
            path,
            cst_node,
            value,
        } = file;

        let value = Manifest::new(cst_node, value.to_owned());

        return Self {
            path: PathBufRef::new(path.to_owned()),
            ast: Node::new(cst_node, value),
        };
    }
}

pub type TopicFile = File<Vec<ProductionRef>>;

impl TopicFile {
    pub fn new(file: &yaml::files::File<types::manifest::TopicFile>) -> Self {
        let yaml::files::File {
            path,
            cst_node,
            value,
        } = file;

        let value = cst_node.zip(value.to_owned(), Production::new);

        return Self {
            path: PathBufRef::new(path.to_owned()),
            ast: Node::new(cst_node, value),
        };
    }
}
