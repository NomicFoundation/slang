use std::iter::once;
use std::ops::Range;

use anyhow::Result;
use ariadne::{Color, Config, FnCache, Label, Report, ReportBuilder, ReportKind, Source};
use metaslang_bindings::ResolutionError;
use slang_solidity::bindings::{BindingGraph, Definition, Reference};
use slang_solidity::diagnostic;

use super::runner::ParsedPart;

type ReportSpan<'a> = (&'a str, Range<usize>);

pub(crate) fn render_bindings(
    binding_graph: &BindingGraph,
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

        let part_references = binding_graph
            .all_references()
            .filter(|reference| reference.get_file().is_user_path(part.path));
        let (bindings_report, part_all_resolved) =
            build_report_for_part(part, &all_definitions, part_references);
        all_resolved = all_resolved && part_all_resolved;

        let file_cache = FnCache::new(
            (move |id| Err(Box::new(format!("Failed to fetch source '{id}'")) as _)) as fn(&_) -> _,
        )
        .with_sources(
            once(part)
                .map(|part| (part.path, Source::from(part.contents)))
                .collect(),
        );
        bindings_report.write(file_cache, &mut buffer)?;
    }

    let result = String::from_utf8(buffer)?;
    Ok((result, all_resolved))
}

// We collect all non built-in definitions in a vector to be able to identify
// them by a numeric index
fn collect_all_definitions(binding_graph: &BindingGraph) -> Vec<Definition<'_>> {
    let mut definitions: Vec<Definition<'_>> = Vec::new();
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

fn build_report_for_part<'a>(
    part: &'a ParsedPart<'a>,
    all_definitions: &'a [Definition<'a>],
    part_references: impl Iterator<Item = Reference<'a>> + 'a,
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

        let message = format!("def: {}", index + 1);
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

        let definition = reference.resolve_definition();
        let message = match definition {
            Ok(definition) => {
                if definition.get_file().is_system() {
                    "ref: built-in".to_string()
                } else {
                    let def_id = all_definitions
                        .iter()
                        .position(|d| *d == definition)
                        .unwrap();
                    format!("ref: {}", def_id + 1)
                }
            }
            Err(ResolutionError::Unresolved) => {
                all_resolved = false;
                "unresolved".to_string()
            }
            Err(ResolutionError::AmbiguousDefinitions(ambiguous_definitions)) => {
                let ref_labels = ambiguous_definitions
                    .iter()
                    .filter_map(|ambiguous_definition| {
                        if ambiguous_definition.get_file().is_system() {
                            Some("built-in".to_string())
                        } else {
                            all_definitions
                                .iter()
                                .position(|d| d == ambiguous_definition)
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
