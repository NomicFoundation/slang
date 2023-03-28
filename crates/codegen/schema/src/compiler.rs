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
        // let mut model = Model::new(&manifest_file, codegen)?;
        let manifest = manifest_file.value;
        return Ok(Grammar {
            manifest_dir,
            title: manifest.title,
            versions: manifest.versions,
            sections: manifest.sections,
            productions: manifest.productions,
        });
    }
}
