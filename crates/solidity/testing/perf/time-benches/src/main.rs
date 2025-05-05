use core::fmt;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::commands::Command;
use infra_utils::config;
use itertools::Itertools;
use serde::Deserialize;
use serde_json::json;

#[derive(Clone, Debug)]
pub enum Options {
    Parse,
    File,
    Project,
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Options::Parse => "parse",
            Options::File => "file",
            Options::Project => "project",
        };
        write!(f, "{string}")
    }
}

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    /// Folder where contracts are stored
    input_folder: String,

    #[arg(short, long, default_value_t = String::from(".*"))]
    pattern: String,
    #[arg(trailing_var_arg = true)]
    extra_args: Vec<String>, // Collects all arguments after `--`
}

#[derive(Debug, Deserialize)]
pub struct Measure {
    #[allow(dead_code)]
    pub name: String,
    pub timings: Vec<Timing>,
}

#[derive(Debug, Deserialize)]
pub struct Timing {
    pub component: String,
    pub time: f64,
}

impl NpmController {
    fn ts_comparisons(&self) -> Result<()> {
        let config = config::read_config()?;
        let input_path = Path::new(&self.input_folder);

        let mut results = vec![];

        for project in config.projects {
            let path = input_path.join(project.hash);

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
                .ok_or_else(|| anyhow::anyhow!("Missing compilerVersion in {compilation_file:?}"))?
                .split('+')
                .next()
                .ok_or_else(|| anyhow::anyhow!("compilerVersion is not well formatted"))?;

            let result = self.execute_comparison(
                compiler_version,
                &path,
                fully_qualified_name,
                Options::Project,
            )?;
            results.push(result);
        }
        let summary = Self::add_summary(&results);
        if let Some(summary) = summary {
            results.push(summary);
        }
        publish(&results)?;
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
        options: Options,
    ) -> Result<Measure, anyhow::Error> {
        let command = Command::new("npx")
            .arg("tsx")
            .flag("--trace-uncaught")
            .flag("--expose-gc")
            .arg("crates/solidity/testing/perf/npm/src/comparisons.mts")
            .property("--version", compiler_version)
            .property("--dir", path.to_string_lossy())
            .property("--file", fully_qualified_name)
            .property("--options", options.to_string())
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
}

fn publish(results: &[Measure]) -> Result<()> {
    let mut benchmark_map = serde_json::Map::new();

    for result in results {
        let measures: serde_json::Value = result
            .timings
            .iter()
            .map(|timing| {
                json!({timing.component.clone(): {
                    "value": timing.time
                }})
            })
            .collect();

        benchmark_map.insert(result.name.clone(), measures);
    }

    println!("{}", serde_json::to_string_pretty(&benchmark_map)?);
    Ok(())
}

fn main() -> Result<()> {
    NpmController::parse().ts_comparisons()
}
