use std::iter::once;
use std::ops::Range;

use anyhow::Result;
use ariadne::{Color, Config, FnCache, Label, Report, ReportBuilder, ReportKind, Source};
use metaslang_bindings::ResolutionError;
use slang_solidity::bindings::{Bindings, Definition, Reference};
use slang_solidity::diagnostic;

use super::runner::ParsedPart;

type ReportSpan<'a> = (&'a str, Range<usize>);

pub(crate) fn render_bindings(
    bindings: &Bindings,
    parsed_parts: &[ParsedPart<'_>],
) -> Result<(String, bool)> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut all_resolved = true;

    let all_definitions = collect_all_definitions(bindings);

    for part in parsed_parts {
        if !part.parse_output.is_valid() {
            let mut parse_errors_report = part.parse_errors_report().into_bytes();
            buffer.extend_from_slice("Parse errors:\n".as_bytes());
            buffer.append(&mut parse_errors_report);
            buffer.push(b'\n');
        }

        let part_references = bindings.all_references().filter(|reference| {
            let ref_file = reference.get_file().expect("reference should be in a file");
            ref_file == part.path
        });
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
fn collect_all_definitions(bindings: &Bindings) -> Vec<Definition<'_>> {
    let mut definitions: Vec<Definition<'_>> = Vec::new();
    for definition in bindings.all_definitions() {
        if !definition.is_built_in() && definition.get_cursor().is_some() {
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
        let Some(cursor) = definition.get_cursor() else {
            continue;
        };
        let def_file = definition
            .get_file()
            .expect("definition should be in a file");
        if def_file != part.path {
            continue;
        }

        let range = {
            let range = cursor.text_range();
            let start = part.contents[..range.start.utf8].chars().count();
            let end = part.contents[..range.end.utf8].chars().count();
            start..end
        };

        let message = format!("def: {}", index + 1);
        builder = builder.with_label(Label::new((part.path, range)).with_message(message));
    }

    let mut all_resolved = true;

    for reference in part_references {
        let Some(cursor) = reference.get_cursor() else {
            continue;
        };

        let range = {
            let range = cursor.text_range();
            let start = part.contents[..range.start.utf8].chars().count();
            let end = part.contents[..range.end.utf8].chars().count();
            start..end
        };

        let definition = reference.jump_to_definition();
        let message = match definition {
            Ok(definition) => {
                if definition.is_built_in() {
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
                        if ambiguous_definition.is_built_in() {
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
