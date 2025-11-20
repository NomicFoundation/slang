#![allow(unused)]
use std::rc::Rc;

use super::input as input_ir;
use crate::backend::SemanticAnalysis;
use crate::cst::TerminalNode;

include!("ast_nodes.generated.rs");

impl SourceUnitStruct {
    pub fn contracts(&self) -> impl Iterator<Item = ContractDefinition> + use<'_> {
        self.members()
            .filter_map(|member| member.as_contract_definition())
    }
}
