use std::{env::var, path::PathBuf};

use crate::paths::PathExtensions;
use anyhow::{bail, Result};
use ariadne::{Color, Label, Report, ReportKind, Source};

#[derive(Debug)]
pub struct InfraErrors {
    contents: Vec<ErrorDescriptor>,
}

impl InfraErrors {
    pub fn new() -> Self {
        return Self { contents: vec![] };
    }

    pub fn len(&self) -> usize {
        return self.contents.len();
    }

    pub fn single(file_path: PathBuf, range: Range, message: String) -> Self {
        let mut errors = Self::new();
        errors.push(file_path, range, message);
        return errors;
    }

    pub fn push(&mut self, file_path: PathBuf, range: Range, message: String) {
        self.contents.push(ErrorDescriptor {
            file_path,
            range,
            message,
        });
    }

    pub fn extend(&mut self, other: Self) {
        self.contents.extend(other.contents);
    }

    pub fn to_result(self) -> Result<()> {
        if self.contents.is_empty() {
            return Ok(());
        } else {
            bail!(self);
        }
    }
}

impl std::fmt::Display for InfraErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.contents {
            writeln!(f, "{error}")?;
        }

        return Ok(());
    }
}

impl std::error::Error for InfraErrors {}

#[derive(Debug, Clone)]
struct ErrorDescriptor {
    file_path: PathBuf,
    range: Range,
    message: String,
}

impl std::fmt::Display for ErrorDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if var("VSCODE_PROBLEM_MATCHER").is_ok() {
            self.write_problem_matcher(f)?;
            writeln!(f, "")?;
        }

        self.write_ariadne_report(f)?;

        return Ok(());
    }
}

impl ErrorDescriptor {
    fn write_ariadne_report(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let source_id = self.file_path.unwrap_str();
        let source = {
            let source = self.file_path.read_to_string().unwrap();
            if source.is_empty() {
                " ".to_string() // Ariadna cannot render on an empty string. Use whitespace instead.
            } else {
                source
            }
        };

        let start = self.range.start.offset.clamp(0, source.len() - 1);
        let end = self.range.end.offset.clamp(start + 1, source.len());

        let label = Label::new((source_id, start..end))
            .with_message(&self.message)
            .with_color(Color::Red);
        let report = Report::build(ReportKind::Error, source_id, start)
            .with_message(&self.message)
            .with_label(label)
            .finish();

        let mut buffer = Vec::<u8>::new();

        report
            .write((source_id, Source::from(&source)), &mut buffer)
            .unwrap();

        writeln!(f, "{}", String::from_utf8(buffer).unwrap())?;
        return Ok(());
    }

    fn write_problem_matcher(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "slang-problem-matcher:{file}:{line}:{column}-{end_line}:{end_column}: {severity}: {message}",
            file = self.file_path.unwrap_str(),
            line = self.range.start.line,
            column = self.range.start.column,
            end_line = self.range.end.line,
            end_column = self.range.end.column,
            severity = "error",
            message = self.message,
        )?;

        return Ok(());
    }
}

pub type Range = std::ops::Range<Position>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(offset: usize, line: usize, column: usize) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }
}
