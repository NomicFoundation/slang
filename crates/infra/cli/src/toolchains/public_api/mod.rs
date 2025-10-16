use anyhow::{Context, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use rayon::iter::{ParallelBridge, ParallelIterator};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, EnumIter)]
#[allow(non_camel_case_types)]
pub enum UserFacingCrate {
    // Sorted by dependency order (from dependencies to dependents):
    metaslang_cst,
    metaslang_graph_builder,
    metaslang_stack_graphs,
    metaslang_bindings,
    slang_solidity,
    slang_solidity_cli,
}

pub fn generate_public_api_snapshots() {
    assert!(env!("RUST_NIGHTLY_VERSION").ge(public_api::MINIMUM_NIGHTLY_RUST_VERSION));

    UserFacingCrate::iter()
        .filter(|&crate_name| has_library_target(crate_name))
        .par_bridge()
        .for_each(|crate_name| generate_public_api(crate_name).unwrap());
}

fn generate_public_api(crate_name: UserFacingCrate) -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate(crate_name.to_string())?;

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

fn has_library_target(crate_name: UserFacingCrate) -> bool {
    match crate_name {
        UserFacingCrate::metaslang_cst => true,
        UserFacingCrate::metaslang_graph_builder => true,
        UserFacingCrate::metaslang_stack_graphs => true,
        UserFacingCrate::metaslang_bindings => true,
        UserFacingCrate::slang_solidity => true,
        UserFacingCrate::slang_solidity_cli => false,
    }
}
