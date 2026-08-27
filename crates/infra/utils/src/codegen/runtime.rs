use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::codegen::CodegenFileSystem;
use crate::codegen::tera::TeraWrapper;
use crate::paths::{PathExtensions, PrivatePathExtensions};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodegenRuntime {
    V1Templates,
    V2Templates,
}

impl CodegenRuntime {
    pub fn render_templates(self, context: &tera::Context) -> Result<()> {
        let repo_root = Path::repo_root();
        let tera = TeraWrapper::new(&repo_root)?;

        let all_templates = tera
            .find_all_templates()?
            .filter(|path|
            // Templates starting with underscore only define shared components. They are still
            // loaded above (which is what registers those components), but never rendered alone.
            !path.unwrap_name().starts_with('_'))
            .filter(|path| self == Self::get_template_owner(path))
            .collect::<Vec<_>>();

        assert!(
            !all_templates.is_empty(),
            "No templates under {repo_root:?}",
        );

        all_templates.par_iter().try_for_each(|template_path| {
            let generated_path = Self::get_generated_path(template_path);
            let rendered = tera.render(template_path, context)?;

            let mut fs = CodegenFileSystem::default();
            fs.write_file_formatted(&generated_path, rendered)
        })
    }

    fn get_template_owner(template_path: &Path) -> Self {
        let template_path = template_path.strip_repo_root().unwrap();

        if template_path.starts_with("crates/language/")
            || template_path.starts_with("crates/solidity/")
        {
            Self::V1Templates
        } else if template_path.starts_with("crates/language-v2/")
            || template_path.starts_with("crates/solidity-v2/")
        {
            Self::V2Templates
        } else {
            panic!("Cannot categorize template: {template_path:?}");
        }
    }

    fn get_generated_path(template_path: &Path) -> PathBuf {
        assert_eq!(template_path.extension(), Some(OsStr::new("jinja2")));

        let template_path = template_path.with_extension("");
        let (base_name, extension) = template_path.unwrap_name().rsplit_once('.').unwrap();

        template_path
            .unwrap_parent()
            .join(format!("{base_name}.generated.{extension}"))
    }
}
