use infra_utils::commands::Command;
use infra_utils::github::GitHub;

// Use a dummy test token for dry runs:
// https://github.com/bencherdev/bencher/issues/468
// Source: https://github.com/bencherdev/bencher/blob/aa31a002842cfb0da9d6c60569396fc5261f5111/tasks/test_api/src/task/test/smoke_test.rs#L20
const BENCHER_TEST_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJhcGlfa2V5IiwiZXhwIjo1OTkzNjQyMTU2LCJpYXQiOjE2OTg2NzQ4NjEsImlzcyI6Imh0dHBzOi8vZGV2ZWwtLWJlbmNoZXIubmV0bGlmeS5hcHAvIiwic3ViIjoibXVyaWVsLmJhZ2dlQG5vd2hlcmUuY29tIiwib3JnIjpudWxsfQ.9z7jmM53TcVzc1inDxTeX9_OR0PQPpZAsKsCE7lWHfo";

pub(crate) fn run_bench(dry_run: bool, project: &str, adapter: &str, test_runner: &str) {
    let token = if dry_run {
        BENCHER_TEST_TOKEN.to_string()
    } else {
        std::env::var("BENCHER_API_TOKEN").expect(
            "BENCHER_API_TOKEN is not set. Either perform a '--dry-run', or set it to your Bencher API token: https://bencher.dev/console"
        )
    };

    let testbed = if GitHub::is_running_in_ci() {
        "ci"
    } else {
        "dev"
    };

    let project = std::env::var("SLANG_BENCHER_PROJECT").unwrap_or(project.to_owned());

    let mut command = Command::new("bencher")
        .arg("run")
        .property("--project", &project)
        .property("--adapter", adapter)
        .property("--testbed", testbed)
        .secret("BENCHER_API_TOKEN", token);

    if dry_run {
        command = command.flag("--dry-run");
    }

    // Has to be the last argument:
    command.arg(test_runner).run();

    println!(
        "

Bencher Run is complete...
Test Results: [https://bencher.dev/console/projects/{project}/reports]
"
    );
}
