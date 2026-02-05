use std::rc::Rc;

use paste::paste;

use super::Definition;
use crate::backend::types::{self, DataLocation, FunctionTypeKind, TypeId};
use crate::backend::SemanticAnalysis;

// __SLANG_TYPE_TYPES__ keep in sync with binder types
#[derive(Clone)]
pub enum Type {
    Address(AddressType),
    Array(ArrayType),
    Boolean(BooleanType),
    ByteArray(ByteArrayType),
    Bytes(BytesType),
    Contract(ContractType),
    Enum(EnumType),
    FixedPointNumber(FixedPointNumberType),
    Function(FunctionType),
    Integer(IntegerType),
    Interface(InterfaceType),
    Literal(LiteralType),
    Mapping(MappingType),
    String(StringType),
    Struct(StructType),
    Tuple(TupleType),
    UserDefinedValue(UserDefinedValueType),
    Void(VoidType),
}

macro_rules! define_type_variant {
    ($type:ident) => {
        paste! {
            #[derive(Clone)]
            pub struct [<$type Type>] {
                type_id: TypeId,
                semantic: Rc<SemanticAnalysis>,
            }

            impl [<$type Type>] {
                #[allow(unused)]
                fn internal_type(&self) -> &types::Type {
                    self.semantic.types.get_type_by_id(self.type_id)
                }
            }
        }
    };
}

define_type_variant!(Address);
define_type_variant!(Array);
define_type_variant!(Boolean);
define_type_variant!(ByteArray);
define_type_variant!(Bytes);
define_type_variant!(Contract);
define_type_variant!(Enum);
define_type_variant!(FixedPointNumber);
define_type_variant!(Function);
define_type_variant!(Integer);
define_type_variant!(Interface);
define_type_variant!(Literal);
define_type_variant!(Mapping);
define_type_variant!(String);
define_type_variant!(Struct);
define_type_variant!(Tuple);
define_type_variant!(UserDefinedValue);
define_type_variant!(Void);

impl Type {
    pub fn create(type_id: TypeId, semantic: &Rc<SemanticAnalysis>) -> Self {
        let type_ = semantic.types().get_type_by_id(type_id);
        let semantic = Rc::clone(semantic);
        match type_ {
            types::Type::Address { .. } => Self::Address(AddressType { type_id, semantic }),
            types::Type::Array { .. } => Self::Array(ArrayType { type_id, semantic }),
            types::Type::Boolean => Self::Boolean(BooleanType { type_id, semantic }),
            types::Type::ByteArray { .. } => Self::ByteArray(ByteArrayType { type_id, semantic }),
            types::Type::Bytes { .. } => Self::Bytes(BytesType { type_id, semantic }),
            types::Type::Contract { .. } => Self::Contract(ContractType { type_id, semantic }),
            types::Type::Enum { .. } => Self::Enum(EnumType { type_id, semantic }),
            types::Type::FixedPointNumber { .. } => {
                Self::FixedPointNumber(FixedPointNumberType { type_id, semantic })
            }
            types::Type::Function(_) => Self::Function(FunctionType { type_id, semantic }),
            types::Type::Integer { .. } => Self::Integer(IntegerType { type_id, semantic }),
            types::Type::Interface { .. } => Self::Interface(InterfaceType { type_id, semantic }),
            types::Type::Literal(_) => Self::Literal(LiteralType { type_id, semantic }),
            types::Type::Mapping { .. } => Self::Mapping(MappingType { type_id, semantic }),
            types::Type::String { .. } => Self::String(StringType { type_id, semantic }),
            types::Type::Struct { .. } => Self::Struct(StructType { type_id, semantic }),
            types::Type::Tuple { .. } => Self::Tuple(TupleType { type_id, semantic }),
            types::Type::UserDefinedValue { .. } => {
                Self::UserDefinedValue(UserDefinedValueType { type_id, semantic })
            }
            types::Type::Void => Self::Void(VoidType { type_id, semantic }),
        }
    }

    pub fn type_id(&self) -> TypeId {
        match self {
            Type::Address(details) => details.type_id,
            Type::Array(details) => details.type_id,
            Type::Boolean(details) => details.type_id,
            Type::ByteArray(details) => details.type_id,
            Type::Bytes(details) => details.type_id,
            Type::Contract(details) => details.type_id,
            Type::Enum(details) => details.type_id,
            Type::FixedPointNumber(details) => details.type_id,
            Type::Function(details) => details.type_id,
            Type::Integer(details) => details.type_id,
            Type::Interface(details) => details.type_id,
            Type::Literal(details) => details.type_id,
            Type::Mapping(details) => details.type_id,
            Type::String(details) => details.type_id,
            Type::Struct(details) => details.type_id,
            Type::Tuple(details) => details.type_id,
            Type::UserDefinedValue(details) => details.type_id,
            Type::Void(details) => details.type_id,
        }
    }
}

impl AddressType {
    pub fn payable(&self) -> bool {
        let types::Type::Address { payable } = self.internal_type() else {
            unreachable!("invalid address type");
        };
        *payable
    }
}

