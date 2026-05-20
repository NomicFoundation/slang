use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate};
use infra_utils::commands::Command;
use strum::IntoEnumIterator;

use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        let mut changed_crates = vec![];

        for crate_name in UserFacingV1Crate::iter() {
            let crate_name = crate_name.to_string();
            if !needs_publish(&crate_name)? {
                continue;
            }
            changed_crates.push(crate_name);
        }

        if changed_crates.is_empty() {
            println!("No crates to publish.");
            return Ok(());
        }

        if self.dry_run.get() {
            // Per-crate dry-runs fail because the bumped versions of internal
            // deps aren't on crates.io yet. Batch all changed crates into one
            // `cargo publish --dry-run` so cargo can resolve internal path-deps
            // locally.
            run_batched_dry_run(&changed_crates);
        } else {
            // Publish each crate in `UserFacingV1Crate` dep order. `--no-verify`
            // is the load-bearing flag: it skips cargo's verification compile,
            // so no `build.rs` or proc-macros from the workspace dep graph run
            // alongside the OIDC-exchanged crates.io token. cargo still does
            // dependency resolution and tarball creation, but those are cargo's
            // own code, not arbitrary user crates.
            for crate_name in &changed_crates {
                run_cargo_publish(crate_name);
            }
        }

        Ok(())
    }
}

fn needs_publish(crate_name: &str) -> Result<bool> {
    if let Ok(published_version) = CargoWorkspace::published_version(crate_name) {
        println!("Published version of {crate_name}: {published_version}");
        let local_version = CargoWorkspace::local_version()?;
        println!("Local version: {local_version}");
        if local_version == published_version {
            println!("Skipping crate {crate_name}, since the local version is already published.");
            return Ok(false);
        }
    } else {
        println!("No published version found for crate {crate_name}.");
    }
    Ok(true)
}

fn run_cargo_publish(crate_name: &str) {
    Command::new("cargo")
        .arg("publish")
        .flag("--no-verify")
        .flag("--all-features")
        .property("--package", crate_name)
        .run();
}

fn run_batched_dry_run(crate_names: &[String]) {
    let mut command = Command::new("cargo")
        .arg("publish")
        .flag("--all-features")
        .flag("--dry-run");
    for crate_name in crate_names {
        command = command.property("--package", crate_name);
    }
    command.run();
}
