use std::path::PathBuf;

use codegen_utils::{context::CodegenContext, errors::CodegenResult};

use super::{
    types::{grammar::Grammar, manifest::Manifest},
    validation::model::Model,
    yaml,
};

impl Grammar {
    pub fn compile(codegen: &mut CodegenContext, manifest_dir: PathBuf) -> CodegenResult<Grammar> {
        let manifest_path = manifest_dir.join("manifest.yml");
        let manifest_file = yaml::files::File::<Manifest>::load(codegen, manifest_path)?;

        let mut model = Model::new(&manifest_file)?;

        let sections =
            model.load_sections(codegen, &manifest_dir, &manifest_file.value.sections)?;

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
