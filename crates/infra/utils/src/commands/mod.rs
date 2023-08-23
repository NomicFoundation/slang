use std::{
    collections::HashMap,
    env::vars,
    fmt::{Display, Formatter},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command as StdCommand, ExitStatus, Output, Stdio},
};

use anyhow::{bail, Context, Result};
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    github::GitHub,
    paths::{PathExtensions, PrivatePathExtensions},
};

#[derive(Clone, Debug)]
pub struct Command {
    name: String,
    args: Vec<String>,
    environment: HashMap<String, String>,
    current_dir: Option<PathBuf>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        return Self {
            name: name.into(),
            args: vec![],
            environment: HashMap::new(),
            current_dir: None,
        };
    }

    pub fn flag(mut self, flag: impl Into<String>) -> Self {
        self.args.push(flag.into());

        return self;
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());

        return self;
    }

    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for arg in args {
            self.args.push(arg.into());
        }

        return self;
    }

    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.push(key.into());
        self.args.push(value.into());

        return self;
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());

        return self;
    }

    pub fn current_dir(mut self, current_dir: impl Into<PathBuf>) -> Self {
        let current_dir = current_dir.into();
        if current_dir != Path::repo_root() {
            self.current_dir = Some(current_dir);
        }

        return self;
    }

    pub fn evaluate(&self) -> Result<String> {
        let output = spawn_with_defaults(self, Stdio::piped)?
            .wait_with_output()
            .with_context(|| format!("Failed to wait for output: {self}"))?;

        return extract_output(self, output);
    }

    pub fn evaluate_with_input(&self, input: impl AsRef<str>) -> Result<String> {
        let mut child = spawn_with_defaults(self, Stdio::piped)?;

        child
            .stdin
            .as_mut()
            .with_context(|| format!("Failed to capture stdin: {self}"))?
            .write_all(input.as_ref().as_bytes())
            .with_context(|| format!("Failed to write to stdin: {self}"))?;

        let output = child
            .wait_with_output()
            .with_context(|| format!("Failed to wait for output: {self}"))?;

        return extract_output(self, output);
    }

    pub fn run(&self) -> Result<()> {
        return GitHub::collapse_group(format!("$ {self}"), || run_with_defaults(self));
    }

    /// A quick replacement for `xargs`.
    /// Splits files into smaller chunks, so that we don't exceed the maximum command line length.
    pub fn run_xargs(&self, files: impl IntoIterator<Item = PathBuf>) -> Result<()> {
        const CHUNK_SIZE: usize = 50;

        return GitHub::collapse_group(format!("$ {self}"), || {
            files
                .into_iter()
                .map(|file| file.unwrap_str().to_owned())
                .chunks(CHUNK_SIZE)
                .into_iter()
                .map(|chunk| chunk.collect_vec())
                .collect_vec()
                .into_par_iter()
                .map(|batch| run_with_defaults(&self.clone().args(batch)))
                .collect()
        });
    }
}

fn run_with_defaults(command: &Command) -> Result<()> {
    let status = spawn_with_defaults(command, Stdio::inherit)?
        .wait()
        .with_context(|| format!("Failed to wait for status: {command}"))?;

    return check_status(command, status).map_err(|error| {
        // Print error and exit process, to skip printing irrelevant backtraces from the parent process:
        eprintln!("{error}");
        std::process::exit(1);
    });
}

fn spawn_with_defaults(command: &Command, stdio: impl Fn() -> Stdio) -> Result<Child> {
    let repo_root = Path::repo_root();

    let mut std_command = StdCommand::new(&command.name);

    let std_command = std_command
        .args(&command.args)
        // First, inherit environment from parent process:
        .envs(vars())
        // Then apply any user provided overrides:
        .envs(&command.environment)
        // Set up stdio:
        .stdin(stdio())
        .stdout(stdio())
        .stderr(stdio());

    if let Some(current_dir) = &command.current_dir {
        std_command.current_dir(current_dir);
    } else {
        std_command.current_dir(repo_root);
    }

    return std_command
        .spawn()
        .with_context(|| format!("Failed to spawn command: {command}"));
}

fn extract_output(command: &Command, output: Output) -> Result<String> {
    let stdout = String::from_utf8(output.stdout)
        .with_context(|| format!("Failed to read stdout: {command}"))?;

    match check_status(command, output.status) {
        Ok(()) => {
            return Ok(stdout);
        }
        Err(error) => {
            let stderr = String::from_utf8(output.stderr)
                .with_context(|| format!("Failed to read stderr: {command}"))?;

            return Err(error)
                .with_context(|| format!("stdout:\n{}", stdout))
                .with_context(|| format!("stderr:\n{}", stderr));
        }
    }
}

fn check_status(command: &Command, status: ExitStatus) -> Result<()> {
    if status.success() {
        return Ok(());
    } else {
        bail!(
            "Command failed with code '{code}': {command}",
            code = status.code().unwrap_or(-1)
        );
    }
}

impl Display for Command {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        let mut parts = vec![];

        for (key, value) in &self.environment {
            parts.push(format!("{key}='{value}'"));
        }

        if let Some(dir) = &self.current_dir {
            parts.push("cd".to_owned());
            parts.push(dir.strip_repo_root().unwrap().unwrap_str().to_owned());
            parts.push("&&".to_owned());
        }

        parts.push(self.name.to_owned());

        for arg in &self.args {
            let delimiter = if arg.contains(" ") {
                if arg.contains("\"") {
                    "'"
                } else {
                    "\""
                }
            } else {
                ""
            };

            parts.push(format!("{delimiter}{arg}{delimiter}"));
        }

        return write!(formatter, "{}", parts.join(" "));
    }
}
