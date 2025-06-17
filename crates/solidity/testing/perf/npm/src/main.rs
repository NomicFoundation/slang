use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

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

type Timings = HashMap<String, f64>;

type Measure = HashMap<&'static str, Duration>;

#[derive(serde::Serialize)]
struct Duration {
    value: f64,
}

fn new_measure(value: f64) -> Measure {
    let mut duration = Measure::with_capacity(1);
    duration.insert("Duration", Duration { value });
    duration
}

impl NpmController {
    fn execute(&self) -> Result<()> {
        let config = config::read_config()?;

        if let Some(hash) = &self.hash {
            let result = self.run_benchmarks("custom", hash, self.entrypoint.as_deref())?;
            publish(result.into_iter())
        } else {
            let file_benchmarks = self.individual_file_benchmarks(&config.files)?;
            let project_benchmarks = self.project_benchmarks(&config.projects)?;
            publish(file_benchmarks.into_iter().chain(project_benchmarks))
        }
    }

    fn run_benchmarks(&self, name: &str, hash: &str, file: Option<&str>) -> Result<Timings> {
        fetch(hash, &config::working_dir_path())?;
        let path = Path::new(config::WORKING_DIR).join(hash);

        let mut results = HashMap::new();
        for sut in SubjectUT::iter() {
            let sut_result = self.run(&path, name, file, sut)?;
            results.extend(sut_result);
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

    fn individual_file_benchmarks(&self, files: &Vec<File>) -> Result<Timings> {
        let mut results = HashMap::<String, f64>::new();

        let pattern_regex = self.compute_regex()?;
        for file in files {
            if !pattern_regex.is_match(&file.name) {
                continue;
            }

            let result = self.run_benchmarks(&file.name, &file.hash, Some(&file.file))?;
            results.extend(result);
        }
        Ok(results)
    }

    fn project_benchmarks(&self, projects: &Vec<Project>) -> Result<Timings> {
        let mut results = HashMap::<String, f64>::new();
        let pattern_regex = self.compute_regex()?;

        for project in projects {
            if !pattern_regex.is_match(&project.name) {
                continue;
            }

            let result = self.run_benchmarks(&project.name, &project.hash, None)?;
            results.extend(result);
        }
        Ok(results)
    }

    fn run(
        &self,
        path: &Path,
        name: &str,
        file: Option<&str>,
        sut: SubjectUT,
    ) -> Result<Timings, anyhow::Error> {
        let perf_crate = CargoWorkspace::locate_source_crate("solidity_testing_perf")?;
        let mut command = Command::new("npx")
            .arg("tsx")
            .flag("--trace-uncaught")
            .flag("--expose-gc")
            .arg(
                perf_crate
                    .join("npm/src/benchmarks/main.mts")
                    .to_str()
                    .unwrap(),
            )
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

fn publish(results: impl Iterator<Item = (String, f64)>) -> Result<()> {
    let results: HashMap<String, Measure> = results.map(|(k, v)| (k, new_measure(v))).collect();
    println!("{}", serde_json::to_string(&results)?);
    Ok(())
}

fn main() -> Result<()> {
    NpmController::parse().execute()
}
