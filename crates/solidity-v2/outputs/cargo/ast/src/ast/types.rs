use std::sync::Arc;

use paste::paste;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{
    self, DataLocation, FunctionTypeMutability, FunctionTypeVisibility, TypeId,
};
pub use slang_solidity_v2_semantic::types::{LiteralKind, Number};

use super::Definition;

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
    FixedSizeArray(FixedSizeArrayType),
    FixedPointNumber(FixedPointNumberType),
    Function(FunctionType),
    Integer(IntegerType),
    Interface(InterfaceType),
    Library(LibraryType),
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
                semantic: Arc<SemanticContext>,
            }

            impl [<$type Type>] {
                #[allow(unused)]
                fn internal_type(&self) -> &types::Type {
                    self.semantic.types().get_type_by_id(self.type_id)
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
define_type_variant!(FixedSizeArray);
define_type_variant!(FixedPointNumber);
define_type_variant!(Function);
define_type_variant!(Integer);
define_type_variant!(Interface);
define_type_variant!(Library);
define_type_variant!(Literal);
define_type_variant!(Mapping);
define_type_variant!(String);
define_type_variant!(Struct);
define_type_variant!(Tuple);
define_type_variant!(UserDefinedValue);
define_type_variant!(Void);

impl Type {
    pub fn try_create_for_node_id(
        node_id: NodeId,
        semantic: &Arc<SemanticContext>,
    ) -> Option<Self> {
        let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
        Some(Self::create(type_id, semantic))
    }

    pub fn create(type_id: TypeId, semantic: &Arc<SemanticContext>) -> Self {
        let type_ = semantic.types().get_type_by_id(type_id);
        let semantic = Arc::clone(semantic);
        match type_ {
            types::Type::Address(_) => Self::Address(AddressType { type_id, semantic }),
            types::Type::Array(_) => Self::Array(ArrayType { type_id, semantic }),
            types::Type::Boolean => Self::Boolean(BooleanType { type_id, semantic }),
            types::Type::ByteArray(_) => Self::ByteArray(ByteArrayType { type_id, semantic }),
            types::Type::Bytes(_) => Self::Bytes(BytesType { type_id, semantic }),
            types::Type::Contract(_) => Self::Contract(ContractType { type_id, semantic }),
            types::Type::Enum(_) => Self::Enum(EnumType { type_id, semantic }),
            types::Type::FixedSizeArray(_) => {
                Self::FixedSizeArray(FixedSizeArrayType { type_id, semantic })
            }
            types::Type::FixedPointNumber(_) => {
                Self::FixedPointNumber(FixedPointNumberType { type_id, semantic })
            }
            types::Type::Function(_) => Self::Function(FunctionType { type_id, semantic }),
            types::Type::Integer(_) => Self::Integer(IntegerType { type_id, semantic }),
            types::Type::Interface(_) => Self::Interface(InterfaceType { type_id, semantic }),
            types::Type::Library(_) => Self::Library(LibraryType { type_id, semantic }),
            types::Type::Literal(_) => Self::Literal(LiteralType { type_id, semantic }),
            types::Type::Mapping(_) => Self::Mapping(MappingType { type_id, semantic }),
            types::Type::String(_) => Self::String(StringType { type_id, semantic }),
            types::Type::Struct(_) => Self::Struct(StructType { type_id, semantic }),
            types::Type::Tuple(_) => Self::Tuple(TupleType { type_id, semantic }),
            types::Type::UserDefinedValue(_) => {
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
            Type::FixedSizeArray(details) => details.type_id,
            Type::FixedPointNumber(details) => details.type_id,
            Type::Function(details) => details.type_id,
            Type::Integer(details) => details.type_id,
            Type::Interface(details) => details.type_id,
            Type::Library(details) => details.type_id,
            Type::Literal(details) => details.type_id,
            Type::Mapping(details) => details.type_id,
            Type::String(details) => details.type_id,
            Type::Struct(details) => details.type_id,
            Type::Tuple(details) => details.type_id,
            Type::UserDefinedValue(details) => details.type_id,
            Type::Void(details) => details.type_id,
        }
    }

    /// Returns the data location of this type when it has one.
    ///
    /// Container types (`Array`, `Bytes`, `FixedSizeArray`, `String`, `Struct`)
    /// carry their data location explicitly. `Mapping` is always `Storage`.
    /// All other types are value types with no associated location and return
    /// `None`.
    pub fn data_location(&self) -> Option<DataLocation> {
        match self {
            Type::Array(details) => Some(details.location()),
            Type::Bytes(details) => Some(details.location()),
            Type::FixedSizeArray(details) => Some(details.location()),
            Type::String(details) => Some(details.location()),
            Type::Struct(details) => Some(details.location()),
            Type::Mapping(_) => Some(DataLocation::Storage),
            _ => None,
        }
    }

    /// Returns `true` if this type is a Solidity reference type
    /// (`array`, `fixed-size array`, `bytes`, `string`, `mapping`, `struct`).
    pub fn is_reference_type(&self) -> bool {
        matches!(
            self,
            Type::Array(_)
                | Type::FixedSizeArray(_)
                | Type::Bytes(_)
                | Type::String(_)
                | Type::Mapping(_)
                | Type::Struct(_)
        )
    }
}

impl AddressType {
    pub fn payable(&self) -> bool {
        let types::Type::Address(details) = self.internal_type() else {
            unreachable!("invalid address type");
        };
        details.payable
    }
}

impl ArrayType {
    pub fn element_type(&self) -> Type {
        let types::Type::Array(details) = self.internal_type() else {
            unreachable!("invalid array type");
        };
        Type::create(details.element_type, &self.semantic)
    }
    pub fn location(&self) -> DataLocation {
        let types::Type::Array(details) = self.internal_type() else {
            unreachable!("invalid array type");
        };
        details.location
    }
}

impl BooleanType {}

impl ByteArrayType {
    pub fn width(&self) -> u32 {
        let types::Type::ByteArray(details) = self.internal_type() else {
            unreachable!("invalid byte array type");
        };
        details.width
    }
}

impl BytesType {
    pub fn location(&self) -> DataLocation {
        let types::Type::Bytes(details) = self.internal_type() else {
            unreachable!("invalid bytes type");
        };
        details.location
    }
}

impl ContractType {
    pub fn definition(&self) -> Definition {
        let types::Type::Contract(details) = self.internal_type() else {
            unreachable!("invalid contract type");
        };
        Definition::try_create(details.definition_id, &self.semantic)
            .expect("invalid contract definition")
    }
}

impl EnumType {
    pub fn definition(&self) -> Definition {
        let types::Type::Enum(details) = self.internal_type() else {
            unreachable!("invalid enum type");
        };
        Definition::try_create(details.definition_id, &self.semantic)
            .expect("invalid enum definition")
    }
}

impl FixedSizeArrayType {
    pub fn element_type(&self) -> Type {
        let types::Type::FixedSizeArray(details) = self.internal_type() else {
            unreachable!("invalid fixed-size array type");
        };
        Type::create(details.element_type, &self.semantic)
    }
    pub fn location(&self) -> DataLocation {
        let types::Type::FixedSizeArray(details) = self.internal_type() else {
            unreachable!("invalid fixed-size array type");
        };
        details.location
    }
    pub fn size(&self) -> usize {
        let types::Type::FixedSizeArray(details) = self.internal_type() else {
            unreachable!("invalid fixed-size array type");
        };
        details.size
    }
}

impl FixedPointNumberType {
    pub fn signed(&self) -> bool {
        let types::Type::FixedPointNumber(details) = self.internal_type() else {
            unreachable!("invalid fixed point number type");
        };
        details.signed
    }
    pub fn bits(&self) -> u32 {
        let types::Type::FixedPointNumber(details) = self.internal_type() else {
            unreachable!("invalid fixed point number type");
        };
        details.bits
    }
    pub fn precision_bits(&self) -> u32 {
        let types::Type::FixedPointNumber(details) = self.internal_type() else {
            unreachable!("invalid fixed point number type");
        };
        details.precision_bits
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

    pub fn is_externally_visible(&self) -> bool {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        function_type.is_externally_visible()
    }

    pub fn visibility(&self) -> FunctionTypeVisibility {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        function_type.visibility
    }

    pub fn mutability(&self) -> FunctionTypeMutability {
        let types::Type::Function(function_type) = self.internal_type() else {
            unreachable!("invalid function type");
        };
        function_type.mutability
    }
}

impl IntegerType {
    pub fn signed(&self) -> bool {
        let types::Type::Integer(details) = self.internal_type() else {
            unreachable!("invalid integer type");
        };
        details.signed
    }
    pub fn bits(&self) -> u32 {
        let types::Type::Integer(details) = self.internal_type() else {
            unreachable!("invalid integer type");
        };
        details.bits
    }
}

impl InterfaceType {
    pub fn definition(&self) -> Definition {
        let types::Type::Interface(details) = self.internal_type() else {
            unreachable!("invalid interface type");
        };
        Definition::try_create(details.definition_id, &self.semantic)
            .expect("invalid interface definition")
    }
}

impl LibraryType {
    pub fn definition(&self) -> Definition {
        let types::Type::Library(details) = self.internal_type() else {
            unreachable!("invalid library type");
        };
        Definition::try_create(details.definition_id, &self.semantic)
            .expect("invalid library definition")
    }
}

impl LiteralType {
    /// Returns the narrowed kind of this literal type.
    pub fn kind(&self) -> LiteralKind {
        let types::Type::Literal(kind) = self.internal_type() else {
            unreachable!("LiteralType wraps a literal variant by construction")
        };
        kind.clone()
    }
}

impl MappingType {
    pub fn key_type(&self) -> Type {
        let types::Type::Mapping(details) = self.internal_type() else {
            unreachable!("invalid mapping type");
        };
        Type::create(details.key_type_id, &self.semantic)
    }
    pub fn value_type(&self) -> Type {
        let types::Type::Mapping(details) = self.internal_type() else {
            unreachable!("invalid mapping type");
        };
        Type::create(details.value_type_id, &self.semantic)
    }
}

impl StringType {
    pub fn location(&self) -> DataLocation {
        let types::Type::String(details) = self.internal_type() else {
            unreachable!("invalid string type");
        };
        details.location
    }
}

impl StructType {
    pub fn definition(&self) -> Definition {
        let types::Type::Struct(details) = self.internal_type() else {
            unreachable!("invalid struct type");
        };
        Definition::try_create(details.definition_id, &self.semantic)
            .expect("invalid struct definition")
    }
    pub fn location(&self) -> DataLocation {
        let types::Type::Struct(details) = self.internal_type() else {
            unreachable!("invalid struct type");
        };
        details.location
    }
}

impl TupleType {
    pub fn types(&self) -> Vec<Type> {
        let types::Type::Tuple(details) = self.internal_type() else {
            unreachable!("invalid tuple type");
        };
        details
            .types
            .iter()
            .map(|type_id| Type::create(*type_id, &self.semantic))
            .collect()
    }
}

impl UserDefinedValueType {
    pub fn definition(&self) -> Definition {
        let types::Type::UserDefinedValue(details) = self.internal_type() else {
            unreachable!("invalid user defined value type");
        };
        Definition::try_create(details.definition_id, &self.semantic)
            .expect("invalid user defined value definition")
    }

    /// Returns the underlying elementary type that this UDVT wraps, or `None`
    /// if the binder did not resolve a target type (e.g., the declaration is
    /// ill-formed or references an unsupported elementary type).
    pub fn target_type(&self) -> Option<Type> {
        let types::Type::UserDefinedValue(details) = self.internal_type() else {
            unreachable!("invalid user defined value type");
        };
        let binder_definition = self
            .semantic
            .binder()
            .find_definition_by_id(details.definition_id)?;
        let binder::Definition::UserDefinedValueType(udvt_definition) = binder_definition else {
            unreachable!("UDVT type id should map to UDVT definition");
        };
        Some(Type::create(
            udvt_definition.target_type_id?,
            &self.semantic,
        ))
    }
}

impl VoidType {}
