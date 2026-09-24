use std::sync::Arc;

use paste::paste;
use ruint::aliases::U256;
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
    ArraySlice(ArraySliceType),
    Boolean(BooleanType),
    ByteArray(ByteArrayType),
    Bytes(BytesType),
    Contract(ContractType),
    Enum(EnumType),
    Error(ErrorType),
    Event(EventType),
    FixedSizeArray(FixedSizeArrayType),
    FixedPointNumber(FixedPointNumberType),
    Function(FunctionType),
    Integer(IntegerType),
    Interface(InterfaceType),
    Library(LibraryType),
    Literal(LiteralType),
    Mapping(MappingType),
    MetaType(MetaType),
    String(StringType),
    Struct(StructType),
    Tuple(TupleType),
    UserDefinedValue(UserDefinedValueType),
    UserMetaType(UserMetaType),
    Void(VoidType),
}

macro_rules! define_value_type_variant {
    ($type:ident) => {
        paste! {
            #[derive(Clone)]
            pub struct [<$type Type>] {
                inner: types::[<$type Type>],
            }
        }
    };
}

macro_rules! define_type_variant {
    ($type:ident) => {
        paste! {
            #[derive(Clone)]
            pub struct [<$type Type>] {
                inner: types::[<$type Type>],
                semantic: Arc<SemanticContext>,
            }
        }
    };
}

// Variants whose accessors only read fields from the inner struct.
define_value_type_variant!(Address);
define_value_type_variant!(ByteArray);
define_value_type_variant!(Bytes);
define_value_type_variant!(FixedPointNumber);
define_value_type_variant!(Integer);
define_value_type_variant!(String);

// Variants that need the semantic context to resolve nested types or
// definitions.
define_type_variant!(Array);
define_type_variant!(ArraySlice);
define_type_variant!(Contract);
define_type_variant!(Enum);
define_type_variant!(Error);
define_type_variant!(Event);
define_type_variant!(FixedSizeArray);
define_type_variant!(Function);
define_type_variant!(Interface);
define_type_variant!(Library);
define_type_variant!(Mapping);
define_type_variant!(Meta);
define_type_variant!(Struct);
define_type_variant!(Tuple);
define_type_variant!(UserDefinedValue);
define_type_variant!(UserMeta);

#[derive(Clone)]
pub struct LiteralType {
    type_id: TypeId,
    semantic: Arc<SemanticContext>,
}

#[derive(Clone)]
pub struct BooleanType;

#[derive(Clone)]
pub struct VoidType;

impl Type {
    pub fn try_create_for_node_id(
        node_id: NodeId,
        semantic: &Arc<SemanticContext>,
    ) -> Option<Self> {
        let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
        Some(Self::create(type_id, semantic))
    }

