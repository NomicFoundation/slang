use std::collections::HashMap;
use std::ops::Index;

use indexmap::IndexSet;
use metaslang_cst::nodes::NodeId;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TypeId(usize);

pub struct TypeRegistry {
    types: IndexSet<Type>,
    definitions: HashMap<NodeId, TypeDefinition>,
}

impl TypeRegistry {
    fn new() -> Self {
        Self {
            types: IndexSet::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn find_type(&self, type_: &Type) -> Option<TypeId> {
        self.types.get_index_of(type_).map(TypeId)
    }

    pub fn register_type(&mut self, type_: Type) -> TypeId {
        let (index, _) = self.types.insert_full(type_);
        TypeId(index)
    }

    pub fn find_definition(&self, node_id: NodeId) -> Option<&TypeDefinition> {
        self.definitions.get(&node_id)
    }

    pub fn register_definition(&mut self, type_definition: TypeDefinition) {
        let node_id = type_definition.node_id();
        _ = self.definitions.insert(node_id, type_definition);
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
}

impl Index<TypeId> for TypeRegistry {
    type Output = Type;

    fn index(&self, index: TypeId) -> &Self::Output {
        self.types.get_index(index.0).unwrap()
    }
}

impl Index<NodeId> for TypeRegistry {
    type Output = TypeDefinition;

    fn index(&self, index: NodeId) -> &Self::Output {
        self.definitions.get(&index).unwrap()
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum TypeDefinition {
    Contract(ContractType),
    Enum(EnumType),
    Interface(InterfaceType),
    Struct(StructType),
    UserDefinedValueType(UserDefinedValueType),
}

impl TypeDefinition {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Contract(ContractType { node_id, .. })
            | Self::Enum(EnumType { node_id, .. })
            | Self::Interface(InterfaceType { node_id, .. })
            | Self::Struct(StructType { node_id, .. })
            | Self::UserDefinedValueType(UserDefinedValueType { node_id, .. }) => *node_id,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ContractType {
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
pub struct InterfaceType {
    pub node_id: NodeId,
    pub name: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StructType {
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
pub struct EnumType {
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
pub struct UserDefinedValueType {
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
    FixedPointNumber {
        signed: bool,
        bits: u32,
        precision_bits: u32,
    },
    Function {
        parameter_types: Vec<TypeId>,
        return_types: Vec<TypeId>,
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
    String {
        location: DataLocation,
    },
    Struct {
        node_id: NodeId,
        location: DataLocation,
    },
    UserDefinedValueType {
        node_id: NodeId,
    },
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
            | Self::Function { .. }
            | Self::FixedPointNumber { .. }
            | Self::Integer { .. }
            | Self::Interface { .. }
            | Self::UserDefinedValueType { .. } => None,
            Self::Mapping { .. } => Some(DataLocation::Storage),
            Self::Array { location, .. } => Some(*location),
            Self::Bytes { location } => Some(*location),
            Self::String { location } => Some(*location),
            Self::Struct { location, .. } => Some(*location),
        }
    }
}
