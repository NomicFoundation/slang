use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use strum::IntoEnumIterator;

use crate::toolchains::public_api::UserFacingCrate;
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        let mut changed_crates = vec![];

        for crate_name in UserFacingCrate::iter() {
            let crate_name = crate_name.to_string();

            if prepare_for_publish(&crate_name)? {
                changed_crates.push(crate_name);
            }
        }

        if changed_crates.is_empty() {
            println!("No crates to publish.");
            return Ok(());
        }

        if self.dry_run.get() {
            // Dry-run all changed crates in a single invocation so cargo can
            // resolve inter-crate path deps locally. Per-crate dry-runs fail
            // because the new versions of internal deps aren't on crates.io yet.
            run_cargo_publish_dry_run(&changed_crates);
        } else {
            for crate_name in &changed_crates {
                run_cargo_publish(crate_name);
            }
        }

        Ok(())
    }
}

fn prepare_for_publish(crate_name: &str) -> Result<bool> {
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
        .property("--package", crate_name)
        .flag("--all-features")
        .run();
}

fn run_cargo_publish_dry_run(crate_names: &[String]) {
    let mut command = Command::new("cargo")
        .arg("publish")
        .flag("--all-features")
        .flag("--dry-run");

    for crate_name in crate_names {
        command = command.property("--package", crate_name);
    }

    command.run();
}
