mod ast;
mod rules;
use ast::files::{ManifestFile, TopicFile};

use codegen_utils::errors::{CodegenErrors, CodegenResult};

use crate::{types, yaml};

pub struct Model {
    manifest_file: ManifestFile,
    topic_files: Vec<TopicFile>,
}

impl Model {
    pub fn new(manifest_file: &yaml::files::File<types::manifest::ManifestFile>) -> Self {
        return Self {
            manifest_file: ast::files::ManifestFile::new(manifest_file),
            topic_files: vec![],
        };
    }

    pub fn add_topic(&mut self, topic_file: &yaml::files::File<types::manifest::TopicFile>) {
        let topic_file = ast::files::TopicFile::new(topic_file);
        self.topic_files.push(topic_file);
    }

    pub fn validate(&self) -> CodegenResult<()> {
        let mut errors = CodegenErrors::new();
        let definitions = rules::definitions::collect(&self, &mut errors);

        rules::versions::check(&self, &mut errors);
        rules::references::check(&self, &definitions, &mut errors);
        rules::empty_productions::check(&self, &mut errors);

        return errors.err_or(());
    }
}
