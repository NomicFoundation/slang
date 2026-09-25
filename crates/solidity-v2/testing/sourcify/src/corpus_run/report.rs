//! Aggregates outcomes into the census and applies the expected-failures gate.

use std::cmp::Reverse;
use std::fmt::Write;

use slang_solidity_v2_common::collections::SortedMap;

use super::expected::{ExpectedFailure, ExpectedFailures};
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

/// What a failure asks of the reader, most urgent first.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Category {
    /// No entry claims it: the gate fails.
    Unexpected,
    /// An issue-tracked entry claims it: work to pick up.
    KnownIssue,
    /// A deliberate entry claims it: Slang differs from solc on purpose.
    Deliberate,
}

impl Category {
    const ALL: [Category; 3] = [Self::Unexpected, Self::KnownIssue, Self::Deliberate];

    fn title(self) -> &'static str {
        match self {
            Self::Unexpected => "Unexpected failures",
            Self::KnownIssue => "Known issues",
            Self::Deliberate => "Deliberate deviations",
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
    /// `check:code` -> every contract failing with that code.
    pub buckets: SortedMap<String, Bucket>,
    /// Panic message and location -> contracts.
    pub panics: SortedMap<String, Tally>,
    pub skips: SortedMap<String, Tally>,
    /// `check: reason` -> contracts the check could not run on.
    pub skipped_checks: SortedMap<String, Tally>,
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
        for failure in &outcome.failures {
            let bucket = self.buckets.entry(failure.key()).or_default();
            bucket.occurrences.push(Occurrence {
                contract: outcome.id.clone(),
                failure: failure.clone(),
            });
        }
    }

    /// Every occurrence against the entries: the first matching entry claims it, the
    /// rest is unexpected; with `stale_check`, an entry that claimed nothing is stale.
    pub fn gate(&self, expected: &ExpectedFailures, stale_check: bool) -> Gate {
        let entries = expected.entries();
        let mut claimed = vec![0usize; entries.len()];
        let mut unexpected: SortedMap<String, Tally> = SortedMap::new();
        let mut claims: SortedMap<String, Vec<Option<usize>>> = SortedMap::new();
        for (key, bucket) in &self.buckets {
            let mut bucket_claims = Vec::with_capacity(bucket.occurrences.len());
            for occurrence in &bucket.occurrences {
                let claim = entries
                    .iter()
                    .position(|entry| entry.matches(&occurrence.contract, &occurrence.failure));
                match claim {
                    Some(index) => claimed[index] += 1,
                    None => unexpected
                        .entry(key.clone())
                        .or_default()
                        .add(&occurrence.contract, &occurrence.failure.message),
                }
                bucket_claims.push(claim);
            }
            claims.insert(key.clone(), bucket_claims);
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
            claims,
        }
    }

