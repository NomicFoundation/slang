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
}
