use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::commands::Command;
use infra_utils::config::{self, File, Project};
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

#[derive(Clone, Copy, Debug, AsRefStr, EnumIter)]
pub enum Runner {
    SlangProject, // resolve bindings of the entire project instead of just the main file, see the options in the npm counterpart
    SolidityParser,
    Solc,
}

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[arg(short, long, default_value_t = String::from(".*"))]
    pattern: String,
    #[arg(trailing_var_arg = true)]
    extra_args: Vec<String>, // Collects all arguments after `--`
}

#[derive(Debug, Deserialize)]
pub struct Timing {
    pub component: String,
    pub time: f64,
}

impl NpmController {
    fn execute(&self) -> Result<()> {
        let config = config::read_config()?;
        let input_path = Path::new(&config::WORKING_DIR);
        let file_benchmarks = self.individual_file_benchmarks(input_path, &config.files)?;
        let project_benchmarks = self.project_benchmarks(input_path, &config.projects)?;
        publish(file_benchmarks.iter().chain(project_benchmarks.iter()))
    }

    fn run_benchmarks(&self, version: &str, path: &Path, file: &str) -> Result<Vec<Timing>> {
        let mut results = vec![];
        for runner in Runner::iter() {
            let mut runner_result = self.run(version, path, file, runner)?;
            results.append(&mut runner_result);
        }

        Ok(results)
    }

    fn individual_file_benchmarks(
        &self,
        input_path: &Path,
        files: &Vec<File>,
    ) -> Result<Vec<Timing>> {
        let mut results = vec![];

        let pattern_regex = regex::Regex::new(&self.pattern)?;
        for file in files {
            if !pattern_regex.is_match(&file.file) {
                continue;
            }

            let path = input_path.join(&file.hash);

            let (compiler_version, _) = Self::compilation_options(&path)?;

            let mut result = self.run_benchmarks(&compiler_version, &path, &file.file)?;
            results.append(&mut result);
        }
        Ok(results)
    }

    fn project_benchmarks(
        &self,
        input_path: &Path,
        projects: &Vec<Project>,
    ) -> Result<Vec<Timing>> {
        let mut results = vec![];
        let pattern_regex = regex::Regex::new(&self.pattern)?;

        for project in projects {
            let path = input_path.join(&project.hash);

            let (compiler_version, fully_qualified_name) = Self::compilation_options(&path)?;

            if !pattern_regex.is_match(path.join(&fully_qualified_name).to_string_lossy().as_ref())
            {
                continue;
            }

            let mut result =
                self.run_benchmarks(&compiler_version, &path, &fully_qualified_name)?;
            results.append(&mut result);
        }
        Ok(results)
    }

    fn compilation_options(path: &Path) -> Result<(String, String)> {
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
                anyhow::anyhow!("Missing fullyQualifiedName field in file: {compilation_file:?}")
            })?
            .rsplit_once(':')
            .map(|(before_last_colon, _)| before_last_colon)
            .ok_or_else(|| anyhow::anyhow!("fullyQualifiedName is not well formatted"))?;

        let compiler_version = json
            .get("compilerVersion")
            .and_then(|f| f.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing compilerVersion in {compilation_file:?}"))?;

        Ok((compiler_version.into(), fully_qualified_name.into()))
    }

    fn run(
        &self,
        compiler_version: &str,
        path: &Path,
        fully_qualified_name: &str,
        runner: Runner,
    ) -> Result<Vec<Timing>, anyhow::Error> {
        let command = Command::new("npx")
            .arg("tsx")
            .flag("--trace-uncaught")
            .flag("--expose-gc")
            .arg("crates/solidity/testing/perf/npm/src/main.mts")
            .property("--version", compiler_version)
            .property("--dir", path.to_string_lossy())
            .property("--file", fully_qualified_name)
            .property("--runner", runner.as_ref())
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

fn publish<'a>(results: impl Iterator<Item = &'a Timing>) -> Result<()> {
    let mut output = std::io::stdout();
    writeln!(output, "{{")?;
    writeln!(output, "\t\"npm_benchmarks\": {{")?;

    let mut buffer = None;
    for timing in results {
        if let Some(value) = buffer {
            writeln!(output, "{value}")?;
        }
        writeln!(output, "\t\t\"{}\": {{", timing.component)?;
        writeln!(output, "\t\t\t\"value\": {}", timing.time)?;
        write!(output, "\t\t}}")?;

        buffer = Some(",");
    }

    writeln!(output, "\n\t}}")?;
    writeln!(output, "}}")?;
    Ok(())
}

fn main() -> Result<()> {
    NpmController::parse().execute()
}
