//! This module contains the serde types for the of 'solc' JSON CLI:
//! <https://docs.soliditylang.org/en/latest/using-the-compiler.html#compiler-input-and-output-json-description>

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize)]
pub struct CliInput {
    pub language: LanguageSelector,
    pub sources: BTreeMap<String, InputSource>,
}

#[derive(Debug, Serialize)]
pub enum LanguageSelector {
    Solidity,
}

#[derive(Debug, Serialize)]
pub struct InputSource {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CliOutput {
    pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Error {
    pub message: String,

    pub severity: Severity,

    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "errorCode")]
    pub error_code: String,

    #[serde(rename = "sourceLocation")]
    pub location: SourceLocation,
}

#[derive(Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SourceLocation {
    pub file: PathBuf,
    /// 0-based character index
    pub start: usize,
    /// 0-based character index
    pub end: usize,
}

#[derive(Display, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    // Ordered from least to most severe:
    Info,
    Warning,
    Error,
}
