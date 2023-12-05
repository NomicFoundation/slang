use anyhow::Result;
use infra_utils::{commands::Command, github::GitHub};

pub fn setup_cargo() -> Result<()> {
    // The bootstrap bash script defined in '$REPO_ROOT/scripts/_common.sh'
    // installs the 'minimal' profile of the '$RUST_VERSION' toolchain.
    // This includes the following components:
    //
    // - 'cargo'
    // - 'rust-std'
    // - 'rustc'
    //
    // Which are enough to build, test, and run source code.
    // But we need these additional optional components:
    //
    // - 'clippy' for linting
    // - 'rust-docs' for local development only
    //
    // So let's install these here:

    rustup_add_component(env!("RUST_VERSION"), "clippy")?;

    if !GitHub::is_running_in_ci() {
        rustup_add_component(env!("RUST_VERSION"), "rust-docs")?;
    }

    // Additionally, we also need 'rustfmt nightly', as we use experimental options.
    // So let's install the 'nightly' toolchain along with the 'rustfmt' component.

    rustup_install_toolchain("nightly")?;

    rustup_add_component("nightly", "rustfmt")?;

    // Warm up IDE tools if running locally.

    if !GitHub::is_running_in_ci() {
        warm_up_ide_tools()?;
    }

    // Make sure we have the latest dependencies:

    run_cargo_fetch()?;

    Ok(())
}

fn rustup_install_toolchain(toolchain: &str) -> Result<()> {
    Command::new("rustup")
        .arg("install")
        .flag("--no-self-update")
        .property("--profile", "minimal")
        .arg(toolchain)
        .arg("nightly")
        .run()
}

fn rustup_add_component(toolchain: &str, component: &str) -> Result<()> {
    Command::new("rustup")
        .args(["component", "add"])
        .property("--toolchain", toolchain)
        .arg(component)
        .run()
}

fn warm_up_ide_tools() -> Result<()> {
    Command::new("rust-analyzer").flag("--version").run()?;

    Command::new("rust-src").flag("--version").run()?;

    Ok(())
}

fn run_cargo_fetch() -> Result<()> {
    let mut command = Command::new("cargo").arg("fetch");

    if GitHub::is_running_in_ci() {
        // In CI, run with '--locked' to make sure `Cargo.lock` is up to date.
        // Don't use '--frozen', because the cache is rarely up to date.
        command = command.flag("--locked");
    }

    command.run()
}
