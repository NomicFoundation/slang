use clap::Parser;
use std::path::PathBuf;

mod builds;
mod mirror;
mod solc_ouput;
mod tests;
mod utils;

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(short, long)]
    tests_dir: String,
}

fn main() {
    println!("\n\n");
    let args = ProgramArgs::parse();

    let binaries_dir = std::env::temp_dir().join("slang-solc-binaries");
    let builds = mirror::fetch_builds(&binaries_dir);

    let tests_dir = PathBuf::from(&args.tests_dir);
    let tests = tests::collect_tests(tests_dir.clone());

    tests
        .iter()
        .for_each(|test| tests::run_test(&tests_dir, test, &builds));
}
