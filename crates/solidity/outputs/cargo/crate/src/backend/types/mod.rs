use crate::cst::NodeId;

mod parsing;
mod registry;

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
        // TODO: this probably should be a u256, although in practice usize or
        // even u32 should suffice
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
    Zero,
    // TODO: collect and store more information about literal numbers
    Rational,
    DecimalInteger,
    HexInteger { bytes: u32 },
    HexString { bytes: u32 },
    String { bytes: u32 },
    Address,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionType {
    pub definition_id: Option<NodeId>, // this may point to a FunctionDefinition
    pub implicit_receiver_type: Option<TypeId>,
    pub parameter_types: Vec<TypeId>,
    pub return_type: TypeId,
    // TODO: a bool is not sufficient in some corner cases and we need to
    // distinguish between public and external
    pub external: bool,
    pub kind: FunctionTypeKind,
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
    pub fn implicitly_convertible_to(&self, target: Self) -> bool {
        match (self, target) {
            (from, to) if *from == to => true,
            (DataLocation::Storage | DataLocation::Calldata, DataLocation::Memory) => true,
            _ => false,
        }
    }

    pub fn overrides(&self, target: Self) -> bool {
        *self == target || (*self == Self::Memory && target == Self::Calldata)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FunctionTypeKind {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl FunctionTypeKind {
    pub fn implicitly_convertible_to(&self, target: Self) -> bool {
        matches!(
            (self, target),
            // pure converts to view or non-payable
            (
                FunctionTypeKind::Pure,
                FunctionTypeKind::Pure | FunctionTypeKind::View | FunctionTypeKind::NonPayable,
            )
                // view converts to non-payable
                | (
                FunctionTypeKind::View,
                FunctionTypeKind::View | FunctionTypeKind::NonPayable
            )
                // non-payable does not implicitly convert to any other kind
                | (FunctionTypeKind::NonPayable, FunctionTypeKind::NonPayable)
                // payable converts to non-payable
                | (
                    FunctionTypeKind::Payable,
                    FunctionTypeKind::Payable | FunctionTypeKind::NonPayable,
                )
        )
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
                LiteralKind::Zero
                    | LiteralKind::DecimalInteger
                    | LiteralKind::HexInteger { .. }
                    | LiteralKind::Rational
            )
        )
    }
}
