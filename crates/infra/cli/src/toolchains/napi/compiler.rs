use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::Codegen;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;
use serde::Serialize;

use crate::toolchains::napi::cli::{BuildTarget, NapiCli, NapiCliOutput};
use crate::toolchains::napi::config::NapiConfig;
use crate::toolchains::napi::resolver::{NapiPackageKind, NapiResolver};

#[derive(Clone, Copy)]
pub enum NapiProfile {
    /// Build only the main package for local development.
    Debug,
    /// Build all packages (all platforms) for publishing to NPM.
    Release,
}

pub struct NapiCompiler;

impl NapiCompiler {
    pub fn run(resolver: &NapiResolver, profile: NapiProfile) -> Result<()> {
        match profile {
            NapiProfile::Debug => {
                // Compiles the default target for local development
                let napi_output = compile_target(resolver, &BuildTarget::Debug)?;

                // Process its generated JavaScript and TypeScript files, and copy any new changes to the source folder.
                process_generated_files(resolver, &napi_output)?;

                // Compile the main cross-platform package, and copy the build node binary to it for debugging/testing.
                compile_root_package(resolver, Some(&napi_output.node_binary))?;
            }
            NapiProfile::Release => {
                // Compile all available targets for publishing to NPM.
                let node_binaries = compile_all_targets(resolver)?;

                // Compile the main cross-platform package, but without any binaries.
                compile_root_package(resolver, None)?;

                // Compile all platform-specific packages, and copy the built node binaries to them.
                compile_platform_packages(resolver, &node_binaries)?;
            }
        }

        Ok(())
    }
}

fn compile_all_targets(resolver: &NapiResolver) -> Result<Vec<PathBuf>> {
    let targets = NapiConfig::list_all_targets(resolver)?;

    assert_ne!(
        targets.len(),
        0,
        "No release targets found. Is this a publicly released package?"
    );

    Command::new("rustup")
        .args(["target", "add"])
        .args(&targets)
        .run()?;

    // Needed for cross-compiling windows targets:
    CargoWorkspace::install_binary("cargo-xwin")?;
    // Needed to reliably target older GBLIC on `-linux-gnu` targets when host-compiling:
    CargoWorkspace::install_binary("cargo-zigbuild")?;

    let mut node_binaries = vec![];

    for target in targets {
        let output = compile_target(resolver, &BuildTarget::ReleaseTarget(target))?;

        node_binaries.push(output.node_binary);
    }

    Ok(node_binaries)
}

fn compile_target(resolver: &NapiResolver, target: &BuildTarget) -> Result<NapiCliOutput> {
    let output_dir = resolver.napi_output_dir(target);

    std::fs::create_dir_all(&output_dir)?;

    NapiCli::build(resolver, output_dir, target)
}

#[derive(Serialize)]
struct LicenseHeaderTemplate {
    contents: String,
}

fn process_generated_files(resolver: &NapiResolver, napi_output: &NapiCliOutput) -> Result<()> {
    let templates_dir = resolver.templates_dir();

    let mut codegen = Codegen::read_write(&templates_dir)?;

    for source in &napi_output.source_files {
        let file_name = source.unwrap_name();
        let contents = source.read_to_string()?;

        let destination_path = resolver.generated_dir().join(file_name);
        let template_path = templates_dir.join(format!("{file_name}.jinja2"));

        codegen.render(
            LicenseHeaderTemplate { contents },
            &template_path,
            destination_path,
        )?;
    }

    Ok(())
}

fn compile_root_package(resolver: &NapiResolver, node_binary: Option<&Path>) -> Result<()> {
    let package_dir = resolver.main_package_dir();
    let output_dir = resolver.npm_output_dir(&NapiPackageKind::Main);

    std::fs::create_dir_all(&output_dir)?;

    Command::new("tsc")
        .property("--project", package_dir.join("tsconfig.json").unwrap_str())
        .property("--outDir", output_dir.unwrap_str())
        .property("--declaration", "true")
        .property("--noEmit", "false")
        .run()?;

    for file_name in &["package.json", "CHANGELOG.md", "LICENSE", "README.md"] {
        let source = package_dir.join(file_name);
        let destination = output_dir.join(file_name);

        std::fs::copy(source, destination)?;
    }

    let generated_dir = resolver.generated_dir();
    let generated_output_dir = resolver.generated_output_dir();

    std::fs::create_dir_all(&generated_output_dir)?;

    for child in generated_dir.collect_children()? {
        let destination = generated_output_dir.join(child.unwrap_name());

        std::fs::copy(child, destination)?;
    }

    if let Some(node_binary) = node_binary {
        let destination = generated_output_dir.join(node_binary.unwrap_name());

        std::fs::copy(node_binary, destination)?;
    }

    Ok(())
}

fn compile_platform_packages(resolver: &NapiResolver, node_binaries: &[PathBuf]) -> Result<()> {
    for platform_dir in resolver.platforms_dir().collect_children()? {
        let platform = platform_dir.unwrap_name();
        let package_kind = NapiPackageKind::Platform(platform.to_owned());
        let output_dir = resolver.npm_output_dir(&package_kind);

        std::fs::create_dir_all(&output_dir)?;

        for file in platform_dir.collect_children()? {
            std::fs::copy(&file, output_dir.join(file.unwrap_name()))?;
        }

        let node_binary = node_binaries
            .iter()
            .find(|node_binary| node_binary.unwrap_name() == format!("index.{platform}.node"))
            .expect("Could not find node binary for platform.");

        std::fs::copy(node_binary, output_dir.join(node_binary.unwrap_name()))?;
    }

    Ok(())
}
