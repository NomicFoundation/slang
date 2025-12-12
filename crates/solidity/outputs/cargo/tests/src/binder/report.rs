use std::collections::HashMap;
use std::fmt::Write;
use std::ops::Range;

use anyhow::Result;
use ariadne::{Color, Config, Label, Report, ReportBuilder, ReportKind, Source};
use slang_solidity::backend::binder::Resolution;
use slang_solidity::backend::{SemanticAnalysis, SemanticFile};
use slang_solidity::cst::{Cursor, NodeId};
use slang_solidity::diagnostic;

use super::report_data::{CollectedDefinition, CollectedReference, ReportData, UnboundIdentifier};

const SEPARATOR: &str =
    "\n------------------------------------------------------------------------\n";

type Span<'a> = (&'a str, Range<usize>);
type BuilderType<'a> = ReportBuilder<'a, Span<'a>>;

pub(crate) fn binder_report(report_data: &'_ ReportData<'_>) -> Result<String> {
    let mut report = String::new();

    let ReportData {
        semantic_analysis,
        all_definitions,
        all_references,
        unbound_identifiers,
        definitions_by_id,
    } = report_data;

    if report_data.has_parse_errors() {
        report_parse_errors(&mut report, semantic_analysis)?;

        writeln!(report, "{SEPARATOR}")?;
    }

    report_all_definitions(&mut report, semantic_analysis, all_definitions)?;

    writeln!(report, "{SEPARATOR}")?;

    report_all_references(&mut report, all_references, definitions_by_id)?;

    writeln!(report, "{SEPARATOR}")?;

    report_unbound_identifiers(&mut report, unbound_identifiers)?;

    for file in &semantic_analysis.files() {
        writeln!(report, "{SEPARATOR}")?;

        render_bindings_for_file(
            &mut report,
            file,
            all_definitions,
            all_references,
            unbound_identifiers,
            definitions_by_id,
        )?;
    }

    Ok(report)
}

fn report_parse_errors(report: &mut String, semantic_analysis: &SemanticAnalysis) -> Result<()> {
    writeln!(report, "Parse errors:")?;
    for file in &semantic_analysis.files() {
        let source_id = file.id();
        let source = file.tree().unparse();
        for error in file.errors() {
            writeln!(
                report,
                "{}",
                diagnostic::render(error, source_id, &source, false)
            )?;
        }
    }
    Ok(())
}

fn report_all_definitions(
    report: &mut String,
    semantic_analysis: &SemanticAnalysis,
    all_definitions: &[CollectedDefinition],
) -> Result<()> {
    writeln!(
        report,
        "Definitions ({definitions_count}):",
        definitions_count = all_definitions.len(),
    )?;
    for definition in all_definitions {
        writeln!(
            report,
            "- {definition}",
            definition = definition.display(semantic_analysis)
        )?;
    }
    Ok(())
}

fn report_all_references(
    report: &mut String,
    all_references: &[CollectedReference],
    definitions_by_id: &HashMap<NodeId, usize>,
) -> Result<()> {
    writeln!(
        report,
        "References ({references_count}):",
        references_count = all_references.len()
    )?;
    for reference in all_references {
        writeln!(
            report,
            "- {reference}",
            reference = reference.display(definitions_by_id)
        )?;
    }
    Ok(())
}

fn report_unbound_identifiers(
    report: &mut String,
    unbound_identifiers: &[UnboundIdentifier],
) -> Result<()> {
    writeln!(
        report,
        "Unbound identifiers ({unbound_identifiers_count}):",
        unbound_identifiers_count = unbound_identifiers.len()
    )?;
    for unbound_identifier in unbound_identifiers {
        writeln!(
            report,
            "- {unbound_identifier}",
            unbound_identifier = unbound_identifier.display()
        )?;
    }
    Ok(())
}

fn render_bindings_for_file(
    report: &mut String,
    file: &SemanticFile,
    all_definitions: &[CollectedDefinition],
    all_references: &[CollectedReference],
    unbound_identifiers: &[UnboundIdentifier],
    definitions_by_id: &HashMap<NodeId, usize>,
) -> Result<()> {
    let contents = file.tree().unparse();
    let mut builder: BuilderType<'_> =
        Report::build(ReportKind::Custom("Bindings", Color::Unset), file.id(), 0)
            .with_config(Config::default().with_color(false));

    let new_label = |cursor: &Cursor, message: &str| -> Label<Span<'_>> {
        let range = {
            let range = cursor.text_range();
            let start = contents[..range.start.utf8].chars().count();
            let end = contents[..range.end.utf8].chars().count();
            start..end
        };
        Label::new((file.id(), range)).with_message(message)
    };

    for definition in all_definitions {
        if definition.file_id != file.id() {
            continue;
        }

        let message = format!(
            "name: {definition_id}",
            definition_id = definition.report_id
        );
        builder.add_label(new_label(&definition.cursor, &message));
    }

    for reference in all_references {
        if reference.file_id != file.id() {
            continue;
        }

        let message = match &reference.resolution {
            Resolution::Unresolved => "unresolved".to_string(),
            Resolution::BuiltIn(_) => "built-in".to_string(),
            Resolution::Definition(definition_id) => {
                format!(
                    "ref: {definition_id}",
                    definition_id = definitions_by_id.get(definition_id).unwrap()
                )
            }
            Resolution::Ambiguous(definitions) => {
                format!(
                    "refs: {ids:?}",
                    ids = definitions
                        .iter()
                        .map(|id| definitions_by_id.get(id).unwrap())
                        .collect::<Vec<_>>()
                )
            }
        };
        builder.add_label(new_label(&reference.cursor, &message));
    }

    for unbound_identifier in unbound_identifiers {
        if unbound_identifier.file_id != file.id() {
            continue;
        }

        builder.add_label(new_label(&unbound_identifier.cursor, "???"));
    }

    let mut buffer = Vec::<u8>::new();
    builder
        .finish()
        .write((file.id(), Source::from(contents)), &mut buffer)?;
    report.extend(String::from_utf8(buffer));

    Ok(())
}