impl ArrayType {
    pub fn element_type(&self) -> Type {
        let types::Type::Array { element_type, .. } = self.internal_type() else {
            unreachable!("invalid array type");
        };
        Type::create(*element_type, &self.semantic)
    }
    pub fn location(&self) -> DataLocation {
        let types::Type::Array { location, .. } = self.internal_type() else {
            unreachable!("invalid array type");
        };
        *location
    }
}

impl BooleanType {}

impl ByteArrayType {
    pub fn width(&self) -> u32 {
        let types::Type::ByteArray { width } = self.internal_type() else {
            unreachable!("invalid byte array type");
        };
        *width
    }
}

impl BytesType {
    pub fn location(&self) -> DataLocation {
        let types::Type::Bytes { location } = self.internal_type() else {
            unreachable!("invalid bytes type");
        };
        *location
    }
}

impl ContractType {
    pub fn definition(&self) -> Definition {
        let types::Type::Contract { definition_id } = self.internal_type() else {
            unreachable!("invalid contract type");
        };
        Definition::try_create(*definition_id, &self.semantic).expect("invalid contract definition")
    }
}

impl EnumType {
    pub fn definition(&self) -> Definition {
        let types::Type::Enum { definition_id } = self.internal_type() else {
            unreachable!("invalid enum type");
        };
        Definition::try_create(*definition_id, &self.semantic).expect("invalid enum definition")
    }
}

impl FixedPointNumberType {
    pub fn signed(&self) -> bool {
        let types::Type::FixedPointNumber { signed, .. } = self.internal_type() else {
            unreachable!("invalid fixed point number type");
        };
        *signed
    }
    pub fn bits(&self) -> u32 {
        let types::Type::FixedPointNumber { bits, .. } = self.internal_type() else {
            unreachable!("invalid fixed point number type");
        };
        *bits
    }
    pub fn precision_bits(&self) -> u32 {
        let types::Type::FixedPointNumber { precision_bits, .. } = self.internal_type() else {
            unreachable!("invalid fixed point number type");
        };
        *precision_bits
    }
}

impl FunctionType {
    pub fn associated_definition(&self) -> Option<Definition> {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        function_type.definition_id.map(|definition_id| {
            Definition::try_create(definition_id, &self.semantic)
                .expect("invalid function definition")
        })
    }

    pub fn implicit_receiver_type(&self) -> Option<Type> {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        function_type
            .implicit_receiver_type
            .map(|type_id| Type::create(type_id, &self.semantic))
    }

    pub fn parameter_types(&self) -> Vec<Type> {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        function_type
            .parameter_types
            .iter()
            .map(|type_id| Type::create(*type_id, &self.semantic))
            .collect()
    }

    pub fn return_type(&self) -> Type {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        Type::create(function_type.return_type, &self.semantic)
    }

    pub fn external(&self) -> bool {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        function_type.external
    }

    pub fn kind(&self) -> FunctionTypeKind {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        function_type.kind
    }
}

impl IntegerType {
    pub fn signed(&self) -> bool {
        let types::Type::Integer { signed, .. } = self.internal_type() else {
            unreachable!("invalid integer type");
        };
        *signed
    }
    pub fn bits(&self) -> u32 {
        let types::Type::Integer { bits, .. } = self.internal_type() else {
            unreachable!("invalid integer type");
        };
        *bits
    }
}

impl InterfaceType {
    pub fn definition(&self) -> Definition {
        let types::Type::Interface { definition_id } = self.internal_type() else {
            unreachable!("invalid interface type");
        };
        Definition::try_create(*definition_id, &self.semantic)
            .expect("invalid interface definition")
    }
}

impl LiteralType {}

impl MappingType {
    pub fn key_type(&self) -> Type {
        let types::Type::Mapping { key_type_id, .. } = self.internal_type() else {
            unreachable!("invalid mapping type");
        };
        Type::create(*key_type_id, &self.semantic)
    }
    pub fn value_type(&self) -> Type {
        let types::Type::Mapping { value_type_id, .. } = self.internal_type() else {
            unreachable!("invalid mapping type");
        };
        Type::create(*value_type_id, &self.semantic)
    }
}

impl StringType {
    pub fn location(&self) -> DataLocation {
        let types::Type::String { location } = self.internal_type() else {
            unreachable!("invalid string type");
        };
        *location
    }
}

impl StructType {
    pub fn definition(&self) -> Definition {
        let types::Type::Struct { definition_id, .. } = self.internal_type() else {
            unreachable!("invalid struct type");
        };
        Definition::try_create(*definition_id, &self.semantic).expect("invalid struct definition")
    }
    pub fn location(&self) -> DataLocation {
        let types::Type::Struct { location, .. } = self.internal_type() else {
            unreachable!("invalid struct type");
        };
        *location
    }
}

impl TupleType {
    pub fn types(&self) -> Vec<Type> {
        let types::Type::Tuple { types } = self.internal_type() else {
            unreachable!("invalid tuple type");
        };
        types
            .iter()
            .map(|type_id| Type::create(*type_id, &self.semantic))
            .collect()
    }
}

impl UserDefinedValueType {
    pub fn definition(&self) -> Definition {
        let types::Type::UserDefinedValue { definition_id } = self.internal_type() else {
            unreachable!("invalid user defined value type");
        };
        Definition::try_create(*definition_id, &self.semantic)
            .expect("invalid user defined value definition")
    }
}

impl VoidType {}
