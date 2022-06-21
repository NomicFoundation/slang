use clap::Parser;
use std::path::PathBuf;
use version_breaks::{
    mirror::fetch_builds,
    tests::{collect_tests, execute_test},
};

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(short, long)]
    tests_dir: String,
}

fn main() {
    println!("\n\n");
    let args = ProgramArgs::parse();

    let binaries_dir = std::env::temp_dir().join("slang-solc-binaries");
    let builds = fetch_builds(&binaries_dir);

    let tests_dir = PathBuf::from(&args.tests_dir);
    let tests = collect_tests(tests_dir.clone());

    tests
        .iter()
        .for_each(|test| execute_test(&tests_dir, test, &builds));
}
