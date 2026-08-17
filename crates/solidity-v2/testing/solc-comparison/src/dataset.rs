use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use infra_utils::commands::Command;
use infra_utils::paths::{FileWalker, PathExtensions};
use rayon::prelude::*;
use slang_solidity_v2_common::versions::LanguageVersion;
use tempfile::TempDir;

const REPOSITORY_URL: &str = "https://github.com/argotorg/solidity.git";

const SEMANTIC_TESTS_PATH: &str = "test/libsolidity/semanticTests";

/// The `solc` release tag for a language version (e.g. `v0.8.20`).
fn release_tag(version: LanguageVersion) -> String {
    format!("v{version}")
}

/// A cloned bare solidity repository under the `target/` directory.
struct Repository {
    directory: PathBuf,
}

impl Repository {
    /// Clones `solc`'s repository, unless the cache already holds it.
    fn clone_or_reuse() -> Result<Self> {
        let repository = Self {
            directory: Path::repo_path("target/solc-comparison/solidity.git"),
        };

        if !repository.directory.join("HEAD").is_file() {
            println!(
                "Cloning {REPOSITORY_URL} into {directory:?}",
                directory = repository.directory
            );

            fs::create_dir_all(repository.directory.unwrap_parent())?;

            Command::new("git")
                .arg("clone")
                .flag("--bare")
                .flag("--quiet")
                .arg(REPOSITORY_URL)
                .arg(repository.directory.unwrap_str())
                .evaluate()
                .with_context(|| format!("Failed to clone {REPOSITORY_URL}"))?;
        }

        let missing: Vec<String> = LanguageVersion::ALL
            .iter()
            .map(|&version| release_tag(version))
            .filter(|tag| repository.resolve_tag(tag).is_err())
            .collect();

        if !missing.is_empty() {
            println!("Fetching tags missing from the local clone: {missing:?}");

            repository
                .git()
                .arg("fetch")
                .flag("--tags")
                .flag("--quiet")
                .evaluate()
                .context("Failed to fetch tags")?;
        }

        Ok(repository)
    }

    /// A `git` invocation against the clone. `--git-dir` rather than a working
    /// directory, since a bare clone has no work tree of its own.
    ///
    /// Checkouts honor the host's gitconfig, where `core.autocrlf=true` (which
    /// Git for Windows writes into the system config by default) would
    /// CRLF-convert the tests and silently change what the suite parses, so
    /// mask both config files off.
    fn git(&self) -> Command {
        Command::new("git")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_SYSTEM", "/dev/null")
            .property("--git-dir", self.directory.unwrap_str())
    }

    /// The commit a release tag points at. Annotated tags are objects in their
    /// own right, hence `^{commit}`.
    fn resolve_tag(&self, tag: &str) -> Result<String> {
        let commit_sha = self
            .git()
            .arg("rev-parse")
            .flag("--verify")
            .flag("--quiet")
            .arg(format!("{tag}^{{commit}}"))
            .evaluate()
            .with_context(|| format!("Failed to resolve tag '{tag}' in the local clone."))?
            .trim()
            .to_owned();

        if commit_sha.is_empty() {
            bail!("Tag '{tag}' does not exist in the local clone.");
        }

        Ok(commit_sha)
    }

    /// Checks the `semanticTests` tree out at `tag`, leaving it at `root`.
    fn checkout_semantic_tests(&self, tag: &str, root: &Path) -> Result<()> {
        let parent = root.unwrap_parent();
        fs::create_dir_all(parent)?;

        // `git checkout` needs a work tree, and writes an index that a bare
        // repository has no useful place for. Both are scratch: it reproduces
        // the path as it is in the repository, so what we want ends up nested
        // under `test/libsolidity/`, and every version runs concurrently so they
        // must not share an index. A directory of its own gives each version
        // both, and takes the leftovers with it when it drops.
        //
        // It sits next to `root` so that moving the tests out below is a rename
        // within one filesystem rather than a cross-device copy.
        let scratch = tempfile::Builder::new()
            .prefix(".checkout-")
            .tempdir_in(parent)
            .context("Failed to create a scratch directory for the checkout")?;

        self.git()
            .property("--work-tree", scratch.path().unwrap_str())
            .env("GIT_INDEX_FILE", scratch.path().join("index").unwrap_str())
            .arg("checkout")
            .arg(tag)
            .flag("--")
            .arg(SEMANTIC_TESTS_PATH)
            .evaluate()
            .with_context(|| {
                format!("Failed to check '{SEMANTIC_TESTS_PATH}' out of tag '{tag}'")
            })?;

        let checked_out = scratch.path().join(SEMANTIC_TESTS_PATH);
        fs::rename(&checked_out, root)
            .with_context(|| format!("Failed to move {checked_out:?} to {root:?}"))?;

        Ok(())
    }
}

