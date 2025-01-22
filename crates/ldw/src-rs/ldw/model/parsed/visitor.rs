// Generated on 2024-11-13T09:33:43.385Z
use super::model::*;

pub trait Visitor {
    #[allow(unused_variables)]
    fn handle_model(self: &mut Self, node: &Box<Model>) {
        self.handle_fqn(&node.name);
        if let Some(value) = &node.parent_name {
            self.handle_fqn(&value);
        }
        for value in &node.values {
            self.handle_model_value(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_fqn(self: &mut Self, node: &Fqn) {}

    #[allow(unused_variables)]
    fn handle_definition(self: &mut Self, node: &Box<Definition>) {
        self.handle_type(&node.r#type);
    }

    #[allow(unused_variables)]
    fn handle_deletion(self: &mut Self, node: &Box<Deletion>) {}

    #[allow(unused_variables)]
    fn handle_member_modification(self: &mut Self, node: &Box<MemberModification>) {
        for value in &node.values {
            self.handle_member_modification_value(&value);
        }
    }

    #[allow(unused_variables)]
    fn handle_member_deletion(self: &mut Self, node: &Box<MemberDeletion>) {}

    #[allow(unused_variables)]
    fn handle_member_addition(self: &mut Self, node: &Box<MemberAddition>) {
        self.handle_member_addition_value(&node.value);
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

    #[allow(unused_variables)]
    fn handle_model_value(self: &mut Self, node: &ModelValue) {
        match node {
            ModelValue::Deletion(value) => self.handle_deletion(value),
            ModelValue::MemberModification(value) => self.handle_member_modification(value),
            ModelValue::Definition(value) => self.handle_definition(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_member_modification_value(self: &mut Self, node: &MemberModificationValue) {
        match node {
            MemberModificationValue::MemberDeletion(value) => self.handle_member_deletion(value),
            MemberModificationValue::MemberAddition(value) => self.handle_member_addition(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_member_addition_value(self: &mut Self, node: &MemberAdditionValue) {
        match node {
            MemberAdditionValue::ProductMember(value) => self.handle_product_member(value),
            MemberAdditionValue::Type(value) => self.handle_type(value),
        }
    }
}
