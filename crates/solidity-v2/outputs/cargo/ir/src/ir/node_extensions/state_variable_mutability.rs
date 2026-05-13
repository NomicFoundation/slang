use slang_solidity_v2_cst::structured_cst::nodes as cst;

use crate::ir;

impl TryFrom<&cst::StateVariableAttribute> for ir::StateVariableMutability {
    type Error = ();

    fn try_from(value: &cst::StateVariableAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::StateVariableAttribute::ConstantKeyword(_) => Ok(Self::Constant),
            cst::StateVariableAttribute::ImmutableKeyword(_) => Ok(Self::Immutable),
            cst::StateVariableAttribute::TransientKeyword(_) => Ok(Self::Transient),
            _ => Err(()),
        }
    }
}
