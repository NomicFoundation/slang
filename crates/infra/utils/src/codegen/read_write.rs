use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use cargo_emit::rerun_if_changed;
use inflector::Inflector;
use serde::Serialize;
use tera::Tera;

use crate::{cargo::CargoWorkspace, codegen::write_only::CodegenWriteOnly, paths::PathExtensions};

pub struct CodegenReadWrite {
    writer: CodegenWriteOnly,
    tera: Tera,
    input_dir: PathBuf,
}

impl CodegenReadWrite {
    pub fn new(input_dir: impl Into<PathBuf>) -> Result<Self> {
        let input_dir = input_dir.into();

        let input_dir = input_dir
            .canonicalize()
            .with_context(|| format!("Directory doesn't exist: {input_dir:?}"))?;

        let writer = CodegenWriteOnly::new()?;

        let tera = {
            let templates_glob = input_dir.join("**/*.jinja2");
            let mut tera = Tera::new(templates_glob.unwrap_str())?;

            fn snake_case(
                value: &tera::Value,
                _params: &HashMap<String, tera::Value>,
            ) -> Result<tera::Value, tera::Error> {
                let value = value.as_str().unwrap();
                let result = value.to_snake_case();
                return Ok(tera::Value::String(result));
            }
            tera.register_filter("snake_case", snake_case);

            tera.autoescape_on(vec![]); // disable autoescaping

            tera
        };

        return Ok(Self {
            input_dir,
            writer,
            tera,
        });
    }

    pub fn write_file(
        &mut self,
        file_path: impl AsRef<Path>,
        contents: impl AsRef<str>,
    ) -> Result<()> {
        return self.writer.write_file(file_path, contents);
    }

    pub fn read_file(&mut self, file_path: impl AsRef<Path>) -> Result<String> {
        let file_path = file_path.as_ref();

        if !file_path.starts_with(&self.input_dir) {
            bail!(
                "File path {:?} is not under input directory {:?}",
                file_path,
                self.input_dir
            );
        }

        if file_path.generated_dir().is_ok() {
            bail!("Cannot read from a generated directory: {file_path:?}");
        }

        return file_path.read_to_string();
    }

    pub fn copy_file(
        &mut self,
        source_path: impl AsRef<Path>,
        destination_path: impl AsRef<Path>,
    ) -> Result<()> {
        // Go through read_file() API, to record the correct metadata for it.
        let contents = self.read_file(source_path)?;

        // Go through write_file() API, to only touch/update the file if it changed.
        self.write_file(destination_path, contents)?;

        return Ok(());
    }

    pub fn render(
        &mut self,
        model: impl Serialize,
        template_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<()> {
        let templates_dir = &self.input_dir;
        let template_path = template_path.as_ref();

        // tera expects these paths to be relative:
        let template_path = template_path
            .strip_prefix(templates_dir)
            .with_context(|| {
                format!("Template path {template_path:?} must be under templates directory {templates_dir:?}")
            })?
            .unwrap_str();

        let model = tera::Context::from_serialize(model)?;
        let rendered = self.tera.render(template_path, &model)?;

        return self.write_file(output_path, rendered);
    }
}

impl Drop for CodegenReadWrite {
    fn drop(&mut self) {
        // Don't pollute test output with markers meant for build scripts:
        if CargoWorkspace::is_running_inside_build_scripts() {
            rerun_if_changed!(self.input_dir.unwrap_str());
        }
    }
}
