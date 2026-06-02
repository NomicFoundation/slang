use std::fs;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;

use crate::commands::publish::artifacts;
use crate::toolchains::npm::Npm;
use crate::toolchains::wasm::{WasmPackage, NPM_CRATE};

/// Build the npm publish artifacts into [`artifacts::DIR`] (or write a SKIPPED
/// marker when the local version is already on npm), and export the decision +
/// artifact names as GitHub Actions outputs for the review/publish jobs.
#[derive(Clone, Debug, Parser)]
pub struct PrepareController {}

impl PrepareController {
    #[allow(clippy::unused_self)] // for compatibility with other controllers:
    pub fn execute(&self) -> Result<()> {
        let staging = artifacts::dir();
        if staging.exists() {
            fs::remove_dir_all(&staging)?;
        }
        fs::create_dir_all(&staging)?;

        // The uploaded artifact is named the same whether or not we pack, so the
        // review/publish jobs can reference it before knowing the publish decision.
        GitHub::set_output("artifact-name", &artifacts::workflow_artifact_name()?)?;

        let package_dir = CargoWorkspace::locate_source_crate(NPM_CRATE)?;
        let local = Npm::local_version(&package_dir)?;
        let published = Npm::published_version(&package_dir).ok();

        if published.as_ref() == Some(&local) {
            println!("npm {local} already published; writing SKIPPED marker.");
            fs::write(artifacts::skipped_marker(), "")?;
            // No tarball-name output: its absence is the downstream skip signal.
            return Ok(());
        }

        // `WasmPackage::build` writes the WASM into the npm source tree, so the
        // `pnpm pack` below picks it up.
        WasmPackage::build()?;

        let output = Command::new("pnpm")
            .arg("pack")
            .current_dir(&package_dir)
            .property("--pack-destination", staging.unwrap_str())
            .evaluate()?;

        // pnpm prints the tarball path as the final line; record just the filename
        // so the publish job can pass the exact path back via `--tarball`.
        let tarball_name = output
            .lines()
            .last()
            .map(|line| line.rsplit('/').next().unwrap_or(line).trim())
            .unwrap_or_default();

        GitHub::set_output("tarball-name", tarball_name)?;

        Ok(())
    }
}
