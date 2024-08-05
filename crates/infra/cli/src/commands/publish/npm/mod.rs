use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::toolchains::napi::{
    NapiCompiler, NapiConfig, NapiPackageKind, NapiProfile, NapiResolver,
};
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl NpmController {
    pub fn execute(&self) -> Result<()> {
        let resolver = NapiResolver::Solidity;

        NapiCompiler::run(resolver, NapiProfile::Release)?;

        // Publish platform-specific packages first, as the main package now depends on their latest version:

        for platform_dir in resolver.platforms_dir().collect_children()? {
            let platform = platform_dir.unwrap_name().to_owned();
            publish_package(
                resolver,
                &platform_dir,
                &NapiPackageKind::Platform(platform),
                self.dry_run,
            )?;
        }

        //  Then publish the main package, that depends on the previously published platform-specific packages:

        let package_dir = resolver.main_package_dir();
        publish_package(resolver, &package_dir, &NapiPackageKind::Main, self.dry_run)
    }
}

fn publish_package(
    resolver: NapiResolver,
    package_dir: &Path,
    kind: &NapiPackageKind,
    dry_run: DryRun,
) -> Result<()> {
    println!("Publishing: {package_dir:?}");

    let local_version = NapiConfig::local_version(package_dir)?;
    println!("Local version: {local_version}");

    let published_version = NapiConfig::published_version(package_dir)?;
    println!("Published version: {published_version}");

    if local_version == published_version {
        println!("Skipping package, since the local version is already published.");
        return Ok(());
    }

    let output_dir = resolver.npm_output_dir(kind);

    let mut command = Command::new("npm")
        .args(["publish", output_dir.unwrap_str()])
        .property("--access", "public");

    if dry_run.get() {
        command = command.flag("--dry-run");
    }

    command.run()
}
