use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder;

use crate::abi::ContractAbi;
use crate::ast::InterfaceDefinitionStruct;

impl InterfaceDefinitionStruct {
    /// Computes the ERC-165 interface identifier: the XOR of the 4-byte selectors of the regular
    /// functions the interface itself declares, excluding inherited ones.
    pub fn compute_interface_id(&self) -> Option<u32> {
        let mut interface_id = 0u32;
        for function in self.members().iter_function_definitions() {
            if function.ir_node.kind != ir::FunctionKind::Regular {
                continue;
            }
            interface_id ^= function.compute_selector()?;
        }
        Some(interface_id)
    }

    /// `None` when a base is a contract, which solc rejects.
    pub fn compute_abi(&self) -> Option<ContractAbi> {
        if self.has_contract_base() {
            return None;
        }
        let mut entries = Vec::new();
        for function in &self.linearised_functions() {
            entries.push(function.compute_abi_entry()?);
        }
        for error in &self.linearised_errors() {
            entries.push(error.compute_abi_entry()?);
        }
        for event in &self.linearised_events() {
            entries.push(event.compute_abi_entry()?);
        }
        Some(ContractAbi::new(
            self.ir_node.id(),
            self.ir_node.name.unparse().to_string(),
            self.get_file_id().clone(),
            entries,
            Vec::new(),
            Vec::new(),
        ))
    }

    // TODO(validation) SDR[1369]: an interface can only inherit from other interfaces.
    fn has_contract_base(&self) -> bool {
        let binder = self.semantic.binder();
        binder
            .get_linearised_bases(self.ir_node.id())
            .into_iter()
            .flatten()
            .any(|base_id| {
                matches!(
                    binder.find_definition_by_id(*base_id),
                    Some(binder::Definition::Contract(_))
                )
            })
    }
}
