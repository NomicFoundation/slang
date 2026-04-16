use std::rc::Rc;

use crate::ir::nodes as output;

pub enum IrFunctionAttribute {
    Visibility(output::FunctionVisibility),
    Mutability(output::FunctionMutability),
    Virtual,
    Override(output::OverridePaths),
    Modifier(output::ModifierInvocation),
}

pub enum IrConstructorAttribute {
    Visibility(output::FunctionVisibility),
    Mutability(output::FunctionMutability),
    Modifier(output::ModifierInvocation),
}

pub enum IrFallbackFunctionAttribute {
    Visibility(output::FunctionVisibility),
    Mutability(output::FunctionMutability),
    Virtual,
    Override(output::OverridePaths),
    Modifier(output::ModifierInvocation),
}

pub enum IrReceiveFunctionAttribute {
    Visibility(output::FunctionVisibility),
    Mutability(output::FunctionMutability),
    Virtual,
    Override(output::OverridePaths),
    Modifier(output::ModifierInvocation),
}

pub enum IrModifierAttribute {
    Virtual,
    Override(output::OverridePaths),
}

pub enum IrStateVariableAttribute {
    Visibility(output::StateVariableVisibility),
    Mutability(output::StateVariableMutability),
    Override(output::OverridePaths),
}

pub enum IrFunctionTypeAttribute {
    Visibility(output::FunctionVisibility),
    Mutability(output::FunctionMutability),
}

pub enum IrContractSpecifier {
    Inheritance(output::InheritanceTypes),
    StorageLayout(output::Expression),
}

pub struct IrFunctionType {
    pub parameters: output::Parameters,
    pub attributes: Vec<IrFunctionTypeAttribute>,
    pub returns: Option<output::Parameters>,
}

impl IrFunctionType {
    pub fn into_output(self) -> output::FunctionType {
        let mut visibility = output::FunctionVisibility::Internal;
        let mut mutability = output::FunctionMutability::NonPayable;
        for attribute in self.attributes {
            match attribute {
                IrFunctionTypeAttribute::Visibility(v) => visibility = v,
                IrFunctionTypeAttribute::Mutability(m) => mutability = m,
            }
        }
        Rc::new(output::FunctionTypeStruct {
            parameters: self.parameters,
            visibility,
            mutability,
            returns: self.returns,
        })
    }
}
