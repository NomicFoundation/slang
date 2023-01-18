use std::path::PathBuf;

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult},
};

use crate::{
    types::{
        grammar::{Grammar, GrammarSection, GrammarTopic},
        manifest::{ManifestFile, TopicFile},
    },
    validation, yaml,
};

impl Grammar {
    pub fn compile(codegen: &mut CodegenContext, manifest_path: PathBuf) -> CodegenResult<Grammar> {
        let manifest_file = yaml::files::File::<ManifestFile>::load(codegen, manifest_path)?;
        validation::check_manifest(&manifest_file)?;

        let sections = load_sections(codegen, &manifest_file)?;

        let productions = sections
            .iter()
            .flat_map(|section| &section.topics)
            .flat_map(|topic| &topic.productions)
            .map(|production| (production.name.to_owned(), production.clone()))
            .collect();

        validation::check_productions(&manifest_file, &productions)?;

        return Ok(Grammar {
            title: manifest_file.value.title,
            versions: manifest_file.value.versions,
            sections,
            productions,
        });
    }
}

fn load_sections(
    codegen: &mut CodegenContext,
    manifest_file: &yaml::files::File<ManifestFile>,
) -> CodegenResult<Vec<GrammarSection>> {
    let manifest_dir = manifest_file.path.parent().unwrap().to_owned();

    let mut sections = Vec::<GrammarSection>::new();
    let mut errors = CodegenErrors::new();

    for section in &manifest_file.value.sections {
        let mut topics = Vec::<GrammarTopic>::new();

        for topic in &section.topics {
            let definition = match &topic.definition {
                Some(definition) => definition,
                None => {
                    topics.push(GrammarTopic {
                        title: topic.title.to_owned(),
                        productions: vec![],
                    });

                    continue;
                }
            };

            let topic_path = manifest_dir.join(definition).canonicalize().unwrap();

            match yaml::files::File::<TopicFile>::load(codegen, topic_path) {
                Ok(topic_file) => {
                    topics.push(GrammarTopic {
                        title: topic.title.to_owned(),
                        productions: topic_file.value,
                    });
                }
                Err(topic_errors) => {
                    errors.extend(topic_errors);
                }
            };
        }

        sections.push(GrammarSection {
            title: section.title.to_owned(),
            topics,
        });
    }

    return errors.err_or(sections);
}
