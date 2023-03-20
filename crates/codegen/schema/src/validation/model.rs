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
        manifest_file: &yaml::files::File<Manifest>,
        codegen: &mut CodegenContext,
    ) -> CodegenResult<Self> {
        let model = Self {
            manifest_file,
            productions_files: vec![],
            grammar_sections: vec![],
        };

        let mut errors = CodegenErrors::new();

        let grammar_dir = &manifest_file.path.parent().unwrap();

        for section in &manifest_file.ast.value.sections {
            let section_path = grammar_dir.join(&section.path.value);

            if section_path.exists() {
                for topic in &section.topics {
                    let topic_path = section_path.join(&topic.path.value);

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

        for section in &manifest_file.ast.value.sections {
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

        rules::versions::check(self, &mut errors);

        rules::empty_productions::check(self, &mut errors);

        let definitions = rules::definitions::collect(self, &mut errors);

        rules::references::check(self, &definitions, &mut errors);

        if errors.len() > 0 {
            return Err(errors);
        }

        return Ok(model);
    }
}
