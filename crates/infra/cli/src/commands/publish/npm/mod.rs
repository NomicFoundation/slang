use std::path::Path;

use anyhow::Result;
use infra_utils::{commands::Command, github::GitHub, paths::PathExtensions};

use crate::toolchains::napi::{
    NapiCompiler, NapiConfig, NapiPackageKind, NapiProfile, NapiResolver,
};

pub fn publish_npm() -> Result<()> {
    NapiCompiler::run(NapiProfile::Release)?;

    // Publish platform-specific packages first, as the main package now depends on their latest version:

    for platform_dir in NapiResolver::platforms_dir().collect_children()? {
        let platform = platform_dir.unwrap_name().to_owned();
        publish_package(&platform_dir, &NapiPackageKind::Platform(platform))?;
    }

    //  Then publish the main package, that depends on the previously published platform-specific packages:

    let package_dir = NapiResolver::main_package_dir();
    publish_package(&package_dir, &NapiPackageKind::Main)
}

fn publish_package(package_dir: &Path, kind: &NapiPackageKind) -> Result<()> {
    println!("Publishing: {package_dir:?}");

    let local_version = NapiConfig::local_version(package_dir)?;
    println!("Local version: {local_version}");

    let published_version = NapiConfig::published_version(package_dir)?;
    println!("Published version: {published_version}");

    if local_version == published_version {
        println!("Skipping package, since the local version is already published.");
        return Ok(());
    }

    let output_dir = NapiResolver::npm_output_dir(kind);

    let mut command = Command::new("npm")
        .args(["publish", output_dir.unwrap_str()])
        .property("--access", "public");

    if !GitHub::is_running_in_ci() {
        println!("Doing a dry run, since we are not running in CI.");
        command = command.flag("--dry-run");
    }

    command.run()
}
