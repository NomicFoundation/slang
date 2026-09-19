use super::super::{FunctionDefinition, FunctionKind, InterfaceDefinitionStruct};

impl InterfaceDefinitionStruct {
    pub fn functions(&self) -> Vec<FunctionDefinition> {
        self.members()
            .iter_function_definitions()
            .filter(|function| matches!(function.kind(), FunctionKind::Regular))
            .collect()
    }
}
