use literals::numbers;
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use ruint::aliases::U160;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

pub mod literals;
mod parsing;
mod registry;

pub use literals::numbers::Number;
pub use registry::TypeRegistry;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(usize);

// __SLANG_TYPE_TYPES__ keep in sync with AST types
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Address(AddressType),
    Array(ArrayType),
    Boolean,
    ByteArray(ByteArrayType),
    Bytes(BytesType),
    Contract(ContractType),
    Enum(EnumType),
    FixedPointNumber(FixedPointNumberType),
    FixedSizeArray(FixedSizeArrayType),
    Function(FunctionType),
    Integer(IntegerType),
    Interface(InterfaceType),
    Library(LibraryType),
    Literal(LiteralKind),
    Mapping(MappingType),
    String(StringType),
    Struct(StructType),
    Tuple(TupleType),
    UserDefinedValue(UserDefinedValueType),
    Void,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AddressType {
    pub is_payable: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ArrayType {
    pub element_type: TypeId,
    pub location: DataLocation,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ByteArrayType {
    pub width: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BytesType {
    pub location: DataLocation,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ContractType {
    pub definition_id: NodeId,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EnumType {
    pub definition_id: NodeId,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FixedPointNumberType {
    pub is_signed: bool,
    pub bits: u32,
    pub decimal_places: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FixedSizeArrayType {
    pub element_type: TypeId,
    pub location: DataLocation,
    // TODO: this should be u256, although in practice usize should suffice
    pub size: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntegerType {
    pub is_signed: bool,
    pub bits: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InterfaceType {
    pub definition_id: NodeId,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LibraryType {
    pub definition_id: NodeId,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MappingType {
    pub key_type_id: TypeId,
    pub value_type_id: TypeId,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StringType {
    pub location: DataLocation,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StructType {
    pub definition_id: NodeId,
    pub location: DataLocation,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TupleType {
    pub types: Vec<TypeId>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UserDefinedValueType {
    pub definition_id: NodeId,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LiteralKind {
    Integer {
        value: BigInt,
    },
    /// A hex-source integer literal. Carries the parsed value plus the
    /// source-text byte width (number of hex digits / 2, rounded up). The
    /// width is what determines convertibility with `bytesN` and is preserved
    /// distinctly from the value because `0x0012` and `0x12` share value `18`
    /// but convert to `bytes2` and `bytes1` respectively. The value is
    /// `BigUint` since hex literals are always non-negative.
    HexInteger {
        value: BigUint,
        bytes: u32,
    },
    Rational {
        value: BigRational,
    },
    HexString {
        bytes: usize,
    },
    String {
        bytes: usize,
    },
    Address {
        value: U160,
    },
}

impl LiteralKind {
    /// Returns the non-literal `Type` this literal can flow into (e.g., the
    /// smallest integer type that fits an integer literal, the smallest
    /// fixed-point type that fits a rational literal, or `string memory`
    /// for a string literal). Returns `None` when the literal cannot be
    /// represented (eg. integer would require more than 256 bits).
    // __SLANG_MOBILE_TYPE__ keep in sync with `ast::LiteralType::mobile_type`
    pub fn mobile_type(&self) -> Option<Type> {
        match self {
            LiteralKind::Integer { value } => numbers::smallest_integer_type_to_fit(value),
            LiteralKind::HexInteger { value, .. } => {
                numbers::smallest_integer_type_to_fit(&BigInt::from(value.clone()))
            }
            LiteralKind::Rational { value } => numbers::smallest_fixed_point_type_to_fit(value),
            LiteralKind::HexString { .. } | LiteralKind::String { .. } => {
                Some(Type::String(StringType {
                    location: DataLocation::Memory,
                }))
            }
            LiteralKind::Address { .. } => Some(Type::Address(AddressType { is_payable: false })),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionType {
    pub definition_id: Option<NodeId>, // this may point to a FunctionDefinition
    pub parameter_types: Vec<TypeId>,
    // Multiple return values are modeled by returning a tuple
    pub return_type: TypeId,
    pub visibility: FunctionTypeVisibility,
    pub mutability: FunctionTypeMutability,
    /// The type of the implicit receiver in contract or interface
    /// methods (ie. the type of `this`)
    pub implicit_receiver_type: Option<TypeId>,
    /// Whether this function has been partially applied.
    /// This happens when its first argument is bound, eg. `a.foo`
    /// where `foo` is from a `using` directive on the type of `a`,
    /// or when call options are pre-applied (eg. `foo{value: 3}`).
    pub partially_applied: bool,
}

impl FunctionType {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.visibility,
            FunctionTypeVisibility::External | FunctionTypeVisibility::Public
        )
    }
}

// Mirrors `ir::FunctionVisibility`
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FunctionTypeVisibility {
    Public,
    Private,
    Internal,
    External,
}

impl From<&ir::FunctionVisibility> for FunctionTypeVisibility {
    fn from(value: &ir::FunctionVisibility) -> Self {
        match value {
            ir::FunctionVisibility::Public => Self::Public,
            ir::FunctionVisibility::Private => Self::Private,
            ir::FunctionVisibility::Internal => Self::Internal,
            ir::FunctionVisibility::External => Self::External,
        }
    }
}

// Mirrors `ir::FunctionMutability`
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FunctionTypeMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl From<&ir::FunctionMutability> for FunctionTypeMutability {
    fn from(value: &ir::FunctionMutability) -> Self {
        match value {
            ir::FunctionMutability::Pure => Self::Pure,
            ir::FunctionMutability::View => Self::View,
            ir::FunctionMutability::NonPayable => Self::NonPayable,
            ir::FunctionMutability::Payable => Self::Payable,
        }
    }
}

pub(crate) trait ImplicitlyConvertible<T> {
    fn implicitly_convertible_to(&self, target: T) -> bool;
}

impl ImplicitlyConvertible<FunctionTypeVisibility> for FunctionTypeVisibility {
    fn implicitly_convertible_to(&self, target: Self) -> bool {
        matches!(
            (self, target),
            // public can convert to public, external or internal (if called
            // internally)
            (
                FunctionTypeVisibility::Public,
                FunctionTypeVisibility::Public
                    | FunctionTypeVisibility::Internal
                    | FunctionTypeVisibility::External,
            )
                // private converts to private or internal
                | (
                    FunctionTypeVisibility::Private,
                    FunctionTypeVisibility::Private | FunctionTypeVisibility::Internal,
                )
                // internal and external only convert to themselves
                | (FunctionTypeVisibility::Internal, FunctionTypeVisibility::Internal)
                | (FunctionTypeVisibility::External, FunctionTypeVisibility::External)
        )
    }
}

impl ImplicitlyConvertible<FunctionTypeMutability> for FunctionTypeMutability {
    fn implicitly_convertible_to(&self, target: Self) -> bool {
        matches!(
            (self, target),
            // pure converts to view or non-payable
            (
                FunctionTypeMutability::Pure,
                FunctionTypeMutability::Pure | FunctionTypeMutability::View | FunctionTypeMutability::NonPayable,
            )
                // view converts to non-payable
                | (
                FunctionTypeMutability::View,
                FunctionTypeMutability::View | FunctionTypeMutability::NonPayable
            )
                // non-payable does not implicitly convert to any other kind
                | (FunctionTypeMutability::NonPayable, FunctionTypeMutability::NonPayable)
                // payable converts to non-payable
                | (
                    FunctionTypeMutability::Payable,
                    FunctionTypeMutability::Payable | FunctionTypeMutability::NonPayable,
                )
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DataLocation {
    Memory,
    Storage,
    Calldata,

    // This applies to struct fields of reference types, where the data location is the struct's.
    Inherited,
}

impl DataLocation {
    // When calling external functions, it is irrelevant if the data location of
    // the parameters is ``calldata`` or ``memory``, the encoding of the data
    // does not change. Because of that, changing the data location when
    // overriding external functions is allowed.
    // See https://github.com/argotorg/solidity/pull/12850
    pub fn overrides_in_external_function(&self, target: Self) -> bool {
        *self == target
            || (*self == Self::Memory && target == Self::Calldata)
            || (*self == Self::Calldata && target == Self::Memory)
    }
}

impl From<&ir::StorageLocation> for DataLocation {
    fn from(value: &ir::StorageLocation) -> Self {
        match value {
            ir::StorageLocation::MemoryKeyword(_) => Self::Memory,
            ir::StorageLocation::StorageKeyword(_) => Self::Storage,
            ir::StorageLocation::CallDataKeyword(_) => Self::Calldata,
        }
    }
}

impl ImplicitlyConvertible<DataLocation> for DataLocation {
    fn implicitly_convertible_to(&self, target: Self) -> bool {
        match (self, target) {
            (from, to) if *from == to => true,
            (DataLocation::Storage | DataLocation::Calldata, DataLocation::Memory) => true,
            _ => false,
        }
    }
}

impl Type {
    pub fn data_location(&self) -> Option<DataLocation> {
        match self {
            Self::Array(ArrayType { location, .. })
            | Self::Bytes(BytesType { location })
            | Self::FixedSizeArray(FixedSizeArrayType { location, .. })
            | Self::String(StringType { location })
            | Self::Struct(StructType { location, .. }) => Some(*location),
            Self::Mapping(_) => Some(DataLocation::Storage),
            _ => None,
        }
    }

    pub fn is_inherited_location(&self) -> bool {
        self.data_location()
            .is_some_and(|location| location == DataLocation::Inherited)
    }

    /// This function determines whether a type can be returned directly from
    /// a getter function without any extra parameter.
    ///
    /// Arrays and mapping can't
    pub fn can_return_from_getter_directly(&self) -> bool {
        match self {
            Type::Address(_)
            | Type::Boolean
            | Type::ByteArray(_)
            | Type::Bytes(_)
            | Type::Contract(_)
            | Type::Enum(_)
            | Type::FixedPointNumber(_)
            | Type::Integer(_)
            | Type::Interface(_)
            | Type::String(_)
            | Type::Struct(_)
            | Type::UserDefinedValue(_) => true,
            Type::Function(FunctionType { visibility, .. }) => {
                // Function types can only be returned if they're external
                visibility == &FunctionTypeVisibility::External
            }

            // Can be returned, just not inside a struct
            Type::Array(_)
            | Type::FixedSizeArray(_)
            | Type::Mapping(_)
            // Actually can't return from a getter
            // TODO(validation) SDR[1394]: Not sure if this is the best place for this check,
            // but it's related.
            | Type::Literal(_)
            | Type::Library(_)
            | Type::Tuple(_)
            | Type::Void => false,
        }
    }

    pub fn is_literal_number(&self) -> bool {
        matches!(
            self,
            Type::Literal(
                LiteralKind::Integer { .. }
                    | LiteralKind::HexInteger { .. }
                    | LiteralKind::Rational { .. }
            )
        )
    }

    /// For literal numeric types, return their value.
    pub fn number_value(&self) -> Option<Number> {
        match self {
            Type::Literal(kind) => Number::from_literal_kind(kind),
            _ => None,
        }
    }

    pub fn is_contract_or_interface(&self) -> bool {
        matches!(self, Type::Contract(_) | Type::Interface(_))
    }

    pub(crate) fn get_definition_id(&self) -> Option<NodeId> {
        match self {
            Type::Contract(ContractType { definition_id })
            | Type::Enum(EnumType { definition_id })
            | Type::Interface(InterfaceType { definition_id })
            | Type::Library(LibraryType { definition_id })
            | Type::Struct(StructType { definition_id, .. })
            | Type::UserDefinedValue(UserDefinedValueType { definition_id }) => {
                Some(*definition_id)
            }
            _ => None,
        }
    }

    /// Whether this type can appear as the element type of an array literal.
    ///
    /// TODO: This probably has a better way to resolve it, looking at the storage location
    pub(crate) fn can_be_array_element(&self) -> bool {
        !matches!(self, Type::Mapping(_) | Type::Tuple(_) | Type::Void)
    }
}
