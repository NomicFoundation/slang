use std::path::PathBuf;

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult},
};

use super::{
    super::{
        types::{
            grammar::{GrammarSection, GrammarTopic},
            manifest::Manifest,
            production::ProductionRef,
        },
        yaml,
    },
    rules,
};

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),
}

pub struct Model {
    manifest_file: yaml::files::File<Manifest>,
    productions_files: Vec<yaml::files::File<Vec<ProductionRef>>>,
    grammar_sections: Vec<GrammarSection>,
}

impl Model {
    pub fn new(
        manifest_file: yaml::files::File<Manifest>,
        codegen: &mut CodegenContext,
    ) -> CodegenResult<Self> {
        let model = Self {
            manifest_file,
            productions_files: vec![],
            grammar_sections: vec![],
        };

        let mut errors = CodegenErrors::new();

        let grammar_dir = &manifest_file.path.parent().unwrap();

        for section in &manifest_file.value.sections {
            let section_path = grammar_dir.join(&section.path);

            if section_path.exists() {
                for topic in &section.topics {
                    let topic_path = section_path.join(&topic.path);

                    if topic_path.exists() {
                        for path in [
                            topic_path.join(GrammarTopic::productions_file()),
                            topic_path.join(GrammarTopic::notes_file()),
                        ] {
                            if !path.exists() {
                                errors.push(
                                    &manifest_file.path,
                                    &topic.path.cst_node,
                                    Errors::PathNotFound(path),
                                );
                            }
                        }
                    } else {
                        errors.push(
                            &manifest_file.path,
                            &topic.path.cst_node,
                            Errors::PathNotFound(topic_path),
                        );
                    }
                }
            } else {
                errors.push(
                    &manifest_file.path,
                    &section.path.cst_node,
                    Errors::PathNotFound(section_path),
                );
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        for section in &manifest_file.value.sections {
            let mut topics = Vec::<GrammarTopic>::new();

            for topic in &section.topics {
                let productions_path = grammar_dir
                    .join(&section.path)
                    .join(&topic.path)
                    .join(GrammarTopic::productions_file());

                match yaml::files::File::<Vec<ProductionRef>>::load(codegen, productions_path) {
                    Ok(productions_file) => {
                        model.productions_files.push(productions_file);
                        topics.push(GrammarTopic {
                            title: topic.title.to_owned(),
                            path: topic.path.to_owned(),
                            productions: productions_file
                                .value
                                .iter()
                                .map(|production| {
                                    (production.name().to_owned(), production.clone())
                                })
                                .collect(),
                        });
                    }
                    Err(errors) => {
                        errors.extend(errors);
                    }
                };
            }

            model.grammar_sections.push(GrammarSection {
                title: section.title.to_owned(),
                path: section.path.to_owned(),
                topics,
            });
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        let versions_validator = rules::versions::Validator::new(&mut errors);
        model.visit(&mut versions_validator);

        let empty_productions_validator = rules::empty_productions::Validator::new(&mut errors);
        model.visit(&mut empty_productions_validator);

        let definitions_validator = rules::definitions::Validator::new(&mut errors);
        model.visit(&mut definitions_validator);

        let references_validator =
            rules::references::Validator::new(&mut errors, &definitions_validator);
        model.visit(&mut references_validator);

        if errors.len() > 0 {
            return Err(errors);
        }

        return Ok(model);
    }
}
