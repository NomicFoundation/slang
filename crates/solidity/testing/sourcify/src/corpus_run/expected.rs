//! The failure buckets a corpus run tolerates, checked in next to the crate.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

use super::outcome::Check;

#[derive(Debug, Deserialize)]
pub struct ExpectedFailures {
    #[serde(default)]
    failures: Vec<ExpectedFailure>,
}

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
                "{path:?}: {check}:{code} needs a reason and an issue (or `deliberate = true`)",
                check = failure.check,
                code = failure.code
            );
        }
        Ok(expected)
    }

    pub fn by_key(&self) -> BTreeMap<String, &ExpectedFailure> {
        self.failures
            .iter()
            .map(|failure| (format!("{}:{}", failure.check, failure.code), failure))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ExpectedFailures;

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
    fn keys_join_check_and_code() {
        let dir = tempfile_dir();
        let path = dir.join("expected.toml");
        std::fs::write(
            &path,
            "[[failures]]\ncheck = \"bind\"\ncode = \"identifier-not-found\"\nreason = \"r\"\nissue = \"i\"\n",
        )
        .unwrap();
        let expected = ExpectedFailures::load(&path).unwrap();
        assert!(expected.by_key().contains_key("bind:identifier-not-found"));
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
