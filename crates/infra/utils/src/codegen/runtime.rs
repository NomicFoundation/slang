use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Result};
use serde::Serialize;

use crate::codegen::tera::TeraWrapper;
use crate::codegen::CodegenFileSystem;
use crate::paths::{FileWalker, PathExtensions};

pub struct CodegenRuntime;

impl CodegenRuntime {
    pub fn render_stubs(
        fs: &mut CodegenFileSystem,
        input_dir: impl Into<PathBuf>,
        model: impl Serialize,
    ) -> Result<()> {
        let tera = TeraWrapper::new(input_dir)?;
        let context = tera::Context::from_serialize(model)?;

        for template_path in tera.find_all_templates()? {
            let stub_path = Self::get_stub_path(&template_path).with_extension("");
            let output = tera.render(&template_path, &context)?;

            fs.write_file_formatted(&stub_path, output)?;
        }

        Ok(())
    }

    pub fn render_templates_in_place(
        fs: &mut CodegenFileSystem,
        dir: impl Into<PathBuf>,
        model: impl Serialize,
    ) -> Result<()> {
        let tera = TeraWrapper::new(dir)?;
        let context = tera::Context::from_serialize(model)?;

        for template_path in tera.find_all_templates()? {
            let generated_path = Self::get_in_place_path(&template_path);
            let rendered = tera.render(&template_path, &context)?;

            fs.write_file_formatted(&generated_path, rendered)?;
        }

        Ok(())
    }

    pub fn render_product(
        fs: &mut CodegenFileSystem,
        input_dir: impl Into<PathBuf>,
        output_dir: impl AsRef<Path>,
        model: impl Serialize,
    ) -> Result<()> {
        let input_dir = input_dir.into();
        let output_dir = output_dir.as_ref();

        let tera = TeraWrapper::new(&input_dir)?;
        let context = tera::Context::from_serialize(model)?;

        let mut handled = HashSet::new();

        for template_path in tera.find_all_templates()? {
            let stub_path = Self::get_stub_path(&template_path).with_extension("");
            let rendered_path = stub_path.replace_prefix(&input_dir, output_dir);
            let rendered = tera.render(&template_path, &context)?;

            fs.write_file_formatted(&rendered_path, rendered)?;

            assert!(handled.insert(template_path));
            assert!(handled.insert(stub_path));
        }

        for source_path in FileWalker::from_directory(&input_dir)
            .find_all()?
            .filter(|source_path| !handled.contains(source_path))
        {
            let contents = source_path.read_to_string()?;
            let output_path = source_path.replace_prefix(&input_dir, output_dir);

            ensure!(
                !source_path.is_generated(),
                "Source file should not be inside a generated directory: {source_path:?}"
            );

            fs.write_file_raw(output_path, contents)?;
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

    fn get_in_place_path(template_path: &Path) -> PathBuf {
        assert_eq!(template_path.extension(), Some(OsStr::new("jinja2")));

        let template_path = template_path.with_extension("");
        let (base_name, extension) = template_path.unwrap_name().rsplit_once('.').unwrap();

        template_path
            .unwrap_parent()
            .join(format!("{base_name}.generated.{extension}"))
    }
}
