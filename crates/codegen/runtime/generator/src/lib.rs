mod ast;
mod bindings;
mod built_ins;
mod kinds;
mod parser;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::Result;
pub use built_ins::render_built_ins;
use codegen_language_definition::model::Language;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use semver::Version;
use serde::Serialize;

use crate::ast::AstModel;
use crate::bindings::BindingsModel;
use crate::kinds::KindsModel;
use crate::parser::ParserModel;

pub enum OutputLanguage {
    Cargo,
    Npm,
}

impl OutputLanguage {
    pub fn generate_runtime(
        &self,
        language: &Rc<Language>,
        output_dir: &Path,
        generate_extra_fn: impl FnOnce(&mut CodegenFileSystem) -> Result<()>,
    ) -> Result<()> {
        let model = ModelWrapper {
            rendering_in_stubs: false,
            model: RuntimeModel::from_language(language)?,
        };

        let runtime = CodegenRuntime::new(self.source_dir()?)?;

        let mut file_system = runtime.render_directory(model, output_dir)?;
        generate_extra_fn(&mut file_system)
    }

    pub fn generate_stubs(&self) -> Result<()> {
        let model = ModelWrapper {
            rendering_in_stubs: true,
            model: RuntimeModel::default(),
        };

        let mut runtime = CodegenRuntime::new(self.source_dir()?)?;

        runtime.render_stubs(&model)
    }

    fn source_dir(&self) -> Result<PathBuf> {
        let crate_name = match self {
            Self::Cargo => "codegen_runtime_cargo",
            Self::Npm => "codegen_runtime_npm",
        };

        Ok(CargoWorkspace::locate_source_crate(crate_name)?.join("src/runtime"))
    }
}

/// A utility wrapper to make it easier to write conditional code in templates
/// by checking `rendering_in_stubs`. Additionally, it makes sure that all
/// model properties are prefixed with `model.` to make it easier to read/refactor.
#[derive(Serialize)]
struct ModelWrapper {
    rendering_in_stubs: bool,
    model: RuntimeModel,
}

#[derive(Serialize)]
struct RuntimeModel {
    slang_version: Version,
    all_language_versions: BTreeSet<Version>,
    breaking_language_versions: BTreeSet<Version>,

    ast: AstModel,
    bindings: BindingsModel,
    kinds: KindsModel,
    parser: ParserModel,
}

impl RuntimeModel {
    pub fn from_language(language: &Rc<Language>) -> Result<Self> {
        Ok(Self {
            slang_version: CargoWorkspace::local_version()?,
            all_language_versions: language.versions.iter().cloned().collect(),
            breaking_language_versions: language.collect_breaking_versions(),

            ast: AstModel::from_language(language),
            bindings: BindingsModel::from_language(language)?,
            parser: ParserModel::from_language(language),
            kinds: KindsModel::from_language(language),
        })
    }
}

impl Default for RuntimeModel {
    fn default() -> Self {
        Self {
            slang_version: Version::new(0, 0, 0),
            all_language_versions: BTreeSet::default(),
            breaking_language_versions: BTreeSet::default(),

            ast: AstModel::default(),
            bindings: BindingsModel::default(),
            kinds: KindsModel::default(),
            parser: ParserModel::default(),
        }
    }
}
