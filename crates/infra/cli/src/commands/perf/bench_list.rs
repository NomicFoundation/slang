//! Compare a benchmark suite's local bench list against the names recorded on
//! bencher.dev, and write a markdown summary the CI workflow surfaces as a
//! sticky PR comment.
//!
//! Used by `infra perf cargo --pr-benchmark`. Suite-specific name parsing
//! (local listing format, remote label canonicalization) lives in the
//! per-suite submodules below — currently only [`gungraun`].

use std::collections::BTreeSet;
use std::path::Path;

use infra_utils::paths::PathExtensions;

/// Diff `local_names` against `remote_names` and write any new (locally
/// added) and orphan (recorded on bencher but no longer in code) entries to
/// `target/perf-new-benches-<bench_name>.md`. The CI workflow conditionally
/// surfaces this file as a sticky PR comment.
///
/// The output file is always written (empty when there is no diff); that lets
/// the workflow distinguish "no diff" (delete the sticky comment) from "step
/// did not run".
pub fn verify_bench_list(
    bench_name: &str,
    bencher_project: &str,
    local_names: Vec<String>,
    remote_names: Vec<String>,
) {
    let output_path = Path::repo_path("target").join(format!("perf-new-benches-{bench_name}.md"));

    let local_set: BTreeSet<String> = local_names.into_iter().collect();
    let remote_set: BTreeSet<String> = remote_names.into_iter().collect();
    let new: Vec<&String> = local_set.difference(&remote_set).collect();
    let orphan: Vec<&String> = remote_set.difference(&local_set).collect();

    let body = render_bench_diff_markdown(bench_name, bencher_project, &new, &orphan);
    if let Err(err) = std::fs::write(&output_path, &body) {
        eprintln!("Failed to write {}: {err:#}", output_path.display());
        return;
    }

    if body.is_empty() {
        println!("Bench list matches bencher project '{bencher_project}'.");
    } else {
        println!(
            "Bench-list diff written to {} ({} new, {} orphan)",
            output_path.display(),
            new.len(),
            orphan.len(),
        );
    }
}

fn render_bench_diff_markdown(
    bench_name: &str,
    bencher_project: &str,
    new: &[&String],
    orphan: &[&String],
) -> String {
    if new.is_empty() && orphan.is_empty() {
        return String::new();
    }

    let mut body =
        format!("### Bench list diff for `{bench_name}` (project `{bencher_project}`)\n\n");

    if !new.is_empty() {
        push_collapsible_list(
            &mut body,
            &format!("<b>New benchmarks ({})</b>", new.len()),
            "present in code, not yet recorded on bencher. \
            Will start a fresh metric history once a <code>ci:perf</code> run uploads results.",
            new,
        );
    }

    if !orphan.is_empty() {
        push_collapsible_list(
            &mut body,
            &format!("<b>Orphan benchmarks ({})</b>", orphan.len()),
            "recorded on bencher but no longer in code. \
            Either renamed in this PR or candidates for archival.",
            orphan,
        );
    }

    body
}

/// Append a `<details>` block
fn push_collapsible_list(
    body: &mut String,
    title_html: &str,
    description_html: &str,
    items: &[&String],
) {
    body.push_str("<details>\n<summary>");
    body.push_str(title_html);
    body.push_str(" — ");
    body.push_str(description_html);
    body.push_str("</summary>\n\n");
    for name in items {
        body.push_str("- `");
        body.push_str(name);
        body.push_str("`\n");
    }
    body.push_str("\n</details>\n\n");
}

/// Suite-specific helpers for the gungraun cargo benches: how to list local
/// names from `cargo bench -- --list`, and how to fetch + canonicalize the
/// names recorded on bencher.
pub mod gungraun {
    use anyhow::{Context, Result};
    use infra_utils::commands::Command;

    /// Collect the local and remote name sets for the given bench. The two
    /// fetches share the same fail-soft fallback at the call site, so they're
    /// bundled into one fallible step.
    pub fn collect_names(
        package: &str,
        bench_name: &str,
        bencher_project: &str,
    ) -> Result<(Vec<String>, Vec<String>)> {
        let output = Command::new("cargo")
            .args([
                "bench",
                "--package",
                package,
                "--bench",
                bench_name,
                "--",
                "--list",
            ])
            .evaluate()
            .with_context(|| format!("failed to list local benchmarks for {bench_name}"))?;

        let local = parse_local_names(&output);
        let remote = list_remote_names(bencher_project)?;
        Ok((local, remote))
    }

    /// Parse libtest-compatible `<name>: benchmark` listings — the format
    /// gungraun produces with `cargo bench -- --list`.
    fn parse_local_names(output: &str) -> Vec<String> {
        output
            .lines()
            .filter_map(|line| line.strip_suffix(": benchmark").map(str::to_owned))
            .collect()
    }

    /// Bulk-fetch every benchmark recorded for `project`, paginating through
    /// the bencher API and canonicalizing each label back to the form
    /// gungraun emits locally so the two sides can be set-compared.
    ///
    /// The slang bencher projects allow read-only public listing, so no token
    /// is required.
    fn list_remote_names(project: &str) -> Result<Vec<String>> {
        const PER_PAGE: usize = 255;
        let mut names = Vec::new();
        let mut page = 1usize;

        loop {
            let output = Command::new("bencher")
                .args([
                    "benchmark",
                    "list",
                    project,
                    "--per-page",
                    &PER_PAGE.to_string(),
                    "--page",
                    &page.to_string(),
                ])
                .evaluate()
                .with_context(|| format!("failed to query bencher project {project}"))?;

            let entries: Vec<serde_json::Value> = serde_json::from_str(&output)
                .with_context(|| format!("failed to parse bencher response for {project}"))?;

            if entries.is_empty() {
                break;
            }

            let count = entries.len();
            names.extend(
                entries
                    .iter()
                    .filter_map(|entry| entry.get("name").and_then(|v| v.as_str()))
                    .map(canonicalize_label),
            );

            if count < PER_PAGE {
                break;
            }
            page += 1;
        }

        Ok(names)
    }

    /// Convert a bencher-stored label like
    /// `slang::pipeline::parser weighted_pool:"weighted_pool"` back to the
    /// canonical `slang::pipeline::parser::weighted_pool` so it can be
    /// set-compared with `parse_local_names`. The truncation bencher applies
    /// to the description tail does not affect the canonical form.
    fn canonicalize_label(name: &str) -> String {
        if let Some((module_path, suffix)) = name.split_once(' ') {
            let bench_id = suffix.split_once(':').map_or(suffix, |(id, _)| id);
            format!("{module_path}::{bench_id}")
        } else {
            name.to_owned()
        }
    }
}
