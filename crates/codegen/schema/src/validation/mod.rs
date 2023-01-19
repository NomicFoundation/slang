mod ast;
mod rules;

use codegen_utils::errors::{CodegenErrors, CodegenResult};

use crate::{
    types,
    validation::ast::files::{ManifestFile, TopicFile},
    yaml,
};

pub struct Model {
    manifest: ManifestFile,
    topics: Vec<TopicFile>,
}

impl Model {
    pub fn new(manifest_file: &yaml::files::File<types::manifest::ManifestFile>) -> Self {
        return Self {
            manifest: ast::files::ManifestFile::new(manifest_file),
            topics: vec![],
        };
    }

    pub fn add_topic(&mut self, topic_file: &yaml::files::File<types::manifest::TopicFile>) {
        let topic_file = ast::files::TopicFile::new(topic_file);
        self.topics.push(topic_file);
    }

    pub fn validate(&self) -> CodegenResult<()> {
        let mut errors = CodegenErrors::new();
        let definitions = rules::definitions::collect(&self, &mut errors);

        rules::versions::check(&self, &mut errors);
        rules::references::check(&self, &definitions, &mut errors);
        rules::empty_tokens::check(&self, &mut errors);

        return errors.err_or(());
    }
}
