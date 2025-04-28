use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::commands::Command;
use itertools::Itertools;
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
pub struct Measure {
    #[allow(dead_code)]
    pub name: String,
    pub timings: Vec<Timing>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    pub component: String,
    pub time: f64,
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
        let summary = Self::add_summary(&results);
        if let Some(summary) = summary {
            results.push(summary);
        }
        println!("{results:?}");
        Ok(())
    }

    fn add_summary(results: &Vec<Measure>) -> Option<Measure> {
        let mut per_component = HashMap::new();
        for result in results {
            for timing in &result.timings {
                per_component
                    .entry(timing.component.clone())
                    .or_insert_with(Vec::new)
                    .push(timing.time);
            }
        }

        if results.len() <= 1 {
            None
        } else {
            #[allow(clippy::cast_precision_loss)]
            let summary = Measure {
                name: "Summary".to_string(),
                timings: per_component
                    .iter()
                    .map(|component| Timing {
                        component: component.0.clone(),
                        time: component.1.iter().sum::<f64>() / component.1.len() as f64,
                    })
                    .collect_vec(),
            };

            Some(summary)
        }
    }

    fn execute_comparison(
        &self,
        compiler_version: &str,
        path: &Path,
        fully_qualified_name: &str,
    ) -> Result<Measure, anyhow::Error> {
        let command = Command::new("npx")
            .arg("tsx")
            .flag("--trace-uncaught")
            .flag("--expose-gc")
            .arg("crates/solidity/testing/perf/npm/src/comparisons.mts")
            .property("--version", compiler_version)
            .property("--dir", path.to_string_lossy())
            .property("--file", fully_qualified_name)
            .args(&self.extra_args);
        let result = command.evaluate()?;

        match serde_json::from_str(&result) {
            Ok(output) => Ok(output),
            Err(e) => {
                eprintln!("Error: Can't parse output as json.\n\tOutput: {result}");
                Err(e.into())
            }
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        Self::install_deps();

        // if self.cases == Cases::VsSolc || self.cases == Cases::All {
        self.compare_with_solc()
    }
}
