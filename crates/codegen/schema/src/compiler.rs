use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::{
    codegen::{Codegen, CodegenReadWrite},
    errors::{InfraErrors, Position},
};

use crate::{
    types::{
        LanguageDefinition, LanguageDefinitionRef, LanguageSection, LanguageTopic, ManifestFile,
        ManifestSection, ManifestTopic, ProductionsFile,
    },
    validation::validate_language,
    yaml::deserialize_yaml,
};

impl LanguageDefinition {
    pub fn compile(language_dir: PathBuf) -> Result<LanguageDefinitionRef> {
        compile_language(language_dir).map_err(|error| {
            if let Some(errors) = error.downcast_ref::<InfraErrors>() {
                eprintln!();
                eprintln!("{errors}");

                eprintln!();
                eprintln!(
                    "Found {count} validation errors. Aborting.",
                    count = errors.len()
                );

                // `process::exit()` instead of `panic!()`. No need to pollute the output with useless stack traces:
                std::process::exit(1);
            } else {
                error
            }
        })
    }
}

fn compile_language(language_dir: PathBuf) -> Result<LanguageDefinitionRef> {
    let mut codegen = Codegen::read_write(&language_dir)?;

    let manifest_path = language_dir.join(LanguageDefinition::MANIFEST_FILE_NAME);
    let manifest = deserialize_yaml::<ManifestFile>(&mut codegen, &manifest_path)?;

    let sections = load_sections(&mut codegen, &language_dir, &manifest.sections)?;

    let productions = sections
        .iter()
        .flat_map(|section| &section.topics)
        .flat_map(|topic| &topic.productions)
        .map(|production| (production.name.to_owned(), production.clone()))
        .collect();

    let language = LanguageDefinitionRef::new(LanguageDefinition {
        title: manifest.title,
        sections,
        versions: manifest.versions,

        root_production: manifest.root_production,
        productions,

        language_dir,
    });

    validate_language(&language)?;

    Ok(language)
}

fn load_sections(
    codegen: &mut CodegenReadWrite,
    manifest_dir: &Path,
    sections: &Vec<ManifestSection>,
) -> Result<Vec<LanguageSection>> {
    let mut results = Vec::<LanguageSection>::new();
    let mut errors = InfraErrors::new();

    for section in sections {
        let section_dir = manifest_dir.join(&section.path);
        let mut topics = Vec::<LanguageTopic>::new();

        for topic in &section.topics {
            match load_topic(codegen, &section_dir, topic) {
                Ok(topic) => {
                    topics.push(topic);
                }
                Err(error) => {
                    errors.extend(error);
                }
            };
        }

        results.push(LanguageSection {
            title: section.title.to_owned(),
            path: section.path.to_owned(),
            topics,
        });
    }

    errors.into_result().map(|_| results)
}

fn load_topic(
    codegen: &mut CodegenReadWrite,
    section_dir: &Path,
    topic: &ManifestTopic,
) -> Result<LanguageTopic, InfraErrors> {
    let topic_dir = section_dir.join(&topic.path);

    let notes_path = topic_dir.join(LanguageTopic::NOTES_FILE_NAME);
    let productions_path = topic_dir.join(LanguageTopic::PRODUCTIONS_FILE_NAME);

    for path in [&notes_path, &productions_path] {
        if !path.exists() {
            let range = Position::new(0, 0, 0)..Position::new(usize::MAX, usize::MAX, usize::MAX);
            let message = "Topic file not found.".to_string();

            return Err(InfraErrors::single(path.to_owned(), range, message));
        }
    }

    let productions = deserialize_yaml::<ProductionsFile>(codegen, &productions_path)?;

    let topic = LanguageTopic {
        title: topic.title.to_owned(),
        path: topic.path.to_owned(),
        productions,
    };

    Ok(topic)
}
