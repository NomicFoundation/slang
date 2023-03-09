mod ast;
mod rules;
use ast::files::{ManifestFile, ProductionsFile};

use codegen_utils::errors::{CodegenErrors, CodegenResult};

use crate::{types, yaml};

pub struct Model {
    manifest_file: ManifestFile,
    productions_files: Vec<ProductionsFile>,
}

impl Model {
    pub fn new(
        manifest_file: &yaml::files::File<types::manifest::ManifestFile>,
    ) -> CodegenResult<Self> {
        let model = Self {
            manifest_file: ast::files::ManifestFile::new(manifest_file),
            productions_files: vec![],
        };

        let mut errors = CodegenErrors::new();
        rules::manifest_paths::check(&model, &mut errors);

        return errors.err_or(model);
    }

    pub fn add_productions_file(
        &mut self,
        productions_file: &yaml::files::File<types::manifest::ProductionsFile>,
    ) {
        let productions_file = ast::files::ProductionsFile::new(productions_file);
        self.productions_files.push(productions_file);
    }

    pub fn validate(&self) -> CodegenResult<()> {
        let mut errors = CodegenErrors::new();
        let definitions = rules::definitions::collect(&self, &mut errors);

        rules::versions::check(&self, &mut errors);
        rules::references::check(&self, &definitions, &mut errors);
        rules::empty_productions::check(&self, &mut errors);

        return errors.err_or(());
    }
}
