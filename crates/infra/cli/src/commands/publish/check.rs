use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::github::GitHub;

#[derive(Clone, Debug, Parser)]
pub struct CheckController;

impl CheckController {
    #[allow(clippy::unused_self)] // Keep consistent with other controllers' fn execute(&self) signature.
    pub fn execute(&self) -> Result<()> {
        let local_version = CargoWorkspace::local_version()?;
        println!("Local version: {local_version}");

        let Ok(published_version) = GitHub::latest_release_version() else {
            println!("No existing release found — publish needed.");
            println!("publishNeeded=true");
            return Ok(());
        };
        println!("Latest published version: {published_version}");

        if local_version == published_version {
            println!("Versions match — nothing to publish.");
            println!("publishNeeded=false");
        } else {
            println!("Version changed — publish needed.");
            println!("publishNeeded=true");
        }

        Ok(())
    }
}
