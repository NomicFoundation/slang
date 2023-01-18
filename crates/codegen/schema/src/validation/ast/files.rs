use crate::{
    types,
    validation::ast::{
        manifest::Manifest,
        productions::{Production, ProductionRef},
        Node,
    },
    yaml,
};

pub type FilePathRef = std::rc::Rc<std::path::PathBuf>;

pub struct File<T> {
    pub path: FilePathRef,
    pub ast: Node<T>,
}

pub type ManifestFile = File<Manifest>;

impl ManifestFile {
    pub fn new(syntax_file: &yaml::files::File<types::manifest::ManifestFile>) -> Self {
        let yaml::files::File {
            path,
            syntax,
            value,
        } = syntax_file;

        let value = Manifest::new(syntax, value.to_owned());

        return Self {
            path: FilePathRef::new(path.to_owned()),
            ast: Node::new(syntax, value),
        };
    }
}

pub type TopicFile = File<Vec<ProductionRef>>;

impl TopicFile {
    pub fn new(syntax_file: &yaml::files::File<types::manifest::TopicFile>) -> Self {
        let yaml::files::File {
            path,
            syntax,
            value,
        } = syntax_file;

        let value = syntax.zip_array(value.to_owned(), Production::new);

        return Self {
            path: FilePathRef::new(path.to_owned()),
            ast: Node::new(syntax, value),
        };
    }
}
