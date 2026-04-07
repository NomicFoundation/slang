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

        let published_version = GitHub::latest_release_version()?;
        println!("Latest published version: {published_version}");

        let publish_needed = local_version != published_version;
        if publish_needed {
            println!("Version changed — publish needed.");
        } else {
            println!("Versions match — nothing to publish.");
        }

        set_github_output("publishNeeded", &publish_needed.to_string())?;

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
