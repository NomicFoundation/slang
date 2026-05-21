use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::toolchains::npm::Npm;
use crate::toolchains::wasm::{WasmPackage, NPM_CRATE};

#[derive(Clone, Debug, Parser)]
pub struct PrepareController {}

impl PrepareController {
    // No flags; `&self` required by the `PublishCommand::execute` dispatch.
    #[allow(clippy::unused_self)]
    pub fn execute(&self) -> Result<()> {
        let root = Path::repo_path("target/publish-artifacts");
        if root.exists() {
            fs::remove_dir_all(&root)
                .with_context(|| format!("Failed to clean staging dir: {root:?}"))?;
        }
        fs::create_dir_all(&root)?;

        let package_dir = CargoWorkspace::locate_source_crate(NPM_CRATE)?;
        let local_version = Npm::local_version(&package_dir)?;
        if let Ok(published_version) = Npm::published_version(&package_dir) {
            println!("Published npm version: {published_version}");
            println!("Local npm version: {local_version}");
            if published_version == local_version {
                println!("npm {local_version} already published; writing .skipped marker.");
                // Marker keeps `if-no-files-found: error` on the artifact upload satisfied
                // without producing a tarball that nothing consumes.
                fs::write(root.join(".skipped"), "")?;
                return Ok(());
            }
        }

        // Generates the WASM payload + transpiled JS into the package source tree
        // so `pnpm pack` below sees the fully-built package.
        WasmPackage::build()?;

        Command::new("pnpm")
            .arg("pack")
            .property("--pack-destination", root.unwrap_str())
            .current_dir(&package_dir)
            .run();
        Ok(())
    }
}
