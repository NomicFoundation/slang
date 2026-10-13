use clap::Parser;

#[derive(Clone, Debug, Default, Parser)]
pub struct SetupController {}

impl SetupController {
    pub fn execute() {
        eprintln!(
            "'infra setup' has been migrated to 'task setup'. Run 'task --list' for the full list of migrated tasks."
        );

        // Exit directly, to skip printing an irrelevant backtrace for an expected failure:
        #[allow(clippy::exit)]
        std::process::exit(1);
    }
}
