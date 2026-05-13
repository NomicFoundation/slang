use slang_solidity_v2_cst::structured_cst::nodes as cst;

use crate::ir;

impl TryFrom<&cst::FunctionAttribute> for ir::FunctionVisibility {
    type Error = ();

    fn try_from(value: &cst::FunctionAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::FunctionAttribute::ExternalKeyword(_) => Ok(Self::External),
            cst::FunctionAttribute::InternalKeyword(_) => Ok(Self::Internal),
            cst::FunctionAttribute::PrivateKeyword(_) => Ok(Self::Private),
            cst::FunctionAttribute::PublicKeyword(_) => Ok(Self::Public),
            _ => Err(()),
        }
    }
}

impl TryFrom<&cst::FunctionTypeAttribute> for ir::FunctionVisibility {
    type Error = ();

    fn try_from(value: &cst::FunctionTypeAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::FunctionTypeAttribute::ExternalKeyword(_) => Ok(Self::External),
            cst::FunctionTypeAttribute::InternalKeyword(_) => Ok(Self::Internal),
            cst::FunctionTypeAttribute::PrivateKeyword(_) => Ok(Self::Private),
            cst::FunctionTypeAttribute::PublicKeyword(_) => Ok(Self::Public),
            _ => Err(()),
        }
    }
}

impl TryFrom<&cst::ConstructorAttribute> for ir::FunctionVisibility {
    type Error = ();

    fn try_from(value: &cst::ConstructorAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::ConstructorAttribute::InternalKeyword(_) => Ok(Self::Internal),
            cst::ConstructorAttribute::PublicKeyword(_) => Ok(Self::Public),
            _ => Err(()),
        }
    }
}
