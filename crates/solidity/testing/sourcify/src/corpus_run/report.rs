//! Aggregates outcomes into the census and applies the expected-failures gate.

use std::collections::BTreeMap;
use std::fmt::Write;

use super::expected::ExpectedFailures;
use super::outcome::{Check, Outcome};

const EXAMPLES: usize = 3;

#[derive(Default)]
pub struct Bucket {
    pub contracts: usize,
    pub diagnostics: usize,
    pub examples: Vec<String>,
    pub message: String,
}

impl Bucket {
    fn add(&mut self, id: &str, count: usize, message: &str) {
        self.contracts += 1;
        self.diagnostics += count;
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
    /// `check:code` -> bucket.
    pub buckets: BTreeMap<String, Bucket>,
    /// Panic message and location -> bucket.
    pub panics: BTreeMap<String, Bucket>,
    pub skips: BTreeMap<String, Bucket>,
}

impl Summary {
    pub fn add(&mut self, outcome: &Outcome) {
        self.contracts += 1;
        if let Some(reason) = &outcome.skipped {
            self.skipped += 1;
            self.skips
                .entry(reason.clone())
                .or_default()
                .add(&outcome.id, 1, "");
            return;
        }
        if let Some(panic) = &outcome.panic {
            self.panicked += 1;
            self.panics
                .entry(panic.clone())
                .or_default()
                .add(&outcome.id, 1, panic);
        }
        if outcome.passed() {
            self.passed += 1;
        } else if outcome.panic.is_none() {
            self.failed += 1;
        }
        let mut checks_hit = Vec::new();
        for failure in &outcome.failures {
            self.buckets.entry(failure.key()).or_default().add(
                &outcome.id,
                failure.count,
                &failure.message,
            );
            if !checks_hit.contains(&failure.check) {
                checks_hit.push(failure.check);
            }
        }
        for check in checks_hit {
            *self.failed_by_check.entry(check).or_default() += 1;
        }
    }

    /// Buckets outside the expected list, and expected buckets that no longer fire.
    pub fn gate(&self, expected: &ExpectedFailures, stale_check: bool) -> Gate {
        let expected = expected.by_key();
        let unexpected = self
            .buckets
            .keys()
            .filter(|key| !expected.contains_key(*key))
            .cloned()
            .collect();
        let stale = if stale_check {
            expected
                .keys()
                .filter(|key| !self.buckets.contains_key(*key))
                .map(|key| (*key).clone())
                .collect()
        } else {
            Vec::new()
        };
        Gate {
            unexpected,
            stale,
            panicked: self.panicked,
        }
    }

    pub fn markdown(&self, expected: &ExpectedFailures) -> String {
        let expected = expected.by_key();
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
            buckets.sort_by(|(_, a), (_, b)| b.contracts.cmp(&a.contracts));
            for (key, bucket) in buckets {
                let status = match expected.get(key) {
                    Some(entry) if entry.deliberate => "deliberate".to_owned(),
                    Some(entry) => format!("yes ({})", entry.issue.as_deref().unwrap_or_default()),
                    None => "**no**".to_owned(),
                };
                writeln!(
                    out,
                    "| `{key}` | {} | {} | {status} | {} | {} |",
                    bucket.contracts,
                    bucket.diagnostics,
                    bucket.examples.join(", "),
                    bucket.message.replace('|', "\\|")
                )
                .unwrap();
            }
        }
        for (title, map) in [("Panics", &self.panics), ("Skipped", &self.skips)] {
            if map.is_empty() {
                continue;
            }
            writeln!(out).unwrap();
            writeln!(out, "{title}:").unwrap();
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by(|(_, a), (_, b)| b.contracts.cmp(&a.contracts));
            for (key, bucket) in entries {
                writeln!(
                    out,
                    "- {} × `{key}` ({})",
                    bucket.contracts,
                    bucket.examples.join(", ")
                )
                .unwrap();
            }
        }
        out
    }
}

pub struct Gate {
    pub unexpected: Vec<String>,
    pub stale: Vec<String>,
    pub panicked: usize,
}

impl Gate {
    pub fn verdict(&self) -> Result<(), String> {
        let mut problems = Vec::new();
        if self.panicked > 0 {
            problems.push(format!("{} contract(s) panicked", self.panicked));
        }
        if !self.unexpected.is_empty() {
            problems.push(format!(
                "unexpected failure buckets: {}",
                self.unexpected.join(", ")
            ));
        }
        if !self.stale.is_empty() {
            problems.push(format!(
                "expected buckets that no longer fail (remove them): {}",
                self.stale.join(", ")
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
    use crate::corpus_run::outcome::Failure;

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
        assert_eq!(gate.unexpected, vec!["validate:member-not-found"]);
        assert!(
            gate.verdict()
                .unwrap_err()
                .contains("validate:member-not-found")
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
        assert_eq!(gate.stale, vec!["parse:gone"]);
        assert!(gate.verdict().is_err());
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
                .contains("| `validate:y` | 1 | 1 | **no** | a |")
        );
    }
}
