use anyhow::{bail, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;

pub fn install_bencher_cli() -> Result<()> {
    CargoWorkspace::install_binary("bencher_cli")
}

pub fn install_iai_callgrind_runner() -> Result<()> {
    CargoWorkspace::install_binary("iai-callgrind-runner")
}

pub fn install_valgrind() -> Result<()> {
    install_from_apt("valgrind", "1:3.22.0-0ubuntu3");

    match Command::new("valgrind").flag("--version").evaluate() {
        Ok(output) if output.starts_with("valgrind-") => Ok(()),
        other => bail!(
            "valgrind needs to be installed on this machine to run perf tests.
            Supported Platforms: https://valgrind.org/downloads/current.html
            {other:?}"
        ),
    }
}

pub fn install_graphviz() -> Result<()> {
    install_from_apt("graphviz", "2.42.2-9ubuntu0.1");

    // dot prints its version to stderr, using the help page instead
    match Command::new("dot").flag("-?").evaluate() {
        Ok(output) if output.starts_with("Usage: dot") => Ok(()),
        other => bail!(
            "graphviz needs to be installed on this machine to generate callgraphs.
            Supported Platforms: https://graphviz.org/download/
            {other:?}"
        ),
    }
}

fn install_from_apt(package_name: &str, version: &str) {
    if GitHub::is_running_in_ci() {
        Command::new("sudo").args(["apt", "update"]).run();

        Command::new("sudo")
            .args(["apt", "install"])
            .arg(format!("{package_name}={version}"))
            .flag("--yes")
            .run();

        return;
    }

    if GitHub::is_running_in_devcontainer() {
        Command::new("sudo").args(["apt-get", "update"]).run();

        Command::new("sudo")
            .args(["apt-get", "install"])
            .arg(format!("{package_name}={version}"))
            .flag("--yes")
            .run();
    }
}
