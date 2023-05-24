use std::path::PathBuf;

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult},
};

use crate::{
    types::{
        manifest::{ManifestFile, ManifestSection, ProductionsFile},
        schema::{Schema, SchemaSection, SchemaTopic},
    },
    validation::Model,
    yaml,
};

impl Schema {
    pub fn compile(codegen: &mut CodegenContext, schema_dir: PathBuf) -> CodegenResult<Schema> {
        let manifest_path = schema_dir.join("manifest.yml");
        let manifest_file = yaml::files::File::<ManifestFile>::load(codegen, manifest_path)?;

        let mut model = Model::new(&manifest_file)?;

        let sections = load_sections(
            codegen,
            &mut model,
            &schema_dir,
            &manifest_file.value.sections,
        )?;

        model.validate()?;

        let productions = sections
            .iter()
            .flat_map(|section| &section.topics)
            .flat_map(|topic| &topic.productions)
            .map(|(name, production)| (name.to_owned(), production.clone()))
            .collect();

        return Ok(Schema {
            title: manifest_file.value.title,
            versions: manifest_file.value.versions,
            sections,

            schema_dir,
            productions,
        });
    }
}

fn load_sections(
    codegen: &mut CodegenContext,
    model: &mut Model,
    schema_dir: &PathBuf,
    sections: &Vec<ManifestSection>,
) -> CodegenResult<Vec<SchemaSection>> {
    let mut results = Vec::<SchemaSection>::new();
    let mut errors = CodegenErrors::new();

    for section in sections {
        let mut topics = Vec::<SchemaTopic>::new();

        for topic in &section.topics {
            let productions_path = schema_dir
                .join(&section.path)
                .join(&topic.path)
                .join(SchemaTopic::productions_file());

            match yaml::files::File::<ProductionsFile>::load(codegen, productions_path) {
                Ok(productions_file) => {
                    model.add_productions_file(&productions_file);

                    topics.push(SchemaTopic {
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

        results.push(SchemaSection {
            title: section.title.to_owned(),
            path: section.path.to_owned(),
            topics,
        });
    }

    return errors.err_or(results);
}
