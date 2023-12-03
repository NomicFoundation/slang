use std::{
    env::var,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use regex::Regex;
use semver::Version;

use crate::{
    cargo::manifest::WorkspaceManifest, commands::Command, github::GitHub, paths::PathExtensions,
};

pub struct CargoWorkspace;

impl CargoWorkspace {
    pub fn install_binary(crate_name: impl AsRef<str>) -> Result<()> {
        let crate_name = crate_name.as_ref();

        let version = WorkspaceManifest::load()?
            .workspace
            .dependencies
            .get(crate_name)
            .with_context(|| format!("Cannot find dependency '{crate_name}' in workspace."))?
            .version
            .to_owned()
            .with_context(|| {
                format!("Dependency '{crate_name}' did not specify a version in Cargo.toml.")
            })?
            .to_string();

        Command::new("cargo")
            .args(["install", crate_name])
            .property("--version", version)
            .run()
    }

    pub fn is_running_inside_build_scripts() -> bool {
        var("CARGO_MANIFEST_DIR").is_ok() && var("TARGET").is_ok()
    }

    pub fn locate_source_crate(crate_name: impl AsRef<str>) -> Result<PathBuf> {
        let crate_name = crate_name.as_ref();

        let relative_path = WorkspaceManifest::load()?
            .workspace
            .dependencies
            .get(crate_name)
            .with_context(|| format!("Cannot find dependency '{crate_name}' in workspace."))?
            .path
            .to_owned()
            .with_context(|| {
                format!("Dependency '{crate_name}' did not specify a path in Cargo.toml.")
            })?;

        Ok(Path::repo_path(relative_path))
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

    pub fn update_version(existing_version: &Version, updated_version: &Version) -> Result<()> {
        // A hack until 'cargo metadata' can support updating versions.
        // Just read the 'Cargo.toml' file manually and update the version:

        let cargo_toml = Path::repo_path("Cargo.toml");
        let existing_contents = cargo_toml.read_to_string()?;

        let existing_header = format!("[workspace.package]\nversion = \"{existing_version}\"\n");
        let updated_header = format!("[workspace.package]\nversion = \"{updated_version}\"\n");
        ensure!(existing_contents.starts_with(&existing_header));

        let updated_contents = existing_contents.replace(&existing_header, &updated_header);
        ensure!(updated_contents.starts_with(&updated_header));

        cargo_toml.write_string(updated_contents)
    }

    pub fn get_command(subcommand: impl AsRef<str>) -> Result<Command> {
        let mut command = Command::new("cargo")
            .arg(subcommand.as_ref())
            .flag("--all")
            .flag("--all-targets")
            .flag("--all-features");

        if GitHub::is_running_in_ci() {
            // Using `$RUSTFLAGS' or '--' overrides any rustflags from `.cargo/config.toml'.
            // Using this syntax instead, as it is concatenated with the existing flags:
            command = command.property(
                "--config",
                format!(
                    "build.rustflags = {rustflags}",
                    rustflags = serde_json::to_string(&["--deny", "warnings"])?,
                ),
            );
        }

        Ok(command)
    }
}
