use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::{cmp, io};

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

#[derive(Clone)]
pub(crate) struct Metric {
    pub file: String,
    pub bytes: usize,
    pub locs: usize,
    pub number_of_contracts: usize,
    pub total_inheritance_count: usize,
    pub max_inheritance_count: usize,
    pub cst_height: usize,
    pub number_of_nodes: usize,
    pub number_of_refs: usize,
    pub parsing_time: u128,
    pub bindings_time: u128,
    pub memory_usage: usize,
}

impl Metric {
    pub fn new() -> Self {
        Metric {
            file: String::new(),
            bytes: 0,
            locs: 0,
            number_of_contracts: 0,
            total_inheritance_count: 0,
            max_inheritance_count: 0,
            cst_height: 0,
            number_of_nodes: 0,
            number_of_refs: 0,
            parsing_time: 0,
            bindings_time: 0,
            memory_usage: 0,
        }
    }
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

    metrics: Mutex<Vec<Metric>>,
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

        let metrics = Mutex::new(vec![]);

        Self {
            reporter,

            all_directories,
            current_directory,

            source_files,

            passed,
            failed,
            incompatible,
            not_found,

            metrics,
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
        };
    }

    pub fn parse_error(&self, message: impl AsRef<str>) {
        self.test_error(message);
    }

    pub fn bindings_error(&self, message: impl AsRef<str>) {
        self.test_error(message);
    }

    pub fn trace(&self, message: impl AsRef<str>) {
        self.reporter.println(message);
    }

    pub fn register_metric(&self, metric: Metric) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.push(metric);
    }

    pub fn print_metrics(&self, metrics_file: Option<String>) -> std::io::Result<()> {
        let writer: &mut dyn Write = if let Some(file_name) = metrics_file {
            &mut File::create(file_name.clone())?
        } else {
            &mut io::stdout()
        };
        writeln!(
            writer,
            "file,bytes,locs,number_of_contracts,total_inheritance_count,max_inheritance_count,cst_height,number_of_nodes,number_of_refs,parsing_time,bindings_time,memory_usage"
        )?;

        let metrics_guard = self.metrics.lock().unwrap();
        let metrics = metrics_guard.as_slice();
        for metric in metrics {
            writeln!(
                writer,
                "{},{},{},{},{},{},{},{},{},{},{},{}",
                metric.file,
                metric.bytes,
                metric.locs,
                metric.number_of_contracts,
                metric.total_inheritance_count,
                metric.max_inheritance_count,
                metric.cst_height,
                metric.number_of_nodes,
                metric.number_of_refs,
                metric.parsing_time,
                metric.bindings_time,
                metric.memory_usage
            )?;
        }
        Ok(())
    }
}
