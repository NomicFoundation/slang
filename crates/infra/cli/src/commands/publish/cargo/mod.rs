pub mod metadata;

use std::fs::File;
use std::time::Duration;
use std::{env, thread};

use anyhow::{bail, Context, Result};
use clap::Parser;
use crates_io::{NewCrate, Registry, Warnings};
use curl::easy::Easy;
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate};
use infra_utils::commands::Command;
use infra_utils::http::fetch_index_versions;
use strum::IntoEnumIterator;

use crate::commands::publish::artifacts::Manifest;
use crate::utils::DryRun;

const CRATES_IO_HOST: &str = "https://crates.io";
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

    // `crates_io::Registry` does all the wire-format work — length-prefixed body,
    // `Authorization` header, response parsing, 30-second timeout heuristic. We
    // just supply a curl handle with the right user-agent.
    let mut handle = Easy::new();
    handle
        .useragent(USER_AGENT_VALUE)
        .context("Failed to set curl user-agent")?;
    let mut registry = Registry::new_handle(CRATES_IO_HOST.to_owned(), Some(token), handle, true);

    for entry in &manifest.cargo {
        let crate_path = Manifest::absolute_path(&entry.crate_path);
        let metadata_path = Manifest::absolute_path(&entry.metadata_path);

        let metadata_bytes = std::fs::read(&metadata_path)
            .with_context(|| format!("Failed to read {metadata_path:?}"))?;
        let new_crate: NewCrate = serde_json::from_slice(&metadata_bytes)
            .with_context(|| format!("Failed to deserialize {metadata_path:?}"))?;
        let tarball =
            File::open(&crate_path).with_context(|| format!("Failed to open {crate_path:?}"))?;

        println!(
            "Publishing {name} v{version}",
            name = entry.crate_name,
            version = entry.version,
        );

        let warnings = registry.publish(&new_crate, &tarball).with_context(|| {
            format!("crates.io rejected {} v{}", entry.crate_name, entry.version)
        })?;
        print_warnings(&entry.crate_name, &warnings);

        wait_for_index_propagation(&entry.crate_name, &entry.version)?;
    }

    Ok(())
}

fn print_warnings(crate_name: &str, warnings: &Warnings) {
    for category in &warnings.invalid_categories {
        eprintln!("{crate_name}: invalid category ignored by crates.io: {category}");
    }
    for badge in &warnings.invalid_badges {
        eprintln!("{crate_name}: invalid badge ignored by crates.io: {badge}");
    }
    for other in &warnings.other {
        eprintln!("{crate_name}: registry warning: {other}");
    }
}

/// crates.io confirms a successful upload immediately, but the sparse index can
/// lag for a few seconds. The next crate in the sequence depends on this version
/// being resolvable, so poll the sparse index directly (CDN-cached, designed for
/// high-frequency reads — not the search API, which is rate-limited).
fn wait_for_index_propagation(crate_name: &str, expected_version: &str) -> Result<()> {
    const MAX_ATTEMPTS: u32 = 30;
    const POLL_INTERVAL: Duration = Duration::from_secs(2);

    for attempt in 1..=MAX_ATTEMPTS {
        if let Ok(versions) = fetch_index_versions(crate_name) {
            if versions.iter().any(|v| v == expected_version) {
                println!("Index updated for {crate_name} (attempt {attempt}).");
                return Ok(());
            }
        }
        thread::sleep(POLL_INTERVAL);
    }
    bail!(
        "Timed out waiting for crates.io sparse index to show {crate_name} v{expected_version} \
         after {seconds}s. The upload may have succeeded; later crates that depend on \
         {crate_name} may fail until the index catches up.",
        seconds = MAX_ATTEMPTS * u32::try_from(POLL_INTERVAL.as_secs()).unwrap(),
    );
}
