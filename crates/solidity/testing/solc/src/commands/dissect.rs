//! Dissection runs from first to last version. It records a snapshot of the output (errors) for each version.
//! If the output changes, it prints the previous snapshot and its version range, then it starts a new one.
//! This way, it can show which versions succeeded/failed and what the errors were.
//! This is useful when working on the Solidity grammar, to detect syntactic breaking changes.

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
    Binary, CliInput, Error, InputSource, LanguageSelector, Severity, SourceLocation,
};

/// Compiles a Solidity file with all versions of `solc`, listing which versions succeeded/failed.
#[derive(Debug, Parser)]
pub struct DissectCommand {
    /// File path to the Solidity (*.sol) source file to parse
    file: PathBuf,
}

impl DissectCommand {
    pub fn execute(self) -> Result<()> {
        let language = SolidityDefinition::create();
        let binaries = Binary::fetch_all(&language)?;

        let mut dissector = Dissector::new(self.file.clone())?;

        for binary in binaries {
            dissector.inspect_version(&binary)?;
        }

        dissector.flush()?;

        Ok(())
    }
}

struct Dissector {
    file: PathBuf,
    source: String,

    snapshot: Option<Snapshot>,
    versions_so_far: Vec<Version>,
}

#[derive(Debug, Eq, PartialEq)]
struct Snapshot {
    errors: Vec<Error>,
}

impl Dissector {
    fn new(file: PathBuf) -> Result<Self> {
        let source = file.read_to_string()?;

        Ok(Self {
            file,
            source,

            snapshot: None,
            versions_so_far: vec![],
        })
    }

    fn inspect_version(&mut self, binary: &Binary) -> Result<()> {
        let new_snapshot = self.calculate_snapshot(binary);

        match self.snapshot {
            // If it is the first snapshot, store it:
            None => self.snapshot = Some(new_snapshot),

            // If it is the same snapshot, do nothing:
            Some(ref existing_snapshot) if existing_snapshot == &new_snapshot => {}

            // Otherwise, print the existing one, and start fresh:
            _ => {
                self.flush()?;
                self.snapshot = Some(new_snapshot);
            }
        };

        self.versions_so_far.push(binary.version.clone());

        Ok(())
    }

    fn calculate_snapshot(&mut self, binary: &Binary) -> Snapshot {
        let input = CliInput {
            language: LanguageSelector::Solidity,
            sources: [(
                self.file.unwrap_string(),
                InputSource {
                    content: self.source.clone(),
                },
            )]
            .into(),
        };

        let errors = match binary.run(&input) {
            // Forward the compiler errors, if any:
            Ok(output) => output.errors.unwrap_or_default(),

            // Normalize any process/execution errors into the same compiler error types:
            Err(error) => vec![Error {
                message: format!("{error}"),
                severity: Severity::Error,
                location: None,
            }],
        };

        Snapshot { errors }
    }

    fn flush(&mut self) -> Result<()> {
        let versions_so_far = std::mem::take(&mut self.versions_so_far);

        let Snapshot { errors } = std::mem::take(&mut self.snapshot).expect("No snapshot to flush");

        let highest_severity = errors.iter().map(|e| &e.severity).max();

        let (color, status) = match highest_severity {
            None => (Color::Green, "Success".to_string()),
            Some(severity) => (get_color(severity), severity.to_string()),
        };

        let version_range = match &versions_so_far[..] {
            [] => panic!("No versions to flush."),
            [single] => format!("'{single}'"),
            [first, .., last] => format!("'{first}' - '{last}'"),
        };

        let header = format!("[{status}] {version_range}");
        Terminal::step(style(header).fg(color).bright().bold().to_string());

        errors
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
