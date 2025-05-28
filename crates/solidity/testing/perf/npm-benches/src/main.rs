use std::io::Write;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

use crate::config::{File, Project};
use crate::fetch::fetch;

mod config;
mod fetch;

#[derive(Clone, Copy, Debug, AsRefStr, EnumIter)]
pub enum SubjectUT {
    SlangProject, // resolve bindings of the entire project instead of just the main file, see the options in the npm counterpart
    Antlr,
    Solc,
}

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[arg(short, long)]
    /// A regex pattern to select which project(s) to run
    pattern: Option<String>,

    #[arg(long)]
    /// A specific hash to load
    hash: Option<String>,

    #[arg(long)]
    /// If hash is specified, it is possible to optionally change the entrypoint file
    entrypoint: Option<String>,

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

        if let Some(hash) = &self.hash {
            let result =
                self.run_benchmarks(input_path, "custom", hash, self.entrypoint.as_deref())?;
            publish(result.iter())
        } else {
            let file_benchmarks = self.individual_file_benchmarks(input_path, &config.files)?;
            let project_benchmarks = self.project_benchmarks(input_path, &config.projects)?;
            publish(file_benchmarks.iter().chain(project_benchmarks.iter()))
        }
    }

    fn run_benchmarks(
        &self,
        input_path: &Path,
        name: &str,
        hash: &str,
        file: Option<&str>,
    ) -> Result<Vec<Timing>> {
        fetch(hash, input_path)?;
        let path = input_path.join(hash);

        let mut results = vec![];
        for sut in SubjectUT::iter() {
            let mut sut_result = self.run(&path, name, file, sut)?;
            results.append(&mut sut_result);
        }

        Ok(results)
    }

    fn compute_regex(&self) -> Result<regex::Regex> {
        if let Some(pattern) = &self.pattern {
            Ok(regex::Regex::new(pattern)?)
        } else {
            Ok(regex::Regex::new(".*")?)
        }
    }

    fn individual_file_benchmarks(
        &self,
        input_path: &Path,
        files: &Vec<File>,
    ) -> Result<Vec<Timing>> {
        let mut results = vec![];

        let pattern_regex = self.compute_regex()?;
        for file in files {
            if !pattern_regex.is_match(&file.name) {
                continue;
            }

            let mut result =
                self.run_benchmarks(input_path, &file.name, &file.hash, Some(&file.file))?;
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
        let pattern_regex = self.compute_regex()?;

        for project in projects {
            if !pattern_regex.is_match(&project.name) {
                continue;
            }

            let mut result = self.run_benchmarks(input_path, &project.name, &project.hash, None)?;
            results.append(&mut result);
        }
        Ok(results)
    }

    fn run(
        &self,
        path: &Path,
        name: &str,
        file: Option<&str>,
        sut: SubjectUT,
    ) -> Result<Vec<Timing>, anyhow::Error> {
        let perf_crate = CargoWorkspace::locate_source_crate("solidity_testing_perf")?;
        let mut command = Command::new("npx")
            .arg("tsx")
            .flag("--trace-uncaught")
            .flag("--expose-gc")
            .arg(perf_crate.join("npm/src/main.mts").to_str().unwrap())
            .property("--dir", path.to_string_lossy())
            .property("--name", name);

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
    // Outputs the timings in Bencher Metrics Format (https://bencher.dev/docs/reference/bencher-metric-format/)
    // In particular, we can't use serde_json serializer out of the box, because the vector format is
    // not what Bencher expects.
    let mut output = std::io::stdout();
    writeln!(output, "{{")?;

    let mut buffer = None; // little trick to write no trailing ,

    for timing in results {
        if let Some(value) = buffer {
            writeln!(output, "{value}")?;
        }
        writeln!(output, "\t\"{}\": {{", timing.component)?;
        writeln!(output, "\t\t\"Duration\": {{")?;
        writeln!(output, "\t\t\t\"value\": {}", timing.time)?;
        writeln!(output, "\t\t}}")?;
        write!(output, "\t}}")?;
        buffer = Some(",");
    }
    writeln!(output, "\n}}")?;
    Ok(())
}

fn main() -> Result<()> {
    NpmController::parse().execute()
}
