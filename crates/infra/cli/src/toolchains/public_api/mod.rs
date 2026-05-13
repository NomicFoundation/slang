use anyhow::{Context, Result};
use infra_utils::cargo::{CargoWorkspace, UserFacingV1Crate, UserFacingV2Crate};
use infra_utils::codegen::CodegenFileSystem;
use rayon::iter::{ParallelBridge, ParallelIterator};
use strum::IntoEnumIterator;

pub fn generate_public_api_snapshots() {
    assert!(env!("RUST_NIGHTLY_VERSION").ge(public_api::MINIMUM_NIGHTLY_RUST_VERSION));

    let v1_libs = UserFacingV1Crate::iter()
        .filter(|c| c.has_library_target())
        .map(|c| c.to_string());

    let v2_libs = UserFacingV2Crate::iter()
        .filter(|c| c.has_library_target())
        .map(|c| c.to_string());

    v1_libs
        .chain(v2_libs)
        .par_bridge()
        .for_each(|crate_name| generate_public_api(&crate_name).unwrap());
}

fn generate_public_api(crate_name: &str) -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate(crate_name)?;

    let rustdoc_json = rustdoc_json::Builder::default()
        .manifest_path(crate_dir.join("Cargo.toml"))
        .toolchain(env!("RUST_NIGHTLY_VERSION"))
        .build()?;

    let public_api = public_api::Builder::from_rustdoc_json(&rustdoc_json)
        .omit_auto_derived_impls(false)
        .omit_auto_trait_impls(true)
        .omit_blanket_impls(true)
        .build()
        .with_context(|| format!("Failed to generate public API from {rustdoc_json:?}"))?;

    let output_path = crate_dir.join("generated/public_api.txt");

    let mut fs = CodegenFileSystem::default();

    fs.write_file_raw(output_path, public_api.to_string())
}
