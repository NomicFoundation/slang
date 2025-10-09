use std::collections::{BTreeSet, HashMap};
use std::fmt::Write;

use anyhow::Result;
use slang_solidity::backend::binder::Resolution;
use slang_solidity::backend::BinderOutput;
use slang_solidity::cst::NodeId;

use super::report_data::{CollectedDefinition, CollectedReference, ReportData};

pub(crate) fn binding_graph_diff_report(report_data: &'_ ReportData<'_>) -> Result<(String, bool)> {
    let mut report = String::new();

    let ReportData {
        binder_output,
        all_definitions,
        all_references,
        definitions_by_id,
        ..
    } = report_data;

    let has_diff = diff_binding_graph_definitions(
        &mut report,
        binder_output,
        all_definitions,
        definitions_by_id,
    )? || diff_binding_graph_references(
        &mut report,
        binder_output,
        all_references,
        definitions_by_id,
    )?;

    if has_diff {
        writeln!(report)?;

        writeln!(
            report,
            "Definitions and/or references from binding graph and new binder DIFFER"
        )?;
    } else {
        writeln!(
            report,
            "No differences between binding graph and new binder definitions or references"
        )?;
    }

    Ok((report, has_diff))
}

fn diff_binding_graph_definitions(
    report: &mut String,
    binder_output: &BinderOutput,
    all_definitions: &[CollectedDefinition],
    definitions_by_id: &HashMap<NodeId, usize>,
) -> Result<bool> {
    let mut has_diff = false;
    let binding_graph = binder_output.compilation_unit.binding_graph();
    let mut binder_definitions = definitions_by_id
        .keys()
        .copied()
        .collect::<BTreeSet<NodeId>>();

    for graph_definition in binding_graph.all_definitions() {
        if graph_definition.get_file().is_built_ins() {
            continue;
        }
        let id = graph_definition.id();
        if binder_definitions.remove(&id) {
            continue;
        }
        has_diff = true;
        writeln!(report, "{graph_definition} not found in new binder output")?;
    }

    for definition_id in &binder_definitions {
        let report_id = definitions_by_id.get(definition_id).unwrap();
        let definition = &all_definitions[report_id - 1];
        writeln!(
            report,
            "{definition} not found in stack graph output",
            definition = definition.display(binder_output),
        )?;
        has_diff = true;
    }

    Ok(has_diff)
}

fn diff_binding_graph_references(
    report: &mut String,
    binder_output: &BinderOutput,
    all_references: &[CollectedReference],
    definitions_by_id: &HashMap<NodeId, usize>,
) -> Result<bool> {
    let mut has_diff = false;
    let binding_graph = binder_output.compilation_unit.binding_graph();
    let mut binder_references = all_references
        .iter()
        .map(|reference| (reference.cursor.node().id(), reference))
        .collect::<HashMap<_, _>>();

    for graph_reference in binding_graph.all_references() {
        if graph_reference.get_file().is_built_ins() {
            continue;
        }
        let id = graph_reference.id();
        let Some(binder_reference) = binder_references.remove(&id) else {
            has_diff = true;
            writeln!(report, "{graph_reference} not found in new binder output")?;
            continue;
        };
        let definitions = graph_reference.definitions();
        match &binder_reference.resolution {
            Resolution::Unresolved => {
                if !definitions.is_empty() {
                    has_diff = true;
                    writeln!(report, "{graph_reference} failed to resolve in new binder")?;
                }
            }
            Resolution::Definition(node_id) => match definitions.len() {
                1 => {
                    if definitions[0].id() != *node_id {
                        has_diff = true;
                        writeln!(
                            report,
                            "{reference} resolved to a different definition",
                            reference = binder_reference.display(definitions_by_id)
                        )?;
                    }
                }
                0 => {
                    has_diff = true;
                    writeln!(
                        report,
                        "{reference} resolved but did not in stack graph output",
                        reference = binder_reference.display(definitions_by_id)
                    )?;
                }
                _ => {
                    has_diff = true;
                    writeln!(
                        report,
                        "{reference} resolved uniquely but resolved ambiguously in stack graph output",
                        reference = binder_reference.display(definitions_by_id)
                    )?;
                }
            },
            Resolution::Ambiguous(node_ids) => {
                if node_ids.len() != definitions.len() {
                    has_diff = true;
                    writeln!(
                        report,
                        "{reference} has a different ambiguous resolution than {graph_reference}",
                        reference = binder_reference.display(definitions_by_id)
                    )?;
                }
            }
            Resolution::BuiltIn(_) => {
                if definitions.is_empty()
                    || definitions
                        .iter()
                        .all(|definition| definition.get_file().is_user())
                {
                    has_diff = true;
                    writeln!(
                        report,
                        "{reference} does not resolve to a built-in in the stack graph output",
                        reference = binder_reference.display(definitions_by_id)
                    )?;
                }
            }
        }
    }

    for reference in binder_references.values() {
        writeln!(
            report,
            "{reference} not found in stack graph output",
            reference = reference.display(definitions_by_id)
        )?;
        has_diff = true;
    }

    Ok(has_diff)
}
