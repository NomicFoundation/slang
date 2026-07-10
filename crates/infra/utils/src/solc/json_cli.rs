//! This module contains the serde types for the of 'solc' JSON CLI:
//! <https://docs.soliditylang.org/en/latest/using-the-compiler.html#compiler-input-and-output-json-description>

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CliInput {
    pub language: LanguageSelector,
    pub sources: BTreeMap<String, InputSource>,
    pub settings: CliSettings,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CliSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<bool>,
}

#[derive(Debug, Serialize)]
pub enum LanguageSelector {
    Solidity,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputSource {
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CliOutput {
    pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub message: String,
    pub severity: Severity,
    pub r#type: String,
    /// `None` for errors that solc emits before the parser is invoked.
    /// e.g. the `JSONError` reported for an invalid [`CliSettings::evm_version`].
    pub error_code: Option<String>,
    /// `None` for errors not associated with a specific source file.
    /// e.g. the `JSONError` reported for an invalid [`CliSettings::evm_version`].
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    pub file: PathBuf,
    /// 0-based character index
    pub start: usize,
    /// 0-based character index
    pub end: usize,
}

#[derive(Display, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    // Ordered from least to most severe:
    Info,
    Warning,
    Error,
}
