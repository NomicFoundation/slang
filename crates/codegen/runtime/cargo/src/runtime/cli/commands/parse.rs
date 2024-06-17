use std::fs;
use std::path::PathBuf;

use semver::Version;

use super::CommandError;
use crate::diagnostic;
use crate::language::Language;
use crate::parse_output::ParseOutput;

pub fn execute(file_path_string: &str, version: Version, json: bool) -> Result<(), CommandError> {
    parse_source_file(file_path_string, version, |output| {
        if json {
            let root_node = output.tree();
            let json = serde_json::to_string_pretty(&root_node).expect("JSON serialization failed");
            println!("{json}");
        }
    })
    .map(|_| ())
}

pub fn parse_source_file<F>(
    file_path_string: &str,
    version: Version,
    run_before_checking: F,
) -> Result<ParseOutput, CommandError>
where
    F: FnOnce(&ParseOutput),
{
    let file_path = PathBuf::from(&file_path_string)
        .canonicalize()
        .map_err(|_| CommandError::FileNotFound(file_path_string.to_string()))?;

    let input = fs::read_to_string(file_path)?;
    let language = Language::new(version)?;
    let parse_output = language.parse(Language::ROOT_KIND, &input);

    run_before_checking(&parse_output);

    if parse_output.is_valid() {
        Ok(parse_output)
    } else {
        const COLOR: bool = true;
        let report = parse_output
            .errors()
            .iter()
            .map(|error| diagnostic::render(error, file_path_string, &input, COLOR))
            .collect::<Vec<_>>()
            .join("\n");
        Err(CommandError::ParseFailed(report))
    }
}
