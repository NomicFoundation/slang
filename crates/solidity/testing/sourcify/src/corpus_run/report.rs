//! Aggregates outcomes into the census and applies the expected-failures gate.

use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::fmt::Write;

use super::expected::ExpectedFailures;
use super::outcome::{Check, Failure, Outcome};

const EXAMPLES: usize = 3;
/// An issue-tracked entry claiming this many contracts or fewer has to list them, so a
/// bucket narrows as its bug gets fixed instead of tolerating new contracts.
const LIST_CONTRACTS_AT: usize = 50;

/// One contract's failure in a bucket.
pub struct Occurrence {
    pub contract: String,
    pub failure: Failure,
}

#[derive(Default)]
pub struct Bucket {
    pub occurrences: Vec<Occurrence>,
    pub diagnostics: usize,
}

impl Bucket {
    fn contracts(&self) -> usize {
        self.occurrences.len()
    }
}

#[derive(Default)]
pub struct Tally {
    pub contracts: usize,
    pub examples: Vec<String>,
    pub message: String,
}

impl Tally {
    fn add(&mut self, id: &str, message: &str) {
        self.contracts += 1;
        if self.examples.len() < EXAMPLES {
            self.examples.push(id.to_owned());
        }
        if self.message.is_empty() {
            self.message = message.lines().next().unwrap_or_default().to_owned();
        }
    }
}

#[derive(Default)]
pub struct Summary {
    pub contracts: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub panicked: usize,
    pub failed_by_check: BTreeMap<Check, usize>,
    /// `check:code` -> every contract failing with that code.
    pub buckets: BTreeMap<String, Bucket>,
    /// Panic message and location -> contracts.
    pub panics: BTreeMap<String, Tally>,
    pub skips: BTreeMap<String, Tally>,
    /// `check: reason` -> contracts the check could not run on.
    pub skipped_checks: BTreeMap<String, Tally>,
}

impl Summary {
    pub fn add(&mut self, outcome: &Outcome) {
        self.contracts += 1;
        if let Some(reason) = &outcome.skipped {
            self.skipped += 1;
            self.skips
                .entry(reason.clone())
                .or_default()
                .add(&outcome.id, "");
            return;
        }
        if let Some(panic) = &outcome.panic {
            self.panicked += 1;
            self.panics
                .entry(panic.clone())
                .or_default()
                .add(&outcome.id, panic);
        }
        if outcome.passed() {
            self.passed += 1;
        } else if outcome.panic.is_none() {
            self.failed += 1;
        }
        for (check, reason) in &outcome.skipped_checks {
            self.skipped_checks
                .entry(format!("{check}: {reason}"))
                .or_default()
                .add(&outcome.id, "");
        }
        let mut checks_hit = Vec::new();
        for failure in &outcome.failures {
            let bucket = self.buckets.entry(failure.key()).or_default();
            bucket.diagnostics += failure.count;
            bucket.occurrences.push(Occurrence {
                contract: outcome.id.clone(),
                failure: failure.clone(),
            });
            if !checks_hit.contains(&failure.check) {
                checks_hit.push(failure.check);
            }
        }
        for check in checks_hit {
            *self.failed_by_check.entry(check).or_default() += 1;
        }
    }

    /// Every occurrence against the entries: the first matching entry claims it, the
    /// rest is unexpected; with `stale_check`, an entry that claimed nothing is stale.
    pub fn gate(&self, expected: &ExpectedFailures, stale_check: bool) -> Gate {
        let entries = expected.entries();
        let mut claimed = vec![0usize; entries.len()];
        let mut unexpected: BTreeMap<String, Tally> = BTreeMap::new();
        let mut per_bucket: BTreeMap<String, Vec<(usize, usize)>> = BTreeMap::new();
        for (key, bucket) in &self.buckets {
            let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
            for occurrence in &bucket.occurrences {
                match entries
                    .iter()
                    .position(|entry| entry.matches(&occurrence.contract, &occurrence.failure))
                {
                    Some(index) => {
                        claimed[index] += 1;
                        *counts.entry(index).or_default() += 1;
                    }
                    None => unexpected
                        .entry(key.clone())
                        .or_default()
                        .add(&occurrence.contract, &occurrence.failure.message),
                }
            }
            per_bucket.insert(key.clone(), counts.into_iter().collect());
        }
        let (mut stale, mut too_broad) = (Vec::new(), Vec::new());
        if stale_check {
            for (entry, claimed) in entries.iter().zip(&claimed) {
                let label = format!("{} ({})", entry.key(), entry.label());
                if *claimed == 0 {
                    stale.push(label);
                } else if *claimed <= LIST_CONTRACTS_AT
                    && !entry.deliberate
                    && entry.contracts.is_empty()
                {
                    too_broad.push(format!("{label} claims {claimed}"));
                }
            }
        }
        Gate {
            unexpected,
            stale,
            too_broad,
            panicked: self.panicked,
            per_bucket,
        }
    }

