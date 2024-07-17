use std::ops::Range;
use std::path::Path;

use anyhow::Result;
use ariadne::{Color, Config, Label, Report, ReportBuilder, ReportKind, Source};
use infra_utils::paths::PathExtensions;
use semver::Version;
use slang_solidity::bindings::{self, Handle};
use slang_solidity::parse_output::ParseOutput;

pub(crate) fn render_bindings(
    version: &Version,
    parse_output: &ParseOutput,
    source: &str,
    source_path: &Path,
) -> Result<String> {
    let mut bindings = bindings::create(version.clone());
    bindings.add_file(
        source_path.to_str().unwrap(),
        parse_output.create_tree_cursor(),
    );

    let source_id = source_path.strip_repo_root()?.unwrap_str();
    let mut builder: ReportBuilder<'_, (&str, Range<usize>)> = Report::build(
        ReportKind::Custom("References and definitions", Color::Unset),
        source_id,
        0,
    )
    .with_config(Config::default().with_color(false));

    if !parse_output.is_valid() {
        builder = builder.with_note("WARNING: Parsing failed. Results may be incomplete.");
    }

    let mut definitions: Vec<Handle<'_>> = Vec::new();

    for definition in bindings.all_definitions() {
        let Some(cursor) = definition.get_cursor() else {
            continue;
        };

        let range = {
            let range = cursor.text_range();
            let start = source[..range.start.utf8].chars().count();
            let end = source[..range.end.utf8].chars().count();
            start..end
        };

        definitions.push(definition);
        let message = format!("def: {}", definitions.len());
        builder = builder.with_label(Label::new((source_id, range)).with_message(message));
    }

    for reference in bindings.all_references() {
        let Some(cursor) = reference.get_cursor() else {
            continue;
        };

        let range = {
            let range = cursor.text_range();
            let start = source[..range.start.utf8].chars().count();
            let end = source[..range.end.utf8].chars().count();
            start..end
        };

        let definition = reference.jump_to_definition();
        let message = match definition {
            None => "unresolved".to_string(),
            Some(definition) => {
                let def_id = definitions.iter().position(|d| *d == definition).unwrap();
                format!("ref: {}", def_id + 1)
            }
        };

        builder = builder.with_label(Label::new((source_id, range)).with_message(message));
    }

    let report = builder.finish();
    let mut buffer = Vec::new();
    report.write((source_id, Source::from(source)), &mut buffer)?;

    let result = String::from_utf8(buffer)?;
    Ok(result)
}
