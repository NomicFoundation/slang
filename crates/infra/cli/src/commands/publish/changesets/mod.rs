use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::{FileWalker, PathExtensions};

use crate::toolchains::napi::{NapiConfig, NapiResolver};

/// This repository versions and releases all its artifacts together, generating the same changelog.
/// Unfortunately, changesets does not support combining changelogs from multiple packages into one.
///
/// So, we let changesets bump the version of the single NPM package we ship, and generate its changelog.
/// Then our build process copies the new version and the single changelog to other packages and crates.
///
/// Additionally, changesets can only bump versions of packages in the root workspace.
/// However, NAPI platform-specific packages cannot be added to the workspace, because other platforms will fail "npm install".
/// So we have to bump the versions over ourselves anyways.
pub fn publish_changesets() -> Result<()> {
    let resolver = NapiResolver::Solidity;
    let package_dir = resolver.main_package_dir();

    let package_version = NapiConfig::local_version(&package_dir)?;
    println!("Package version: {package_version}");

    let workspace_version = CargoWorkspace::local_version()?;
    println!("Workspace version: {workspace_version}");

    assert_eq!(
        package_version, workspace_version,
        "Package version does not match workspace version."
    );

    // This command will:
    // 1) Consume/delete any changeset files currently in "$REPO_ROOT/.changeset"
    // 2) Update the CHANGELOG.md file for the NPM package.
    // 3) Bump the version in its package.json accordingly.

    Command::new("changeset").arg("version").run()?;

    let updated_version = NapiConfig::local_version(&package_dir)?;
    println!("Updated version: {updated_version}");

    if package_version == updated_version {
        println!("No version changes. Skipping.");
        return Ok(());
    }

    // Format the updated package files:

    let package_dir = resolver.main_package_dir();
    Command::new("prettier")
        .property("--write", package_dir.unwrap_str())
        .run()?;

    // Update NPM lock file:

    Command::new("npm")
        .arg("install")
        .flag("--package-lock-only")
        .run()?;

    // Update Cargo workspace:

    println!("Updating Cargo workspace version.");
    CargoWorkspace::update_version(&package_version, &updated_version)?;

    // Update Cargo lock file:

    Command::new("cargo")
        .arg("update")
        .flag("--workspace")
        .run()?;

    // Update other CHANGELOG files:

    let source_changelog = package_dir.join("CHANGELOG.md");

    for destination_changelog in FileWalker::from_repo_root().find(["**/CHANGELOG.md"])? {
        if source_changelog != destination_changelog {
            println!("Updating: {destination_changelog:?}");
            std::fs::copy(&source_changelog, &destination_changelog)?;
        }
    }

    println!();
    println!("Source files are now updated.");
    println!("The calling CI workflow will now publish them in a pull request.");
    println!();

    Ok(())
}