    pub fn markdown(&self, expected: &ExpectedFailures) -> String {
        let gate = self.gate(expected, false);
        let entries = expected.entries();
        let mut out = String::new();
        writeln!(out, "| contracts | passed | failed | panicked | skipped |").unwrap();
        writeln!(out, "|---|---|---|---|---|").unwrap();
        writeln!(
            out,
            "| {} | {} | {} | {} | {} |",
            self.contracts, self.passed, self.failed, self.panicked, self.skipped
        )
        .unwrap();
        if !self.failed_by_check.is_empty() {
            writeln!(out).unwrap();
            writeln!(
                out,
                "Failed contracts per check: {}",
                self.failed_by_check
                    .iter()
                    .map(|(check, count)| format!("{check} {count}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .unwrap();
        }
        if !self.buckets.is_empty() {
            writeln!(out).unwrap();
            writeln!(
                out,
                "| bucket | contracts | diagnostics | expected | examples | message |"
            )
            .unwrap();
            writeln!(out, "|---|---|---|---|---|---|").unwrap();
            let mut buckets: Vec<_> = self.buckets.iter().collect();
            buckets.sort_by_key(|(_, bucket)| Reverse(bucket.contracts()));
            for (key, bucket) in buckets {
                let mut status: Vec<String> = gate.per_bucket[key]
                    .iter()
                    .map(|(index, count)| format!("{} ×{count}", entries[*index].label()))
                    .collect();
                // Unexpected contracts are what the reader has to look at, so they take the
                // example and message columns when there are any.
                let (examples, message) = if let Some(tally) = gate.unexpected.get(key) {
                    status.push(format!("**{} unexpected**", tally.contracts));
                    (tally.examples.clone(), tally.message.clone())
                } else {
                    (
                        bucket
                            .occurrences
                            .iter()
                            .take(EXAMPLES)
                            .map(|occurrence| occurrence.contract.clone())
                            .collect(),
                        bucket.occurrences[0]
                            .failure
                            .message
                            .lines()
                            .next()
                            .unwrap_or_default()
                            .to_owned(),
                    )
                };
                writeln!(
                    out,
                    "| `{key}` | {} | {} | {} | {} | {} |",
                    bucket.contracts(),
                    bucket.diagnostics,
                    status.join(", "),
                    examples.join(", "),
                    message.replace('|', "\\|")
                )
                .unwrap();
            }
        }
        for (title, map) in [
            ("Panics", &self.panics),
            ("Skipped", &self.skips),
            ("Checks skipped", &self.skipped_checks),
        ] {
            if map.is_empty() {
                continue;
            }
            writeln!(out).unwrap();
            writeln!(out, "{title}:").unwrap();
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by_key(|(_, tally)| Reverse(tally.contracts));
            for (key, tally) in entries {
                writeln!(
                    out,
                    "- {} × `{key}` ({})",
                    tally.contracts,
                    tally.examples.join(", ")
                )
                .unwrap();
            }
        }
        out
    }
}

pub struct Gate {
    /// `check:code` -> the contracts no entry claims.
    pub unexpected: BTreeMap<String, Tally>,
    pub stale: Vec<String>,
    /// Issue-tracked entries small enough to list their contracts, but not doing so.
    pub too_broad: Vec<String>,
    pub panicked: usize,
    /// `check:code` -> (entry index, contracts it claimed).
    pub per_bucket: BTreeMap<String, Vec<(usize, usize)>>,
}

impl Gate {
    pub fn verdict(&self) -> Result<(), String> {
        let mut problems = Vec::new();
        if self.panicked > 0 {
            problems.push(format!("{} contract(s) panicked", self.panicked));
        }
        if !self.unexpected.is_empty() {
            problems.push(format!(
                "unexpected failures: {}",
                self.unexpected
                    .iter()
                    .map(|(key, tally)| format!(
                        "{key} ({} contract(s), e.g. {})",
                        tally.contracts,
                        tally.examples.join(", ")
                    ))
                    .collect::<Vec<_>>()
                    .join("; ")
            ));
        }
        if !self.stale.is_empty() {
            problems.push(format!(
                "expected entries that no longer match anything (remove them): {}",
                self.stale.join(", ")
            ));
        }
        if !self.too_broad.is_empty() {
            problems.push(format!(
                "expected entries with {LIST_CONTRACTS_AT} contracts or fewer must list them: {}",
                self.too_broad.join(", ")
            ));
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems.join("; "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn outcome(id: &str, failures: Vec<Failure>, panic: Option<&str>) -> Outcome {
        Outcome {
            id: id.to_owned(),
            version: "0.8.30".to_owned(),
            evm_target: None,
            ms: 0,
            skipped: None,
            panic: panic.map(str::to_owned),
            failures,
            warnings: 0,
            skipped_checks: BTreeMap::new(),
        }
    }

    fn failure(check: Check, code: &str) -> Failure {
        Failure {
            check,
            code: code.to_owned(),
            count: 1,
            message: format!("{code} message"),
        }
    }

    fn expected(entries: &str) -> ExpectedFailures {
        toml::from_str(entries).unwrap()
    }

    #[test]
    fn unexpected_buckets_fail_the_gate() {
        let mut summary = Summary::default();
        summary.add(&outcome(
            "a",
            vec![failure(Check::Validate, "member-not-found")],
            None,
        ));
        let gate = summary.gate(&expected(""), false);
        assert_eq!(gate.unexpected["validate:member-not-found"].contracts, 1);
        let verdict = gate.verdict().unwrap_err();
        assert!(
            verdict.contains("validate:member-not-found (1 contract(s), e.g. a)"),
            "{verdict}"
        );
    }

    #[test]
    fn expected_buckets_pass_and_stale_ones_fail_only_when_asked() {
        let mut summary = Summary::default();
        summary.add(&outcome(
            "a",
            vec![failure(Check::Bind, "identifier-not-found")],
            None,
        ));
        let expected = expected(
            "[[failures]]\ncheck = \"bind\"\ncode = \"identifier-not-found\"\nreason = \"r\"\nissue = \"i\"\n\
             [[failures]]\ncheck = \"parse\"\ncode = \"gone\"\nreason = \"r\"\nissue = \"i\"\n",
        );
        assert!(summary.gate(&expected, false).verdict().is_ok());
        let gate = summary.gate(&expected, true);
        assert_eq!(gate.stale, vec!["parse:gone (i)"]);
        assert!(gate.verdict().is_err());
    }

    #[test]
    fn an_entry_narrowed_to_contracts_leaves_the_others_unexpected() {
        let mut summary = Summary::default();
        summary.add(&outcome("listed", vec![failure(Check::Bind, "x")], None));
        summary.add(&outcome("other", vec![failure(Check::Bind, "x")], None));
        let expected = expected(
            "[[failures]]\ncheck = \"bind\"\ncode = \"x\"\nreason = \"r\"\ndeliberate = true\ncontracts = [\"listed\"]\n",
        );
        let gate = summary.gate(&expected, true);
        assert_eq!(gate.unexpected["bind:x"].examples, vec!["other"]);
        assert!(gate.stale.is_empty());
        assert!(
            summary
                .markdown(&expected)
                .contains("deliberate [1 listed] ×1, **1 unexpected**")
        );
    }

    #[test]
    fn a_small_bucket_wide_entry_has_to_list_its_contracts() {
        let mut summary = Summary::default();
        summary.add(&outcome("a", vec![failure(Check::Bind, "x")], None));
        let broad = expected(
            "[[failures]]\ncheck = \"bind\"\ncode = \"x\"\nreason = \"r\"\nissue = \"i\"\n",
        );
        assert!(summary.gate(&broad, false).verdict().is_ok());
        let verdict = summary.gate(&broad, true).verdict().unwrap_err();
        assert!(
            verdict.contains("must list them: bind:x (i) claims 1"),
            "{verdict}"
        );
        let deliberate = expected(
            "[[failures]]\ncheck = \"bind\"\ncode = \"x\"\nreason = \"r\"\ndeliberate = true\n",
        );
        assert!(summary.gate(&deliberate, true).verdict().is_ok());
    }

    #[test]
    fn panics_always_fail_and_are_bucketed_by_message() {
        let mut summary = Summary::default();
        summary.add(&outcome("a", vec![], Some("boom at x.rs:1:1")));
        summary.add(&outcome("b", vec![], Some("boom at x.rs:1:1")));
        assert_eq!(summary.panics["boom at x.rs:1:1"].contracts, 2);
        assert_eq!(summary.passed, 0);
        assert!(
            summary
                .gate(&expected(""), false)
                .verdict()
                .unwrap_err()
                .contains("2 contract(s) panicked")
        );
    }

    #[test]
    fn a_contract_failing_two_checks_counts_once_per_check() {
        let mut summary = Summary::default();
        summary.add(&outcome(
            "a",
            vec![
                failure(Check::Bind, "x"),
                failure(Check::Validate, "y"),
                failure(Check::Validate, "z"),
            ],
            None,
        ));
        assert_eq!(summary.failed, 1);
        assert_eq!(summary.failed_by_check[&Check::Bind], 1);
        assert_eq!(summary.failed_by_check[&Check::Validate], 1);
        assert!(
            summary
                .markdown(&expected(""))
                .contains("| `validate:y` | 1 | 1 | **1 unexpected** | a |")
        );
    }
}
