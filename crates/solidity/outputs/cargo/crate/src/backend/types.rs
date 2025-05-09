use std::collections::HashMap;

use indexmap::IndexSet;
use metaslang_cst::nodes::NodeId;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(usize);

pub struct TypeRegistry {
    types: IndexSet<Type>,
    definitions: HashMap<NodeId, TypeDefinition>,

    // Pre-defined core types
    address_type_id: TypeId,
    bool_type_id: TypeId,
    error_type_id: TypeId,
    rational_type_id: TypeId,
    string_type_id: TypeId,
    uint256_type_id: TypeId,
    void_type_id: TypeId,
}

impl TypeRegistry {
    fn new() -> Self {
        let mut types = IndexSet::new();
        let (address_type, _) = types.insert_full(Type::Address { payable: false });
        let (bool_type, _) = types.insert_full(Type::Boolean);
        let (error_type, _) = types.insert_full(Type::Error { node_id: None });
        let (rational_type, _) = types.insert_full(Type::Rational);
        let (string_type, _) = types.insert_full(Type::String {
            location: DataLocation::Memory,
        });
        let (uint256_type, _) = types.insert_full(Type::Integer {
            signed: false,
            bits: 256,
        });
        let (void_type, _) = types.insert_full(Type::Void);

        Self {
            types,
            definitions: HashMap::new(),

            address_type_id: TypeId(address_type),
            bool_type_id: TypeId(bool_type),
            error_type_id: TypeId(error_type),
            rational_type_id: TypeId(rational_type),
            string_type_id: TypeId(string_type),
            uint256_type_id: TypeId(uint256_type),
            void_type_id: TypeId(void_type),
        }
    }

    pub fn find_type(&self, type_: &Type) -> Option<TypeId> {
        self.types.get_index_of(type_).map(TypeId)
    }

    pub fn register_type(&mut self, type_: Type) -> TypeId {
        let (index, _) = self.types.insert_full(type_);
        TypeId(index)
    }

    pub fn get_type_by_id(&self, type_id: TypeId) -> Option<&Type> {
        self.types.get_index(type_id.0)
    }

    pub fn get_type_definition_by_node_id(&self, node_id: NodeId) -> Option<&TypeDefinition> {
        self.definitions.get(&node_id)
    }

    pub fn register_definition(&mut self, type_definition: TypeDefinition) {
        let node_id = type_definition.node_id();
        let previous_def = self.definitions.insert(node_id, type_definition);
        if let Some(previous_def) = previous_def {
            unimplemented!("attempt to re-register type definition {previous_def:?}");
        }
    }

    pub fn validate(&self) {
        // A valid type registry has definitions for all registered user types
        for type_ in &self.types {
            if let Some(node_id) = type_.node_id() {
                assert!(
                    self.definitions.contains_key(&node_id),
                    "Missing type definition for {node_id:?} in {type_:?}"
                );
            }
        }
    }

    pub fn implicitly_convertible_to(&self, from_type_id: TypeId, to_type_id: TypeId) -> bool {
        if from_type_id == to_type_id {
            return true;
        }
        let from_type = self.get_type_by_id(from_type_id).unwrap();
        let to_type = self.get_type_by_id(to_type_id).unwrap();

        match (from_type, to_type) {
            (
                Type::Address {
                    payable: from_payable,
                },
                Type::Address { .. },
            ) => *from_payable,

            (
                Type::Integer {
                    signed: from_signed,
                    bits: from_bits,
                },
                Type::Integer {
                    signed: to_signed,
                    bits: to_bits,
                },
            ) => {
                if from_signed == to_signed {
                    from_bits <= to_bits
                } else if *from_signed {
                    false
                } else {
                    from_bits < to_bits
                }
            }

            (Type::Rational, Type::Integer { .. }) => true,

            // TODO: add more implicit conversion rules
            _ => unimplemented!(
                "implicitly converting from {from_type:?} to {to_type:?} not supported"
            ),
        }
    }
}

