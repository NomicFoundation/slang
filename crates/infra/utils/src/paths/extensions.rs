use std::env::var;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

pub trait PathExtensions {
    fn collect_children(&self) -> Result<Vec<PathBuf>>;

    fn generated_dir(&self) -> Result<PathBuf>;

    fn repo_path(relative_path: impl AsRef<Path>) -> PathBuf;

    fn strip_repo_root(&self) -> Result<&Path>;

    fn replace_prefix(&self, old_prefix: impl AsRef<Path>, new_prefix: impl AsRef<Path>)
        -> PathBuf;

    fn unwrap_str(&self) -> &str;

    fn unwrap_string(&self) -> String;

    fn unwrap_name(&self) -> &str;

    fn unwrap_parent(&self) -> &Path;

    fn unwrap_ext(&self) -> &str;

    fn read_to_string(&self) -> Result<String>;

    fn write_string(&self, contents: impl AsRef<str>) -> Result<()>;
}

impl PathExtensions for Path {
    fn collect_children(&self) -> Result<Vec<PathBuf>> {
        let mut children = vec![];

        for entry in self
            .read_dir()
            .with_context(|| format!("Failed to read directory: {self:?}"))?
        {
            let entry = entry.unwrap().file_name();
            let file_name = entry.to_str().unwrap();

            children.push(self.join(file_name));
        }

        Ok(children)
    }

    fn generated_dir(&self) -> Result<PathBuf> {
        let mut parts = self.iter().collect_vec();

        while parts.last().is_some_and(|part| *part != "generated") {
            parts.pop();
        }

        ensure!(
            !parts.is_empty(),
            "Generated file path should have a 'generated' ancestor dir: {self:?}"
        );

        Ok(parts.iter().collect())
    }

    fn repo_path(relative_path: impl AsRef<Path>) -> PathBuf {
        Path::repo_root().join(relative_path)
    }

    fn strip_repo_root(&self) -> Result<&Path> {
        let repo_root = Path::repo_root();

        return self
            .strip_prefix(&repo_root)
            .with_context(|| format!("Failed to strip repo root from: {self:?}"));
    }

    fn replace_prefix(
        &self,
        old_prefix: impl AsRef<Path>,
        new_prefix: impl AsRef<Path>,
    ) -> PathBuf {
        let old_prefix = old_prefix.as_ref();
        let new_prefix = new_prefix.as_ref();

        let suffix = self
            .strip_prefix(old_prefix)
            .with_context(|| format!("Failed to strip prefix: {old_prefix:?} from: {self:?}"))
            .unwrap();

        new_prefix.join(suffix)
    }

    fn unwrap_str(&self) -> &str {
        return self
            .to_str()
            .with_context(|| format!("Failed to convert path to str: {self:?}"))
            .unwrap();
    }

    fn unwrap_string(&self) -> String {
        self.unwrap_str().to_owned()
    }

    fn unwrap_name(&self) -> &str {
        return self
            .file_name()
            .with_context(|| format!("Failed to extract file name of: {self:?}"))
            .unwrap()
            .to_str()
            .with_context(|| format!("Failed convert path to str: {self:?}"))
            .unwrap();
    }

    fn unwrap_parent(&self) -> &Path {
        return self
            .parent()
            .with_context(|| format!("Failed to extract parent directory of: {self:?}"))
            .unwrap();
    }

    fn unwrap_ext(&self) -> &str {
        return self
            .extension()
            .unwrap_or_default()
            .to_str()
            .with_context(|| format!("Failed to convert extension to str: {self:?}"))
            .unwrap();
    }

    fn read_to_string(&self) -> Result<String> {
        std::fs::read_to_string(self).with_context(|| format!("Failed to read file: {self:?}"))
    }

    fn write_string(&self, contents: impl AsRef<str>) -> Result<()> {
        return std::fs::write(self, contents.as_ref())
            .with_context(|| format!("Failed to write file: {self:?}"));
    }
}

/// Internal only. Instead, other crates should use other exposed utils from this crate.
pub(crate) trait PrivatePathExtensions {
    fn repo_root() -> PathBuf;
}

impl PrivatePathExtensions for Path {
    fn repo_root() -> PathBuf {
        PathBuf::from(var("REPO_ROOT").expect("$REPO_ROOT is not defined."))
    }
}
