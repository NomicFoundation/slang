use slang_solidity_v2_semantic::binder::Typing;

use super::super::{Definition, SuperKeywordStruct};
use super::ContractBase;

impl SuperKeywordStruct {
    /// The contract or interface `super` is written in: the anchor after which
    /// a member of `super` is searched in the linearisation of the contract
    /// being compiled. `None` where Slang reported the keyword unresolved.
    pub fn anchor(&self) -> Option<ContractBase> {
        let Typing::Super(anchor) = self.semantic.binder().node_typing(self.ir_node.id()) else {
            return None;
        };
        ContractBase::from_definition(&Definition::try_create(*anchor, &self.semantic)?)
    }
}
