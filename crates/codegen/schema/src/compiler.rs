use std::path::PathBuf;

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult, Position},
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
    pub fn compile(
        codegen: &mut CodegenContext,
        language_dir: PathBuf,
    ) -> CodegenResult<LanguageDefinitionRef> {
        let manifest_path = language_dir.join(LanguageDefinition::MANIFEST_FILE_NAME);
        let manifest = deserialize_yaml::<ManifestFile>(codegen, &manifest_path)?;

        let sections = load_sections(codegen, &language_dir, &manifest.sections)?;

        let productions = sections
            .iter()
            .flat_map(|section| &section.topics)
            .flat_map(|topic| &topic.productions)
            .map(|production| (production.name().to_owned(), production.clone()))
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

        return Ok(language);
    }
}

fn load_sections(
    codegen: &mut CodegenContext,
    manifest_dir: &PathBuf,
    sections: &Vec<ManifestSection>,
) -> CodegenResult<Vec<LanguageSection>> {
    let mut results = Vec::<LanguageSection>::new();
    let mut errors = CodegenErrors::new();

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

    return errors.to_result().map(|_| results);
}

fn load_topic(
    codegen: &mut CodegenContext,
    section_dir: &PathBuf,
    topic: &ManifestTopic,
) -> CodegenResult<LanguageTopic> {
    let topic_dir = section_dir.join(&topic.path);

    let notes_path = topic_dir.join(LanguageTopic::NOTES_FILE_NAME);
    let productions_path = topic_dir.join(LanguageTopic::PRODUCTIONS_FILE_NAME);

    for path in [&notes_path, &productions_path] {
        if !path.exists() {
            let range = Position::new(0, 0, 0)..Position::new(usize::MAX, usize::MAX, usize::MAX);
            let message = format!("Topic file not found.");
            return Err(CodegenErrors::single(path.to_owned(), range, message));
        }
    }

    let productions = deserialize_yaml::<ProductionsFile>(codegen, &productions_path)?;

    let topic = LanguageTopic {
        title: topic.title.to_owned(),
        path: topic.path.to_owned(),
        productions,
    };

    return Ok(topic);
}
