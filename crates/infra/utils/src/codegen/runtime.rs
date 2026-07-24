use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::codegen::CodegenFileSystem;
use crate::codegen::tera::TeraWrapper;
use crate::paths::{PathExtensions, PrivatePathExtensions};

pub struct CodegenRuntime;

impl CodegenRuntime {
    pub fn render_templates_in_place<'c>(
        context_for: impl Fn(&Path) -> &'c tera::Context + Sync,
    ) -> Result<()> {
        let repo_root = Path::repo_root();
        let tera = TeraWrapper::new(&repo_root)?;

        let all_templates = tera
            .find_all_templates()?
            .filter(|path|
            // Templates starting with underscore are meant to contain common macros.
            // They are not rendered directly, but imported by other templates.
            !path.unwrap_name().starts_with('_'))
            .collect::<Vec<_>>();

        assert!(
            !all_templates.is_empty(),
            "No templates under {repo_root:?}",
        );

        all_templates.par_iter().try_for_each(|template_path| {
            let generated_path = Self::get_in_place_path(template_path);
            let rendered = tera.render(template_path, context_for(template_path))?;

            let mut fs = CodegenFileSystem::default();
            fs.write_file_formatted(&generated_path, rendered)
        })
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
