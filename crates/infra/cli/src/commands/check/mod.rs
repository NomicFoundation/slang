use clap::Parser;

#[derive(Clone, Debug, Default, Parser)]
pub struct CheckController {}

impl CheckController {
    pub fn execute() {
        eprintln!(
            "'infra check' has been migrated to 'task check'. Run 'task --list' for the full list of migrated tasks."
        );

        // Exit directly, to skip printing an irrelevant backtrace for an expected failure:
        #[allow(clippy::exit)]
        std::process::exit(1);
    }
}
