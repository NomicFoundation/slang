use slang_solidity_v2_ir::ir;

use crate::ast::{create_function_definition, Definition, InterfaceDefinitionStruct};

impl InterfaceDefinitionStruct {
    /// Computes the ERC-165 interface identifier: the XOR of the 4-byte selectors of every function
    /// the interface declares, including those inherited from base interfaces (its C3 linearisation).
    pub fn compute_interface_id(&self) -> u32 {
        let Some(linearised_bases) = self
            .semantic
            .binder()
            .get_linearised_bases(self.ir_node.id())
        else {
            return 0;
        };

        let mut interface_id = 0u32;
        for base_id in linearised_bases {
            let Some(Definition::Interface(interface)) =
                Definition::try_create(*base_id, &self.semantic)
            else {
                continue;
            };
            for member in &interface.ir_node.members {
                let ir::ContractMember::FunctionDefinition(function) = member else {
                    continue;
                };
                if let Some(selector) =
                    create_function_definition(function, &self.semantic).compute_selector()
                {
                    interface_id ^= selector;
                }
            }
        }
        interface_id
    }
}
