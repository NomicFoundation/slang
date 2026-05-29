use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::Serialize;

use crate::codegen::tera::TeraWrapper;
use crate::codegen::CodegenFileSystem;
use crate::paths::{PathExtensions, PrivatePathExtensions};

pub struct CodegenRuntime;

impl CodegenRuntime {
    pub fn render_templates_in_place(
        model: impl Serialize,
        filter: impl Fn(&Path) -> bool,
    ) -> Result<()> {
        let repo_root = Path::repo_root();
        let tera = TeraWrapper::new(&repo_root)?;

        let all_templates = tera
            .find_all_templates()?
            .filter(|path|
            // Templates starting with underscore are meant to contain common macros.
            // They are not rendered directly, but imported by other templates.
            !path.unwrap_name().starts_with('_'))
            .filter(|path| filter(path))
            .collect::<Vec<_>>();

        assert!(
            !all_templates.is_empty(),
            "No templates under {repo_root:?} matched the provided filter.",
        );

        let mut fs = CodegenFileSystem::default();

        let mut context = tera::Context::new();
        context.insert("model", &model);

        for template_path in all_templates {
            let generated_path = Self::get_in_place_path(&template_path);
            let rendered = tera.render(&template_path, &context)?;

            fs.write_file_formatted(&generated_path, rendered)?;
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
