use anyhow::{anyhow, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::PathExtensions;
use metaslang_bindings::PathResolver;
use semver::Version;
use slang_solidity::backend::passes::{self, p2_collect_definitions};
use slang_solidity::compilation::{CompilationUnit, InternalCompilationBuilder};

use crate::generated::VERSION_BREAKS;
use crate::multi_part_file::{split_multi_file, MultiPart};
use crate::resolver::TestsPathResolver;

type Binder = p2_collect_definitions::Output;

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_output")
        .join(group_name)
        .join(test_name);

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    for version in &VERSION_BREAKS {
        let multi_part = split_multi_file(&contents);

        let compilation_unit = build_compilation_unit(version, &multi_part)?;
        let _ = build_binder(compilation_unit);
    }

    Ok(())
}

fn build_compilation_unit(
    version: &Version,
    multi_part: &MultiPart<'_>,
) -> Result<CompilationUnit> {
    let mut internal_builder = InternalCompilationBuilder::create(version.clone())?;
    let resolver = TestsPathResolver {};

    for part in &multi_part.parts {
        let add_file_response = internal_builder.add_file(part.name.to_string(), part.contents);
        for import_path in &add_file_response.import_paths {
            if let Some(resolved_path) = resolver.resolve_path(part.name, import_path) {
                internal_builder
                    .resolve_import(part.name, import_path, resolved_path)
                    .unwrap(); // this can only fail if `part.name` is not a valid file ID
            } else {
                return Err(anyhow!(
                    "Failed to resolve {import_path} in the context of {context_path}",
                    import_path = import_path.node().unparse(),
                    context_path = part.name
                ));
            }
        }
    }

    Ok(internal_builder.build())
}

fn build_binder(compilation_unit: CompilationUnit) -> Binder {
    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    passes::p2_collect_definitions::run(data)
}