    pub(crate) fn create(type_id: TypeId, semantic: &Arc<SemanticContext>) -> Self {
        // Matched by reference, so that a literal is not copied: it may own a
        // large string value, and is read back from the registry instead.
        let type_ = semantic.types().get_type_by_id(type_id);
        let semantic = Arc::clone(semantic);
        match type_ {
            types::Type::Address(inner) => Self::Address(AddressType {
                inner: inner.clone(),
            }),
            types::Type::Array(inner) => Self::Array(ArrayType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::ArraySlice(inner) => Self::ArraySlice(ArraySliceType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Boolean => Self::Boolean(BooleanType),
            types::Type::ByteArray(inner) => Self::ByteArray(ByteArrayType {
                inner: inner.clone(),
            }),
            types::Type::Bytes(inner) => Self::Bytes(BytesType {
                inner: inner.clone(),
            }),
            types::Type::Contract(inner) => Self::Contract(ContractType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Enum(inner) => Self::Enum(EnumType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Error(inner) => Self::Error(ErrorType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Event(inner) => Self::Event(EventType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::FixedSizeArray(inner) => Self::FixedSizeArray(FixedSizeArrayType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::FixedPointNumber(inner) => Self::FixedPointNumber(FixedPointNumberType {
                inner: inner.clone(),
            }),
            types::Type::Function(inner) => Self::Function(FunctionType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Integer(inner) => Self::Integer(IntegerType {
                inner: inner.clone(),
            }),
            types::Type::Interface(inner) => Self::Interface(InterfaceType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Library(inner) => Self::Library(LibraryType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Literal(_) => Self::Literal(LiteralType { type_id, semantic }),
            types::Type::Mapping(inner) => Self::Mapping(MappingType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::MetaType(inner) => Self::MetaType(MetaType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::String(inner) => Self::String(StringType {
                inner: inner.clone(),
            }),
            types::Type::Struct(inner) => Self::Struct(StructType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Tuple(inner) => Self::Tuple(TupleType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::UserDefinedValue(inner) => Self::UserDefinedValue(UserDefinedValueType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::UserMetaType(inner) => Self::UserMetaType(UserMetaType {
                inner: inner.clone(),
                semantic,
            }),
            types::Type::Void => Self::Void(VoidType),
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
            // A slice inherits its underlying array's location (always calldata).
            Type::ArraySlice(details) => details.array_type().data_location(),
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
                | Type::ArraySlice(_)
                | Type::FixedSizeArray(_)
                | Type::Bytes(_)
                | Type::String(_)
                | Type::Mapping(_)
                | Type::Struct(_)
        )
    }
}

impl AddressType {
    pub fn is_payable(&self) -> bool {
        self.inner.is_payable
    }
}

impl ArrayType {
    pub fn element_type(&self) -> Type {
        Type::create(self.inner.element_type, &self.semantic)
    }
    pub fn location(&self) -> DataLocation {
        self.inner.location
    }
}

impl ArraySliceType {
    /// The array this is a slice of (an `Array`, `Bytes`, or `String`, always in
    /// `calldata`).
    pub fn array_type(&self) -> Type {
        Type::create(self.inner.array_type_id, &self.semantic)
    }
}

impl BooleanType {}

impl ByteArrayType {
    pub fn width(&self) -> u32 {
        self.inner.width
    }
}

impl BytesType {
    pub fn location(&self) -> DataLocation {
        self.inner.location
    }
}

impl ContractType {
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid contract definition")
    }
}

impl EnumType {
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid enum definition")
    }
}

impl ErrorType {
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid error definition")
    }
}

impl EventType {
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid event definition")
    }
}

impl FixedSizeArrayType {
    pub fn element_type(&self) -> Type {
        Type::create(self.inner.element_type, &self.semantic)
    }
    pub fn location(&self) -> DataLocation {
        self.inner.location
    }
    pub fn size(&self) -> U256 {
        self.inner.size
    }
}

impl FixedPointNumberType {
    pub fn is_signed(&self) -> bool {
        self.inner.is_signed
    }
    pub fn bits(&self) -> u32 {
        self.inner.bits
    }
    pub fn decimal_places(&self) -> u32 {
        self.inner.decimal_places
    }
}

impl FunctionType {
    pub fn associated_definition(&self) -> Option<Definition> {
        self.inner.definition_id.map(|definition_id| {
            Definition::try_create(definition_id, &self.semantic)
                .expect("invalid function definition")
        })
    }

    pub fn implicit_receiver_type(&self) -> Option<Type> {
        self.inner
            .implicit_receiver_type
            .map(|type_id| Type::create(type_id, &self.semantic))
    }

    pub fn parameter_types(&self) -> Vec<Type> {
        self.inner
            .parameter_types
            .iter()
            .map(|type_id| Type::create(*type_id, &self.semantic))
            .collect()
    }

    pub fn return_type(&self) -> Type {
        Type::create(self.inner.return_type, &self.semantic)
    }

    pub fn is_externally_visible(&self) -> bool {
        self.inner.is_externally_visible()
    }

    pub fn visibility(&self) -> FunctionTypeVisibility {
        self.inner.visibility
    }

    pub fn mutability(&self) -> FunctionTypeMutability {
        self.inner.mutability
    }
}

impl IntegerType {
    pub fn is_signed(&self) -> bool {
        self.inner.is_signed
    }
    pub fn bits(&self) -> u32 {
        self.inner.bits
    }
}

impl InterfaceType {
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid interface definition")
    }
}

impl LibraryType {
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid library definition")
    }
}

impl LiteralType {
    /// Returns the narrowed kind of this literal type.
    pub fn kind(&self) -> &LiteralKind {
        let types::Type::Literal(kind) = self.semantic.types().get_type_by_id(self.type_id) else {
            unreachable!("a literal type wrapper refers to a literal type");
        };
        kind
    }

    /// Returns the non-literal mobile type this literal can flow into (e.g.,
    /// the smallest integer type that fits an integer literal, the smallest
    /// fixed-point type that fits a rational literal, or `string memory`
    /// for a string literal). Returns `None` when the literal cannot be
    /// represented (eg. integer would require more than 256 bits).
    // __SLANG_MOBILE_TYPE__ keep in sync with `LiteralKind::mobile_type`
    pub fn mobile_type(&self) -> Option<Type> {
        self.kind().mobile_type().and_then(|mobile| match mobile {
            types::Type::Integer(inner) => Some(Type::Integer(IntegerType { inner })),
            types::Type::FixedPointNumber(inner) => {
                Some(Type::FixedPointNumber(FixedPointNumberType { inner }))
            }
            types::Type::String(inner) => Some(Type::String(StringType { inner })),
            types::Type::Address(inner) => Some(Type::Address(AddressType { inner })),
            _ => None,
        })
    }
}

impl MetaType {
    /// Returns the type this is a meta-type of, eg. `uint` for the meta-type of
    /// the `uint` in `uint(x)`.
    pub fn meta_type(&self) -> Type {
        Type::create(self.inner.type_id, &self.semantic)
    }
}

impl UserMetaType {
    /// Returns the user-defined type definition this meta-type refers to, eg.
    /// the contract, struct or enum named by the expression.
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid user meta-type definition")
    }
}

impl MappingType {
    pub fn key_type(&self) -> Type {
        Type::create(self.inner.key_type_id, &self.semantic)
    }
    pub fn value_type(&self) -> Type {
        Type::create(self.inner.value_type_id, &self.semantic)
    }
}

impl StringType {
    pub fn location(&self) -> DataLocation {
        self.inner.location
    }
}

impl StructType {
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid struct definition")
    }
    pub fn location(&self) -> DataLocation {
        self.inner.location
    }
}

impl TupleType {
    pub fn types(&self) -> Vec<Type> {
        self.inner
            .types
            .iter()
            .map(|type_id| Type::create(*type_id, &self.semantic))
            .collect()
    }
}

impl UserDefinedValueType {
    pub fn definition(&self) -> Definition {
        Definition::try_create(self.inner.definition_id, &self.semantic)
            .expect("invalid user defined value definition")
    }

    /// Returns the underlying elementary type that this UDVT wraps, or `None`
    /// if the binder did not resolve a target type (e.g., the declaration is
    /// ill-formed or references an unsupported elementary type).
    pub fn target_type(&self) -> Option<Type> {
        let binder_definition = self
            .semantic
            .binder()
            .find_definition_by_id(self.inner.definition_id)?;
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
