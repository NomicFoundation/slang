use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::Result;
use codegen_language_definition::model::Language;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenTemplates;
use serde::Serialize;

use crate::model::RuntimeModel;

mod ast;
mod kinds;
mod model;
mod parser;

pub enum OutputLanguage {
    Cargo,
    Npm,
}

/// A utility wrapper to make it easier to write conditional code in templates
/// by checking `rendering_in_stubs`. Additionally, it makes sure that all
/// model properties are prefixed with `model.` to make it easier to read/refactor.
#[derive(Serialize)]
struct ModelWrapper {
    rendering_in_stubs: bool,
    model: RuntimeModel,
}

impl OutputLanguage {
    pub fn generate_runtime(&self, language: &Rc<Language>, output_dir: &Path) -> Result<()> {
        let model = ModelWrapper {
            rendering_in_stubs: false,
            model: RuntimeModel::from_language(language),
        };

        let mut templates = CodegenTemplates::new(self.source_dir()?)?;

        templates.render_directory(model, output_dir)
    }

    pub fn generate_stubs(&self) -> Result<()> {
        let model = ModelWrapper {
            rendering_in_stubs: true,
            model: RuntimeModel::for_stubs(),
        };

        let mut templates = CodegenTemplates::new(self.source_dir()?)?;

        templates.render_stubs(&model)
    }

    fn source_dir(&self) -> Result<PathBuf> {
        let crate_name = match self {
            Self::Cargo => "codegen_runtime_cargo",
            Self::Npm => "codegen_runtime_npm",
        };

        Ok(CargoWorkspace::locate_source_crate(crate_name)?.join("src/runtime"))
    }
}
