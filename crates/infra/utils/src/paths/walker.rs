use std::path::{Path, PathBuf};

use anyhow::Result;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;

use crate::paths::PrivatePathExtensions;

pub struct FileWalker {
    root_dir: PathBuf,
}

impl FileWalker {
    pub fn from_repo_root() -> Self {
        Self::from_directory(Path::repo_root())
    }

    pub fn from_directory(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }

    pub fn find_all(&self) -> Result<impl Iterator<Item = PathBuf>> {
        self.find(["**/*"])
    }

    pub fn find<G>(&self, globs: impl AsRef<[G]>) -> Result<impl Iterator<Item = PathBuf>>
    where
        G: AsRef<str>,
    {
        let globs = globs.as_ref();
        assert!(!globs.is_empty(), "Must provide at least one glob.");

        let overrides = {
            let builder = &mut OverrideBuilder::new(&self.root_dir);

            // Since we allow hidden (dot) files below, we need to explicitly ignore the .git directory:
            builder.add("!.git/")?;

            for glob in globs {
                builder.add(glob.as_ref())?;
            }

            builder.build()?
        };

        let walk = WalkBuilder::new(&self.root_dir)
            .hidden(false)
            .overrides(overrides)
            .follow_links(false)
            .sort_by_file_path(std::cmp::Ord::cmp)
            .build();

        let iterator = walk.filter_map(|entry| {
            let entry = entry.unwrap();

            if entry.file_type().unwrap().is_file() {
                Some(entry.into_path())
            } else {
                None
            }
        });

        Ok(iterator)
    }
}
