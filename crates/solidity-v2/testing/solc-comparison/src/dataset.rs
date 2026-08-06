use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};
use flate2::read::GzDecoder;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::http::{DownloadResult, request_download};
use infra_utils::paths::{FileWalker, PathExtensions};
use rayon::prelude::*;
use semver::Version;
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::versions::LanguageVersion;
use tar::{Archive, EntryType};

use crate::generated_file;

/// Checked-in JSON file (`{ "<version>": "<commit-sha>" }`) pinning the exact
/// commit each version's semantic tests were fetched from, so we notice if a
/// tag is later re-pointed at a different commit.
const PINNED_COMMITS_FILE: &str = "pinned-commits.generated.json";

const SEMANTIC_TESTS_PATH: &str = "test/libsolidity/semanticTests";

/// The `solc` release tag for a language version (e.g. `v0.8.20`).
pub(crate) fn release_tag(version: LanguageVersion) -> String {
    format!("v{version}", version = Version::from(version))
}

/// The cache directory holding every version's extracted `semanticTests` tree,
/// as `<cache>/v<version>/semanticTests/...`.
pub fn cache_dir() -> PathBuf {
    Path::repo_path("target/solc-comparison")
}

/// A local, on-disk copy of the `libsolidity` semantic tests for one version.
pub struct Dataset {
    /// The Solidity version these tests come from.
    version: LanguageVersion,
    /// The extracted `semanticTests` tree for this version.
    root: PathBuf,
    /// The commit the tag resolved to when fetched (from the tarball's
    /// `pax_global_header`), or empty if a legacy cache didn't record it.
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
    /// Ensures the semantic tests for `version` are available locally,
    /// downloading and extracting them from the matching `solc` release tag if
    /// necessary, and returns a handle to the extracted tree (carrying the
    /// commit SHA the tag resolved to). Checking that SHA against the pinned
    /// baseline is the caller's job (see [`write_pinned_commits`]).
    pub fn fetch(version: LanguageVersion) -> Result<Self> {
        let tag = release_tag(version);
        let version_dir = cache_dir().join(&tag);
        let root = version_dir.join("semanticTests");
        let sha_path = version_dir.join(".commit-sha");

        if is_populated(&root) {
            let commit_sha = sha_path
                .read_to_string()
                .map(|s| s.trim().to_owned())
                .unwrap_or_default();
            if commit_sha.is_empty() {
                bail!(
                    "cached semantic tests at {root:?} are missing their recorded commit SHA \
                     ({sha_path:?}); delete the directory and re-run to re-download."
                );
            }
            return Ok(Self {
                version,
                root,
                commit_sha,
            });
        }

        let url = format!("https://codeload.github.com/argotorg/solidity/tar.gz/{tag}");
        let commit_sha = match request_download(&url) {
            DownloadResult::Ok(response) => {
                println!("Downloading semantic tests from {url}");
                let commit_sha = extract_semantic_tests(response, &root)?;

                if !is_populated(&root) {
                    bail!(
                        "Extraction completed but no semantic tests were found under {root:?}. \
                         The tag '{tag}' may not contain '{SEMANTIC_TESTS_PATH}'."
                    );
                }
                commit_sha
            }
            DownloadResult::NotModified => {
                unreachable!("`request_download` never revalidates, so it cannot report 304");
            }
            DownloadResult::Error(error) => {
                bail!("Failed to download semantic tests from {url}: {error}");
            }
        };

        // The commit SHA is our only defense against a re-pointed tag, so a
        // download that didn't yield one is a hard error rather than something
        // we quietly tolerate.
        if commit_sha.is_empty() {
            bail!("Downloaded tarball for tag '{tag}' carried no commit SHA in its pax header.");
        }
        sha_path.write_string(&commit_sha)?;

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

/// Fetches every supported version's semantic tests, and pins the commit each
/// tag resolved to. Only a cold cache actually hits the network.
pub fn fetch_all_versions(fs: &mut CodegenFileSystem) -> Result<Vec<Dataset>> {
    // Downloads are independent and I/O-bound; fetching ~three dozen tarballs
    // one at a time dominates a cold-cache run, so fan them out across rayon.
    let datasets: Vec<Dataset> = LanguageVersion::ALL
        .par_iter()
        .map(|&version| Dataset::fetch(version))
        .collect::<Result<_>>()?;

    write_pinned_commits(fs, &datasets)?;

    Ok(datasets)
}

/// Writes out the commit each version's tag resolved to. Locally that re-pins
/// the checked-in file; in CI it asserts nothing moved, so a tag that gets
/// re-pointed at different content fails the run instead of silently changing
/// what we test against.
fn write_pinned_commits(fs: &mut CodegenFileSystem, datasets: &[Dataset]) -> Result<()> {
    let pinned: SortedMap<LanguageVersion, &str> = datasets
        .iter()
        .map(|dataset| (dataset.version(), dataset.commit_sha()))
        .collect();

    fs.write_file_formatted(
        generated_file(PINNED_COMMITS_FILE)?,
        serde_json::to_string(&pinned)?,
    )
}

fn is_populated(root: &Path) -> bool {
    root.is_dir() && fs::read_dir(root).is_ok_and(|mut entries| entries.next().is_some())
}

/// Extracts the `semanticTests/` tree into `root` and returns the commit SHA the
/// tarball was built from (read from its `pax_global_header`, empty if absent).
fn extract_semantic_tests(reader: impl Read, root: &Path) -> Result<String> {
    let decoder = GzDecoder::new(reader);
    let mut archive = Archive::new(decoder);

    let mut commit_sha = String::new();
    let mut extracted = 0usize;
    for entry in archive.entries()? {
        let mut entry = entry?;

        // GitHub archives lead with a pax global header whose `comment` record
        // is the commit SHA the tag resolved to.
        if entry.header().entry_type() == EntryType::XGlobalHeader {
            let mut header = String::new();
            entry.read_to_string(&mut header).ok();
            if let Some(sha) = parse_pax_comment(&header) {
                commit_sha = sha;
            }
            continue;
        }

        let entry_path = entry.path()?.into_owned();

        let Some(relative) = strip_to_semantic_tests(&entry_path) else {
            continue;
        };

        // `relative` comes from the archive, and we're about to join it onto
        // `root`. A `..` or absolute component would escape the cache directory,
        // so refuse to unpack rather than trusting the tarball.
        if !is_contained_relative(&relative) {
            bail!("Refusing to unpack entry with a non-relative path: {entry_path:?}");
        }

        let dest = root.join(relative);
        if entry.header().entry_type().is_dir() {
            continue;
        }

        fs::create_dir_all(dest.unwrap_parent())?;
        entry
            .unpack(&dest)
            .with_context(|| format!("Failed to unpack entry into {dest:?}"))?;
        extracted += 1;
    }

    println!("Extracted {extracted} file(s) into {root:?}");
    Ok(commit_sha)
}

/// Extracts the `comment=<sha>` value from a pax header's records (each record
/// is `"<len> key=value\n"`).
fn parse_pax_comment(header: &str) -> Option<String> {
    let start = header.find("comment=")? + "comment=".len();
    let value = header[start..].lines().next()?.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

/// Whether `path` stays inside whatever directory it is joined onto: every
/// component must be an ordinary name, ruling out `..` (which escapes upwards)
/// and root/prefix components (which discard the base path entirely).
fn is_contained_relative(path: &Path) -> bool {
    path.components()
        .all(|component| matches!(component, Component::Normal(_)))
}

/// Returns the portion of `path` after the `test/libsolidity/semanticTests`
/// segment, if present.
fn strip_to_semantic_tests(path: &Path) -> Option<PathBuf> {
    // GitHub tarballs nest everything under a top-level directory, so the
    // anchor sits at an unknown depth. Match it component-wise (rather than as a
    // substring, which could match inside a file name) and keep what follows.
    let anchor: Vec<&OsStr> = Path::new(SEMANTIC_TESTS_PATH).iter().collect();
    let components: Vec<&OsStr> = path.iter().collect();
    let anchor_end = components
        .windows(anchor.len())
        .position(|window| window == anchor.as_slice())?
        + anchor.len();
    let relative: PathBuf = components[anchor_end..].iter().collect();
    (!relative.as_os_str().is_empty()).then_some(relative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_solc_release_tags() {
        assert_eq!(release_tag(LanguageVersion::V0_8_0), "v0.8.0");
        assert_eq!(release_tag(LanguageVersion::V0_8_20), "v0.8.20");
    }

    #[test]
    fn strips_to_semantic_tests() {
        // A real tarball entry: top-level dir, the anchor, then the test path.
        assert_eq!(
            strip_to_semantic_tests(Path::new(
                "solidity-0.8.20/test/libsolidity/semanticTests/various/erc20.sol"
            )),
            Some(PathBuf::from("various/erc20.sol"))
        );

        // Sibling trees under `libsolidity` (e.g. syntaxTests) are not matched.
        assert_eq!(
            strip_to_semantic_tests(Path::new(
                "solidity-0.8.20/test/libsolidity/syntaxTests/x.sol"
            )),
            None
        );

        // The anchor directory itself, with nothing after it, yields nothing.
        assert_eq!(
            strip_to_semantic_tests(Path::new("solidity-0.8.20/test/libsolidity/semanticTests")),
            None
        );
    }

    #[test]
    fn rejects_paths_escaping_the_cache_directory() {
        assert!(is_contained_relative(Path::new("various/erc20.sol")));

        // `..` climbs out of the destination, and an absolute path replaces it.
        assert!(!is_contained_relative(Path::new("../../etc/passwd")));
        assert!(!is_contained_relative(Path::new(
            "various/../../escaped.sol"
        )));
        assert!(!is_contained_relative(Path::new("/etc/passwd")));
    }
}
