/// These test cases are not actually part of this compilation, so they are not imported via 'mod' declarations here.
/// Instead, they are individually picked up during runtime, and each is compiled separately, checking any errors they produce.
/// For more info: <https://crates.io/crates/trybuild>
use infra_utils::{cargo::CargoWorkspace, github::GitHub, paths::FileWalker};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[test]
fn run_trybuild() {
    if GitHub::is_running_in_ci() {
        // This instructs 'trybuild' to fail the tests if any snapshots are out of date.
        // Snapshots are written to a 'wip' directory (git-ignored).
        std::env::set_var("TRYBUILD", "wip");
    } else {
        // This instructs 'trybuild' to overwrite (or generate new) snapshots next to test files that generate them.
        // This is useful for local development, where we always want to see the updates as we edit the tests.
        std::env::set_var("TRYBUILD", "overwrite");
    }

    let crate_dir = &CargoWorkspace::locate_source_crate("language_tests").unwrap();

    let tests = FileWalker::from_directory(crate_dir)
        .find(["src/fail/**/test.rs"])
        .unwrap()
        .collect::<Vec<_>>();

    assert!(!tests.is_empty(), "No tests found.");

    tests.par_iter().for_each(|test| {
        let relative_path = test.strip_prefix(crate_dir).unwrap();
        trybuild::TestCases::new().compile_fail(relative_path);
    });
}
