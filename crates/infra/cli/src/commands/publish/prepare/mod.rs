use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate};
use infra_utils::commands::Command;
use infra_utils::hash::sha256_hex_of_file;
use infra_utils::paths::PathExtensions;
use strum::IntoEnumIterator;

use crate::commands::publish::artifacts::{ArtifactPaths, CargoArtifact, Manifest, NpmArtifact};
use crate::commands::publish::cargo::metadata::build_new_crate;
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
        fs::create_dir_all(ArtifactPaths::cargo_dir())?;

        let workspace_version = CargoWorkspace::local_version()?.to_string();

        let npm = prepare_npm()?;
        let cargo = prepare_cargo()?;

        let manifest = Manifest {
            workspace_version,
            npm,
            cargo,
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

    // The npm package's WASM payload + transpiled JS is built into the package source tree.
    // Re-running `WasmPackage::build()` here keeps the build step in prepare; publish never
    // touches the toolchain.
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

fn prepare_cargo() -> Result<Vec<CargoArtifact>> {
    let local_version = CargoWorkspace::local_version()?;
    let mut to_package = vec![];

    for crate_name in UserFacingV1Crate::iter() {
        let crate_name = crate_name.to_string();
        if let Ok(published) = CargoWorkspace::published_version(&crate_name) {
            println!("Published version of {crate_name}: {published}");
            if published == local_version {
                println!(
                    "Skipping crate {crate_name}: local version {local_version} matches published."
                );
                continue;
            }
        } else {
            println!("No published version found for crate {crate_name}.");
        }
        to_package.push(crate_name);
    }

    if to_package.is_empty() {
        println!("No cargo crates to prepare.");
        return Ok(vec![]);
    }

    // `--no-verify` skips the verification compile (`cargo build` of the freshly tarballed
    // source) — that's the load-bearing part. Without it, every build.rs and proc-macro in
    // the dep graph runs here too; with it, prepare only resolves deps and copies files.
    // The verification we lose by skipping is the safety net for cross-crate path-dep
    // rewrites; the batched workspace `cargo publish --dry-run` in the review job is what
    // catches problems there.
    //
    // `--allow-dirty` is required because `WasmPackage::build()` ran before this step and
    // mutated generated files under `crates/solidity/outputs/npm/package/`. Those changes
    // are intentional and don't affect the source crates we're packaging, but cargo would
    // otherwise refuse to package with a dirty worktree.
    let mut command = Command::new("cargo")
        .arg("package")
        .flag("--no-verify")
        .flag("--allow-dirty");
    for name in &to_package {
        command = command.property("--package", name);
    }
    command.run();

    let package_output_dir = Path::repo_path("target/package");
    let mut artifacts = vec![];

    for crate_name in &to_package {
        let filename = format!("{crate_name}-{local_version}.crate");
        let src = package_output_dir.join(&filename);
        if !src.exists() {
            bail!("Expected packaged crate not found: {src:?}");
        }
        let dest_crate = ArtifactPaths::cargo_dir().join(&filename);
        fs::copy(&src, &dest_crate)
            .with_context(|| format!("Failed to copy {src:?} -> {dest_crate:?}"))?;
        let crate_sha256 = sha256_hex_of_file(&dest_crate)?;
        let crate_relative = dest_crate
            .strip_prefix(ArtifactPaths::root())?
            .to_path_buf();

        // Extract the rewritten Cargo.toml from the .crate so we can build the
        // registry-publish JSON. `cargo package --no-verify` doesn't leave a
        // directory around — only the .crate itself — so we read from inside it.
        let version_str = local_version.to_string();
        let normalized_manifest =
            extract_normalized_manifest(&dest_crate, crate_name, &version_str)?;
        let metadata = build_new_crate(&normalized_manifest)?;
        let metadata_filename = format!("{crate_name}-{local_version}.json");
        let dest_metadata = ArtifactPaths::cargo_dir().join(&metadata_filename);
        fs::write(&dest_metadata, serde_json::to_vec_pretty(&metadata)?)
            .with_context(|| format!("Failed to write metadata: {dest_metadata:?}"))?;
        let metadata_sha256 = sha256_hex_of_file(&dest_metadata)?;
        let metadata_relative = dest_metadata
            .strip_prefix(ArtifactPaths::root())?
            .to_path_buf();

        artifacts.push(CargoArtifact {
            crate_name: crate_name.clone(),
            version: local_version.to_string(),
            crate_path: crate_relative.unwrap_string(),
            crate_sha256,
            metadata_path: metadata_relative.unwrap_string(),
            metadata_sha256,
        });
    }

    Ok(artifacts)
}

/// Extract `<crate>-<version>/Cargo.toml` (the *rewritten* manifest cargo writes
/// alongside `Cargo.toml.orig`) from inside the `.crate` gzipped tar and write
/// it to disk. The crate file is just bytes — no code runs during extraction,
/// by design.
fn extract_normalized_manifest(
    crate_path: &Path,
    crate_name: &str,
    version: &str,
) -> Result<PathBuf> {
    let inner_path = PathBuf::from(format!("{crate_name}-{version}/Cargo.toml"));
    let extracted = ArtifactPaths::cargo_dir().join(format!("{crate_name}-{version}.Cargo.toml"));

    let file = fs::File::open(crate_path)
        .with_context(|| format!("Failed to open .crate: {crate_path:?}"))?;
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));

    for entry in archive
        .entries()
        .with_context(|| format!("Failed to read tar entries from {crate_path:?}"))?
    {
        let mut entry =
            entry.with_context(|| format!("Failed to read tar entry from {crate_path:?}"))?;
        let path = entry
            .path()
            .with_context(|| format!("Invalid path in {crate_path:?}"))?
            .into_owned();
        if path == inner_path {
            let mut contents = Vec::new();
            std::io::copy(&mut entry, &mut contents)
                .with_context(|| format!("Failed to read {inner_path:?} from {crate_path:?}"))?;
            fs::write(&extracted, contents)
                .with_context(|| format!("Failed to write extracted manifest: {extracted:?}"))?;
            return Ok(extracted);
        }
    }

    bail!("Did not find {inner_path:?} inside {crate_path:?}");
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
