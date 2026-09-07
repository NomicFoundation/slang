use slang_solidity_v2_semantic::binder::Typing;

use super::super::{SuperKeywordStruct, Type};

impl SuperKeywordStruct {
    /// The contract or interface `super` is written in: the anchor after which
    /// a member of `super` is searched in the linearisation of the contract
    /// being compiled. `None` where Slang reported the keyword unresolved.
    pub fn anchor(&self) -> Option<Type> {
        match self.semantic.binder().node_typing(self.ir_node.id()) {
            Typing::Super(type_id) => Some(Type::create(*type_id, &self.semantic)),
            _ => None,
        }
    }
}
