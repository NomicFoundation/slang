use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::cargo::{CargoWorkspace, CargoWorkspaceCommands};
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::commands::Command;
use infra_utils::paths::{FileWalker, PathExtensions};
use strum_macros::EnumIter;

pub const WASM_TARGET: &str = "wasm32-wasip1";

#[derive(Clone, Copy, EnumIter)]
pub enum WasmPackage {
    Codegen,
    Solidity,
    Testlang,
}

impl WasmPackage {
    pub fn build(self) -> Result<()> {
        let wasm_component = self.generate_component()?;

        self.transpile_wasm(&wasm_component)?;

        self.transpile_sources()?;

        Ok(())
    }

    fn generate_component(self) -> Result<PathBuf> {
        let wasm_crate = self.wasm_crate();

        Command::new("cargo")
            .arg("build")
            .property("--target", WASM_TARGET)
            .property("--package", wasm_crate)
            .flag("--all-features")
            .flag("--release")
            .add_build_rustflags()
            .run();

        let wasm_binary =
            Path::repo_path(format!("target/{WASM_TARGET}/release/{wasm_crate}.wasm"));

        let wasm_opt_binary = Path::repo_path(format!(
            "target/{WASM_TARGET}/release/{wasm_crate}.optimized.wasm"
        ));

        Command::new("node")
            .args(["submodules/jco/src/jco.js", "opt", wasm_binary.unwrap_str()])
            .property("--output", wasm_opt_binary.unwrap_str())
            .arg("--")
            .property("--optimize-level", "4")
            .flag("--strip-debug")
            .run();

        CargoWorkspace::install_binary("wasm-tools")?;

        let wasm_adapter =
            Path::repo_path("submodules/jco/lib/wasi_snapshot_preview1.reactor.wasm");

        let wasm_component = Path::repo_path(format!(
            "target/{WASM_TARGET}/release/{wasm_crate}.component.wasm"
        ));

        Command::new("wasm-tools")
            .args(["component", "new", wasm_opt_binary.unwrap_str()])
            .property("--adapt", wasm_adapter.unwrap_str())
            .property("--output", wasm_component.unwrap_str())
            .run();

        Ok(wasm_component)
    }

    fn transpile_wasm(self, wasm_component: &Path) -> Result<()> {
        let temp_dir_handle = tempfile::tempdir()?;
        let temp_dir = temp_dir_handle.path();

        {
            let wasm_crate = self.wasm_crate();
            let jco_config = CargoWorkspace::locate_source_crate(wasm_crate)?
                .join(self.runtime_dir())
                .join("generated/config.json");

            Command::new("node")
                .args([
                    "submodules/jco/src/jco.js",
                    "transpile",
                    wasm_component.unwrap_str(),
                ])
                .property("--configuration-file", jco_config.unwrap_str())
                .property("--out-dir", temp_dir.unwrap_str())
                .property("--base64-cutoff", "0") // disable inlining core Wasm binaries as base64
                .flag("--no-namespaced-exports") // disable namespaced exports for typescript compatibility
                .flag("--valid-lifting-optimization") // optimize component binary validations assuming all lifted values are valid
                .run();
        }

        let npm_crate = self.npm_crate();
        let output_dir = CargoWorkspace::locate_source_crate(npm_crate)?.join("wasm/generated");

        let mut fs = CodegenFileSystem::new(&output_dir)?;

        for temp_path in FileWalker::from_directory(temp_dir).find_all()? {
            let output_path = temp_path.replace_prefix(temp_dir, &output_dir);

            match temp_path.unwrap_ext() {
                "ts" => {
                    // Copy definition files as-is:
                    let contents = temp_path.read_to_string()?;
                    fs.write_file(output_path, contents)?;
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

    fn transpile_sources(self) -> Result<()> {
        let npm_crate = self.npm_crate();
        let project_dir = CargoWorkspace::locate_source_crate(npm_crate)?;

        let temp_dir_handle = tempfile::tempdir()?;
        let temp_dir = temp_dir_handle.path();

        Command::new("tsc")
            .property("--project", project_dir.join("tsconfig.json").unwrap_str())
            .property("--outDir", temp_dir.unwrap_str())
            .property("--noEmit", "false")
            .run();

        let temp_dir = temp_dir.join(self.runtime_dir());
        let output_dir = project_dir.join("target/generated");

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

    pub fn wasm_crate(self) -> &'static str {
        match self {
            Self::Codegen => "codegen_runtime_cargo_wasm",
            Self::Solidity => "solidity_cargo_wasm",
            Self::Testlang => "testlang_cargo_wasm",
        }
    }

    pub fn npm_crate(self) -> &'static str {
        match self {
            Self::Codegen => "codegen_runtime_npm_package",
            Self::Solidity => "solidity_npm_package",
            Self::Testlang => "testlang_npm_package",
        }
    }

    fn runtime_dir(self) -> &'static str {
        match self {
            Self::Codegen => "src/runtime",
            Self::Solidity | Self::Testlang => "src/generated",
        }
    }
}
