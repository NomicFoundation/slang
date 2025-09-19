use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::toolchains::npm::Npm;
use crate::toolchains::wasm::{WasmPackage, NPM_CRATE};
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl NpmController {
    pub fn execute(&self) -> Result<()> {
        let package_dir = CargoWorkspace::locate_source_crate(NPM_CRATE)?;

        WasmPackage::build()?;

        println!("Publishing: {package_dir:?}");

        let local_version = Npm::local_version(&package_dir)?;
        println!("Local version: {local_version}");

        let published_version = Npm::published_version(&package_dir)?;
        println!("Published version: {published_version}");

        if local_version == published_version {
            println!("Skipping package, since the local version is already published.");
            return Ok(());
        }

        let mut command = Command::new("npm")
            .args(["publish", package_dir.unwrap_str()])
            .property("--access", "public");

        if self.dry_run.get() {
            command = command.flag("--dry-run");
        }

        command.run();

        Ok(())
    }
}
