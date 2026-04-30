use num_bigint::BigInt;
use num_rational::BigRational;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir::{self, FunctionMutability, FunctionVisibility};

mod constants;
mod parsing;
mod registry;

pub(crate) use constants::ConstantValue;
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
    Integer(BigInt),
    /// A hex-source integer literal. Carries the parsed value plus the
    /// source-text byte width (number of hex digits / 2, rounded up). The
    /// width is what determines convertability with `bytesN` and is preserved
    /// distinctly from the value because `0x0012` and `0x12` share value `18`
    /// but convert to `bytes2` and `bytes1` respectively.
    HexInteger {
        value: BigInt,
        bytes: u32,
    },
    Rational(BigRational),
    HexString {
        bytes: u32,
    },
    String {
        bytes: u32,
    },
    Address,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionType {
    pub definition_id: Option<NodeId>, // this may point to a FunctionDefinition
    pub implicit_receiver_type: Option<TypeId>,
    pub parameter_types: Vec<TypeId>,
    pub return_type: TypeId,
    pub visibility: FunctionVisibility,
    pub mutability: FunctionMutability,
}

impl FunctionType {
    pub fn is_externally_visible(&self) -> bool {
        matches!(
            self.visibility,
            FunctionVisibility::External | FunctionVisibility::Public
        )
    }
}

pub trait ImplicitlyConvertible<T> {
    fn implicitly_convertible_to(&self, target: T) -> bool;
}

impl ImplicitlyConvertible<FunctionVisibility> for FunctionVisibility {
    fn implicitly_convertible_to(&self, target: Self) -> bool {
        matches!(
            (self, target),
            // public can convert to public, external or internal (if called
            // internally)
            (
                FunctionVisibility::Public,
                FunctionVisibility::Public
                    | FunctionVisibility::Internal
                    | FunctionVisibility::External,
            )
                // private converts to private or internal
                | (
                    FunctionVisibility::Private,
                    FunctionVisibility::Private | FunctionVisibility::Internal,
                )
                // internal and external only convert to themselves
                | (FunctionVisibility::Internal, FunctionVisibility::Internal)
                | (FunctionVisibility::External, FunctionVisibility::External)
        )
    }
}

impl ImplicitlyConvertible<FunctionMutability> for FunctionMutability {
    fn implicitly_convertible_to(&self, target: Self) -> bool {
        matches!(
            (self, target),
            // pure converts to view or non-payable
            (
                FunctionMutability::Pure,
                FunctionMutability::Pure | FunctionMutability::View | FunctionMutability::NonPayable,
            )
                // view converts to non-payable
                | (
                FunctionMutability::View,
                FunctionMutability::View | FunctionMutability::NonPayable
            )
                // non-payable does not implicitly convert to any other kind
                | (FunctionMutability::NonPayable, FunctionMutability::NonPayable)
                // payable converts to non-payable
                | (
                    FunctionMutability::Payable,
                    FunctionMutability::Payable | FunctionMutability::NonPayable,
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
            ir::StorageLocation::MemoryKeyword => Self::Memory,
            ir::StorageLocation::StorageKeyword => Self::Storage,
            ir::StorageLocation::CallDataKeyword => Self::Calldata,
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
                LiteralKind::Integer(_) | LiteralKind::HexInteger { .. } | LiteralKind::Rational(_)
            )
        )
    }
}
