use std::path::PathBuf;

use ariadne::{Color, Label, Report, ReportKind, Source};

pub type CodegenResult<T> = Result<T, CodegenErrors>;

#[derive(Debug)]
pub struct CodegenErrors {
    contents: Vec<ErrorDescriptor>,
}

impl CodegenErrors {
    pub fn new() -> Self {
        return Self { contents: vec![] };
    }

    pub fn len(&self) -> usize {
        return self.contents.len();
    }

    pub fn single<TError: std::error::Error>(
        file_path: &PathBuf,
        range: &Range,
        error: TError,
    ) -> Self {
        let mut errors = Self::new();
        errors.push(file_path, range, error);
        return errors;
    }

    pub fn push<TError: std::error::Error>(
        &mut self,
        file_path: &PathBuf,
        range: &Range,
        error: TError,
    ) {
        self.contents.push(ErrorDescriptor {
            file_path: file_path.to_owned(),
            range: range.to_owned(),
            message: error.to_string(),
        });
    }

    pub fn extend(&mut self, other: Self) {
        self.contents.extend(other.contents);
    }

    pub fn err_or<TSuccess>(self, success: TSuccess) -> CodegenResult<TSuccess> {
        if self.contents.is_empty() {
            return Ok(success);
        } else {
            return Err(self);
        }
    }
}

impl std::fmt::Display for CodegenErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in &self.contents {
            writeln!(f, "{error}").unwrap();
        }

        return Ok(());
    }
}

impl std::error::Error for CodegenErrors {}

#[derive(Debug, Clone)]
struct ErrorDescriptor {
    file_path: PathBuf,
    range: Range,
    message: String,
}

impl std::fmt::Display for ErrorDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if std::env::var("VSCODE_PROBLEM_MATCHER").is_ok() {
            writeln!(f, "{}", self.to_problem_matcher())?;
            writeln!(f, "")?;
        }

        writeln!(f, "{}", self.to_ariadne_report())?;

        return Ok(());
    }
}

impl ErrorDescriptor {
    fn to_ariadne_report(&self) -> String {
        let source_id = self.file_path.to_str().unwrap();
        let source = {
            let source = std::fs::read_to_string(&self.file_path).unwrap();
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

        let mut w = Vec::<u8>::new();
        report
            .write((source_id, Source::from(&source)), &mut w)
            .unwrap();

        return String::from_utf8(w).unwrap();
    }

    fn to_problem_matcher(&self) -> String {
        return format!(
            "slang-problem-matcher:{file}:{line}:{column}-{end_line}:{end_column}: {severity}: {message}",
            file = self.file_path.to_str().unwrap(),
            line = self.range.start.line,
            column = self.range.start.column,
            end_line = self.range.end.line,
            end_column = self.range.end.column,
            severity = "error",
            message = self.message,
        );
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

    pub fn zero() -> Self {
        Self::new(0, 1, 1)
    }

    pub fn inside(&self, range: &Range) -> bool {
        return range.start.offset <= self.offset && self.offset <= range.end.offset;
    }
}
