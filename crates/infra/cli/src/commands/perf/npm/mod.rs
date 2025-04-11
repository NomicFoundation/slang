use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::commands::Command;

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cases {
    VsSolc,
    All,
}

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    /// Folder where contracts are stored
    input_folder: String,

    #[arg(value_enum, default_value_t = Cases::All)]
    cases: Cases,

    #[arg(trailing_var_arg = true)]
    extra_args: Vec<String>, // Collects all arguments after `--`
}

impl NpmController {
    fn install_deps() {
        let command = Command::new("npm").arg("install").arg("tsx");
        command.run();
    }

    fn compare_with_solc(&self) -> Result<()> {
        let input_path = Path::new(&self.input_folder);

        for entry in fs::read_dir(input_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let compilation_file = path.join("compilation.json");

                if !compilation_file.exists() {
                    return Err(anyhow::anyhow!(
                        "Missing compilation.json in folder: {:?}",
                        path
                    ));
                }

                let content = fs::read_to_string(&compilation_file)?;
                let json: serde_json::Value = serde_json::from_str(&content)?;

                let fully_qualified_name = json
                    .get("fullyQualifiedName")
                    .and_then(|f| f.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "Missing fullyQualifiedName field in file: {compilation_file:?}"
                        )
                    })?
                    .split(':')
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("fullyQualifiedName is not well formatted"))?;

                let compiler_version = json
                    .get("compilerVersion")
                    .and_then(|f| f.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Missing compilerVersion in {compilation_file:?}")
                    })?
                    .split('+')
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("compilerVersion is not well formatted"))?;

                let command = Command::new("npx")
                    .arg("tsx")
                    .flag("--trace-uncaught")
                    .arg("crates/solidity/testing/perf/npm/src/slang.vs.solc.mts")
                    .property("--version", compiler_version)
                    .property("--dir", path.to_string_lossy())
                    .property("--file", fully_qualified_name)
                    .args(&self.extra_args);

                command.run();
            }
        }

        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        Self::install_deps();

        // if self.cases == Cases::VsSolc || self.cases == Cases::All {
        self.compare_with_solc()
    }
}
