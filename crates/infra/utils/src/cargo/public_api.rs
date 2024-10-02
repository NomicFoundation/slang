use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, EnumIter)]
#[allow(non_camel_case_types)]
pub enum UserFacingCrate {
    // Sorted by dependency order (from dependencies to dependents):
    metaslang_cst,
    metaslang_graph_builder,
    metaslang_bindings,
    slang_solidity,
    slang_solidity_cli,
}

#[cfg(test)]
mod public_api_snapshots {
    use anyhow::Result;
    use rayon::iter::{ParallelBridge, ParallelIterator};
    use strum::IntoEnumIterator;

    use crate::cargo::{CargoWorkspace, UserFacingCrate};
    use crate::codegen::CodegenFileSystem;

    #[test]
    fn public_api_snapshots() {
        UserFacingCrate::iter()
            .filter(|&crate_name| has_library_target(crate_name))
            .par_bridge()
            .for_each(|crate_name| generate_public_api(crate_name).unwrap());
    }

    fn generate_public_api(crate_name: UserFacingCrate) -> Result<()> {
        let crate_dir = CargoWorkspace::locate_source_crate(crate_name.to_string())?;

        let rustdoc_json = rustdoc_json::Builder::default()
            .manifest_path(crate_dir.join("Cargo.toml"))
            .all_features(true)
            .toolchain(env!("RUST_NIGHTLY_VERSION"))
            .build()?;

        let public_api = public_api::Builder::from_rustdoc_json(rustdoc_json)
            .omit_auto_derived_impls(false)
            .omit_auto_trait_impls(true)
            .omit_blanket_impls(true)
            .build()?;

        let output_path = crate_dir.join("generated/public_api.txt");

        let mut fs = CodegenFileSystem::new(&crate_dir)?;

        fs.write_file(output_path, public_api.to_string())
    }

    fn has_library_target(crate_name: UserFacingCrate) -> bool {
        match crate_name {
            UserFacingCrate::metaslang_cst => true,
            UserFacingCrate::metaslang_graph_builder => true,
            UserFacingCrate::metaslang_bindings => true,
            UserFacingCrate::slang_solidity => true,
            UserFacingCrate::slang_solidity_cli => false,
        }
    }
}
