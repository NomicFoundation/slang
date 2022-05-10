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

    pub ast: Option<AstNode>,

    #[serde(rename = "legacyAST")]
    legacy_ast: Option<Value>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AstNode {
    #[serde(skip_serializing, rename = "absolutePath")]
    absolute_path: Option<String>,

    #[serde(skip_serializing, rename = "abstract")]
    _abstract: Option<bool>,

    #[serde(skip_serializing, rename = "argumentTypes")]
    argument_types: Option<Value>,

    #[serde(skip_serializing)]
    assignments: Option<Vec<u32>>,

    #[serde(skip_serializing, rename = "baseContracts")]
    base_contracts: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Box<AstNode>>,

    #[serde(skip_serializing, rename = "canonicalName")]
    canonical_name: Option<String>,

    #[serde(skip_serializing)]
    constant: Option<bool>,

    #[serde(skip_serializing, rename = "contractDependencies")]
    contract_dependencies: Option<Value>,

    #[serde(skip_serializing, rename = "contractKind")]
    contract_kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    declarations: Option<Vec<AstNode>>,

    #[serde(skip_serializing)]
    documentation: Option<String>,

    #[serde(skip_serializing, rename = "exportedSymbols")]
    exported_symbols: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<Box<AstNode>>,

    #[serde(skip_serializing, rename = "functionReturnParameters")]
    function_return_parameters: Option<Value>,

    #[serde(skip_serializing, rename = "fullyImplemented")]
    fully_implemented: Option<bool>,

    #[serde(skip_serializing, rename = "functionSelector")]
    function_selector: Option<String>,

    #[serde(skip_serializing, rename = "hexValue")]
    hex_value: Option<String>,

    #[serde(skip_serializing)]
    id: Option<u32>,

    #[serde(skip_serializing)]
    implemented: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "initialValue")]
    initial_value: Option<Box<AstNode>>,

    #[serde(skip_serializing, rename = "isConstant")]
    is_constant: Option<bool>,

    #[serde(skip_serializing, rename = "isConstructor")]
    is_constructor: Option<bool>,

    #[serde(skip_serializing, rename = "isDeclaredConst")]
    is_declared_const: Option<bool>,

    #[serde(skip_serializing, rename = "isLValue")]
    is_lvalue: Option<bool>,

    #[serde(skip_serializing, rename = "isPure")]
    is_pure: Option<bool>,

    #[serde(skip_serializing, rename = "lValueRequested")]
    lvalue_requested: Option<bool>,

    #[serde(skip_serializing)]
    kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "leftHandSide")]
    left_hand_side: Option<Box<AstNode>>,

    #[serde(skip_serializing)]
    license: Option<String>,

    #[serde(skip_serializing, rename = "linearizedBaseContracts")]
    linearized_base_contracts: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    literals: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<Vec<String>>,

    #[serde(skip_serializing)]
    mutability: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing, rename = "nameLocation")]
    name_location: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "nodeType")]
    node_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nodes: Option<Vec<AstNode>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<String>,

    #[serde(skip_serializing, rename = "overloadedDeclarations")]
    overloaded_declarations: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Parameters>,

    #[serde(skip_serializing)]
    payable: Option<bool>,

    #[serde(skip_serializing)]
    prefix: Option<bool>,

    #[serde(skip_serializing, rename = "referencedDeclaration")]
    referenced_declaration: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "returnParameters")]
    return_parameters: Option<Parameters>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "rightHandSide")]
    right_hand_side: Option<Box<AstNode>>,

    #[serde(skip_serializing)]
    scope: Option<u32>,

    #[serde(skip_serializing)]
    src: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    statements: Option<Vec<AstNode>>,

    #[serde(skip_serializing, rename = "stateMutability")]
    state_mutability: Option<String>,

    #[serde(skip_serializing, rename = "stateVariable")]
    state_variable: Option<bool>,

    #[serde(skip_serializing, rename = "storageLocation")]
    storage_location: Option<String>,

    #[serde(skip_serializing, rename = "subdenomination")]
    sub_denomination: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "subExpression")]
    sub_expression: Option<Box<AstNode>>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "superFunction")]
    super_function: Option<String>,

    #[serde(skip_serializing, rename = "typeDescriptions")]
    type_descriptions: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "typeName")]
    type_name: Option<Box<AstNode>>,

    #[serde(skip_serializing, rename = "usedErrors")]
    used_errors: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<AstValue>,

    #[serde(skip_serializing, rename = "virtual")]
    _virtual: Option<bool>,

    #[serde(skip_serializing)]
    visibility: Option<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Parameters {
    #[serde(skip_serializing)]
    id: Option<u32>,

    #[serde(skip_serializing, rename = "nodeType")]
    node_type: Option<String>,

    parameters: Vec<AstNode>,

    #[serde(skip_serializing)]
    src: Option<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, untagged)]
pub enum AstValue {
    String(String),
    Node(Box<AstNode>),
}
