use std::cmp;

use console::Color;
use indicatif::ProgressBar;
use infra_utils::github::GitHub;

use crate::reporting::Reporter;
use crate::results::ShardResults;

const MAX_PRINTED_FAILURES: u64 = 1000;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TestOutcome {
    Passed,
    Failed,
    Unresolved,
    Incompatible,
}

pub struct Events {
    reporter: Reporter,

    all_archives: ProgressBar,
    current_archive: ProgressBar,

    source_files: ProgressBar,

    passed: ProgressBar,
    failed: ProgressBar,
    unresolved: ProgressBar,
    incompatible: ProgressBar,

    definitions: ProgressBar,
    references: ProgressBar,
    unresolved_references: ProgressBar,
}

impl Events {
    pub fn new(archives_count: usize, files_count: usize) -> Self {
        let mut reporter = Reporter::new();

        reporter.add_blank();

        let all_archives = reporter.add_progress("All Archives", archives_count);
        let current_archive = reporter.add_progress("Current Archives", 0);

        reporter.add_blank();

        let source_files = reporter.add_counter("📄 Source Files", Color::White, files_count);

        reporter.add_blank();
        reporter.add_label("Contract Stats:");

        let passed = reporter.add_counter("✅ Passed", Color::Green, 0);
        let failed = reporter.add_counter("❌ Failed", Color::Red, 0);
        let unresolved = reporter.add_counter("❔ Unresolved", Color::White, 0);
        let incompatible = reporter.add_counter("❕ Incompatible", Color::White, 0);

        let definitions = reporter.add_counter("Definitions", Color::White, 0);
        let references = reporter.add_counter("References", Color::White, 0);
        let unresolved_references = reporter.add_counter("Unresolved refs", Color::White, 0);

        reporter.add_blank();

        Self {
            reporter,

            all_archives,
            current_archive,

            source_files,

            passed,
            failed,
            unresolved,
            incompatible,

            definitions,
            references,
            unresolved_references,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn failure_count(&self) -> usize {
        self.failed.position() as usize
    }

    pub fn start_archive(&mut self, contract_count: usize) {
        self.current_archive.reset();
        self.current_archive.set_length(contract_count as u64);

        self.reporter.show();
    }

    pub fn inc_files_count(&self, additional_files: usize) {
        self.source_files.inc_length(additional_files as u64);
    }

    pub fn inc_files_processed(&self, files_processed: usize) {
        self.source_files.inc(files_processed as u64);
    }

    pub fn inc_definitions(&self, value: usize) {
        self.definitions.inc(value as u64);
    }

    pub fn inc_references(&self, value: usize) {
        self.references.inc(value as u64);
    }

    pub fn inc_unresolved_references(&self, value: usize) {
        self.unresolved_references.inc(value as u64);
    }

    pub fn finish_archive(&mut self) {
        self.all_archives.inc(1);

        self.reporter.hide();

        GitHub::collapse_group("Statistics:", || {
            self.reporter.print_full_report();
        });
    }

    pub fn test(&self, outcome: TestOutcome) {
        self.current_archive.inc(1);

        self.passed.inc_length(1);
        self.failed.inc_length(1);
        self.unresolved.inc_length(1);
        self.incompatible.inc_length(1);

        match outcome {
            TestOutcome::Passed => self.passed.inc(1),
            TestOutcome::Failed => self.failed.inc(1),
            TestOutcome::Unresolved => self.unresolved.inc(1),
            TestOutcome::Incompatible => self.incompatible.inc(1),
        }
    }

    fn test_error(&self, message: impl AsRef<str>) {
        match self.failed.position().cmp(&MAX_PRINTED_FAILURES) {
            cmp::Ordering::Less => {
                self.reporter.println(message);
            }
            cmp::Ordering::Equal => {
                self.reporter.println(format!(
                    "More than {MAX_PRINTED_FAILURES} failures shown. Additional failures will be silent."
                ));
            }
            cmp::Ordering::Greater => {
                // Don't print any more messages...
            }
        }
    }

    pub fn parse_error(&self, message: impl AsRef<str>) {
        self.test_error(message);
    }

    pub fn version_error(&self, message: impl AsRef<str>) {
        self.test_error(message);
    }

    pub fn bindings_error(&self, message: impl AsRef<str>) {
        self.test_error(message);
    }

    pub fn trace(&self, message: impl AsRef<str>) {
        self.reporter.println(message);
    }

    pub fn to_results(&self) -> ShardResults {
        ShardResults {
            source_files: self.source_files.position(),
            passed: self.passed.position(),
            failed: self.failed.position(),
            unresolved: self.unresolved.position(),
            incompatible: self.incompatible.position(),
            elapsed: self.all_archives.elapsed(),
        }
    }
}
