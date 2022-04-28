use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct StandardInput {
    pub language: String,

    pub sources: HashMap<String, InputSource>,

    pub settings: OutputSettings,
}

#[derive(Serialize)]
pub struct InputSource {
    pub content: String,
}

#[derive(Serialize)]
pub struct OutputSettings {
    #[serde(rename = "outputSelection")]
    pub output_selection: HashMap<String, HashMap<String, Vec<String>>>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StandardOutput {
    pub errors: Option<Vec<ErrorMessage>>,

    pub sources: Option<HashMap<String, OutputSourceFile>>,

    contracts: Option<Value>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorMessage {
    component: String,

    #[serde(rename = "formattedMessage")]
    pub formatted_message: String,

    message: String,

    #[serde(rename = "errorCode")]
    error_code: Option<String>,

    pub severity: String,

    #[serde(rename = "sourceLocation")]
    source_location: Option<Value>,

    #[serde(rename = "type")]
    _type: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputSourceFile {
    id: u32,

    pub ast: Option<Ast>,

    #[serde(rename = "legacyAST")]
    legacy_ast: Option<Value>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(Clone)]
pub struct Ast {
    #[serde(skip_serializing, rename = "absolutePath")]
    absolute_path: String,

    #[serde(skip_serializing, rename = "exportedSymbols")]
    exported_symbols: Option<Value>,

    #[serde(skip_serializing)]
    id: u32,

    #[serde(skip_serializing)]
    license: Option<String>,

    #[serde(rename = "nodeType")]
    node_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<AstNode>>,

    #[serde(skip_serializing)]
    src: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(Clone)]
struct AstNode {
    #[serde(skip_serializing, rename = "abstract")]
    _abstract: Option<bool>,

    #[serde(skip_serializing, rename = "baseContracts")]
    base_contracts: Option<Value>,

    #[serde(skip_serializing, rename = "canonicalName")]
    canonical_name: Option<String>,

    #[serde(skip_serializing, rename = "contractDependencies")]
    contract_dependencies: Option<Value>,

    #[serde(skip_serializing, rename = "contractKind")]
    contract_kind: Option<String>,

    #[serde(skip_serializing)]
    documentation: Option<String>,

    #[serde(skip_serializing, rename = "fullyImplemented")]
    fully_implemented: Option<bool>,

    #[serde(skip_serializing)]
    id: u32,

    #[serde(skip_serializing, rename = "linearizedBaseContracts")]
    linearized_base_contracts: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    literals: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing, rename = "nameLocation")]
    name_location: Option<String>,

    #[serde(rename = "nodeType")]
    node_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<AstNode>>,

    #[serde(skip_serializing)]
    scope: Option<u32>,

    #[serde(skip_serializing)]
    src: String,

    #[serde(skip_serializing, rename = "usedErrors")]
    used_errors: Option<Value>,
}
