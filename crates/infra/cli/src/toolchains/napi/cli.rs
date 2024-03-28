use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;
use semver::Version;

use crate::toolchains::napi::resolver::NapiResolver;
use crate::toolchains::napi::NapiConfig;

pub enum BuildTarget {
    Debug,
    ReleaseTarget(String),
}

pub struct NapiCliOutput {
    /// `index.d.ts` and `index.js`
    pub source_files: Vec<PathBuf>,
    /// `index.TARGET_NAME.node`
    pub node_binary: PathBuf,
}

pub struct NapiCli;

impl NapiCli {
    pub fn build(
        resolver: &NapiResolver,
        output_dir: impl AsRef<Path>,
        target: &BuildTarget,
    ) -> Result<NapiCliOutput> {
        let output_dir = output_dir.as_ref();
        let package_dir = resolver.main_package_dir();
        let crate_dir = resolver.crate_dir();

        let mut command = Command::new("napi");

        {
            // Note: NAPI expects all arguments to be relative to the current directory.
            let output_dir = output_dir.strip_repo_root()?;
            let package_dir = package_dir.strip_repo_root()?;
            let crate_dir = crate_dir.strip_repo_root()?;

            command = command
                .args(["build", output_dir.unwrap_str()])
                .property("--config", package_dir.join("package.json").unwrap_str())
                .property("--cargo-cwd", crate_dir.unwrap_str());
        }

        command = command
            // Add platform triple to the binary file name. Example: "index.linux-x64-gnu.node"
            .flag("--platform")
            // Generate string enums, for serialization and debugging:
            .flag("--no-const-enum");

        match target {
            BuildTarget::Debug => {
                // Default
            }
            BuildTarget::ReleaseTarget(target) => {
                command = command.flag("--release").property("--target", target);
            }
        };

        command.run()?;

        #[cfg(target_env = "gnu")]
        ensure_correct_glibc_for_vscode(resolver, output_dir, target)?;

        let mut source_files = vec![];
        let mut node_binary = None;

        for child in output_dir.collect_children()? {
            let file_name = child.unwrap_name();

            // NAPI emits files with lowercase names.
            #[allow(clippy::case_sensitive_file_extension_comparisons)]
            match file_name {
                "index.js" | "index.d.ts" => {
                    source_files.push(output_dir.join(file_name));
                }
                file if file.ends_with(".node") && node_binary.is_none() => {
                    node_binary = Some(output_dir.join(file));
                }
                _ => {
                    bail!("Unexpected file {file_name:?} in {output_dir:?}");
                }
            };
        }

        assert_eq!(
            source_files.len(),
            2,
            "Missing source files in {output_dir:?}",
        );

        let node_binary =
            node_binary.with_context(|| format!("No .node file in {output_dir:?}"))?;

        Ok(NapiCliOutput {
            source_files,
            node_binary,
        })
    }

    pub fn prepublish(resolver: &NapiResolver) -> Result<()> {
        let package_dir = resolver.main_package_dir();
        let platforms_dir = resolver.platforms_dir();

        // Note: NAPI expects all arguments to be relative to the current directory.
        let package_dir = package_dir.strip_repo_root()?;
        let platforms_dir = platforms_dir.strip_repo_root()?;

        return Command::new("napi")
            .arg("prepublish")
            .flag("--skip-gh-release")
            .property("--config", package_dir.join("package.json").unwrap_str())
            .property("--prefix", platforms_dir.unwrap_str())
            .env("npm_config_dry_run", "true")
            .run();
    }
}

#[cfg(target_env = "gnu")]
/// On a GNU host, cross-compile the native addon to only target the oldest supported GLIBC version by VS Code.
///
/// By default, compiling on the host targets the host's GLIBC version, which is usually newer.
/// To prevent that, we need to explicitly cross-compile for the desired GLIBC version.
///
/// This is necessary to retain extension compatibility with as many systems as possible:
/// <https://code.visualstudio.com/docs/supporting/requirements#_additional-linux-requirements>.
fn ensure_correct_glibc_for_vscode(
    resolver: &NapiResolver,
    output_dir: &Path,
    target: &BuildTarget,
) -> Result<()> {
    let target_triple = match target {
        BuildTarget::ReleaseTarget(target) if target.ends_with("-linux-gnu") => target,
        _ => return Ok(()),
    };

    let min_glibc = NapiConfig::target_glibc(resolver)?;

    let output_artifact = match target_triple.split('-').next() {
        Some("x86_64") => "index.linux-x64-gnu.node",
        Some("aarch64") => "index.linux-arm64-gnu.node",
        _ => bail!("Unsupported target {target_triple} for `cargo-zigbuild`."),
    };
    let output_artifact_path = output_dir.join(output_artifact);

    let is_host_compiling = target_triple.starts_with(std::env::consts::ARCH);
    if is_host_compiling {
        let rust_crate_name = resolver.rust_crate_name();

        // Don't clobber the existing output directory.
        let zigbuild_output = tempfile::tempdir()?;

        // Until `@napi-rs/cli` v3 is released with a fixed `zig` support and a new `--cross-compile`,
        // we explicitly compile ourselves again with `cargo-zigbuild` to target the desired GLIBC
        // version, without having to separately compile on the target platform (e.g. via Docker).
        Command::new("cargo")
            .arg("zigbuild")
            .property("-p", rust_crate_name)
            .flag("--release")
            .property("--target", format!("{target_triple}.{min_glibc}"))
            .property("--target-dir", zigbuild_output.path().to_string_lossy())
            .run()?;

        // Overwrite the existing artifact with the cross-compiled one.
        let zigbuild_output = zigbuild_output.into_path();
        let artifact_path = zigbuild_output
            .join(target_triple)
            .join("release")
            .join(format!("lib{rust_crate_name}.so"));

        std::fs::copy(artifact_path, &output_artifact_path)?;
    } else {
        // Already cross-compiled with the correct GLIBC version. Just verify for sanity.
    }

    // Verify that the artifact is compatible with the desired GLIBC version.
    let library_glibc_version = Command::new("scripts/min_glibc_version.sh")
        .arg(output_artifact_path.to_string_lossy())
        .evaluate()?;

    if lenient_semver(&library_glibc_version)? > lenient_semver(&min_glibc)? {
        bail!("The compiled artifact {output_artifact_path:?} targets GLIBC {library_glibc_version}, which is higher than the minimum specified version {min_glibc}.");
    }

    Ok(())
}

/// Like `Version::parse`, but allows for a missing patch version, defaulting to `0`.
fn lenient_semver(value: &str) -> Result<Version> {
    let components: Vec<_> = value
        .trim()
        .split('.')
        .map(|part| part.parse::<u64>().map_err(Into::into))
        .collect::<Result<_>>()?;

    match &components[..] {
        [major, minor] => Ok(Version::new(*major, *minor, 0)),
        [major, minor, patch] => Ok(Version::new(*major, *minor, *patch)),
        _ => bail!("Invalid semver version components: {components:?}"),
    }
}
