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

        let dependency = manifest
            .workspace
            .dependencies
            .get(crate_name)
            .with_context(|| format!("Cannot find dependency '{crate_name}' in workspace."))?;

        let mut command = Command::new("cargo")
            .args(["install", crate_name])
            .flag("--locked");

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

        command.run()
    }

    pub fn is_running_inside_build_scripts() -> bool {
        var("CARGO_MANIFEST_DIR").is_ok() && var("TARGET").is_ok()
    }

    pub fn locate_source_crate(crate_name: impl AsRef<str>) -> Result<PathBuf> {
        let crate_name = crate_name.as_ref();

        let manifest = WorkspaceManifest::load()?;

        let dependency = manifest
            .workspace
            .dependencies
            .get(crate_name)
            .with_context(|| format!("Cannot find dependency '{crate_name}' in workspace."))?;

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
        Ok(WorkspaceManifest::load()?.workspace.package.version)
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
            .run()
    }

    pub fn get_command(subcommand: impl AsRef<str>) -> Result<Command> {
        let subcommand = subcommand.as_ref();

        let mut command = Command::new("cargo")
            .arg(subcommand)
            .flag("--workspace")
            .flag("--all-features");

        if GitHub::is_running_in_ci() {
            // Using `$RUSTFLAGS' or '--' overrides any rustflags from `.cargo/config.toml'.
            // Using this syntax instead, as it is concatenated with the existing flags:
            command = command.property(
                "--config",
                format!(
                    "build.rustflags = {rustflags}",
                    rustflags = serde_json::to_string(&[
                        // Deny any warnings in CI:
                        "-Dwarnings",
                        // Lint against leftover `dbg/todo!` macros in CI:
                        "-Wclippy::dbg_macro",
                        "-Wclippy::todo"
                    ])?,
                ),
            );
            // Rustdoc requires specifying RUSTDOCFLAGS, instead:
            // See <https://github.com/rust-lang/cargo/issues/8424#issuecomment-1070988443>.
            command = command.property(
                "--config",
                format!(
                    "build.rustdocflags = {rustdocflags}",
                    rustdocflags = serde_json::to_string(&[
                        // Deny any warnings in CI:
                        "-Dwarnings"
                    ])?,
                ),
            );
        }

        Ok(command)
    }
}
