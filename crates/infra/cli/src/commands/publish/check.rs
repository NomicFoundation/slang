use std::io::Write;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::github::GitHub;

#[derive(Clone, Debug, Parser)]
pub struct CheckController {}

impl CheckController {
    #[allow(clippy::unused_self)] // for compatibility with other controllers:
    pub fn execute(&self) -> Result<()> {
        let local_version = CargoWorkspace::local_version()?;
        println!("Local version: {local_version}");

        let Ok(published_version) = GitHub::latest_release_version() else {
            println!("No existing release found — publish needed.");
            set_github_output("publishNeeded", "true")?;
            return Ok(());
        };
        println!("Latest published version: {published_version}");

        if local_version == published_version {
            println!("Versions match — nothing to publish.");
            set_github_output("publishNeeded", "false")?;
        } else {
            println!("Version changed — publish needed.");
            set_github_output("publishNeeded", "true")?;
        }

        Ok(())
    }
}

fn set_github_output(key: &str, value: &str) -> Result<()> {
    if let Ok(path) = std::env::var("GITHUB_OUTPUT") {
        let mut file = std::fs::OpenOptions::new().append(true).open(path)?;
        writeln!(file, "{key}={value}")?;
    }
    Ok(())
}
