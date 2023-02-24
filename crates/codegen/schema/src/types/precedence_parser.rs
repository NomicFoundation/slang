use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::parser::ParserDefinition;

#[allow(dead_code)]
pub type PrecedenceParserRef = std::rc::Rc<PrecedenceParser>;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct PrecedenceParser {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub definition: PrecedenceParserDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PrecedenceParserDefinition {
    #[schemars(title = "Operator Definitions")]
    pub operators: Vec<OperatorDefinition>,

    #[schemars(title = "Primary Expression References")]
    pub primary_expressions: Vec<Reference>,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OperatorDefinition {
    pub name: String,
    pub model: OperatorModel,
    #[serde(flatten)]
    pub definition: ParserDefinition,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum OperatorModel {
    BinaryLeftAssociative,
    BinaryRightAssociative,
    UnaryPrefix,
    UnarySuffix,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Reference {
    #[schemars(title = "Production Reference")]
    pub reference: String,
}
