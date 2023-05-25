use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::ParserRef;

pub type PrecedenceParserRef = std::rc::Rc<PrecedenceParser>;

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PrecedenceParser {
    #[schemars(title = "Operator Definitions")]
    pub operators: Vec<OperatorDefinition>,

    #[schemars(title = "Primary Expression")]
    pub primary_expression: ParserRef,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OperatorDefinition {
    pub name: String,
    pub model: OperatorModel,
    pub operator: ParserRef,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, JsonSchema, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub enum OperatorModel {
    BinaryLeftAssociative,
    BinaryRightAssociative,
    UnaryPrefix,
    UnaryPostfix,
}
