use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::commands::Command;
use serde::Deserialize;

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cases {
    VsSolc,
    All,
}

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    /// Folder where contracts are stored
    input_folder: String,

    #[arg(short, long, default_value_t = String::from(".*"))]
    pattern: String,

    #[arg(short, long, value_enum, default_value_t = Cases::All)]
    cases: Cases,

    #[arg(trailing_var_arg = true)]
    extra_args: Vec<String>, // Collects all arguments after `--`
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[allow(dead_code)]
    pub name: String,
    pub cold_slang: f64,
    pub cold_solc: f64,
    pub cold_ratio: f64,
    pub cold_heap: i64,
    pub cold_external: i64,
    pub hot_slang: f64,
    pub hot_solc: f64,
    pub hot_ratio: f64,
    pub hot_heap: i64,
    pub hot_external: i64,
}

impl NpmController {
    fn install_deps() {
        let command = Command::new("npm").arg("install").arg("tsx");
        command.run();
    }

    fn compare_with_solc(&self) -> Result<()> {
        let input_path = Path::new(&self.input_folder);

        let mut results = vec![];

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
                    .rsplit_once(':')
                    .map(|(before_last_colon, _)| before_last_colon)
                    .ok_or_else(|| anyhow::anyhow!("fullyQualifiedName is not well formatted"))?;

                // Skip the iteration if the compilation_file does not match the pattern
                if !regex::Regex::new(&self.pattern)?
                    .is_match(path.join(fully_qualified_name).to_string_lossy().as_ref())
                {
                    continue;
                }

                let compiler_version = json
                    .get("compilerVersion")
                    .and_then(|f| f.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Missing compilerVersion in {compilation_file:?}")
                    })?
                    .split('+')
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("compilerVersion is not well formatted"))?;

                let result =
                    self.execute_comparison(compiler_version, &path, fully_qualified_name)?;
                results.push(result);
            }
        }
        Self::add_summary(&mut results);
        println!("{results:?}");
        Ok(())
    }

    fn add_summary(results: &mut Vec<Output>) {
        let len = i64::try_from(results.len()).unwrap();
        if len > 1 {
            #[allow(clippy::cast_precision_loss)]
            let len_f = len as f64;
            let summary = Output {
                name: "Summary".to_string(),
                cold_slang: results.iter().map(|r| r.cold_slang).sum::<f64>() / len_f,
                cold_solc: results.iter().map(|r| r.cold_solc).sum::<f64>() / len_f,
                cold_ratio: results.iter().map(|r| r.cold_ratio).sum::<f64>() / len_f,
                cold_heap: results.iter().map(|r| r.cold_heap).sum::<i64>() / len,
                cold_external: results.iter().map(|r| r.cold_external).sum::<i64>() / len,
                hot_slang: results.iter().map(|r| r.hot_slang).sum::<f64>() / len_f,
                hot_solc: results.iter().map(|r| r.hot_solc).sum::<f64>() / len_f,
                hot_ratio: results.iter().map(|r| r.hot_ratio).sum::<f64>() / len_f,
                hot_heap: results.iter().map(|r| r.hot_heap).sum::<i64>() / len,
                hot_external: results.iter().map(|r| r.hot_external).sum::<i64>() / len,
            };

            results.push(summary);
        }
    }

    fn execute_comparison(
        &self,
        compiler_version: &str,
        path: &Path,
        fully_qualified_name: &str,
    ) -> Result<Output, anyhow::Error> {
        let command = Command::new("npx")
            .arg("tsx")
            .flag("--trace-uncaught")
            .flag("--expose-gc")
            .arg("crates/solidity/testing/perf/npm/src/slang.vs.solc.mts")
            .property("--version", compiler_version)
            .property("--dir", path.to_string_lossy())
            .property("--file", fully_qualified_name)
            .args(&self.extra_args);
        let result = command.evaluate()?;

        let output: Output = serde_json::from_str(&result)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON output: {}", e))?;

        Ok(output)
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        Self::install_deps();

        // if self.cases == Cases::VsSolc || self.cases == Cases::All {
        self.compare_with_solc()
    }
}
