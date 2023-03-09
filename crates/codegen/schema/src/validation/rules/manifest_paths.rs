use std::path::PathBuf;

use codegen_utils::errors::CodegenErrors;

use crate::{
    types::grammar::GrammarTopic,
    validation::{
        ast::{
            files::ManifestFile,
            visitors::{Reporter, Visitor, VisitorExtensions, VisitorResponse},
        },
        Model,
    },
};

pub fn check(model: &Model, errors: &mut CodegenErrors) {
    let mut visitor = PathsChecker::new(model);

    visitor.visit(model, errors);
}

struct PathsChecker {
    grammar_dir: PathBuf,
}

impl PathsChecker {
    fn new(model: &Model) -> Self {
        return Self {
            grammar_dir: model.manifest_file.path.parent().unwrap().to_owned(),
        };
    }
}

impl Visitor for PathsChecker {
    fn visit_manifest(
        &mut self,
        manifest_file: &ManifestFile,
        reporter: &mut Reporter,
    ) -> VisitorResponse {
        for section in &manifest_file.ast.value.sections {
            let section_path = self.grammar_dir.join(&section.path.value);

            if !section_path.exists() {
                reporter.report(&section.path.cst_node, Errors::PathNotFound(section_path));
                continue;
            }

            for topic in &section.topics {
                let topic_path = section_path.join(&topic.path.value);

                if !topic_path.exists() {
                    reporter.report(&topic.path.cst_node, Errors::PathNotFound(topic_path));
                    continue;
                }

                for path in [
                    topic_path.join(GrammarTopic::productions_file()),
                    topic_path.join(GrammarTopic::notes_file()),
                ] {
                    if !path.exists() {
                        reporter.report(&topic.path.cst_node, Errors::PathNotFound(path));
                    }
                }
            }
        }

        return VisitorResponse::StepOut;
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),
}
