use std::collections::HashSet;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::codegen::tera::TeraWrapper;
use crate::codegen::wit::generate_rust_bindings;
use crate::codegen::{CodegenFileSystem, JINJA_GLOB, WIT_GLOB};
use crate::paths::{FileWalker, PathExtensions};

pub struct CodegenRuntime {
    input_dir: PathBuf,
    fs: CodegenFileSystem,
    tera: TeraWrapper,
}

impl CodegenRuntime {
    pub fn new(input_dir: impl Into<PathBuf>) -> Result<Self> {
        let input_dir = input_dir.into();
        let fs = CodegenFileSystem::new(&input_dir)?;
        let tera = TeraWrapper::new(&input_dir)?;

        Ok(Self {
            input_dir,
            fs,
            tera,
        })
    }

    pub fn render_stubs(&mut self, model: impl Serialize) -> Result<()> {
        let context = tera::Context::from_serialize(model)?;

        for template_path in FileWalker::from_directory(&self.input_dir).find([JINJA_GLOB])? {
            let stub_path = Self::get_stub_path(&template_path).with_extension("");
            let output = self.tera.render(&template_path, &context)?;

            self.fs.write_file(&stub_path, output)?;
        }

        for wit_path in FileWalker::from_directory(&self.input_dir).find([WIT_GLOB])? {
            let bindings_path = Self::get_stub_path(&wit_path).with_extension("rs");
            let bindings = generate_rust_bindings(&wit_path)?;

            self.fs.write_file(&bindings_path, bindings)?;
        }

        Ok(())
    }

    pub fn render_directory(
        &mut self,
        model: impl Serialize,
        output_dir: impl AsRef<Path>,
    ) -> Result<()> {
        let context = tera::Context::from_serialize(model)?;
        let output_dir = output_dir.as_ref();

        let mut handled = HashSet::new();

        for template_path in FileWalker::from_directory(&self.input_dir).find([JINJA_GLOB])? {
            let stub_path = Self::get_stub_path(&template_path).with_extension("");
            let rendered_path = self.get_output_path(&stub_path, output_dir)?;
            let rendered = self.tera.render(&template_path, &context)?;

            self.fs.write_file(&rendered_path, rendered)?;

            assert!(handled.insert(template_path));
            assert!(handled.insert(stub_path));
        }

        for wit_path in FileWalker::from_directory(&self.input_dir).find([WIT_GLOB])? {
            let stub_path = Self::get_stub_path(&wit_path).with_extension("rs");
            let wit_output_path = self.get_output_path(&wit_path, output_dir)?;

            // ".wit" files can be generated from templates:
            if !handled.contains(&wit_path) {
                self.fs.copy_file(&wit_path, &wit_output_path)?;

                assert!(handled.insert(wit_path));
            }

            let bindings_path = self.get_output_path(&stub_path, output_dir)?;
            let bindings = generate_rust_bindings(&wit_output_path)?;

            self.fs.write_file(&bindings_path, bindings)?;

            assert!(handled.insert(stub_path));
        }

        for source_path in FileWalker::from_directory(&self.input_dir)
            .find_all()?
            .filter(|source_path| !handled.contains(source_path))
        {
            let output_path = self.get_output_path(&source_path, output_dir)?;

            // Preserve the source of otherwise-generated files:
            if source_path.generated_dir().is_ok() {
                self.fs.mark_generated_file(output_path)?;
            } else {
                self.fs.copy_file(source_path, output_path)?;
            }
        }

        Ok(())
    }

    fn get_stub_path(template_path: &Path) -> PathBuf {
        let base_name = template_path.unwrap_name();

        template_path
            .unwrap_parent()
            .join("generated")
            .join(base_name)
    }

    fn get_output_path(&self, source_path: &Path, output_dir: &Path) -> Result<PathBuf> {
        Ok(output_dir.join(source_path.strip_prefix(&self.input_dir)?))
    }
}
