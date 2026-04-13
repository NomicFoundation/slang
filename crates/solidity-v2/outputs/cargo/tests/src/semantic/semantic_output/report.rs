use std::collections::HashMap;
use std::fmt::Write;
use std::ops::Range;

use anyhow::Result;
use ariadne::{Color, Config, Label, Report, ReportBuilder, ReportKind, Source};
use slang_solidity_v2_ir::ir::NodeId;
use slang_solidity_v2_parser::ParserError;
use slang_solidity_v2_semantic::binder::Resolution;
use slang_solidity_v2_semantic::context::{SemanticContext, SemanticFile};
use solidity_v2_testing_utils::reporting::diagnostic;

use super::report_data::{
    CollectedDefinition, CollectedIdentifier, CollectedReference, ReportData,
};

const SEPARATOR: &str =
    "\n------------------------------------------------------------------------\n";

type Span<'a> = (&'a str, Range<usize>);
type BuilderType<'a> = ReportBuilder<'a, Span<'a>>;

pub(crate) fn binder_report(report_data: &'_ ReportData<'_>) -> Result<String> {
    let mut report = String::new();

    let ReportData {
        compilation,
        files,
        parse_errors,
        all_definitions,
        all_references,
        unbound_identifiers,
        definitions_by_id,
    } = report_data;

    if !parse_errors.is_empty() {
        report_parse_errors(&mut report, parse_errors, files)?;
        writeln!(report, "{SEPARATOR}")?;
    }

    report_all_definitions(&mut report, all_definitions, compilation.semantic())?;

    writeln!(report, "{SEPARATOR}")?;

    report_all_references(&mut report, all_references, definitions_by_id)?;

    writeln!(report, "{SEPARATOR}")?;

    report_unbound_identifiers(&mut report, unbound_identifiers)?;

    for file in &compilation.files() {
        writeln!(report, "{SEPARATOR}")?;

        let file_id = file.id();
        if let Some(contents) = files.get(file_id) {
            render_bindings_for_file(
                &mut report,
                file_id,
                contents,
                all_definitions,
                all_references,
                unbound_identifiers,
                definitions_by_id,
            )?;
        }
    }

    Ok(report)
}

fn report_all_definitions(
    report: &mut String,
    all_definitions: &[CollectedDefinition],
    semantic: &SemanticContext,
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
            definition = definition.display(semantic)
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
    unbound_identifiers: &[CollectedIdentifier],
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
            unbound_identifier = unbound_identifier.display_unbound(),
        )?;
    }
    Ok(())
}

fn report_parse_errors(
    report: &mut String,
    parse_errors: &[(String, ParserError)],
    file_contents: &HashMap<String, String>,
) -> Result<()> {
    writeln!(report, "Parse errors:")?;
    for (file_id, error) in parse_errors {
        let source = file_contents.get(file_id).cloned().unwrap_or_default();
        let rendered = diagnostic::render(error, file_id, &source, false);
        write!(report, "{rendered}")?;
    }
    Ok(())
}

fn render_bindings_for_file(
    report: &mut String,
    file_id: &str,
    contents: &str,
    all_definitions: &[CollectedDefinition],
    all_references: &[CollectedReference],
    unbound_identifiers: &[CollectedIdentifier],
    definitions_by_id: &HashMap<NodeId, usize>,
) -> Result<()> {
    let mut builder: BuilderType<'_> =
        Report::build(ReportKind::Custom("Bindings", Color::Unset), file_id, 0)
            .with_config(Config::default().with_color(false));

    let new_label = |range: &Range<usize>, message: &str| -> Label<Span<'_>> {
        // ariadne works with character offsets, not byte offsets, so we need to
        // convert ranges
        // TODO: the next ariadne release should allow byte offsets (see
        // https://github.com/NomicFoundation/slang/issues/1536)
        let char_range = {
            let start = contents[..range.start].chars().count();
            let end = contents[..range.end].chars().count();
            start..end
        };
        Label::new((file_id, char_range)).with_message(message)
    };

    for definition in all_definitions {
        if definition.identifier.file_id != file_id {
            continue;
        }

        let message = format!(
            "name: {definition_id}",
            definition_id = definition.report_id
        );
        builder.add_label(new_label(&definition.identifier.range, &message));
    }

    for reference in all_references {
        if reference.identifier.file_id != file_id {
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
        builder.add_label(new_label(&reference.identifier.range, &message));
    }

    for unbound_identifier in unbound_identifiers {
        if unbound_identifier.file_id != file_id {
            continue;
        }

        builder.add_label(new_label(&unbound_identifier.range, "???"));
    }

    let mut buffer = Vec::<u8>::new();
    builder
        .finish()
        .write((file_id, Source::from(contents)), &mut buffer)?;
    report.extend(String::from_utf8(buffer));

    Ok(())
}
