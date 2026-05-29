use literals::numbers;
use num_bigint::BigInt;
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
    Address {
        payable: bool,
    },
    Array {
        element_type: TypeId,
        location: DataLocation,
    },
    Boolean,
    ByteArray {
        width: u32,
    },
    Bytes {
        location: DataLocation,
    },
    Contract {
        definition_id: NodeId,
    },
    Enum {
        definition_id: NodeId,
    },
    FixedPointNumber {
        signed: bool,
        bits: u32,
        precision_bits: u32,
    },
    FixedSizeArray {
        element_type: TypeId,
        location: DataLocation,
        // TODO: this should be u256, although in practice usize should suffice
        size: usize,
    },
    Function(FunctionType),
    Integer {
        signed: bool,
        bits: u32,
    },
    Interface {
        definition_id: NodeId,
    },
    Literal(LiteralKind),
    Mapping {
        key_type_id: TypeId,
        value_type_id: TypeId,
    },
    String {
        location: DataLocation,
    },
    Struct {
        definition_id: NodeId,
        location: DataLocation,
    },
    Tuple {
        types: Vec<TypeId>,
    },
    UserDefinedValue {
        definition_id: NodeId,
    },
    Void,
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
    /// but convert to `bytes2` and `bytes1` respectively.
    HexInteger {
        value: BigInt,
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
    /// Returns the non-literal `Type` this literal flows into when its source
    /// position needs a concrete EVM type.
    pub(crate) fn mobile_type(&self) -> Option<Type> {
        match self {
            LiteralKind::Integer { value } | LiteralKind::HexInteger { value, .. } => {
                numbers::smallest_integer_type_to_fit(value)
            }
            // TODO: not supported yet, but narrow the rational type to the
            // smallest fixed/ufixed available (eg. 1.2 -> ufixed8x1).
            LiteralKind::Rational { .. } => None,
            LiteralKind::HexString { .. } | LiteralKind::String { .. } => Some(Type::String {
                location: DataLocation::Memory,
            }),
            LiteralKind::Address { .. } => Some(Type::Address { payable: false }),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionType {
    pub definition_id: Option<NodeId>, // this may point to a FunctionDefinition
    pub implicit_receiver_type: Option<TypeId>,
    pub parameter_types: Vec<TypeId>,
    pub return_type: TypeId,
    pub visibility: FunctionTypeVisibility,
    pub mutability: FunctionTypeMutability,
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
            Self::Array { location, .. }
            | Self::Bytes { location }
            | Self::FixedSizeArray { location, .. }
            | Self::String { location }
            | Self::Struct { location, .. } => Some(*location),
            Self::Mapping { .. } => Some(DataLocation::Storage),
            _ => None,
        }
    }

    pub fn is_inherited_location(&self) -> bool {
        self.data_location()
            .is_some_and(|location| location == DataLocation::Inherited)
    }

    pub fn can_return_from_getter(&self) -> bool {
        match self {
            Type::Address { .. }
            | Type::Boolean
            | Type::ByteArray { .. }
            | Type::Bytes { .. }
            | Type::Contract { .. }
            | Type::Enum { .. }
            | Type::FixedPointNumber { .. }
            | Type::Integer { .. }
            | Type::Interface { .. }
            | Type::String { .. }
            | Type::UserDefinedValue { .. } => true,

            Type::Array { .. }
            | Type::FixedSizeArray { .. }
            | Type::Function(_)
            | Type::Mapping { .. }
            | Type::Literal(_)
            | Type::Struct { .. }
            | Type::Tuple { .. }
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
        matches!(self, Type::Contract { .. } | Type::Interface { .. })
    }

    pub(crate) fn get_definition_id(&self) -> Option<NodeId> {
        match self {
            Type::Contract { definition_id }
            | Type::Enum { definition_id }
            | Type::Interface { definition_id }
            | Type::Struct { definition_id, .. }
            | Type::UserDefinedValue { definition_id } => Some(*definition_id),
            _ => None,
        }
    }

    /// Whether this type can appear as the element type of an array literal.
    ///
    /// TODO: This probably has a better way to resolve it, looking at the storage location
    pub(crate) fn can_be_array_element(&self) -> bool {
        !matches!(self, Type::Mapping { .. } | Type::Tuple { .. } | Type::Void)
    }
}
