// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use semver::Version;

use crate::diagnostic;
use crate::parser::Language;

#[derive(Parser, Debug)]
pub struct ParseCommand {
    /// File path to the source file to parse
    file_path: PathBuf,

    /// The language version to use for parsing
    #[arg(short, long)]
    version: Version,

    /// Print the concrete syntax tree as JSON
    #[clap(long)]
    json: bool,
}

impl ParseCommand {
    pub fn execute(self) {
        let Self {
            file_path,
            version,
            json,
        } = self;

        let file_path = file_path
            .canonicalize()
            .unwrap_or_else(|_| panic!("File not found: {file_path:?}"));

        let input = fs::read_to_string(&file_path).unwrap();
        let language = Language::new(version).unwrap();
        let parse_output = language.parse(Language::ROOT_KIND, &input);

        if !parse_output.is_valid() {
            const COLOR: bool = true;

            let report = parse_output
                .errors()
                .iter()
                .map(|error| diagnostic::render(error, file_path.to_str().unwrap(), &input, COLOR))
                .collect::<Vec<_>>()
                .join("\n");

            panic!("Parse failed:\n{report}");
        }

        if json {
            let json = serde_json::to_string_pretty(&parse_output.tree()).unwrap();

            println!("{json}");
        }
    }
}
