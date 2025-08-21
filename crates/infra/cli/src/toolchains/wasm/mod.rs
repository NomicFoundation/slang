use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::cargo::{CargoWorkspace, CargoWorkspaceCommands};
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::{FileWalker, PathExtensions};

pub const WASM_TARGET: &str = "wasm32-wasip1";
pub const WASM_CRATE: &str = "solidity_cargo_wasm";
pub const NPM_CRATE: &str = "solidity_npm_package";

pub struct WasmPackage {}

impl WasmPackage {
    pub fn build() -> Result<()> {
        let wasm_component = Self::generate_component()?;

        Self::transpile_wasm(&wasm_component)?;

        Self::transpile_sources()?;

        Ok(())
    }

    fn generate_component() -> Result<PathBuf> {
        Command::new("cargo")
            .arg("build")
            .property("--target", WASM_TARGET)
            .property("--package", WASM_CRATE)
            .flag("--all-features")
            .add_build_rustflags()
            .run();

        let profile = if GitHub::is_running_in_ci() {
            "release"
        } else {
            "debug"
        };

        let wasm_binary =
            Path::repo_path(format!("target/{WASM_TARGET}/{profile}/{WASM_CRATE}.wasm"));

        let wasm_opt_binary = Path::repo_path(format!(
            "target/{WASM_TARGET}/{profile}/{WASM_CRATE}.optimized.wasm"
        ));

        Command::new("slang-jco")
            .args(["opt", wasm_binary.unwrap_str()])
            .property("--output", wasm_opt_binary.unwrap_str())
            .arg("--")
            .property("--optimize-level", "4")
            .flag("--strip-debug")
            .run();

        CargoWorkspace::install_binary("wasm-tools")?;

        let wasm_adapter = Path::repo_path(
            "node_modules/@ignored/slang-jco/lib/wasi_snapshot_preview1.reactor.wasm",
        );

        let wasm_component = Path::repo_path(format!(
            "target/{WASM_TARGET}/{profile}/{WASM_CRATE}.component.wasm"
        ));

        Command::new("wasm-tools")
            .args(["component", "new", wasm_opt_binary.unwrap_str()])
            .property("--adapt", wasm_adapter.unwrap_str())
            .property("--output", wasm_component.unwrap_str())
            .run();

        Ok(wasm_component)
    }

    fn transpile_wasm(wasm_component: &Path) -> Result<()> {
        let temp_dir_handle = tempfile::tempdir()?;
        let temp_dir = temp_dir_handle.path();

        let runtime_dir = CargoWorkspace::locate_source_crate(WASM_CRATE)?.join("src/generated");
        let jco_config = runtime_dir.join("generated/config.json");

        {
            Command::new("slang-jco")
                .args(["transpile", wasm_component.unwrap_str()])
                .property("--configuration-file", jco_config.unwrap_str())
                .property("--out-dir", temp_dir.unwrap_str())
                .property("--base64-cutoff", "0") // disable inlining core Wasm binaries as base64
                .flag("--no-namespaced-exports") // disable namespaced exports for typescript compatibility
                .flag("--no-typescript") // disable generating .d.ts files - we'll do this with `jco types` below
                .flag("--valid-lifting-optimization") // optimize component binary validations assuming all lifted values are valid
                .run();
        }

        {
            let wit_directory = runtime_dir.join("interface/generated");
            Command::new("slang-jco")
                .args(["types", wit_directory.unwrap_str()])
                .property("--name", format!("{WASM_CRATE}.component"))
                .property("--configuration-file", jco_config.unwrap_str())
                .property("--out-dir", temp_dir.unwrap_str())
                .run();
        }

        let output_dir = CargoWorkspace::locate_source_crate(NPM_CRATE)?.join("wasm/generated");

        let mut fs = CodegenFileSystem::default();

        for temp_path in FileWalker::from_directory(temp_dir).find_all()? {
            let output_path = temp_path.replace_prefix(temp_dir, &output_dir);

            match temp_path.unwrap_ext() {
                "ts" => {
                    // Copy definition files as-is:
                    let contents = temp_path.read_to_string()?;
                    fs.write_file_formatted(output_path, contents)?;
                }

                "js" => {
                    // Disable type checking for JS, since we have no control over the generated output:
                    let mut contents = temp_path.read_to_string()?;
                    contents.insert_str(0, "// @ts-nocheck\n\n");

                    // Files git-ignored. Don't go through our codegen/formatting APIs:
                    std::fs::write(&output_path, contents)?;
                    fs.mark_generated_file(output_path)?;
                }

                "wasm" => {
                    // Files git-ignored. Don't go through our codegen/formatting APIs:
                    std::fs::copy(&temp_path, &output_path)?;
                    fs.mark_generated_file(output_path)?;
                }

                other => panic!("Unexpected file extension: {other}"),
            }
        }
        Ok(())
    }

    fn transpile_sources() -> Result<()> {
        let project_dir = CargoWorkspace::locate_source_crate(NPM_CRATE)?;

        let temp_dir_handle = tempfile::tempdir()?;
        let temp_dir = temp_dir_handle.path();

        Command::new("tsc")
            .property("--project", project_dir.join("tsconfig.json").unwrap_str())
            .property("--outDir", temp_dir.unwrap_str())
            .property("--noEmit", "false")
            .run();

        let temp_dir = temp_dir.join("src");
        let output_dir = project_dir.join("target");

        // remove any old generated files
        if output_dir.exists() {
            std::fs::remove_dir_all(&output_dir)?;
        }

        for temp_path in FileWalker::from_directory(&temp_dir).find_all()? {
            let output_path = temp_path.replace_prefix(&temp_dir, &output_dir);

            std::fs::create_dir_all(output_path.unwrap_parent())?;
            std::fs::copy(&temp_path, &output_path)?;
        }
        Ok(())
    }
}
