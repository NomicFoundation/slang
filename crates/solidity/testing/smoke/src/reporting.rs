use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use anyhow::Result;
use indicatif::ProgressBar;
use semver::Version;
use solidity_rust_lib::generated::language::ParserOutput;

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

        return self.failed_tests.load(Ordering::Relaxed);
    }

    pub fn report_file_completed(&self) {
        self.progress_bar.inc(1);

        let failed_tests = self.failed_tests.load(Ordering::Relaxed);
        let total_tests = self.total_tests.load(Ordering::Relaxed);
        let failure_percent = (100_f64 * (failed_tests as f64) / (total_tests as f64)) as usize;

        self.progress_bar.set_message(format!(
            "{failed_tests}/{total_tests} tests failed ({failure_percent}%)",
        ));
    }

    pub fn report_test_result(
        &self,
        source_id: &str,
        source: &str,
        version: &Version,
        output: &ParserOutput,
    ) {
        self.total_tests.fetch_add(1, Ordering::Relaxed);

        if output.error_count() == 0 {
            return;
        }

        let failures_before_update = self.failed_tests.fetch_add(1, Ordering::Relaxed);

        if failures_before_update < Self::MAX_PRINTED_FAILURES {
            let errors = output.errors_as_strings(source_id, source, /* with_colour */ true);

            self.progress_bar.suspend(|| {
                for error in errors
                    .iter()
                    .take(Self::MAX_PRINTED_FAILURES - failures_before_update)
                {
                    eprintln!();
                    eprintln!("[{version}] {error}");
                }
            });
        } else if failures_before_update == Self::MAX_PRINTED_FAILURES {
            self.progress_bar.suspend(|| {
                eprintln!();
                eprintln!(
                    "More than {max_failures} tests failed. Further errors will not be shown.",
                    max_failures = Self::MAX_PRINTED_FAILURES
                );
            });
        }
    }
}
