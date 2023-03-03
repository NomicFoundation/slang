use std::path::PathBuf;

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult},
};

use crate::{
    types::{
        grammar::{Grammar, GrammarSection, GrammarTopic},
        manifest::{ManifestFile, ManifestSection, ProductionsFile},
    },
    validation::Model,
    yaml,
};

impl Grammar {
    pub fn compile(codegen: &mut CodegenContext, manifest_dir: PathBuf) -> CodegenResult<Grammar> {
        let manifest_path = manifest_dir.join("manifest.yml");
        let manifest_file = yaml::files::File::<ManifestFile>::load(codegen, manifest_path)?;

        let mut model = Model::new(&manifest_file)?;

        let sections = load_sections(
            codegen,
            &mut model,
            &manifest_dir,
            &manifest_file.value.sections,
        )?;

        model.validate()?;

        let productions = sections
            .iter()
            .flat_map(|section| &section.topics)
            .flat_map(|topic| &topic.productions)
            .map(|(name, production)| (name.to_owned(), production.clone()))
            .collect();

        return Ok(Grammar {
            title: manifest_file.value.title,
            versions: manifest_file.value.versions,
            sections,

            manifest_dir,
            productions,
        });
    }
}

fn load_sections(
    codegen: &mut CodegenContext,
    model: &mut Model,
    grammar_dir: &PathBuf,
    sections: &Vec<ManifestSection>,
) -> CodegenResult<Vec<GrammarSection>> {
    let mut results = Vec::<GrammarSection>::new();
    let mut errors = CodegenErrors::new();

    for section in sections {
        let mut topics = Vec::<GrammarTopic>::new();

        for topic in &section.topics {
            let productions_path = grammar_dir
                .join(&section.path)
                .join(&topic.path)
                .join(GrammarTopic::productions_file());

            match yaml::files::File::<ProductionsFile>::load(codegen, productions_path) {
                Ok(productions_file) => {
                    model.add_productions_file(&productions_file);

                    topics.push(GrammarTopic {
                        title: topic.title.to_owned(),
                        path: topic.path.to_owned(),
                        productions: productions_file
                            .value
                            .iter()
                            .map(|production| (production.name().to_owned(), production.clone()))
                            .collect(),
                    });
                }
                Err(error) => {
                    errors.extend(error);
                }
            };
        }

        results.push(GrammarSection {
            title: section.title.to_owned(),
            path: section.path.to_owned(),
            topics,
        });
    }

    return errors.err_or(results);
}
