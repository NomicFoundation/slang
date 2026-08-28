use anyhow::Result;
use clap::Parser;

#[derive(Clone, Debug, Default, Parser)]
pub struct LintController {}

impl LintController {
    pub fn execute() -> Result<()> {
        eprintln!(
            "'infra lint' has been migrated to 'task lint'. Run 'task --list' for the full list of migrated tasks."
        );

        // Exit directly, to skip printing an irrelevant backtrace for an expected failure:
        #[allow(clippy::exit)]
        std::process::exit(1);
    }
}
