use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity::backend::passes;
use slang_solidity::backend::passes::p5_resolve_references::Output;
use slang_solidity::compilation::CompilationUnit;

use super::renderer::binder_report;
use crate::compilation::compilation_unit::build_compilation_unit_from_multi_part_file;
use crate::cst::generated::VERSION_BREAKS;

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_output")
        .join(group_name)
        .join(test_name);
    let mut fs = CodegenFileSystem::default();

    let compare_with_binding_graph = std::env::var("__SLANG_NEW_BINDER_COMPARE__").is_ok();

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let mut last_report = None;

    for version in &VERSION_BREAKS {
        let compilation_unit = build_compilation_unit_from_multi_part_file(version, &contents)?;
        let has_parse_errors = compilation_unit
            .files()
            .iter()
            .any(|file| !file.errors().is_empty());

        let binder_data = build_binder(compilation_unit);

        let (report, all_resolved) = binder_report(&binder_data, compare_with_binding_graph)?;

        match last_report {
            Some(ref last) if last == &report => (),
            _ => {
                let snapshot_path = test_dir.join("v2/generated").join(format!(
                    "{version}-{status}.txt",
                    status = if has_parse_errors || !all_resolved {
                        "failure"
                    } else {
                        "success"
                    }
                ));
                fs.write_file_raw(snapshot_path, &report)?;
                last_report = Some(report);
            }
        }
    }

    Ok(())
}

fn build_binder(compilation_unit: CompilationUnit) -> Output {
    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    let data = passes::p3_linearise_contracts::run(data);
    let data = passes::p4_type_definitions::run(data);
    passes::p5_resolve_references::run(data)
}