impl TypeRegistry {
    pub fn address(&self) -> TypeId {
        self.address_type_id
    }
    pub fn bool(&self) -> TypeId {
        self.bool_type_id
    }
    pub fn error(&self) -> TypeId {
        self.error_type_id
    }
    pub fn rational(&self) -> TypeId {
        self.rational_type_id
    }
    pub fn string(&self) -> TypeId {
        self.string_type_id
    }
    pub fn uint256(&self) -> TypeId {
        self.uint256_type_id
    }
    pub fn void(&self) -> TypeId {
        self.void_type_id
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TypeDefinition {
    Contract(ContractTypeDefinition),
    Enum(EnumTypeDefinition),
    Error(ErrorTypeDefinition),
    Interface(InterfaceTypeDefinition),
    Struct(StructTypeDefinition),
    UserDefinedValueType(UserDefinedValueTypeDefinition),
}

impl TypeDefinition {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Contract(ContractTypeDefinition { node_id, .. })
            | Self::Enum(EnumTypeDefinition { node_id, .. })
            | Self::Error(ErrorTypeDefinition { node_id, .. })
            | Self::Interface(InterfaceTypeDefinition { node_id, .. })
            | Self::Struct(StructTypeDefinition { node_id, .. })
            | Self::UserDefinedValueType(UserDefinedValueTypeDefinition { node_id, .. }) => {
                *node_id
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ContractTypeDefinition {
    pub node_id: NodeId,
    pub name: String,
    pub state_variables: Vec<StateVariable>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StateVariable {
    pub node_id: NodeId,
    pub name: String,
    pub type_id: TypeId,
}

#[derive(Debug, Eq, PartialEq)]
pub struct InterfaceTypeDefinition {
    pub node_id: NodeId,
    pub name: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructTypeDefinition {
    pub node_id: NodeId,
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructField {
    pub node_id: NodeId,
    pub name: String,
    pub type_id: TypeId,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EnumTypeDefinition {
    pub node_id: NodeId,
    pub name: String,
    pub members: Vec<EnumMember>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EnumMember {
    pub node_id: NodeId,
    pub name: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorTypeDefinition {
    pub node_id: NodeId,
    pub name: String,
    pub members: Vec<ErrorField>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorField {
    pub node_id: NodeId,
    pub name: Option<String>,
    pub type_id: TypeId,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UserDefinedValueTypeDefinition {
    pub node_id: NodeId,
    pub name: String,
    pub type_id: TypeId,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Address {
        payable: bool,
    },
    Array {
        element_type: TypeId,
        location: DataLocation,
        // TODO: handle fixed-size array types?
    },
    Boolean,
    ByteArray {
        width: u32,
    },
    Bytes {
        location: DataLocation,
    },
    Contract {
        node_id: NodeId,
    },
    Enum {
        node_id: NodeId,
    },
    Error {
        node_id: Option<NodeId>,
    },
    FixedPointNumber {
        signed: bool,
        bits: u32,
        precision_bits: u32,
    },
    Function {
        parameter_types: Vec<TypeId>,
        return_type: TypeId,
        external: bool,
        kind: FunctionTypeKind,
    },
    Integer {
        signed: bool,
        bits: u32,
    },
    Interface {
        node_id: NodeId,
    },
    Mapping {
        key_name: Option<String>,
        key_type_id: TypeId,
        value_name: Option<String>,
        value_type_id: TypeId,
    },
    Rational,
    String {
        location: DataLocation,
    },
    Struct {
        node_id: NodeId,
        location: DataLocation,
    },
    Tuple {
        types: Vec<TypeId>,
    },
    Undecided {
        choices: Vec<TypeId>,
    },
    UserDefinedValueType {
        node_id: NodeId,
    },
    Void,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DataLocation {
    Memory,
    Storage,
    Calldata,

    // This applies to struct fields of reference types, where the data location is the struct's.
    Inherited,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum FunctionTypeKind {
    Pure,
    View,
    Payable,
}

impl Type {
    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::Contract { node_id, .. }
            | Self::Struct { node_id, .. }
            | Self::Enum { node_id, .. }
            | Self::Interface { node_id, .. }
            | Self::UserDefinedValueType { node_id, .. } => Some(*node_id),
            _ => None,
        }
    }

    pub fn data_location(&self) -> Option<DataLocation> {
        match self {
            Self::Address { .. }
            | Self::Boolean
            | Self::ByteArray { .. }
            | Self::Contract { .. }
            | Self::Enum { .. }
            | Self::Error { .. }
            | Self::Function { .. }
            | Self::FixedPointNumber { .. }
            | Self::Integer { .. }
            | Self::Interface { .. }
            | Self::Rational
            | Self::Tuple { .. }
            | Self::Undecided { .. }
            | Self::UserDefinedValueType { .. }
            | Self::Void => None,
            Self::Mapping { .. } => Some(DataLocation::Storage),
            Self::Array { location, .. } => Some(*location),
            Self::Bytes { location } => Some(*location),
            Self::String { location } => Some(*location),
            Self::Struct { location, .. } => Some(*location),
        }
    }
}
