use slang_solidity_v2_cst::structured_cst::nodes as cst;

use crate::ir;

impl TryFrom<&cst::StateVariableAttribute> for ir::StateVariableVisibility {
    type Error = ();

    fn try_from(value: &cst::StateVariableAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::StateVariableAttribute::InternalKeyword(_) => Ok(Self::Internal),
            cst::StateVariableAttribute::PrivateKeyword(_) => Ok(Self::Private),
            cst::StateVariableAttribute::PublicKeyword(_) => Ok(Self::Public),
            // A state variable can't be external.
            // TODO(v2): If a state variable is marked external, Slang fails to parse and emits
            // wrong keyword diagnostic. Instead a post-parse diagnostic should be emitted.
            _ => Err(()),
        }
    }
}
