pub mod metadata;

use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use clap::Parser;
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate};
use infra_utils::commands::Command;
use infra_utils::http::put_crate_to_registry;
use strum::IntoEnumIterator;

use crate::commands::publish::artifacts::Manifest;
use crate::utils::DryRun;

const CRATES_IO_PUBLISH_URL: &str = "https://crates.io/api/v1/crates/new";
const TOKEN_ENV: &str = "CARGO_REGISTRY_TOKEN";
const USER_AGENT_VALUE: &str = "slang-infra-publish (https://github.com/NomicFoundation/slang)";

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        if self.dry_run.get() {
            // Dry-run can't be split per-crate: each bumped version's deps aren't yet on
            // crates.io, so per-crate resolution fails. The batched workspace `cargo publish
            // --dry-run` is the safety net for cross-crate path-dep rewrites we lose by
            // using `cargo package --no-verify` in the prepare step.
            run_batched_dry_run()
        } else {
            run_registry_upload()
        }
    }
}

fn run_batched_dry_run() -> Result<()> {
    let mut changed_crates = vec![];

    for crate_name in UserFacingV1Crate::iter() {
        let crate_name = crate_name.to_string();
        if let Ok(published_version) = CargoWorkspace::published_version(&crate_name) {
            println!("Published version of {crate_name}: {published_version}");
            let local_version = CargoWorkspace::local_version()?;
            println!("Local version: {local_version}");
            if local_version == published_version {
                println!(
                    "Skipping crate {crate_name}, since the local version is already published."
                );
                continue;
            }
        } else {
            println!("No published version found for crate {crate_name}.");
        }
        changed_crates.push(crate_name);
    }

    if changed_crates.is_empty() {
        println!("No crates to publish.");
        return Ok(());
    }

    let mut command = Command::new("cargo")
        .arg("publish")
        .flag("--all-features")
        .flag("--dry-run");
    for crate_name in &changed_crates {
        command = command.property("--package", crate_name);
    }
    command.run();

    Ok(())
}

fn run_registry_upload() -> Result<()> {
    let manifest = Manifest::load()?;
    manifest.verify_integrity()?;

    if manifest.cargo.is_empty() {
        println!("No cargo crates to publish.");
        return Ok(());
    }

    let token = env::var(TOKEN_ENV)
        .with_context(|| format!("${TOKEN_ENV} not set; cannot authenticate with crates.io"))?;

    for entry in &manifest.cargo {
        let crate_bytes = fs::read(manifest.absolute_path(&entry.crate_path))?;
        let metadata_bytes = fs::read(manifest.absolute_path(&entry.metadata_path))?;

        println!(
            "Publishing {name} v{version}",
            name = entry.crate_name,
            version = entry.version,
        );

        put_crate_to_registry(
            CRATES_IO_PUBLISH_URL,
            &metadata_bytes,
            &crate_bytes,
            &token,
            USER_AGENT_VALUE,
        )
        .with_context(|| format!("Failed to publish {}", entry.crate_name))?;

        wait_for_index_propagation(&entry.crate_name, &entry.version)?;
    }

    Ok(())
}

/// crates.io confirms a successful upload immediately, but the sparse index can
/// lag for a few seconds. The next crate in the sequence depends on this version
/// being resolvable, so poll until `cargo search` reports the new version (or we
/// time out).
fn wait_for_index_propagation(crate_name: &str, expected_version: &str) -> Result<()> {
    const MAX_ATTEMPTS: u32 = 30;
    const POLL_INTERVAL: Duration = Duration::from_secs(2);

    for attempt in 1..=MAX_ATTEMPTS {
        if let Ok(reported) = CargoWorkspace::published_version(crate_name) {
            if reported.to_string() == expected_version {
                println!("Index updated for {crate_name} (attempt {attempt}).");
                return Ok(());
            }
        }
        thread::sleep(POLL_INTERVAL);
    }
    bail!(
        "Timed out waiting for crates.io index to show {crate_name} v{expected_version} \
         after {seconds}s. The upload may have succeeded; later crates that depend on \
         {crate_name} may fail until the index catches up.",
        seconds = MAX_ATTEMPTS * u32::try_from(POLL_INTERVAL.as_secs()).unwrap(),
    );
}
