use slang_solidity_v2_cst::structured_cst::nodes as cst;

use crate::ir;

impl TryFrom<&cst::FunctionAttribute> for ir::FunctionMutability {
    type Error = ();

    fn try_from(value: &cst::FunctionAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::FunctionAttribute::PayableKeyword(_) => Ok(Self::Payable),
            cst::FunctionAttribute::PureKeyword(_) => Ok(Self::Pure),
            cst::FunctionAttribute::ViewKeyword(_) => Ok(Self::View),
            _ => Err(()),
        }
    }
}

impl TryFrom<&cst::FunctionTypeAttribute> for ir::FunctionMutability {
    type Error = ();

    fn try_from(value: &cst::FunctionTypeAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::FunctionTypeAttribute::PayableKeyword(_) => Ok(Self::Payable),
            cst::FunctionTypeAttribute::PureKeyword(_) => Ok(Self::Pure),
            cst::FunctionTypeAttribute::ViewKeyword(_) => Ok(Self::View),
            _ => Err(()),
        }
    }
}

impl TryFrom<&cst::ConstructorAttribute> for ir::FunctionMutability {
    type Error = ();

    fn try_from(value: &cst::ConstructorAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::ConstructorAttribute::PayableKeyword(_) => Ok(Self::Payable),
            _ => Err(()),
        }
    }
}

impl TryFrom<&cst::FallbackFunctionAttribute> for ir::FunctionMutability {
    type Error = ();

    fn try_from(value: &cst::FallbackFunctionAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::FallbackFunctionAttribute::PayableKeyword(_) => Ok(Self::Payable),
            cst::FallbackFunctionAttribute::PureKeyword(_) => Ok(Self::Pure),
            cst::FallbackFunctionAttribute::ViewKeyword(_) => Ok(Self::View),
            _ => Err(()),
        }
    }
}

impl TryFrom<&cst::ReceiveFunctionAttribute> for ir::FunctionMutability {
    type Error = ();

    fn try_from(value: &cst::ReceiveFunctionAttribute) -> Result<Self, Self::Error> {
        match value {
            cst::ReceiveFunctionAttribute::PayableKeyword(_) => Ok(Self::Payable),
            _ => Err(()),
        }
    }
}
