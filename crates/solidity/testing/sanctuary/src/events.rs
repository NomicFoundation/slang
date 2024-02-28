use std::cmp;

use console::Color;
use indicatif::ProgressBar;
use infra_utils::github::GitHub;

use crate::reporting::Reporter;

const MAX_PRINTED_FAILURES: u64 = 1000;

#[derive(Clone, Copy)]
pub enum TestOutcome {
    Passed,
    Failed,
    Incompatible,
    NotFound,
}

pub struct Events {
    reporter: Reporter,

    all_directories: ProgressBar,
    current_directory: ProgressBar,

    source_files: ProgressBar,

    passed: ProgressBar,
    failed: ProgressBar,
    incompatible: ProgressBar,
    not_found: ProgressBar,
}

impl Events {
    pub fn new(directories_count: usize, files_count: usize) -> Self {
        let mut reporter = Reporter::new();

        reporter.add_blank();

        let all_directories = reporter.add_progress("All Directories", directories_count);
        let current_directory = reporter.add_progress("Current Directory", 0);

        reporter.add_blank();

        let source_files = reporter.add_counter("📄 Source Files", Color::White, files_count);

        reporter.add_blank();

        let passed = reporter.add_counter("✅ Passed", Color::Green, 0);
        let failed = reporter.add_counter("❌ Failed", Color::Red, 0);
        let incompatible = reporter.add_counter("❕ Incompatible", Color::White, 0);
        let not_found = reporter.add_counter("❔ Missing", Color::White, 0);

        reporter.add_blank();

        Self {
            reporter,

            all_directories,
            current_directory,

            source_files,

            passed,
            failed,
            incompatible,
            not_found,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn failure_count(&self) -> usize {
        self.failed.position() as usize
    }

    pub fn start_directory(&mut self, directory_files: usize) {
        self.current_directory.reset();
        self.current_directory.set_length(directory_files as u64);

        self.reporter.show();
    }

    pub fn finish_directory(&mut self) {
        self.all_directories.inc(1);

        self.reporter.hide();

        GitHub::collapse_group("Statistics:", || {
            self.reporter.print_full_report();
        });
    }

    pub fn test(&self, outcome: TestOutcome) {
        self.current_directory.inc(1);
        self.source_files.inc(1);

        self.passed.inc_length(1);
        self.failed.inc_length(1);
        self.incompatible.inc_length(1);
        self.not_found.inc_length(1);

        match outcome {
            TestOutcome::Passed => self.passed.inc(1),
            TestOutcome::Failed => self.failed.inc(1),
            TestOutcome::Incompatible => self.incompatible.inc(1),
            TestOutcome::NotFound => self.not_found.inc(1),
        };
    }

    pub fn parse_error(&self, message: impl AsRef<str>) {
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
        };
    }

    pub fn trace(&self, message: impl AsRef<str>) {
        self.reporter.println(message);
    }
}
