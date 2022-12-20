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
    total_parses: AtomicUsize,
    total_errors: AtomicUsize,
}

impl Reporter {
    const MAX_PRINTED_ERRORS: usize = 100;

    pub fn new(total_files: usize) -> Result<Self> {
        let bar_style = indicatif::ProgressStyle::with_template(
            "\n[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) │ ETA: {eta_precise} │ {msg}",
        )?.progress_chars("#>-");

        let progress_bar = indicatif::ProgressBar::new(total_files as u64);
        progress_bar.set_style(bar_style);
        progress_bar.enable_steady_tick(Duration::from_secs(1));

        Ok(Self {
            progress_bar,
            total_parses: AtomicUsize::new(0),
            total_errors: AtomicUsize::new(0),
        })
    }

    pub fn finish(&self) -> usize {
        self.progress_bar.finish();

        return self.total_errors.load(Ordering::Relaxed);
    }

    pub fn report_file_completed(&self) {
        self.progress_bar.inc(1);

        self.progress_bar.set_message(format!(
            "{total_parses} tests | {total_errors} errors",
            total_parses = self.total_parses.load(Ordering::Relaxed),
            total_errors = self.total_errors.load(Ordering::Relaxed),
        ));
    }

    pub fn report_test_result(
        &self,
        source_id: &str,
        source: &str,
        version: &Version,
        output: &ParserOutput,
    ) {
        self.total_parses.fetch_add(1, Ordering::Relaxed);

        let current_errors = output.error_count();
        if current_errors == 0 {
            return;
        }

        let errors_before_update = self
            .total_errors
            .fetch_add(current_errors, Ordering::Relaxed);

        if errors_before_update < Self::MAX_PRINTED_ERRORS {
            let errors = output.errors_as_strings(source_id, source, /* with_colour */ true);

            self.progress_bar.suspend(|| {
                for error in errors
                    .iter()
                    .take(Self::MAX_PRINTED_ERRORS - errors_before_update)
                {
                    eprintln!();
                    eprintln!("[{version}] {error}");
                }
            });
        }
    }
}
