use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::hash::sha256_hex_of_file;
use infra_utils::paths::PathExtensions;

use crate::commands::publish::artifacts::{ArtifactPaths, Manifest, NpmArtifact};
use crate::toolchains::npm::Npm;
use crate::toolchains::wasm::{WasmPackage, NPM_CRATE};

#[derive(Clone, Debug, Parser)]
pub struct PrepareController {}

impl PrepareController {
    // Empty controller — no flags, just runs the pipeline. `&self` is required by
    // the surrounding `PublishCommand::execute` dispatch, hence the allow.
    #[allow(clippy::unused_self)]
    pub fn execute(&self) -> Result<()> {
        let root = ArtifactPaths::root();
        if root.exists() {
            fs::remove_dir_all(&root)
                .with_context(|| format!("Failed to clean staging dir: {root:?}"))?;
        }
        fs::create_dir_all(ArtifactPaths::npm_dir())?;

        let workspace_version = CargoWorkspace::local_version()?.to_string();
        let npm = prepare_npm()?;

        let manifest = Manifest {
            workspace_version,
            npm,
        };
        manifest.save()?;

        println!("Wrote manifest: {:?}", ArtifactPaths::manifest_path());
        Ok(())
    }
}

fn prepare_npm() -> Result<Option<NpmArtifact>> {
    let package_dir = CargoWorkspace::locate_source_crate(NPM_CRATE)?;

    let local_version = Npm::local_version(&package_dir)?;
    if let Ok(published_version) = Npm::published_version(&package_dir) {
        println!("Published npm version: {published_version}");
        println!("Local npm version: {local_version}");
        if published_version == local_version {
            println!("Skipping npm: local version {local_version} matches published.");
            return Ok(None);
        }
    }

    // Generates the WASM payload + transpiled JS into the package source tree
    // so `pnpm pack` below sees the fully-built package.
    WasmPackage::build()?;

    let dest = ArtifactPaths::npm_dir();

    Command::new("pnpm")
        .arg("pack")
        .property("--pack-destination", dest.unwrap_str())
        .current_dir(&package_dir)
        .run();

    // pnpm pack produces exactly one .tgz in --pack-destination; find it.
    let tarball = find_single_file_with_extension(&dest, "tgz")?;
    let sha256 = sha256_hex_of_file(&tarball)?;
    let relative = tarball.strip_prefix(ArtifactPaths::root())?.to_path_buf();

    Ok(Some(NpmArtifact {
        path: relative.unwrap_string(),
        sha256,
    }))
}

fn find_single_file_with_extension(dir: &Path, extension: &str) -> Result<PathBuf> {
    let mut matches = vec![];
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read dir: {dir:?}"))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some(extension) {
            matches.push(path);
        }
    }
    match matches.len() {
        1 => Ok(matches.into_iter().next().unwrap()),
        0 => bail!("No .{extension} files found in {dir:?}"),
        n => bail!("Expected exactly one .{extension} file in {dir:?}, found {n}: {matches:?}"),
    }
}
