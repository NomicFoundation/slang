use super::super::{FunctionDefinition, InterfaceDefinitionStruct};

impl InterfaceDefinitionStruct {
    pub fn functions(&self) -> Vec<FunctionDefinition> {
        self.members().iter_function_definitions().collect()
    }
}
