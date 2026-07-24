use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use flate2::read::GzDecoder;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::github::GitHub;
use infra_utils::http::{DownloadResult, request_download_if_modified};
use infra_utils::paths::PathExtensions;
use rayon::prelude::*;
use semver::Version;
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::versions::LanguageVersion;
use tar::{Archive, EntryType};

/// Checked-in JSON file (`{ "<version>": "<commit-sha>" }`) pinning the exact
/// commit each version's semantic tests were fetched from, so we notice if a
/// tag is later re-pointed at a different commit. Regenerated in update mode.
const PINNED_COMMITS_FILE: &str = "pinned-commits.json";

const SEMANTIC_TESTS_PATH: &str = "test/libsolidity/semanticTests";

/// The `solc` release tag for a language version (e.g. `v0.8.20`).
pub(crate) fn release_tag(version: LanguageVersion) -> String {
    format!("v{version}", version = Version::from(version))
}

/// The inverse of [`release_tag`]: the language version a tag names, or `None`
/// if it isn't a tag we'd have produced (an unparseable version, or one this
/// slang doesn't support). Kept next to `release_tag` so the tag format is
/// written down in exactly one place.
pub(crate) fn version_from_release_tag(tag: &str) -> Option<LanguageVersion> {
    let version = Version::parse(tag.strip_prefix('v')?).ok()?;
    LanguageVersion::try_from(version).ok()
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
    /// The commit the tag resolved to when fetched (from the tarball's
    /// `pax_global_header`), or empty if a legacy cache didn't record it.
    commit_sha: String,
}

impl Dataset {
    /// Ensures the semantic tests for `version` are available locally,
    /// downloading and extracting them from the matching `solc` release tag if
    /// necessary, and returns a handle to the extracted tree (carrying the
    /// commit SHA the tag resolved to). Reconciling that SHA against the pinned
    /// baseline is the caller's job (see [`fetch_all_versions`]).
    pub fn fetch(version: LanguageVersion) -> Result<Self> {
        let tag = release_tag(version);
        let version_dir = cache_dir().join(&tag);
        let root = version_dir.join("semanticTests");
        let sha_path = version_dir.join(".commit-sha");

        // Release tags are immutable, so a populated cache is always current —
        // skip the network entirely. This matters here because we fetch dozens
        // of versions, and the cache lives under `target/` (cached in CI).
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
                commit_sha,
            });
        }

        // Reuse the shared download helper (as the sourcify runner does).
        let url = format!("https://codeload.github.com/argotorg/solidity/tar.gz/{tag}");
        let commit_sha = match request_download_if_modified(&url, &root) {
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
                bail!("Unexpected 'not modified' response downloading {url} into an empty cache");
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
            commit_sha,
        })
    }

    pub fn version(&self) -> LanguageVersion {
        self.version
    }

    pub fn commit_sha(&self) -> &str {
        &self.commit_sha
    }
}

/// Fetches every supported version's semantic tests (in parallel — only a cold
/// cache actually hits the network), then reconciles the commit SHAs the tags
/// resolved to against the checked-in `pinned-commits.json`.
pub fn fetch_all_versions() -> Result<Vec<Dataset>> {
    // Downloads are independent and I/O-bound; fetching ~three dozen tarballs
    // one at a time dominates a cold-cache run, so fan them out across rayon.
    let datasets = LanguageVersion::ALL
        .par_iter()
        .map(|&version| Dataset::fetch(version))
        .collect::<Result<Vec<_>>>()?;

    reconcile_pinned_commits(&datasets)?;

    Ok(datasets)
}

/// Reconciles the datasets' commit SHAs with the pinned-commits baseline, which
/// is loaded (and, in update mode, rewritten) exactly once here rather than
/// per version.
fn reconcile_pinned_commits(datasets: &[Dataset]) -> Result<()> {
    let mut pinned = pinned_commits();

    if !GitHub::is_running_in_ci() {
        for dataset in datasets {
            pinned.insert(dataset.version(), dataset.commit_sha().to_owned());
        }
        return write_pinned_commits(&pinned);
    }

    for dataset in datasets {
        let version = dataset.version();
        let Some(expected) = pinned.get(&version) else {
            bail!(
                "no pinned commit recorded for v{version} in {PINNED_COMMITS_FILE}. \
                 Regenerate the baseline by running the suite locally (outside CI)."
            );
        };
        if dataset.commit_sha() != expected {
            bail!(
                "tag v{version} now points to commit {actual}, but the pinned commit is \
                 {expected}. The tag appears to have been moved. If this is intended, \
                 regenerate the baseline by running the suite locally (outside CI) and \
                 review the diff.",
                actual = dataset.commit_sha()
            );
        }
    }

    // The pin file must not carry versions slang no longer supports — a stale
    // extra entry means the baseline wasn't regenerated after a version was
    // dropped.
    for pinned_version in pinned.keys() {
        if !datasets
            .iter()
            .any(|dataset| dataset.version() == *pinned_version)
        {
            bail!(
                "{PINNED_COMMITS_FILE} pins v{pinned_version}, which is not a supported version. \
                 Regenerate the baseline by running the suite locally (outside CI)."
            );
        }
    }

    Ok(())
}