    pub fn markdown(&self, expected: &ExpectedFailures) -> String {
        let gate = self.gate(expected, false);
        let census = Census::new(self, &gate, expected.entries());
        let mut out = String::new();
        census.write_totals(&mut out, self);
        for category in Category::ALL {
            census.write_table(&mut out, category);
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

/// The failures sorted by what they ask of the reader: each contract once overall and once
/// per check under its most urgent category, and each (category, bucket, entry) as a row.
struct Census<'a> {
    entries: &'a [ExpectedFailure],
    by_contract: SortedMap<&'a str, Category>,
    by_check: SortedMap<(Check, &'a str), Category>,
    rows: SortedMap<(Category, &'a str, Option<usize>), Vec<&'a Occurrence>>,
}

impl<'a> Census<'a> {
    fn new(summary: &'a Summary, gate: &Gate, entries: &'a [ExpectedFailure]) -> Self {
        let mut census = Census {
            entries,
            by_contract: SortedMap::new(),
            by_check: SortedMap::new(),
            rows: SortedMap::new(),
        };
        for (key, bucket) in &summary.buckets {
            for (occurrence, claim) in bucket.occurrences.iter().zip(&gate.claims[key]) {
                let category = match claim {
                    None => Category::Unexpected,
                    Some(index) if entries[*index].deliberate => Category::Deliberate,
                    Some(_) => Category::KnownIssue,
                };
                let contract = occurrence.contract.as_str();
                let overall = census.by_contract.entry(contract).or_insert(category);
                *overall = (*overall).min(category);
                let per_check = census
                    .by_check
                    .entry((occurrence.failure.check, contract))
                    .or_insert(category);
                *per_check = (*per_check).min(category);
                census
                    .rows
                    .entry((category, key.as_str(), *claim))
                    .or_default()
                    .push(occurrence);
            }
        }
        census
    }

    fn contracts(&self, wanted: Category) -> usize {
        self.by_contract
            .values()
            .filter(|category| **category == wanted)
            .count()
    }

    fn write_totals(&self, out: &mut String, summary: &Summary) {
        writeln!(
            out,
            "| contracts | passed | unexpected | known issues | deliberate | panicked | skipped |\n\
             |---|---|---|---|---|---|---|\n\
             | {} | {} | {} | {} | {} | {} | {} |",
            summary.contracts,
            summary.passed,
            self.contracts(Category::Unexpected),
            self.contracts(Category::KnownIssue),
            self.contracts(Category::Deliberate),
            summary.panicked,
            summary.skipped
        )
        .unwrap();
        if !self.by_check.is_empty() {
            writeln!(out).unwrap();
        }
        for category in Category::ALL {
            let mut per_check: SortedMap<Check, usize> = SortedMap::new();
            for ((check, _), found) in &self.by_check {
                if *found == category {
                    *per_check.entry(*check).or_default() += 1;
                }
            }
            if !per_check.is_empty() {
                writeln!(
                    out,
                    "- {} per check: {}",
                    category.title(),
                    per_check
                        .iter()
                        .map(|(check, count)| format!("{check} {count}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .unwrap();
            }
        }
    }

    fn write_table(&self, out: &mut String, category: Category) {
        let mut table: Vec<_> = self
            .rows
            .iter()
            .filter(|((found, _, _), _)| *found == category)
            .collect();
        if table.is_empty() {
            return;
        }
        table.sort_by_key(|(_, occurrences)| Reverse(occurrences.len()));
        writeln!(out).unwrap();
        match category {
            Category::Unexpected => writeln!(
                out,
                "### Unexpected failures\n\n\
                 | bucket | contracts | diagnostics | examples | message |\n\
                 |---|---|---|---|---|"
            ),
            Category::KnownIssue => writeln!(
                out,
                "### Known issues\n\n\
                 | issue | bucket | contracts | diagnostics | examples | message |\n\
                 |---|---|---|---|---|---|"
            ),
            Category::Deliberate => {
                let contracts = self.contracts(category);
                let plural = if contracts == 1 { "" } else { "s" };
                writeln!(
                    out,
                    "<details><summary>Deliberate deviations ({contracts} contract{plural})\
                     </summary>\n\n\
                     | bucket | contracts | narrowed to | reason | examples |\n\
                     |---|---|---|---|---|"
                )
            }
        }
        .unwrap();
        for ((_, key, claim), occurrences) in table {
            let examples = occurrences
                .iter()
                .take(EXAMPLES)
                .map(|occurrence| occurrence.contract.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let diagnostics: usize = occurrences
                .iter()
                .map(|occurrence| occurrence.failure.count)
                .sum();
            let message = occurrences[0]
                .failure
                .message
                .lines()
                .next()
                .unwrap_or_default()
                .replace('|', "\\|");
            let entry = claim.map(|index| &self.entries[index]);
            let narrowing = entry.map(ExpectedFailure::narrowing).unwrap_or_default();
            let contracts = occurrences.len();
            match category {
                Category::Unexpected => writeln!(
                    out,
                    "| `{key}` | {contracts} | {diagnostics} | {examples} | {message} |"
                ),
                Category::KnownIssue => writeln!(
                    out,
                    "| {} | `{key}`{narrowing} | {contracts} | {diagnostics} | {examples} | {message} |",
                    entry.map(ExpectedFailure::issue_ref).unwrap_or_default()
                ),
                Category::Deliberate => writeln!(
                    out,
                    "| `{key}` | {contracts} | {} | {} | {examples} |",
                    narrowing.trim(),
                    entry
                        .map(|entry| entry.reason.replace('|', "\\|"))
                        .unwrap_or_default()
                ),
            }
            .unwrap();
        }
        if category == Category::Deliberate {
            writeln!(out, "\n</details>").unwrap();
        }
    }
}

pub struct Gate {
    /// `check:code` -> the contracts no entry claims.
    pub unexpected: SortedMap<String, Tally>,
    pub stale: Vec<String>,
    /// Issue-tracked entries small enough to list their contracts, but not doing so.
    pub too_broad: Vec<String>,
    pub panicked: usize,
    /// `check:code` -> the entry claiming each of the bucket's occurrences, if any.
    pub claims: SortedMap<String, Vec<Option<usize>>>,
}

impl Gate {
    fn problems(&self) -> Vec<String> {
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
        problems
    }

    pub fn verdict(&self) -> Result<(), String> {
        let problems = self.problems();
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems.join("; "))
        }
    }

    /// The verdict as the report's closing section, so a failed gate reads in the
    /// census itself and not only in the exit code.
    pub fn markdown(&self) -> String {
        let problems = self.problems();
        if problems.is_empty() {
            return "**Gate: passed.**\n".to_owned();
        }
        let mut markdown = "**Gate: failed.**\n\n".to_owned();
        for problem in problems {
            writeln!(markdown, "- {problem}").expect("writing to a String cannot fail");
        }
        markdown
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
            skipped_checks: SortedMap::new(),
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
    fn the_report_splits_unexpected_known_issue_and_deliberate() {
        let mut summary = Summary::default();
        summary.add(&outcome("u", vec![failure(Check::Bind, "x")], None));
        summary.add(&outcome("k", vec![failure(Check::Bind, "y")], None));
        summary.add(&outcome("d", vec![failure(Check::Bind, "y")], None));
        summary.add(&outcome(
            "m",
            vec![failure(Check::Bind, "x"), failure(Check::Bind, "y")],
            None,
        ));
        let expected = expected(
            "[[failures]]\ncheck = \"bind\"\ncode = \"y\"\nreason = \"on purpose\"\ndeliberate = true\ncontracts = [\"d\"]\n\
             [[failures]]\ncheck = \"bind\"\ncode = \"y\"\nreason = \"r\"\nissue = \"https://github.com/NomicFoundation/slang/issues/123\"\n",
        );
        let report = summary.markdown(&expected);

        // `m` fails both ways and counts once, as unexpected.
        assert!(report.contains("| 4 | 0 | 2 | 1 | 1 | 0 | 0 |"), "{report}");
        let unexpected = report.find("### Unexpected failures").expect(&report);
        let known = report.find("### Known issues").expect(&report);
        let deliberate = report
            .find("Deliberate deviations (1 contract)")
            .expect(&report);
        assert!(unexpected < known && known < deliberate, "{report}");
        assert!(
            report[unexpected..known].contains("| `bind:x` | 2 |"),
            "{report}"
        );
        assert!(
            report[known..deliberate].contains("| #123 | `bind:y` | 2 |"),
            "{report}"
        );
        assert!(report[deliberate..].contains("on purpose"), "{report}");
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
        let report = summary.markdown(&expected);
        assert!(report.contains("| `bind:x` | 1 | 1 | other |"), "{report}");
        assert!(
            report.contains("| `bind:x` | 1 | [1 listed] | r | listed |"),
            "{report}"
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
        let report = summary.markdown(&expected(""));
        assert!(
            report.contains("- Unexpected failures per check: bind 1, validate 1"),
            "{report}"
        );
        assert!(report.contains("| `validate:y` | 1 | 1 | a |"), "{report}");
    }
}
