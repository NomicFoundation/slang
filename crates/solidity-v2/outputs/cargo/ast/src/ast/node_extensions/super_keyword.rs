use slang_solidity_v2_semantic::binder::Typing;

use super::super::{ContractDefinition, Definition, SuperKeywordStruct};

impl SuperKeywordStruct {
    /// The contract `super` is written in: the enclosing contract after which a member of
    /// `super` is searched in the linearisation of the contract being compiled.
    /// `None` where Slang reported the keyword unresolved.
    pub fn enclosing_contract(&self) -> Option<ContractDefinition> {
        let Typing::Super(enclosing_contract) =
            self.semantic.binder().node_typing(self.ir_node.id())
        else {
            return None;
        };
        let Definition::Contract(enclosing_contract) =
            Definition::try_create(*enclosing_contract, &self.semantic)
                .expect("the `super` enclosing_contract is a definition")
        else {
            unreachable!("expected the `super` enclosing_contract to be a contract");
        };
        Some(enclosing_contract)
    }
}
