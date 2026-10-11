use anyhow::Result;
use clap::{Parser, ValueEnum};

pub trait OrderedCommand: Clone + Ord + PartialEq + ValueEnum {
    fn execute(&self) -> Result<()>;
}

pub trait ClapExtensions {
    fn clap_name(&self) -> String;
}

impl<T: ValueEnum> ClapExtensions for T {
    fn clap_name(&self) -> String {
        self.to_possible_value()
            .expect("Expected Clap ValueEnum to have a name (not skipped).")
            .get_name()
            .to_owned()
    }
}

#[derive(Clone, Copy, Debug, Parser)]
pub struct DryRun {
    /// Attempt a local run and report results, without publishing any user-facing artifacts.
    #[arg(long)]
    dry_run: bool,
}

impl DryRun {
    pub fn get(self) -> bool {
        if self.dry_run {
            println!("Performing a dry run, since it was requested on the command line.");
            true
        } else {
            println!("Performing a full run. You can pass '--dry-run' to simulate the execution.");
            false
        }
    }
}
