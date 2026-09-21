use super::super::{FunctionCallExpressionStruct, Type};

impl FunctionCallExpressionStruct {
    /// Returns `true` if this call is a type conversion (e.g. `uint256(x)`,
    /// `address(y)`) rather than a function call: ie. its operand names a
    /// type — types as a meta-type — rather than a value.
    pub fn is_type_conversion(&self) -> bool {
        self.operand().get_type().is_some_and(|operand_type| {
            matches!(operand_type, Type::MetaType(_) | Type::UserMetaType(_))
        })
    }

    /// For an `abi.encodeCall`, the function type the call encodes its
    /// arguments against: the callee as solc dispatches it, with every
    /// `calldata` parameter turned into `memory`. The callee's own type is
    /// unchanged by this; `None` for any other call, and for a callee that is
    /// no externally callable function.
    pub fn encode_call_callee_type(&self) -> Option<Type> {
        let type_id = self
            .semantic
            .binder()
            .encode_call_callee_type_id(self.ir_node.id())?;
        Some(Type::create(type_id, &self.semantic))
    }
}
