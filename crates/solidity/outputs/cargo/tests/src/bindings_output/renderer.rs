use std::ops::Range;

use anyhow::Result;
use ariadne::{Color, Config, FnCache, Label, Report, ReportBuilder, ReportKind, Source};
use metaslang_bindings::ResolutionError;
use slang_solidity::bindings::{Bindings, Definition};
use slang_solidity::diagnostic;

use super::runner::ParsedPart;

pub(crate) fn render_bindings(
    bindings: &Bindings,
    parsed_parts: &[ParsedPart<'_>],
) -> Result<String> {
    let mut buffer: Vec<u8> = Vec::new();

    let mut definitions: Vec<Definition<'_>> = Vec::new();
    for definition in bindings.all_definitions() {
        if !definition.is_builtin() && definition.get_cursor().is_some() {
            definitions.push(definition);
        };
    }

    for part in parsed_parts {
        let mut builder: ReportBuilder<'_, (&str, Range<usize>)> = Report::build(
            ReportKind::Custom("References and definitions", Color::Unset),
            part.path,
            0,
        )
        .with_config(Config::default().with_color(false));

        if !part.parse_output.is_valid() {
            let mut report = part
                .parse_output
                .errors()
                .iter()
                .map(|error| diagnostic::render(error, part.path, part.contents, false))
                .collect::<Vec<_>>()
                .join("\n")
                .into_bytes();
            buffer.extend_from_slice("Parse errors:\n".as_bytes());
            buffer.append(&mut report);
            buffer.push(b'\n');
        }

        for (index, definition) in definitions.iter().enumerate() {
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

        for reference in bindings.all_references() {
            let Some(cursor) = reference.get_cursor() else {
                continue;
            };
            let ref_file = reference.get_file().expect("reference should be in a file");
            if ref_file != part.path {
                continue;
            }

            let range = {
                let range = cursor.text_range();
                let start = part.contents[..range.start.utf8].chars().count();
                let end = part.contents[..range.end.utf8].chars().count();
                start..end
            };

            let definition = reference.jump_to_definition();
            let message = match definition {
                Ok(definition) => {
                    if definition.is_builtin() {
                        "ref: built-in".to_string()
                    } else {
                        let def_id = definitions.iter().position(|d| *d == definition).unwrap();
                        format!("ref: {}", def_id + 1)
                    }
                }
                Err(ResolutionError::Unresolved) => "unresolved".to_string(),
                Err(ResolutionError::AmbiguousDefinitions(ambiguous_definitions)) => {
                    let ref_labels = ambiguous_definitions
                        .iter()
                        .filter_map(|ambiguous_definition| {
                            definitions
                                .iter()
                                .position(|d| d == ambiguous_definition)
                                .map(|index| format!("ref: {}", index + 1))
                        })
                        .collect::<Vec<_>>();
                    format!("ambiguous: {}", ref_labels.join(", "))
                }
            };

            builder = builder.with_label(Label::new((part.path, range)).with_message(message));
        }

        let report = builder.finish();
        let file_cache = FnCache::new(
            (move |id| Err(Box::new(format!("Failed to fetch source '{id}'")) as _)) as fn(&_) -> _,
        )
        .with_sources(
            parsed_parts
                .iter()
                .map(|part| (part.path, Source::from(part.contents)))
                .collect(),
        );
        report.write(file_cache, &mut buffer)?;
    }

    let result = String::from_utf8(buffer)?;
    Ok(result)
}