/// A local, on-disk copy of the `libsolidity` semantic tests for one version.
///
/// Only ever handed out as a reference, by [`Datasets::versions`]: `root` points
/// into a temporary directory that [`Datasets`] owns and deletes on drop, so a
/// `Dataset` that outlived it would name files that are no longer there.
pub struct Dataset {
    /// The Solidity version these tests come from.
    version: LanguageVersion,
    /// The checked-out `semanticTests` tree for this version.
    root: PathBuf,
    /// The commit this version's release tag resolves to.
    commit_sha: String,
}

/// A single standalone test file within a [`Dataset`].
pub struct TestFile {
    /// Where the file lives on disk.
    pub path: PathBuf,
    /// Its path relative to `semanticTests`, which is how the baseline names it.
    pub relative_path: String,
}

impl Dataset {
    /// Checks the semantic tests for `version` out of `repository` at that
    /// version's release tag, into `parent`, and returns a handle to the tree
    /// carrying the commit the tag resolved to.
    fn checkout(repository: &Repository, version: LanguageVersion, parent: &Path) -> Result<Self> {
        let tag = release_tag(version);
        let root = parent.join(&tag);

        let commit_sha = repository.resolve_tag(&tag)?;

        repository.checkout_semantic_tests(&tag, &root)?;

        if !is_populated(&root) {
            bail!(
                "Checkout completed but no semantic tests were found under {root:?}. \
                 The tag '{tag}' may not contain '{SEMANTIC_TESTS_PATH}'."
            );
        }

        Ok(Self {
            version,
            root,
            commit_sha,
        })
    }

    pub fn version(&self) -> LanguageVersion {
        self.version
    }

    pub fn commit_sha(&self) -> &str {
        &self.commit_sha
    }

    /// Every standalone test in this dataset, in a stable order.
    ///
    /// `_`-prefixed entries hold `ExternalSource` fixtures shared between tests
    /// rather than tests of their own, so they (and anything under them) are
    /// excluded.
    pub fn test_files(&self) -> Result<Vec<TestFile>> {
        FileWalker::from_directory(&self.root)
            .find(["**/*.sol", "!**/_*"])?
            .map(|path| {
                let relative_path = path
                    .strip_prefix(&self.root)
                    .with_context(|| format!("Test file outside the dataset root: {path:?}"))?
                    .unwrap_str()
                    .to_owned();

                Ok(TestFile {
                    path,
                    relative_path,
                })
            })
            .collect()
    }
}

/// Every supported version's semantic tests, checked out for the duration of one
/// run.
pub struct Datasets {
    /// Deletes the checkouts on drop, so it has to outlive every [`Dataset`]
    /// below, all of which point into it.
    _directory: TempDir,
    versions: Vec<Dataset>,
}

impl Datasets {
    /// Makes every supported version's semantic tests available locally: clones
    /// `solc`'s repository if the cache doesn't already hold it, then checks
    /// each version's tests out of it. Only a missing or incomplete clone
    /// actually hits the network.
    pub fn create() -> Result<Self> {
        let repository = Repository::clone_or_reuse()?;

        let directory = tempfile::Builder::new()
            .prefix("slang-solc-comparison-")
            .tempdir()
            .context("Failed to create a temporary directory for the checkouts")?;

        // Checking a tag out is CPU- and I/O-bound and each version writes to its
        // own directory (and its own index), while the clone is only ever read,
        // so the versions fan out across rayon.
        let versions = LanguageVersion::ALL
            .par_iter()
            .map(|&version| Dataset::checkout(&repository, version, directory.path()))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            _directory: directory,
            versions,
        })
    }

    pub fn versions(&self) -> &[Dataset] {
        &self.versions
    }
}

fn is_populated(root: &Path) -> bool {
    root.is_dir() && fs::read_dir(root).is_ok_and(|mut entries| entries.next().is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_solc_release_tags() {
        assert_eq!(release_tag(LanguageVersion::V0_8_0), "v0.8.0");
        assert_eq!(release_tag(LanguageVersion::V0_8_20), "v0.8.20");
    }
}
