use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use console::{style, Color};
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::Terminal;
use itertools::Itertools;
use semver::Version;
use solidity_language::SolidityDefinition;

use crate::utils::{
    ApiInput, Binary, Error, InputSource, LanguageSelector, Severity, SourceLocation,
};

#[derive(Debug, Parser)]
pub struct DissectCommand {
    /// File path to the Solidity (*.sol) source file to parse
    file: PathBuf,
}

impl DissectCommand {
    pub fn execute(self) -> Result<()> {
        let language = SolidityDefinition::create();
        let binaries = Binary::fetch_all(&language)?;

        let mut state = State::new(self.file.to_owned())?;

        for binary in binaries {
            let input = InputSource {
                content: state.source.to_owned(),
            };

            let output = binary.run(&ApiInput {
                language: LanguageSelector::Solidity,
                sources: [(self.file.unwrap_string(), input)].into(),
            });

            let errors = match output {
                Ok(output) => output.errors.unwrap_or_default(),
                Err(error) => vec![Error {
                    message: format!("{error}"),
                    severity: Severity::Error,
                    location: None,
                }],
            };

            state.apply(binary.version.to_owned(), errors)?;
        }

        state.print_all()?;

        Ok(())
    }
}

struct State {
    file: PathBuf,
    source: String,

    errors: Vec<Error>,
    versions: Vec<Version>,
}

impl State {
    fn new(file: PathBuf) -> Result<Self> {
        let source = file.read_to_string()?;

        Ok(Self {
            file,
            source,
            errors: Vec::new(),
            versions: Vec::new(),
        })
    }

    fn apply(&mut self, version: Version, errors: Vec<Error>) -> Result<()> {
        if self.versions.is_empty() {
            assert_eq!(self.errors, vec![], "Impossible state");
            self.errors = errors;
            self.versions.push(version);
        } else if self.errors == errors {
            self.versions.push(version);
        } else {
            self.print_all()?;
            self.errors = errors;
            self.versions = vec![version];
        }

        Ok(())
    }

    fn print_all(&self) -> Result<()> {
        let highest_severity = self.errors.iter().map(|e| &e.severity).max();

        let (color, status) = match highest_severity {
            None => (Color::Green, "Success".into()),
            Some(severity) => (get_color(severity), severity.to_string()),
        };

        let versions = match &self.versions[..] {
            [] => unreachable!("Impossible state"),
            [single] => format!("'{single}'"),
            [first, .., last] => format!("'{first}' - '{last}'"),
        };

        let header = format!("[{status}] {versions}");
        Terminal::step(style(header).fg(color).bright().bold().to_string());

        self.errors
            .iter()
            .sorted_by(|a, b| Ord::cmp(&b.severity, &a.severity))
            .map(|error| self.print_error(error))
            .try_collect()
    }

    fn print_error(&self, error: &Error) -> Result<()> {
        let Error {
            message,
            severity,
            location,
        } = error;

        let (start, end) = match location {
            Some(SourceLocation { start, end, file })
                if !start.is_negative() && !end.is_negative() =>
            {
                assert_eq!(file, &self.file);

                #[allow(clippy::cast_sign_loss)]
                (*start as usize, *end as usize)
            }
            _ => {
                println!(
                    "[{severity}] {message}",
                    severity = style(severity).fg(get_color(severity))
                );

                return Ok(());
            }
        };

        let kind = match severity {
            Severity::Info => ariadne::ReportKind::Advice,
            Severity::Warning => ariadne::ReportKind::Warning,
            Severity::Error => ariadne::ReportKind::Error,
        };

        let source_id = self.file.unwrap_str();

        let label = ariadne::Label::new((source_id, start..end))
            .with_message("Error occurred here.".to_string());

        let source = ariadne::Source::from(&self.source);

        ariadne::Report::build(kind, source_id, start)
            .with_message(message)
            .with_label(label)
            .finish()
            .write((source_id, source), &mut std::io::stdout())?;

        Ok(())
    }
}

fn get_color(severity: &Severity) -> Color {
    match severity {
        Severity::Info => Color::White,
        Severity::Warning => Color::Yellow,
        Severity::Error => Color::Red,
    }
}
