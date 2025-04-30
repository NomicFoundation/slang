use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;

use crate::utils::DryRun;

// Source: https://github.com/bencherdev/bencher/blob/aa31a002842cfb0da9d6c60569396fc5261f5111/tasks/test_api/src/task/test/smoke_test.rs#L20
const BENCHER_TEST_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJhcGlfa2V5IiwiZXhwIjo1OTkzNjQyMTU2LCJpYXQiOjE2OTg2NzQ4NjEsImlzcyI6Imh0dHBzOi8vZGV2ZWwtLWJlbmNoZXIubmV0bGlmeS5hcHAvIiwic3ViIjoibXVyaWVsLmJhZ2dlQG5vd2hlcmUuY29tIiwib3JnIjpudWxsfQ.9z7jmM53TcVzc1inDxTeX9_OR0PQPpZAsKsCE7lWHfo";

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    /// Folder where contracts are stored
    input_folder: String,

    #[arg(short, long, default_value_t = String::from(".*"))]
    pattern: String,

    #[command(flatten)]
    dry_run: DryRun,

    #[arg(trailing_var_arg = true)]
    extra_args: Vec<String>, // Collects all arguments after `--`
}

impl NpmController {
    fn install_deps() -> Result<()> {
        let command = Command::new("npm").arg("install").arg("tsx");
        command.evaluate()?;

        CargoWorkspace::install_binary("bencher_cli")?;
        Ok(())
    }

    fn ts_comparisons(&self) {
        let dry_run = self.dry_run.get();

        let token = if dry_run {
            // Use a dummy test token for dry runs:
            // https://github.com/bencherdev/bencher/issues/468
            BENCHER_TEST_TOKEN.to_string()
        } else {
            std::env::var("BENCHER_API_TOKEN").expect(
              "BENCHER_API_TOKEN is not set. Either perform a '--dry-run', or set it to your Bencher API token: https://bencher.dev/console"
            )
        };

        let package_name = "solidity_testing_perf_npmdriver";
        let input_folder = &self.input_folder;
        let pattern = &self.pattern;
        let extra_args = if self.extra_args.is_empty() {
            String::new()
        } else {
            "-- ".to_owned() + &self.extra_args.join(" ")
        };
        let test_runner = format!(
            "cargo run --package {package_name} -- {input_folder} --pattern={pattern} {extra_args}"
        );

        let testbed = if GitHub::is_running_in_ci() {
            "ci"
        } else {
            "dev"
        };

        let mut command = Command::new("bencher")
            .arg("run")
            .property("--project", "slang")
            .property("--adapter", "json")
            .property("--testbed", testbed)
            .secret("BENCHER_API_TOKEN", token);

        if dry_run {
            command = command.flag("--dry-run");
        }

        // Has to be the last argument:
        command.arg(test_runner).run();

        println!(
            "

Bencher Run is complete...
Test Results: [https://bencher.dev/console/projects/slang/reports]
"
        );
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        Self::install_deps()?;

        self.ts_comparisons();
        Ok(())
    }
}
