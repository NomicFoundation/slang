use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate};
use infra_utils::commands::Command;
use semver::Version;
use strum::IntoEnumIterator;

use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        let local_version = CargoWorkspace::local_version()?;
        let changed_crates: Vec<String> = UserFacingV1Crate::iter()
            .map(|c| c.to_string())
            .filter(|name| needs_publish(name, &local_version))
            .collect();

        if changed_crates.is_empty() {
            println!("No crates to publish.");
            return Ok(());
        }

        if self.dry_run.get() {
            run_batched_dry_run(&changed_crates);
        } else {
            for crate_name in &changed_crates {
                run_cargo_publish(crate_name);
            }
        }
        Ok(())
    }
}

fn needs_publish(crate_name: &str, local_version: &Version) -> bool {
    if let Ok(published_version) = CargoWorkspace::published_version(crate_name) {
        println!("Published version of {crate_name}: {published_version}");
        println!("Local version: {local_version}");
        if *local_version == published_version {
            println!("Skipping crate {crate_name}, since the local version is already published.");
            return false;
        }
    } else {
        println!("No published version found for crate {crate_name}.");
    }
    true
}

/// `cargo publish` always packages the `.crate` (the upload artifact); `--no-verify`
/// skips only the post-package verify-*compile*, so no workspace `build.rs` or
/// proc-macros run alongside the OIDC crates.io token. cargo's own code (dep
/// resolution, packaging) still runs. The verify-build isn't lost — the `review`
/// job already ran it, tokenless, via `cargo publish --dry-run` on the same commit.
fn run_cargo_publish(crate_name: &str) {
    Command::new("cargo")
        .arg("publish")
        .flag("--locked")
        .flag("--no-verify")
        .flag("--all-features")
        .property("--package", crate_name)
        .run();
}

/// Per-crate dry-runs against crates.io fail because the bumped versions
/// of internal deps aren't on crates.io yet. Batch them into one invocation
/// so cargo resolves internal path-deps locally.
fn run_batched_dry_run(crate_names: &[String]) {
    let mut command = Command::new("cargo")
        .arg("publish")
        .flag("--locked")
        .flag("--all-features")
        .flag("--dry-run");
    for crate_name in crate_names {
        command = command.property("--package", crate_name);
    }
    command.run();
}
