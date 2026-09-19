use clap::Parser;

#[derive(Clone, Debug, Default, Parser)]
pub struct TestController {}

impl TestController {
    pub fn execute() {
        eprintln!(
            "'infra test' has been migrated to 'task test'. Run 'task --list' for the full list of migrated tasks."
        );

        // Exit directly, to skip printing an irrelevant backtrace for an expected failure:
        #[allow(clippy::exit)]
        std::process::exit(1);
    }
}
