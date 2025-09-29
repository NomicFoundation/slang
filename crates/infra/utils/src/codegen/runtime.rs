use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::codegen::tera::TeraWrapper;
use crate::codegen::CodegenFileSystem;
use crate::paths::PathExtensions;

pub struct CodegenRuntime;

impl CodegenRuntime {
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

        for template_path in tera.find_all_templates()? {
            let path = Self::get_in_place_path(&template_path);
            let rendered_path = path.replace_prefix(&input_dir, output_dir);

            let rendered = tera.render(&template_path, &context)?;

            fs.write_file_formatted(&rendered_path, rendered)?;
        }

        Ok(())
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
