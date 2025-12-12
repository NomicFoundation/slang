use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;

use super::diff_report::binding_graph_diff_report;
use super::report::binder_report;
use super::report_data::ReportData;
use crate::compilation::compilation_unit::build_compilation_unit_from_multi_part_file;
use crate::cst::generated::VERSION_BREAKS;

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_output")
        .join(group_name)
        .join(test_name);
    let mut fs = CodegenFileSystem::default();

    let skip_diff_report = std::env::var("__SLANG_NEW_BINDER_SKIP_DIFF__").is_ok();

    let input_path = test_dir.join("input.sol");
    let contents = input_path.read_to_string()?;

    let mut last_report = None;
    let mut last_diff_report = None;

    for version in &VERSION_BREAKS {
        let compilation_unit = build_compilation_unit_from_multi_part_file(version, &contents)?;
        let has_parse_errors = compilation_unit
            .files()
            .iter()
            .any(|file| !file.errors().is_empty());

        let semantic_analysis = compilation_unit.semantic_analysis();
        let report_data = ReportData::prepare(semantic_analysis);
        let all_resolved = report_data.all_resolved();
        let status = if has_parse_errors || !all_resolved {
            "failure"
        } else {
            "success"
        };

        let report = binder_report(&report_data)?;

        match last_report {
            Some(ref last) if last == &report => (),
            _ => {
                let snapshot_path = test_dir
                    .join("v2/generated")
                    .join(format!("{version}-{status}.txt"));
                fs.write_file_raw(snapshot_path, &report)?;
                last_report = Some(report);
            }
        }

        if !skip_diff_report {
            // TODO: we need to review all the generated diff reports and double
            // check what changed wrt the stack graph binder
            let binding_graph = compilation_unit.binding_graph();
            let (diff_report, has_diff) = binding_graph_diff_report(&report_data, binding_graph)?;
            match last_diff_report {
                Some(ref last) if last == &diff_report => (),
                _ => {
                    let snapshot_path = test_dir.join("diff/generated").join(format!(
                        "{version}-{status}-{diff}.txt",
                        diff = if has_diff { "diff" } else { "same" }
                    ));
                    fs.write_file_raw(snapshot_path, &diff_report)?;
                    last_diff_report = Some(diff_report);
                }
            }
        }
    }

    Ok(())
}
