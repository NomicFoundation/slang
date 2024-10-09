use infra_utils::commands::Command;
use infra_utils::github::GitHub;

use crate::toolchains::wasm::WASM_TARGET;

pub fn setup_cargo() {
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
    rustup_add_components(env!("RUST_STABLE_VERSION"), ["clippy"]);
    if !GitHub::is_running_in_ci() {
        rustup_add_components(
            env!("RUST_STABLE_VERSION"),
            ["rust-analyzer", "rust-docs", "rust-src"],
        );
    }

    // Additionally, we also need the following nightly components:
    //
    // - 'rustfmt' as we use experimental options.
    // - 'rust-docs' for the '--json' output to document the public API.
    //
    // So let's install the '$RUST_NIGHTLY_VERSION' toolchain along with these components.
    rustup_install_toolchain(env!("RUST_NIGHTLY_VERSION"));
    rustup_add_components(env!("RUST_NIGHTLY_VERSION"), ["rustfmt", "rust-docs"]);

    // Needed for the TypeScript packages:
    rustup_add_targets(env!("RUST_STABLE_VERSION"), [WASM_TARGET]);

    // Make sure we have the latest dependencies:
    run_cargo_fetch();
}

fn rustup_add_targets(toolchain: &str, targets: impl IntoIterator<Item = impl Into<String>>) {
    Command::new("rustup")
        .args(["target", "add"])
        .property("--toolchain", toolchain)
        .args(targets)
        .run();
}

fn rustup_install_toolchain(toolchain: &str) {
    Command::new("rustup")
        .arg("install")
        .flag("--no-self-update")
        .property("--profile", "minimal")
        .arg(toolchain)
        .run();
}

fn rustup_add_components(toolchain: &str, components: impl IntoIterator<Item = impl Into<String>>) {
    Command::new("rustup")
        .args(["component", "add"])
        .property("--toolchain", toolchain)
        .args(components)
        .run();
}

fn run_cargo_fetch() {
    let mut command = Command::new("cargo").arg("fetch");

    if GitHub::is_running_in_ci() {
        // In CI, run with '--locked' to make sure `Cargo.lock` is up to date.
        // Don't use '--frozen', because the cache is rarely up to date.
        command = command.flag("--locked");
    }

    command.run();
}
