// Generated on 2024-11-13T12:02:52.897Z
pub type ModelRef = Box<Model>;
pub struct Model {
    pub name: Fqn,
    pub parent: Option<ModelRef>,
    pub definitions: std::collections::HashMap<String, DefinitionRef>,
}

pub type Fqn = Vec<String>;

pub type DefinitionRef = Box<Definition>;
pub struct Definition {
    pub name: String,
    pub r#type: Type,
}

pub enum Type {
    VoidType(VoidTypeRef),
    PrimitiveType(PrimitiveType),
    EnumType(EnumTypeRef),
    TypeWithStructure(TypeWithStructure),
    NamedTypeReference(NamedTypeReferenceRef),
}

pub type VoidTypeRef = Box<VoidType>;
pub struct VoidType {}

pub enum PrimitiveType {
    Boolean,
    String,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

pub type EnumTypeRef = Box<EnumType>;
pub struct EnumType {
    pub members: Vec<String>,
}

pub enum TypeWithStructure {
    SumType(SumTypeRef),
    ProductType(ProductTypeRef),
    GenericType(GenericType),
}

pub type SumTypeRef = Box<SumType>;
pub struct SumType {
    pub members: Vec<Type>,
}

pub type ProductTypeRef = Box<ProductType>;
pub struct ProductType {
    pub members: Vec<ProductMemberRef>,
}

pub type ProductMemberRef = Box<ProductMember>;
pub struct ProductMember {
    pub name: String,
    pub r#type: Type,
}

pub enum GenericType {
    MapType(MapTypeRef),
    SetType(SetTypeRef),
    SequenceType(SequenceTypeRef),
    OptionType(OptionTypeRef),
}

pub type MapTypeRef = Box<MapType>;
pub struct MapType {
    pub key_type: Type,
    pub value_type: Type,
}

pub type SetTypeRef = Box<SetType>;
pub struct SetType {
    pub key_type: Type,
}

pub type SequenceTypeRef = Box<SequenceType>;
pub struct SequenceType {
    pub element_type: Type,
}

pub type OptionTypeRef = Box<OptionType>;
pub struct OptionType {
    pub r#type: Type,
}

pub type NamedTypeReferenceRef = Box<NamedTypeReference>;
pub struct NamedTypeReference {
    pub fqn: Fqn,
}
