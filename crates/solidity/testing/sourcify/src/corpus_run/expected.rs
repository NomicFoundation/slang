//! The failure buckets a corpus run tolerates, checked in next to the crate.

use std::fmt::Write;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use super::outcome::{Check, Failure};

#[derive(Debug, Deserialize)]
pub struct ExpectedFailures {
    #[serde(default)]
    failures: Vec<ExpectedFailure>,
}

/// One tolerated bucket: a diagnostic code, narrowed to a message prefix and/or a
/// list of contracts when the code also fires for reasons the entry does not cover.
#[derive(Debug, Deserialize)]
pub struct ExpectedFailure {
    pub check: Check,
    pub code: String,
    pub reason: String,
    /// The issue tracking the fix; absent only for a deliberate divergence from solc.
    #[serde(default)]
    pub issue: Option<String>,
    #[serde(default)]
    pub deliberate: bool,
    /// Only failures whose first message starts with this.
    #[serde(default)]
    pub message: Option<String>,
    /// Only these contracts (corpus file stems); empty means any.
    #[serde(default)]
    pub contracts: Vec<String>,
}

impl ExpectedFailure {
    pub fn key(&self) -> String {
        format!("{}:{}", self.check, self.code)
    }

    pub fn matches(&self, contract: &str, failure: &Failure) -> bool {
        self.check == failure.check
            && self.code == failure.code
            && self
                .message
                .as_ref()
                .is_none_or(|prefix| failure.message.starts_with(prefix))
            && (self.contracts.is_empty() || self.contracts.iter().any(|c| c == contract))
    }

    /// How the report names the entry: the issue or "deliberate", plus its narrowing.
    pub fn label(&self) -> String {
        let mut label = match &self.issue {
            Some(issue) if !self.deliberate => issue.clone(),
            _ => "deliberate".to_owned(),
        };
        if let Some(prefix) = &self.message {
            write!(label, " [{prefix}…]").unwrap();
        }
        if !self.contracts.is_empty() {
            write!(label, " [{} listed]", self.contracts.len()).unwrap();
        }
        label
    }
}

impl ExpectedFailures {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("Could not read expected failures {path:?}"))?;
        let expected: Self = toml::from_str(&contents)
            .with_context(|| format!("Malformed expected failures {path:?}"))?;
        for failure in &expected.failures {
            let tracked = failure
                .issue
                .as_ref()
                .is_some_and(|issue| !issue.is_empty());
            anyhow::ensure!(
                !failure.reason.is_empty() && (tracked || failure.deliberate),
                "{path:?}: {key} needs a reason and an issue (or `deliberate = true`)",
                key = failure.key()
            );
        }
        Ok(expected)
    }

    pub fn entries(&self) -> &[ExpectedFailure] {
        &self.failures
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(entries: &str) -> ExpectedFailures {
        toml::from_str(entries).unwrap()
    }

    fn failure(code: &str, message: &str) -> Failure {
        Failure {
            check: Check::Bind,
            code: code.to_owned(),
            count: 1,
            message: message.to_owned(),
        }
    }

    #[test]
    fn entries_need_a_reason_and_an_issue() {
        let dir = tempfile_dir();
        let path = dir.join("expected.toml");
        std::fs::write(
            &path,
            "[[failures]]\ncheck = \"validate\"\ncode = \"member-not-found\"\nreason = \"r\"\n",
        )
        .unwrap();
        let error = ExpectedFailures::load(&path).unwrap_err().to_string();
        assert!(
            error.contains("validate:member-not-found needs a reason"),
            "{error}"
        );
    }

    #[test]
    fn deliberate_divergences_need_no_issue() {
        let dir = tempfile_dir();
        let path = dir.join("expected.toml");
        std::fs::write(
            &path,
            "[[failures]]\ncheck = \"parse\"\ncode = \"x\"\nreason = \"r\"\ndeliberate = true\n",
        )
        .unwrap();
        assert!(ExpectedFailures::load(&path).is_ok());
    }

    #[test]
    fn a_message_prefix_narrows_the_entry() {
        let expected = parse(
            "[[failures]]\ncheck = \"bind\"\ncode = \"x\"\nreason = \"r\"\nissue = \"i\"\nmessage = \"Unexpected Pragma\"\n",
        );
        let entry = &expected.entries()[0];
        assert!(entry.matches("a", &failure("x", "Unexpected PragmaSemicolon")));
        assert!(!entry.matches("a", &failure("x", "Unexpected IndexedKeyword")));
    }

    #[test]
    fn a_contract_list_narrows_the_entry() {
        let expected = parse(
            "[[failures]]\ncheck = \"bind\"\ncode = \"x\"\nreason = \"r\"\ndeliberate = true\ncontracts = [\"1_0xaa\"]\n",
        );
        let entry = &expected.entries()[0];
        assert!(entry.matches("1_0xaa", &failure("x", "m")));
        assert!(!entry.matches("1_0xbb", &failure("x", "m")));
        assert_eq!(entry.label(), "deliberate [1 listed]");
    }

    fn tempfile_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "sourcify-expected-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
