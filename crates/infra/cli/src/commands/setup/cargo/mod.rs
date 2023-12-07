use anyhow::Result;
use infra_utils::{commands::Command, github::GitHub};

pub fn setup_cargo() -> Result<()> {
    // The bootstrap bash script defined in '$REPO_ROOT/scripts/_common.sh'
    // installs the 'minimal' profile of the '$RUST_STABLE_VERSION' toolchain.
    // This includes the following components:
    //
    // - 'cargo'
    // - 'rust-std'
    // - 'rustc'
    //
    // Which are enough to run infra scripts.
    // But we need these additional optional components for local development:
    rustup_add_components(env!("RUST_STABLE_VERSION"), ["clippy"])?;
    if !GitHub::is_running_in_ci() {
        rustup_add_components(
            env!("RUST_STABLE_VERSION"),
            ["rust-analyzer", "rust-docs", "rust-src"],
        )?;
    }

    // Additionally, we also need 'rustfmt nightly', as we use experimental options.
    // So let's install the '$RUST_NIGHTLY_VERSION' toolchain along with the 'rustfmt' component.
    rustup_install_toolchain(env!("RUST_NIGHTLY_VERSION"))?;
    rustup_add_components(env!("RUST_NIGHTLY_VERSION"), ["rustfmt"])?;

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
        .run()
}

fn rustup_add_components(
    toolchain: &str,
    components: impl IntoIterator<Item = impl Into<String>>,
) -> Result<()> {
    Command::new("rustup")
        .args(["component", "add"])
        .property("--toolchain", toolchain)
        .args(components)
        .run()
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
