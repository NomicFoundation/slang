// Generated on 2024-11-13T09:24:13.244Z
use super::model::*;

pub trait Handler {
    #[allow(unused_variables)]
    fn handle_model(self: &mut Self, node: &Box<Model>) {}

    #[allow(unused_variables)]
    fn handle_fqn(self: &mut Self, node: &Fqn) {}

    #[allow(unused_variables)]
    fn handle_definition(self: &mut Self, node: &Box<Definition>) {}

    #[allow(unused_variables)]
    fn handle_deletion(self: &mut Self, node: &Box<Deletion>) {}

    #[allow(unused_variables)]
    fn handle_member_modification(self: &mut Self, node: &Box<MemberModification>) {}

    #[allow(unused_variables)]
    fn handle_member_deletion(self: &mut Self, node: &Box<MemberDeletion>) {}

    #[allow(unused_variables)]
    fn handle_member_addition(self: &mut Self, node: &Box<MemberAddition>) {}

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
    fn handle_sum_type(self: &mut Self, node: &Box<SumType>) {}

    #[allow(unused_variables)]
    fn handle_product_type(self: &mut Self, node: &Box<ProductType>) {}

    #[allow(unused_variables)]
    fn handle_product_member(self: &mut Self, node: &Box<ProductMember>) {}

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
    fn handle_map_type(self: &mut Self, node: &Box<MapType>) {}

    #[allow(unused_variables)]
    fn handle_set_type(self: &mut Self, node: &Box<SetType>) {}

    #[allow(unused_variables)]
    fn handle_sequence_type(self: &mut Self, node: &Box<SequenceType>) {}

    #[allow(unused_variables)]
    fn handle_option_type(self: &mut Self, node: &Box<OptionType>) {}

    #[allow(unused_variables)]
    fn handle_named_type_reference(self: &mut Self, node: &Box<NamedTypeReference>) {}

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
