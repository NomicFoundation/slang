use infra_utils::commands::Command;
use infra_utils::github::GitHub;

// Three modes:
// 1. dry-run: runs benchmarks locally, uploads to bencher with a dummy token (no dashboard update).
// 2. pr-benchmark: uploads to a temporary PR branch in bencher, compares against the base branch
//    with inline thresholds (percentage), and posts results as a PR comment. Mutually exclusive with dry-run.
// 3. normal (neither flag): uploads results to the main branch on the bencher dashboard.
//
// Use a dummy test token for dry runs:
// https://github.com/bencherdev/bencher/issues/468
// Source: https://github.com/bencherdev/bencher/blob/aa31a002842cfb0da9d6c60569396fc5261f5111/tasks/test_api/src/task/test/smoke_test.rs#L20
const BENCHER_TEST_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJhcGlfa2V5IiwiZXhwIjo1OTkzNjQyMTU2LCJpYXQiOjE2OTg2NzQ4NjEsImlzcyI6Imh0dHBzOi8vZGV2ZWwtLWJlbmNoZXIubmV0bGlmeS5hcHAvIiwic3ViIjoibXVyaWVsLmJhZ2dlQG5vd2hlcmUuY29tIiwib3JnIjpudWxsfQ.9z7jmM53TcVzc1inDxTeX9_OR0PQPpZAsKsCE7lWHfo";

/// Represents a performance threshold for a Bencher benchmark comparison, used in PR benchmarking mode.
#[derive(Clone)]
pub(crate) struct BencherThreshold {
    pub measure: String,
    pub upper_boundary: String,
    pub max_sample_size: Option<String>,
}

impl BencherThreshold {
    pub fn new(measure: &str, upper_boundary: &str) -> Self {
        Self {
            measure: measure.to_string(),
            upper_boundary: upper_boundary.to_string(),
            max_sample_size: None,
        }
    }
    pub fn with_max_sample_size(mut self, max_sample_size: &str) -> Self {
        self.max_sample_size = Some(max_sample_size.to_string());
        self
    }
}

pub(crate) fn run_bench(
    dry_run: bool,
    pr_benchmark: bool,
    project: &str,
    adapter: &str,
    thresholds: &[BencherThreshold],
    test_runner: &str,
) {
    assert!(
        !(dry_run && pr_benchmark),
        "--dry-run and --pr-benchmark are mutually exclusive. \
         PR benchmarks must upload to bencher for comparison."
    );

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

    if pr_benchmark {
        let head_ref = std::env::var("GITHUB_HEAD_REF")
            .expect("GITHUB_HEAD_REF must be set for --pr-benchmark (are you running in a PR?)");
        let base_ref = std::env::var("GITHUB_BASE_REF")
            .expect("GITHUB_BASE_REF must be set for --pr-benchmark (are you running in a PR?)");
        let start_point_hash = std::env::var("BENCHER_PR_START_POINT_HASH")
            .expect("BENCHER_PR_START_POINT_HASH must be set for --pr-benchmark");
        let head_hash = std::env::var("BENCHER_PR_HEAD_HASH")
            .expect("BENCHER_PR_HEAD_HASH must be set for --pr-benchmark");

        command = command
            .property("--branch", &head_ref)
            .property("--start-point", &base_ref)
            .property("--start-point-hash", &start_point_hash)
            .flag("--start-point-reset")
            .property("--hash", &head_hash);

        for BencherThreshold {
            measure,
            upper_boundary,
            max_sample_size,
        } in thresholds
        {
            command = command
                .property("--threshold-measure", measure)
                .property("--threshold-test", "percentage")
                .property("--threshold-upper-boundary", upper_boundary)
                // Since we have to show it for all or for none of the thresholds, we always
                // show it and default to "_" if not set.
                // Bencher will ignore options specified with "_".
                .property(
                    "--threshold-max-sample-size",
                    max_sample_size.as_ref().map_or("_", String::as_str),
                );
        }

        let github_token =
            std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set for --pr-benchmark");
        command = command.property("--github-actions", &github_token);
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

pub(crate) fn archive_branch(project: &str, branch: &str) {
    bencher_archive_command("archive", project, branch);
}

pub(crate) fn unarchive_branch(project: &str, branch: &str) {
    bencher_archive_command("unarchive", project, branch);
}

fn bencher_archive_command(action: &str, project: &str, branch: &str) {
    let token = std::env::var("BENCHER_API_TOKEN").expect(
        "BENCHER_API_TOKEN is not set. Set it to your Bencher API token: https://bencher.dev/console",
    );

    Command::new("bencher")
        .arg(action)
        .property("--project", project)
        .property("--branch", branch)
        .secret("BENCHER_API_TOKEN", token)
        .run();
}
