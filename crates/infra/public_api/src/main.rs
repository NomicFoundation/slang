use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate, UserFacingV2Crate};
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::IntoEnumIterator;

/// Writes `generated/public_api.txt` for each user-facing crate, from its `rustdoc` JSON output.
#[derive(Debug, Parser)]
struct Cli {
    /// Only generate the snapshots of this version's crates. Generates both when omitted.
    #[arg(long)]
    filter: Option<Filter>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Filter {
    V1,
    V2,
}

fn main() -> Result<()> {
    let Cli { filter } = Cli::parse();

    let v1_crates = UserFacingV1Crate::iter()
        .filter(|c| c.has_library_target())
        .map(|c| c.to_string());

    let v2_crates = UserFacingV2Crate::iter()
        .filter(|c| c.has_library_target())
        .map(|c| c.to_string());

    let crate_names: Vec<_> = match filter {
        None => v1_crates.chain(v2_crates).collect(),
        Some(Filter::V1) => v1_crates.collect(),
        Some(Filter::V2) => v2_crates.collect(),
    };

    generate_public_api_snapshots(&crate_names)
}

fn generate_public_api_snapshots(crate_names: &[String]) -> Result<()> {
    assert!(env!("RUST_NIGHTLY_VERSION").ge(public_api::MINIMUM_NIGHTLY_RUST_VERSION));

    // 'rustdoc' runs under 'cargo', which parallelizes and caches it across crates.
    build_rustdoc_json(crate_names)?;

    // Parsing the (large) JSON files afterwards is pure CPU work, so it is worth parallelizing here:
    crate_names
        .par_iter()
        .try_for_each(|crate_name| generate_public_api(crate_name))
}

/// Runs `rustdoc` once for every crate, producing `$TARGET_DIR/doc/$CRATE_NAME.json`.
fn build_rustdoc_json(crate_names: &[String]) -> Result<()> {
    // 'cargo' hard links every crate's JSON output into this directory, but does not create it
    // first, and the parallel jobs race each other to do so. Let's create it ourselves beforehand:
    let doc_dir = rustdoc_json_dir();
    std::fs::create_dir_all(&doc_dir)?;

    // Note that '--output-format' is a 'cargo' flag here, and is not forwarded to 'rustdoc' after a
    // '--' separator. Otherwise, 'cargo' still expects the HTML output that it did not ask 'rustdoc'
    // to produce, fails to find '$TARGET_DIR/doc/$CRATE_NAME/index.html', and considers the crate
    // stale on every single run. See <https://github.com/rust-lang/cargo/issues/12103>.
    //
    // The flag is still unstable, which is fine, since we pin '$RUST_NIGHTLY_VERSION' anyway.
    // Tracked in <https://github.com/rust-lang/cargo/issues/13283>.
    let mut command = Command::new("rustup")
        .args(["run", env!("RUST_NIGHTLY_VERSION"), "cargo"])
        .arg("doc")
        // Only document the library target of our own crates:
        .flag("--lib")
        .flag("--no-deps")
        .flag("-Zunstable-options")
        .property("--output-format", "json");

    for crate_name in crate_names {
        command = command.property("--package", crate_name);
    }

    command.run();

    Ok(())
}

fn generate_public_api(crate_name: &str) -> Result<()> {
    let rustdoc_json = rustdoc_json_dir().join(format!("{crate_name}.json"));

    let public_api = public_api::Builder::from_rustdoc_json(&rustdoc_json)
        .omit_auto_derived_impls(false)
        .omit_auto_trait_impls(true)
        .omit_blanket_impls(true)
        .build()
        .with_context(|| format!("Failed to generate public API from {rustdoc_json:?}"))?;

    let crate_dir = CargoWorkspace::locate_source_crate(crate_name)?;
    let output_path = crate_dir.join("generated/public_api.txt");

    let mut fs = CodegenFileSystem::default();
    fs.write_file_raw(output_path, public_api.to_string())?;

    Ok(())
}

fn rustdoc_json_dir() -> PathBuf {
    Path::repo_path("target/doc")
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}
