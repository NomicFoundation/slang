//! Compare a benchmark suite's local bench list against the names recorded on
//! bencher.dev, and write a markdown summary the CI workflow surfaces as a
//! sticky PR comment.
//!
//! Used by `infra perf cargo --pr-benchmark`. Suite-specific name parsing
//! (local listing format, remote label canonicalization) lives in the
//! per-suite submodules below — currently only [`iai_callgrind`].

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use infra_utils::paths::PathExtensions;

/// Diff `local_names` against `remote_names` and write any new (locally
/// added) and orphan (recorded on bencher but no longer in code) entries to
/// `target/perf-new-benches-<bench_name>.md`. The CI workflow conditionally
/// surfaces this file as a sticky PR comment.
///
/// The output file is always written when verification succeeds: an empty body
/// means "no diff" (the workflow deletes the sticky), and a non-empty body is
/// the markdown the workflow stickies.
pub fn verify_bench_list(
    bench_name: &str,
    bencher_project: &str,
    local_names: Vec<String>,
    remote_names: Vec<String>,
) -> Result<()> {
    let local_set: BTreeSet<String> = local_names.into_iter().collect();
    let remote_set: BTreeSet<String> = remote_names.into_iter().collect();
    let new: Vec<&String> = local_set.difference(&remote_set).collect();
    let orphan: Vec<&String> = remote_set.difference(&local_set).collect();

    let body = render_bench_diff_markdown(bench_name, bencher_project, &new, &orphan);
    let output_path = write_output(bench_name, &body)?;

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
    Ok(())
}

fn write_output(bench_name: &str, body: &str) -> Result<PathBuf> {
    let target_dir = Path::repo_path("target");
    std::fs::create_dir_all(&target_dir)
        .with_context(|| format!("failed to create {}", target_dir.display()))?;
    let output_path = target_dir.join(format!("perf-new-benches-{bench_name}.md"));
    std::fs::write(&output_path, body)
        .with_context(|| format!("failed to write {}", output_path.display()))?;
    Ok(output_path)
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

/// Append a `<details>` block; renders as a collapsible disclosure in
/// GitHub markdown.
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

/// Suite-specific helpers for the iai-callgrind cargo benches: how to list local
/// names from `cargo bench -- --list`, and how to fetch + canonicalize the
/// names recorded on bencher.
pub mod iai_callgrind {
    use anyhow::Result;
    use infra_utils::commands::Command;

    use crate::toolchains::bencher;

    /// Collect the local and remote name sets for the given bench.
    pub fn collect_names(
        package: &str,
        bench_name: &str,
        bencher_project: &str,
    ) -> Result<(Vec<String>, Vec<String>)> {
        let output = Command::new("cargo")
            .args([
                "bench",
                "--profile",
                "dev",
                "--package",
                package,
                "--bench",
                bench_name,
                "--",
                "--list",
            ])
            .evaluate()?;

        let local = parse_local_names(&output);

        // Compare against benchmarks reported on the `main` branch
        let remote = bencher::list_benchmarks_latest_report(bencher_project, "main")?
            .iter()
            .map(|name| canonicalize_label(name))
            .collect();

        Ok((local, remote))
    }

    /// Parse libtest-compatible `<name>: benchmark` listings — the format
    /// iai-callgrind produces with `cargo bench -- --list`.
    fn parse_local_names(output: &str) -> Vec<String> {
        output
            .lines()
            .filter_map(|line| line.strip_suffix(": benchmark").map(str::to_owned))
            .collect()
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

    #[cfg(test)]
    mod tests {
        use super::{canonicalize_label, parse_local_names};

        #[test]
        fn parse_local_names_strips_libtest_suffix() {
            let output = "\
                slang::pipeline::parser::merkle_proof: benchmark\n\
                slang::pipeline::parser::weighted_pool: benchmark\n\
                \n2 benchmarks\n";
            assert_eq!(
                parse_local_names(output),
                vec![
                    "slang::pipeline::parser::merkle_proof".to_string(),
                    "slang::pipeline::parser::weighted_pool".to_string(),
                ],
            );
        }

        #[test]
        fn canonicalize_label_strips_quoted_description_tail() {
            assert_eq!(
                canonicalize_label("slang::pipeline::parser weighted_pool:\"weighted_pool\""),
                "slang::pipeline::parser::weighted_pool",
            );
        }

        #[test]
        fn canonicalize_label_passes_through_unsplittable_input() {
            assert_eq!(canonicalize_label("free_form_name"), "free_form_name");
        }
    }
}
