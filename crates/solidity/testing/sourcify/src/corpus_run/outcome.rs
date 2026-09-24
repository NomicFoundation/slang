//! What one corpus contract produced, and the classification of its diagnostics.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use slang_solidity_v2::diagnostics::{DiagnosticExtensions, DiagnosticKind, DiagnosticSeverity};

/// Every check groups the error diagnostics of one subsystem, so a failure names the
/// layer at fault and the gate can be turned on per layer.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Check {
    /// Syntax errors.
    Parse,
    /// Import and name resolution errors.
    Bind,
    /// Structural, type-system and semantic errors.
    Validate,
}

impl Check {
    pub fn of(kind: &DiagnosticKind) -> Check {
        match kind {
            DiagnosticKind::Syntax(_) => Check::Parse,
            DiagnosticKind::Compilation(_) | DiagnosticKind::Resolution(_) => Check::Bind,
            DiagnosticKind::Structure(_)
            | DiagnosticKind::TypeSystem(_)
            | DiagnosticKind::Semantic(_) => Check::Validate,
        }
    }
}

impl std::fmt::Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Check::Parse => f.write_str("parse"),
            Check::Bind => f.write_str("bind"),
            Check::Validate => f.write_str("validate"),
        }
    }
}

/// One failure bucket of one contract: all error diagnostics sharing a code.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Failure {
    pub check: Check,
    pub code: String,
    pub count: usize,
    /// The first diagnostic's message, for the report.
    pub message: String,
}

impl Failure {
    pub fn key(&self) -> String {
        format!("{check}:{code}", check = self.check, code = self.code)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outcome {
    /// The corpus file stem, `<chain>_<address>`.
    pub id: String,
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evm_target: Option<String>,
    pub ms: u128,
    /// The record could not be compiled at all (unsupported version or target).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skipped: Option<String>,
    /// Slang panicked; the message and location, which is the bucket key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub panic: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failures: Vec<Failure>,
    #[serde(default)]
    pub warnings: usize,
}

impl Outcome {
    pub fn passed(&self) -> bool {
        self.skipped.is_none() && self.panic.is_none() && self.failures.is_empty()
    }
}

/// Groups error diagnostics by (check, code); warnings are only counted.
pub fn classify<'a>(
    diagnostics: impl Iterator<Item = &'a DiagnosticKind>,
) -> (Vec<Failure>, usize) {
    let mut warnings = 0;
    let mut buckets: BTreeMap<(Check, &'static str), Failure> = BTreeMap::new();
    for kind in diagnostics {
        if kind.severity() != DiagnosticSeverity::Error {
            warnings += 1;
            continue;
        }
        let check = Check::of(kind);
        buckets
            .entry((check, kind.code()))
            .and_modify(|failure| failure.count += 1)
            .or_insert_with(|| Failure {
                check,
                code: kind.code().to_owned(),
                count: 1,
                message: kind.message(),
            });
    }
    (buckets.into_values().collect(), warnings)
}
