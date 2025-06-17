use std::fmt::Write;
use std::rc::Rc;

use anyhow::{anyhow, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use metaslang_bindings::PathResolver;
use semver::Version;
use slang_solidity::backend::binder::Binder;
use slang_solidity::backend::passes;
use slang_solidity::backend::passes::p3_resolve_untyped::Output;
use slang_solidity::bindings::{BindingGraph, BindingLocation};
use slang_solidity::compilation::{CompilationUnit, InternalCompilationBuilder};

use crate::generated::VERSION_BREAKS;
use crate::multi_part_file::{split_multi_file, MultiPart};
use crate::resolver::TestsPathResolver;

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_output")
        .join(group_name)
        .join(test_name);
    let target_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("target/binder")
        .join(group_name)
        .join(test_name);

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let mut fs = CodegenFileSystem::default();
    let mut last_report = None;

    for version in &VERSION_BREAKS {
        let multi_part = split_multi_file(&contents);

        let compilation_unit = build_compilation_unit(version, &multi_part)?;
        let data = build_binder(compilation_unit);

        let binding_graph = data.compilation_unit.binding_graph();
        let binder = data.binder;

        let report = binder_report(&binder, binding_graph)?;

        match last_report {
            Some(ref last) if last == &report => (),
            _ => {
                let snapshot_path = target_dir.join("generated").join(format!("{version}.txt"));

                fs.write_file_raw(snapshot_path, &report)?;
                last_report = Some(report);
            }
        }
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

fn build_binder(compilation_unit: CompilationUnit) -> Output {
    let data = passes::p0_build_ast::run(compilation_unit);
    let data = passes::p1_flatten_contracts::run(data);
    let data = passes::p2_collect_definitions::run(data);
    passes::p3_resolve_untyped::run(data)
}

const SEPARATOR: &str =
    "\n------------------------------------------------------------------------\n";

fn binder_report(binder: &Binder, binding_graph: &Rc<BindingGraph>) -> Result<String> {
    let mut report = String::new();

    let found_definitions = binder.definitions.len();
    let total_definitions = binding_graph
        .all_definitions()
        .filter(|definition| {
            matches!(
                definition.definiens_location(),
                BindingLocation::UserFile(_)
            )
        })
        .count();

    let found_references = binder.references.len();
    let total_references = binding_graph
        .all_references()
        .filter(|reference| matches!(reference.location(), BindingLocation::UserFile(_)))
        .count();

    writeln!(
        report,
        "Definitions: found {found_definitions} out of {total_definitions}"
    )?;

    writeln!(report)?;
    for definition in binder.definitions.values() {
        writeln!(report, "- {definition:?}")?;
    }

    writeln!(report, "{SEPARATOR}")?;

    writeln!(
        report,
        "References: found {found_references} out of {total_references}"
    )?;

    writeln!(report)?;
    for reference in binder.references.values() {
        writeln!(report, "- {reference:?}")?;
    }

    Ok(report)
}
