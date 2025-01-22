// Generated on 2024-11-13T12:00:22.739Z
use super::model::*;

pub trait InPlaceTransformer {
    #[allow(unused_mut)]
    fn transform_model(self: &mut Self, mut node: Box<Model>) -> Box<Model> {
        node.name = self.transform_fqn(node.name);
        node.parent = node.parent.map(|value| self.transform_model(value));
        node.definitions = node
            .definitions
            .into_iter()
            .map(|(key, value)| (key, self.transform_definition(value)))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_fqn(self: &mut Self, mut node: Fqn) -> Fqn {
        node.into_iter().map(|value| value).collect()
    }

    #[allow(unused_mut)]
    fn transform_definition(self: &mut Self, mut node: Box<Definition>) -> Box<Definition> {
        node.r#type = self.transform_type(node.r#type);
        node
    }

    #[allow(unused_mut)]
    fn transform_type(self: &mut Self, mut node: Type) -> Type {
        match node {
            Type::VoidType(value) => Type::VoidType(self.transform_void_type(value)),
            Type::PrimitiveType(value) => Type::PrimitiveType(self.transform_primitive_type(value)),
            Type::EnumType(value) => Type::EnumType(self.transform_enum_type(value)),
            Type::TypeWithStructure(value) => {
                Type::TypeWithStructure(self.transform_type_with_structure(value))
            }
            Type::NamedTypeReference(value) => {
                Type::NamedTypeReference(self.transform_named_type_reference(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_void_type(self: &mut Self, mut node: Box<VoidType>) -> Box<VoidType> {
        node
    }

    #[allow(unused_mut)]
    fn transform_primitive_type(self: &mut Self, mut node: PrimitiveType) -> PrimitiveType {
        node
    }

    #[allow(unused_mut)]
    fn transform_enum_type(self: &mut Self, mut node: Box<EnumType>) -> Box<EnumType> {
        node
    }

    #[allow(unused_mut)]
    fn transform_type_with_structure(
        self: &mut Self,
        mut node: TypeWithStructure,
    ) -> TypeWithStructure {
        match node {
            TypeWithStructure::SumType(value) => {
                TypeWithStructure::SumType(self.transform_sum_type(value))
            }
            TypeWithStructure::ProductType(value) => {
                TypeWithStructure::ProductType(self.transform_product_type(value))
            }
            TypeWithStructure::GenericType(value) => {
                TypeWithStructure::GenericType(self.transform_generic_type(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_sum_type(self: &mut Self, mut node: Box<SumType>) -> Box<SumType> {
        node.members = node
            .members
            .into_iter()
            .map(|value| self.transform_type(value))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_product_type(self: &mut Self, mut node: Box<ProductType>) -> Box<ProductType> {
        node.members = node
            .members
            .into_iter()
            .map(|value| self.transform_product_member(value))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_product_member(
        self: &mut Self,
        mut node: Box<ProductMember>,
    ) -> Box<ProductMember> {
        node.r#type = self.transform_type(node.r#type);
        node
    }

    #[allow(unused_mut)]
    fn transform_generic_type(self: &mut Self, mut node: GenericType) -> GenericType {
        match node {
            GenericType::MapType(value) => GenericType::MapType(self.transform_map_type(value)),
            GenericType::SetType(value) => GenericType::SetType(self.transform_set_type(value)),
            GenericType::SequenceType(value) => {
                GenericType::SequenceType(self.transform_sequence_type(value))
            }
            GenericType::OptionType(value) => {
                GenericType::OptionType(self.transform_option_type(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_map_type(self: &mut Self, mut node: Box<MapType>) -> Box<MapType> {
        node.key_type = self.transform_type(node.key_type);
        node.value_type = self.transform_type(node.value_type);
        node
    }

    #[allow(unused_mut)]
    fn transform_set_type(self: &mut Self, mut node: Box<SetType>) -> Box<SetType> {
        node.key_type = self.transform_type(node.key_type);
        node
    }

    #[allow(unused_mut)]
    fn transform_sequence_type(self: &mut Self, mut node: Box<SequenceType>) -> Box<SequenceType> {
        node.element_type = self.transform_type(node.element_type);
        node
    }

    #[allow(unused_mut)]
    fn transform_option_type(self: &mut Self, mut node: Box<OptionType>) -> Box<OptionType> {
        node.r#type = self.transform_type(node.r#type);
        node
    }

    #[allow(unused_mut)]
    fn transform_named_type_reference(
        self: &mut Self,
        mut node: Box<NamedTypeReference>,
    ) -> Box<NamedTypeReference> {
        node.fqn = self.transform_fqn(node.fqn);
        node
    }
}
