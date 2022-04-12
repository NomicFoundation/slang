use crate::builds::BuildInfo;
use regex::Regex;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{path::PathBuf, process::Command};

pub struct TestOutput {
    pub version: Version,
    pub success: bool,
    pub contents: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AstOutput {
    #[serde(skip_serializing, alias = "absolutePath")]
    absolute_path: String,

    #[serde(skip_serializing, alias = "exportedSymbols")]
    exported_symbols: Value,

    #[serde(skip_serializing)]
    id: u32,

    #[serde(skip_serializing)]
    license: Option<String>,

    #[serde(alias = "nodeType")]
    node_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<AstNode>>,

    #[serde(skip_serializing)]
    src: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AstNode {
    #[serde(skip_serializing, alias = "abstract")]
    _abstract: Option<bool>,

    #[serde(skip_serializing, alias = "baseContracts")]
    base_contracts: Option<Value>,

    #[serde(skip_serializing, alias = "canonicalName")]
    canonical_name: Option<String>,

    #[serde(skip_serializing, alias = "contractDependencies")]
    contract_dependencies: Option<Value>,

    #[serde(skip_serializing, alias = "contractKind")]
    contract_kind: Option<String>,

    #[serde(skip_serializing)]
    documentation: Option<String>,

    #[serde(skip_serializing, alias = "fullyImplemented")]
    fully_implemented: Option<bool>,

    #[serde(skip_serializing)]
    id: u32,

    #[serde(skip_serializing, alias = "linearizedBaseContracts")]
    linearized_base_contracts: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    literals: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing, alias = "nameLocation")]
    name_location: Option<String>,

    #[serde(alias = "nodeType")]
    node_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<AstNode>>,

    #[serde(skip_serializing)]
    scope: Option<u32>,

    #[serde(skip_serializing)]
    src: String,

    #[serde(skip_serializing, alias = "usedErrors")]
    used_errors: Option<Value>,
}

pub fn fetch_ast_json(test: &PathBuf, build: &BuildInfo) -> TestOutput {
    let command = if build.version.lt(&Version::parse("0.4.12").unwrap()) {
        "--ast-json"
    } else {
        "--ast-compact-json"
    };

    let output = Command::new(&build.local_path)
        .arg(command)
        .arg(test)
        .output()
        .unwrap();

    if !output.status.success() {
        return TestOutput {
            version: build.version.to_owned(),
            success: false,
            contents: String::from_utf8(output.stderr)
                .unwrap()
                // Replace test file path:
                .replace(test.to_str().unwrap(), "input.sol"),
        };
    }

    let mut stdout = String::from_utf8(output.stdout).unwrap();

    // Remove AST header:
    stdout = stdout.replace("JSON AST (compact format):", "");

    // Remove File header: "======= path/to/test/file/input.sol ======="
    stdout = Regex::new(r"=======(.*)=======")
        .unwrap()
        .replace_all(&stdout, "")
        .to_string();

    // Make sure we accounted for all possible AST shapes
    let deserialized: AstOutput = match serde_json::from_str(&stdout) {
        Ok(value) => value,
        Err(error) => panic!(
            "Failed to parse AST version {} for file: {}\n{:?}\n{}",
            build.version,
            test.to_str().unwrap(),
            error,
            stdout
        ),
    };

    // Serialize again to a pretty JSON, ignoring values we don't care about:
    return TestOutput {
        version: build.version.to_owned(),
        success: true,
        contents: serde_json::to_string_pretty(&deserialized).unwrap(),
    };
}
