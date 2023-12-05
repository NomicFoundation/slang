use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use anyhow::Result;
use indicatif::ProgressBar;
use semver::Version;
use slang_solidity::parse_output::ParseOutput;

pub struct Reporter {
    progress_bar: ProgressBar,
    total_tests: AtomicUsize,
    failed_tests: AtomicUsize,
}

impl Reporter {
    const MAX_PRINTED_FAILURES: usize = 100;

    pub fn new(total_files: usize) -> Result<Self> {
        let bar_style = indicatif::ProgressStyle::with_template(
            "\n[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files checked ({percent}%) │ {msg} │ ETA: {eta_precise}",
        )?.progress_chars("#>-");

        let progress_bar = indicatif::ProgressBar::new(total_files as u64);
        progress_bar.set_style(bar_style);
        progress_bar.enable_steady_tick(Duration::from_secs(1));

        Ok(Self {
            progress_bar,
            total_tests: AtomicUsize::new(0),
            failed_tests: AtomicUsize::new(0),
        })
    }

    pub fn finish(&self) -> usize {
        self.progress_bar.finish();

        self.failed_tests.load(Ordering::Relaxed)
    }

    pub fn report_file_completed(&self) {
        self.progress_bar.inc(1);

        let failed_tests = self.failed_tests.load(Ordering::Relaxed);
        let total_tests = self.total_tests.load(Ordering::Relaxed);
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let failure_percent = (100_f64 * (failed_tests as f64) / (total_tests as f64)) as isize;

        self.progress_bar.set_message(format!(
            "{failed_tests}/{total_tests} tests failed ({failure_percent}%)",
        ));
    }

    pub fn report_test_result(
        &self,
        source_id: &str,
        source: &str,
        version: &Version,
        output: &ParseOutput,
    ) {
        self.total_tests.fetch_add(1, Ordering::Relaxed);

        let errors = output.errors();
        if errors.is_empty() {
            return;
        }

        let failures_before_update = self.failed_tests.fetch_add(1, Ordering::Relaxed);

        match Self::MAX_PRINTED_FAILURES.checked_sub(failures_before_update) {
            None => { /* Don't print more errors */ }
            Some(0) => {
                self.progress_bar.suspend(|| {
                    eprintln!();
                    eprintln!(
                        "More than {max_failures} tests failed. Further errors will not be shown.",
                        max_failures = Self::MAX_PRINTED_FAILURES
                    );
                });
            }
            Some(remaining) => {
                let reports: Vec<_> = errors
                    .iter()
                    .take(remaining)
                    .map(|error| {
                        error.to_error_report(source_id, source, /* with_color */ true)
                    })
                    .collect();

                self.progress_bar.suspend(|| {
                    for report in reports {
                        eprintln!();
                        eprintln!("[{version}] {report}");
                    }
                });
            }
        }
    }
}
