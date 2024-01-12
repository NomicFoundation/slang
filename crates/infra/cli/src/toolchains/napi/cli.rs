use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::toolchains::napi::resolver::NapiResolver;

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
    pub fn build(output_dir: impl AsRef<Path>, target: &BuildTarget) -> Result<NapiCliOutput> {
        let output_dir = output_dir.as_ref();
        let package_dir = NapiResolver::main_package_dir();
        let crate_dir = NapiResolver::crate_dir();

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

    pub fn prepublish() -> Result<()> {
        let package_dir = NapiResolver::main_package_dir();
        let platforms_dir = NapiResolver::platforms_dir();

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
