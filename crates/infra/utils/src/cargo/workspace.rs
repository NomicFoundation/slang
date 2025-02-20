use std::env::var;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use regex::Regex;
use semver::Version;

use crate::cargo::manifest::{Dependency, WorkspaceManifest};
use crate::commands::Command;
use crate::github::GitHub;
use crate::paths::PathExtensions;

pub struct CargoWorkspace;

impl CargoWorkspace {
    pub fn install_binary(crate_name: impl AsRef<str>) -> Result<()> {
        let crate_name = crate_name.as_ref();

        let manifest = WorkspaceManifest::load()?;

        let mut command = Command::new("cargo")
            .args(["install", crate_name])
            .flag("--locked");

        let dependency = manifest.dependency(crate_name)?;

        command = match dependency {
            Dependency::Local { path, version } => command
                .property("--path", path.unwrap_str())
                .property("--version", version.to_string()),

            Dependency::CratesIO { version } => command.property("--version", version.to_string()),

            Dependency::GitBranch { git, branch } => command
                .property("--git", git.as_str())
                .property("--branch", branch),

            Dependency::GitRevision { git, rev } => command
                .property("--git", git.as_str())
                .property("--rev", rev),
        };

        command.run();

        Ok(())
    }

    pub fn is_running_inside_build_scripts() -> bool {
        var("CARGO_MANIFEST_DIR").is_ok() && var("TARGET").is_ok()
    }

    pub fn locate_source_crate(crate_name: impl AsRef<str>) -> Result<PathBuf> {
        let manifest = WorkspaceManifest::load()?;
        let dependency = manifest.dependency(crate_name)?;

        match dependency {
            Dependency::Local { path, version: _ } => Ok(Path::repo_path(path)),

            Dependency::CratesIO { .. }
            | Dependency::GitBranch { .. }
            | Dependency::GitRevision { .. } => {
                bail!("Cannot locate source crate for non-local dependencies.")
            }
        }
    }

    pub fn local_version() -> Result<Version> {
        Ok(WorkspaceManifest::load()?.version().to_owned())
    }

    pub fn published_version(crate_name: impl AsRef<str>) -> Result<Version> {
        let crate_name = crate_name.as_ref();

        // Expected Output from 'cargo search crate_name':
        //
        // crate_name = "1.2.3" # description
        //
        // Extract and parse the version (middle part).

        let output = Command::new("cargo")
            .args(["search", crate_name])
            .evaluate()?;

        let (_full, [version]) = Regex::new(&format!(r#"^{crate_name} = "(\d+\.\d+\.\d+)" *#"#))?
            .captures(&output)
            .with_context(|| format!("Failed to extract version:\n{output}"))?
            .extract();

        Version::parse(version).with_context(|| format!("Failed to parse version: '{version}'"))
    }

    pub fn update_version(new_version: &Version) -> Result<()> {
        CargoWorkspace::install_binary("cargo-edit")?;

        // This will update the '[workspace.package.version]' field of the root 'Cargo.toml' file.
        // And also the '[version]' field of all internal crates in its '[workspace.dependencies]' table.
        Command::new("cargo")
            .arg("set-version")
            .flag("--workspace")
            .arg(new_version.to_string())
            .run();

        Ok(())
    }
}

pub trait CargoWorkspaceCommands {
    #[must_use]
    fn add_build_rustflags(self) -> Self;
}

impl CargoWorkspaceCommands for Command {
    fn add_build_rustflags(self) -> Self {
        if !GitHub::is_running_in_ci() {
            // Nothing to add locally:
            return self;
        }

        // Using `$RUSTFLAGS' or '--' overrides any rustflags from `.cargo/config.toml'.
        // Using this syntax instead, as it is concatenated with the existing flags:
        self.flag("--release")
            .property(
                "--config",
                format!(
                    "build.rustflags = {rustflags}",
                    rustflags = serde_json::to_string(&[
                        // Deny any warnings in CI:
                        "-Dwarnings",
                        // Lint against leftover `dbg/todo!` macros in CI:
                        "-Wclippy::dbg_macro",
                        "-Wclippy::todo"
                    ])
                    .unwrap(),
                ),
            )
            // Rustdoc requires specifying RUSTDOCFLAGS, instead:
            // See <https://github.com/rust-lang/cargo/issues/8424#issuecomment-1070988443>.
            .property(
                "--config",
                format!(
                    "build.rustdocflags = {rustdocflags}",
                    rustdocflags = serde_json::to_string(&[
                        // Deny any warnings in CI:
                        "-Dwarnings"
                    ])
                    .unwrap(),
                ),
            )
    }
}
