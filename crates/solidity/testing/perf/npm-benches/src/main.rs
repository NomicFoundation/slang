use std::io::Write;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::config::{self, File, Project};
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

use crate::fetch::fetch;

mod fetch;

#[derive(Clone, Copy, Debug, AsRefStr, EnumIter)]
pub enum SubjectUT {
    SlangProject, // resolve bindings of the entire project instead of just the main file, see the options in the npm counterpart
    Antlr,
    Solc,
}

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[arg(short, long, default_value_t = String::from(".*"))]
    /// A regex pattern to select which project(s) to run
    pattern: String,

    #[arg(long, default_value_t = 2)]
    /// The number of cold runs
    cold: usize,

    #[arg(long, default_value_t = 5)]
    /// The number of hot runs
    hot: usize,
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

    fn run_benchmarks(&self, path: &Path, file: Option<&str>) -> Result<Vec<Timing>> {
        let mut results = vec![];
        for sut in SubjectUT::iter() {
            let mut sut_result = self.run(path, file, sut)?;
            results.append(&mut sut_result);
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

            fetch(&file.hash, input_path)?;

            let path = input_path.join(&file.hash);

            let mut result = self.run_benchmarks(&path, Some(&file.file))?;
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

            if !pattern_regex.is_match(&project.name) {
                continue;
            }

            fetch(&project.hash, input_path)?;

            let mut result = self.run_benchmarks(&path, None)?;
            results.append(&mut result);
        }
        Ok(results)
    }

    fn run(
        &self,
        path: &Path,
        file: Option<&str>,
        sut: SubjectUT,
    ) -> Result<Vec<Timing>, anyhow::Error> {
        let perf_crate = CargoWorkspace::locate_source_crate("solidity_testing_perf")?;
        let mut command = Command::new("npx")
            .arg("tsx")
            .flag("--trace-uncaught")
            .flag("--expose-gc")
            .arg(perf_crate.join("npm/src/main.mts").to_str().unwrap())
            .property("--dir", path.to_string_lossy());

        if let Some(file) = file {
            command = command.property("--file", file);
        }

        command = command
            .property("--runner", sut.as_ref())
            .property("--cold", self.cold.to_string())
            .property("--hot", self.hot.to_string());

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
