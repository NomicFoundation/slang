use crate::ast::InterfaceDefinitionStruct;

impl InterfaceDefinitionStruct {
    /// Computes the ERC-165 interface identifier: the XOR of the 4-byte selectors of the functions
    /// the interface itself declares, excluding inherited ones.
    pub fn compute_interface_id(&self) -> Option<u32> {
        let mut interface_id = 0u32;
        for function in self.members().iter_function_definitions() {
            interface_id ^= function.compute_selector()?;
        }
        Some(interface_id)
    }
}
