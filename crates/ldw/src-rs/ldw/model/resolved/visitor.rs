// Generated on 2024-11-13T09:33:44.739Z
use super::model::*;

pub trait Visitor {
    #[allow(unused_variables)]
    fn handle_model(self: &mut Self, node: &Box<Model>) {
        self.handle_fqn(&node.name);
        if let Some(value) = &node.parent {
            self.handle_model(&value);
        }
        for (_, value) in &node.definitions {
            self.handle_definition(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_fqn(self: &mut Self, node: &Fqn) {}

    #[allow(unused_variables)]
    fn handle_definition(self: &mut Self, node: &Box<Definition>) {
        self.handle_type(&node.r#type);
    }

    #[allow(unused_variables)]
    fn handle_type(self: &mut Self, node: &Type) {
        match node {
            Type::VoidType(value) => self.handle_void_type(value),
            Type::PrimitiveType(value) => self.handle_primitive_type(value),
            Type::EnumType(value) => self.handle_enum_type(value),
            Type::TypeWithStructure(value) => self.handle_type_with_structure(value),
            Type::NamedTypeReference(value) => self.handle_named_type_reference(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_void_type(self: &mut Self, node: &Box<VoidType>) {}

    #[allow(unused_variables)]
    fn handle_primitive_type(self: &mut Self, node: &PrimitiveType) {}

    #[allow(unused_variables)]
    fn handle_enum_type(self: &mut Self, node: &Box<EnumType>) {}

    #[allow(unused_variables)]
    fn handle_type_with_structure(self: &mut Self, node: &TypeWithStructure) {
        match node {
            TypeWithStructure::SumType(value) => self.handle_sum_type(value),
            TypeWithStructure::ProductType(value) => self.handle_product_type(value),
            TypeWithStructure::GenericType(value) => self.handle_generic_type(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_sum_type(self: &mut Self, node: &Box<SumType>) {
        for value in &node.members {
            self.handle_type(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_product_type(self: &mut Self, node: &Box<ProductType>) {
        for value in &node.members {
            self.handle_product_member(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_product_member(self: &mut Self, node: &Box<ProductMember>) {
        self.handle_type(&node.r#type);
    }

    #[allow(unused_variables)]
    fn handle_generic_type(self: &mut Self, node: &GenericType) {
        match node {
            GenericType::MapType(value) => self.handle_map_type(value),
            GenericType::SetType(value) => self.handle_set_type(value),
            GenericType::SequenceType(value) => self.handle_sequence_type(value),
            GenericType::OptionType(value) => self.handle_option_type(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_map_type(self: &mut Self, node: &Box<MapType>) {
        self.handle_type(&node.key_type);
        self.handle_type(&node.value_type);
    }

    #[allow(unused_variables)]
    fn handle_set_type(self: &mut Self, node: &Box<SetType>) {
        self.handle_type(&node.key_type);
    }

    #[allow(unused_variables)]
    fn handle_sequence_type(self: &mut Self, node: &Box<SequenceType>) {
        self.handle_type(&node.element_type);
    }

    #[allow(unused_variables)]
    fn handle_option_type(self: &mut Self, node: &Box<OptionType>) {
        self.handle_type(&node.r#type);
    }

    #[allow(unused_variables)]
    fn handle_named_type_reference(self: &mut Self, node: &Box<NamedTypeReference>) {
        self.handle_fqn(&node.fqn);
    }
}