fn pinned_commits_path() -> Result<PathBuf> {
    Ok(
        CargoWorkspace::locate_source_crate("solidity_testing_solc_comparison")?
            .join(PINNED_COMMITS_FILE),
    )
}

/// Loads the pinned `version -> commit SHA` map from the checked-in file.
fn pinned_commits() -> SortedMap<LanguageVersion, String> {
    pinned_commits_path()
        .ok()
        .filter(|path| path.exists())
        .and_then(|path| path.read_to_string().ok())
        .and_then(|contents| serde_json::from_str(&contents).ok())
        .unwrap_or_default()
}

/// Rewrites the checked-in pinned-commits file from the given map.
fn write_pinned_commits(pinned: &SortedMap<LanguageVersion, String>) -> Result<()> {
    let json = serde_json::to_string_pretty(pinned)?;
    let path = pinned_commits_path()?;
    path.write_string(format!("{json}\n"))?;
    println!(
        "Wrote {count} pinned commit(s) to {path:?}",
        count = pinned.len()
    );
    Ok(())
}

pub fn dataset_root() -> String {
    fetch_all_versions().expect("failed to fetch semantic tests");
    cache_dir().unwrap_string()
}

/// The regex (matched against each file's path relative to [`dataset_root`])
/// selecting real, standalone test files: `v<version>/semanticTests/<test>.sol`,
/// excluding any path segment starting with `_` (those are `ExternalSource`
/// fixtures, not standalone tests).
pub const HARNESS_PATTERN: &str =
    r"^v[0-9]+\.[0-9]+\.[0-9]+/semanticTests/(?:[^/_][^/]*/)*[^/_][^/]*\.sol$";

/// Parses the `(language version, test path relative to `semanticTests`)` out
/// of a full test-file path of the form `.../v<version>/semanticTests/<rel>`.
pub fn parse_version_and_relpath(path: &Path) -> Option<(LanguageVersion, String)> {
    let components: Vec<&str> = path.iter().filter_map(|c| c.to_str()).collect();
    let index = components.iter().position(|&c| c == "semanticTests")?;
    let version_tag = components.get(index.checked_sub(1)?)?;
    let language_version = version_from_release_tag(version_tag)?;
    let relative_path = components[index + 1..].join("/");
    Some((language_version, relative_path))
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
    fn parses_version_and_relpath() {
        let (version, relative_path) = parse_version_and_relpath(Path::new(
            "target/solc-comparison/v0.8.20/semanticTests/various/erc20.sol",
        ))
        .expect("a well-formed dataset path parses");
        assert_eq!(relative_path, "various/erc20.sol");
        assert_eq!(Version::from(version).to_string(), "0.8.20");

        // A path without the `semanticTests` anchor doesn't parse.
        assert!(
            parse_version_and_relpath(Path::new("target/solc-comparison/v0.8.20/README.md"))
                .is_none()
        );
        // A version tag slang v2 doesn't support (< 0.8.0) doesn't parse.
        assert!(parse_version_and_relpath(Path::new("x/v0.7.0/semanticTests/a.sol")).is_none());
    }

    #[test]
    fn release_tag_round_trips() {
        // Every supported version's tag maps back to that same version.
        for &version in LanguageVersion::ALL {
            assert_eq!(
                version_from_release_tag(&release_tag(version)),
                Some(version)
            );
        }

        // Tags we'd never have produced don't parse: a missing `v` prefix, a
        // version slang v2 doesn't support, and outright junk.
        assert_eq!(version_from_release_tag("0.8.20"), None);
        assert_eq!(version_from_release_tag("v0.7.0"), None);
        assert_eq!(version_from_release_tag("vnonsense"), None);
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
}
