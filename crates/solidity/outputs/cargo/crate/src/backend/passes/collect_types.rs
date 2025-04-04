use crate::backend::l1_typed_cst;

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Contract(String),
    Struct(String),
    Enum(String),
    UserDefinedValueType(String),
}

pub struct Pass {
    pub types: Vec<Type>,
}

impl Pass {
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }
}

impl l1_typed_cst::visitor::Visitor for Pass {
    fn leave_contract_definition(&mut self, target: &l1_typed_cst::ContractDefinition) {
        self.types.push(Type::Contract(target.name.unparse()));
    }

    fn leave_struct_definition(&mut self, target: &l1_typed_cst::StructDefinition) {
        self.types.push(Type::Struct(target.name.unparse()));
    }

    fn leave_enum_definition(&mut self, target: &l1_typed_cst::EnumDefinition) {
        self.types.push(Type::Enum(target.name.unparse()));
    }

    fn leave_user_defined_value_type_definition(
        &mut self,
        target: &l1_typed_cst::UserDefinedValueTypeDefinition,
    ) {
        self.types
            .push(Type::UserDefinedValueType(target.name.unparse()));
    }
}
