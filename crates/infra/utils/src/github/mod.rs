use std::env::var;
use std::path::Path;

use anyhow::{Context, Ok, Result};
use semver::Version;
use serde::Deserialize;

use crate::commands::Command;
use crate::paths::PathExtensions;

pub struct GitHub;

impl GitHub {
    pub fn is_running_in_ci() -> bool {
        var("CI").is_ok()
    }

    pub fn is_running_in_devcontainer() -> bool {
        var("IS_INSIDE_SLANG_DEVCONTAINER").is_ok()
    }

    /// Collapses the output of the given operation in the GitHub log viewer.
    /// They can be expanded individually when needed.
    /// This has no effect when running locally.
    pub fn collapse_group<T>(title: impl AsRef<str>, operation: impl FnOnce() -> T) -> T {
        let title = title.as_ref();

        if Self::is_running_in_ci() {
            eprintln!("::group::{title}");
        } else {
            eprintln!();
            eprintln!("{title}");
            eprintln!();
        }

        let result = operation();

        if Self::is_running_in_ci() {
            eprintln!("::endgroup::");
        } else {
            eprintln!();
        }

        result
    }

    pub fn latest_release_version() -> Result<Version> {
        #[derive(Debug, Deserialize)]
        struct Release {
            tag_name: String,
        }

        let json = Command::new("gh")
            .arg("api")
            .arg("repos/{owner}/{repo}/releases/latest")
            .evaluate()?;

        let tag_name = serde_json::from_str::<Release>(&json)?.tag_name;

        // tag_name is in the form 'v1.2.3', so remove the 'v' prefix before parsing the version:
        let version = tag_name
            .strip_prefix('v')
            .with_context(|| format!("Cannot extract version out of tag: {tag_name:#?}"))?;

        Ok(Version::parse(version)?)
    }

    pub fn create_new_release(tag_name: impl AsRef<str>, notes: impl AsRef<str>) -> Result<()> {
        let notes_file = Path::repo_path("target/publish/release-notes.md");

        std::fs::create_dir_all(notes_file.unwrap_parent())?;
        notes_file.write_string(notes)?;

        Command::new("gh")
            .args(["release", "create", tag_name.as_ref()])
            .property("--title", tag_name.as_ref())
            .property("--notes-file", notes_file.unwrap_str())
            .run();

        Ok(())
    }
}
