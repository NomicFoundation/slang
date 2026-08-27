use std::iter::once;
use std::ops::Range;
use std::rc::Rc;

use anyhow::Result;
use ariadne::{Color, Config, FnCache, Label, Report, ReportBuilder, ReportKind, Source};
use slang_solidity::bindings::{BindingGraph, Definition, Reference};
use slang_solidity::cst::{NodeKind, TerminalKindExtensions};
use slang_solidity::diagnostic;

use super::runner::ParsedPart;

type ReportSpan<'a> = (&'a str, Range<usize>);

pub(crate) fn render_bindings(
    binding_graph: &Rc<BindingGraph>,
    parsed_parts: &[ParsedPart<'_>],
) -> Result<(String, bool)> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut all_resolved = true;

    let all_definitions = collect_all_definitions(binding_graph);

    for part in parsed_parts {
        if !part.parse_output.is_valid() {
            let mut parse_errors_report = part.parse_errors_report().into_bytes();
            buffer.extend_from_slice("Parse errors:\n".as_bytes());
            buffer.append(&mut parse_errors_report);
            buffer.push(b'\n');
        }

        let (coverage_report, part_identifiers_bound) =
            check_bindings_coverage(part, binding_graph);
        all_resolved = all_resolved && part_identifiers_bound;
        if !part_identifiers_bound {
            write_part_report(part, &coverage_report, &mut buffer)?;
        }

        let part_references = binding_graph
            .all_references()
            .filter(|reference| reference.get_file().is_user_path(part.path));
        let (bindings_report, part_all_resolved) =
            build_report_for_part(part, &all_definitions, part_references);
        all_resolved = all_resolved && part_all_resolved;
        write_part_report(part, &bindings_report, &mut buffer)?;

        let definiens_report = build_definiens_report(part, &all_definitions);
        write_part_report(part, &definiens_report, &mut buffer)?;
    }

    let result = String::from_utf8(buffer)?;
    Ok((result, all_resolved))
}

fn write_part_report<'a>(
    part: &'a ParsedPart<'a>,
    report: &Report<'a, ReportSpan<'a>>,
    buffer: &'a mut Vec<u8>,
) -> Result<()> {
    let file_cache = FnCache::new(
        (move |id| Err(Box::new(format!("Failed to fetch source '{id}'")) as _)) as fn(&_) -> _,
    )
    .with_sources(
        once(part)
            .map(|part| (part.path, Source::from(part.contents)))
            .collect(),
    );
    report.write(file_cache, buffer)?;
    Ok(())
}

// We collect all non built-in definitions in a vector to be able to identify
// them by a numeric index
fn collect_all_definitions(binding_graph: &Rc<BindingGraph>) -> Vec<Definition> {
    let mut definitions: Vec<Definition> = Vec::new();
    for definition in binding_graph.all_definitions() {
        if definition.get_file().is_user() {
            definitions.push(definition);
        }
    }
    definitions
}

impl ParsedPart<'_> {
    fn parse_errors_report(&self) -> String {
        self.parse_output
            .errors()
            .iter()
            .map(|error| diagnostic::render(error, self.path, self.contents, false))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn check_bindings_coverage<'a>(
    part: &'a ParsedPart<'a>,
    binding_graph: &Rc<BindingGraph>,
) -> (Report<'a, ReportSpan<'a>>, bool) {
    let mut all_identifiers_bound = true;
    let mut builder: ReportBuilder<'_, ReportSpan<'_>> = Report::build(
        ReportKind::Custom("Missing definitions/references:", Color::Unset),
        part.path,
        0,
    )
    .with_config(Config::default().with_color(false));

    let mut cursor = part.parse_output.create_tree_cursor();

    while cursor.go_to_next_terminal() {
        if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
            continue;
        }

        if binding_graph.definition_at(&cursor).is_none()
            && binding_graph.reference_at(&cursor).is_none()
        {
            let range = {
                let range = cursor.text_range();
                let start = part.contents[..range.start.utf8].chars().count();
                let end = part.contents[..range.end.utf8].chars().count();
                start..end
            };

            builder = builder.with_label(
                Label::new((part.path, range)).with_message("missing definition/reference"),
            );
            all_identifiers_bound = false;
        }
    }

    (builder.finish(), all_identifiers_bound)
}

fn build_report_for_part<'a>(
    part: &'a ParsedPart<'a>,
    all_definitions: &'a [Definition],
    part_references: impl Iterator<Item = Reference> + 'a,
) -> (Report<'a, ReportSpan<'a>>, bool) {
    let mut builder: ReportBuilder<'_, ReportSpan<'_>> = Report::build(
        ReportKind::Custom("References and definitions", Color::Unset),
        part.path,
        0,
    )
    .with_config(Config::default().with_color(false));

    for (index, definition) in all_definitions.iter().enumerate() {
        if !definition.get_file().is_user_path(part.path) {
            continue;
        }

        let range = {
            let range = definition.get_cursor().text_range();
            let start = part.contents[..range.start.utf8].chars().count();
            let end = part.contents[..range.end.utf8].chars().count();
            start..end
        };

        let message = format!("name: {}", index + 1);
        builder = builder.with_label(Label::new((part.path, range)).with_message(message));
    }

    let mut all_resolved = true;

    for reference in part_references {
        let range = {
            let range = reference.get_cursor().text_range();
            let start = part.contents[..range.start.utf8].chars().count();
            let end = part.contents[..range.end.utf8].chars().count();
            start..end
        };

        let definitions = reference.definitions();
        let message = match definitions.len() {
            0 => {
                all_resolved = false;
                "unresolved".to_string()
            }
            1 => {
                let definition = definitions.first().unwrap();
                if definition.get_file().is_built_ins() {
                    "ref: built-in".to_string()
                } else {
                    let def_id = all_definitions
                        .iter()
                        .position(|d| d == definition)
                        .unwrap();
                    format!("ref: {}", def_id + 1)
                }
            }
            _ => {
                let ref_labels = definitions
                    .iter()
                    .filter_map(|definition| {
                        if definition.get_file().is_built_ins() {
                            Some("built-in".to_string())
                        } else {
                            all_definitions
                                .iter()
                                .position(|d| d == definition)
                                .map(|index| format!("{}", index + 1))
                        }
                    })
                    .collect::<Vec<_>>();
                format!("refs: {ref_labels}", ref_labels = ref_labels.join(", "))
            }
        };

        builder = builder.with_label(Label::new((part.path, range)).with_message(message));
    }

    (builder.finish(), all_resolved)
}

fn build_definiens_report<'a>(
    part: &'a ParsedPart<'a>,
    all_definitions: &'a [Definition],
) -> Report<'a, ReportSpan<'a>> {
    let mut builder: ReportBuilder<'_, ReportSpan<'_>> =
        Report::build(ReportKind::Custom("Definiens", Color::Unset), part.path, 0)
            .with_config(Config::default().with_color(false));

    for (index, definition) in all_definitions.iter().enumerate() {
        if !definition.get_file().is_user_path(part.path) {
            continue;
        }

        let range = {
            let range = definition.get_definiens_cursor().text_range();
            let start = part.contents[..range.start.utf8].chars().count();
            let end = part.contents[..range.end.utf8].chars().count();
            start..end
        };

        let message = format!("definiens: {}", index + 1);
        builder = builder.with_label(Label::new((part.path, range)).with_message(message));
    }

    builder.finish()
}
