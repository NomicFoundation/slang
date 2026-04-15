use std::cmp::Ordering;
use std::rc::Rc;

use sha3::{Digest, Keccak256};
use slang_solidity_v2_ir::ir::{self, NodeId};
use slang_solidity_v2_semantic::binder::Definition;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{Type, TypeId};

pub struct ContractAbi {
    pub node_id: NodeId,
    pub name: String,
    pub file_id: String,
    pub entries: Vec<AbiEntry>,
    pub storage_layout: Vec<StorageItem>,
    pub transient_storage_layout: Vec<StorageItem>,
}

pub type AbiMutability = ir::FunctionMutability;

#[derive(Clone, Debug)]
pub enum AbiEntry {
    Constructor {
        node_id: NodeId,
        inputs: Vec<AbiParameter>,
        state_mutability: AbiMutability,
    },
    Error {
        node_id: NodeId,
        name: String,
        inputs: Vec<AbiParameter>,
    },
    Event {
        node_id: NodeId,
        name: String,
        inputs: Vec<AbiParameter>,
        anonymous: bool,
    },
    Fallback {
        node_id: NodeId,
        state_mutability: AbiMutability,
    },
    Function {
        node_id: NodeId,
        name: String,
        inputs: Vec<AbiParameter>,
        outputs: Vec<AbiParameter>,
        state_mutability: AbiMutability,
    },
    Receive {
        node_id: NodeId,
        state_mutability: AbiMutability,
    },
}

impl AbiEntry {
    pub fn node_id(&self) -> NodeId {
        match self {
            AbiEntry::Constructor { node_id, .. }
            | AbiEntry::Error { node_id, .. }
            | AbiEntry::Event { node_id, .. }
            | AbiEntry::Fallback { node_id, .. }
            | AbiEntry::Function { node_id, .. }
            | AbiEntry::Receive { node_id, .. } => *node_id,
        }
    }
}

impl PartialEq for AbiEntry {
    fn eq(&self, other: &Self) -> bool {
        self.node_id() == other.node_id()
    }
}

impl Eq for AbiEntry {}

impl Ord for AbiEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Constructor { .. }, Self::Constructor { .. })
            | (Self::Fallback { .. }, Self::Fallback { .. })
            | (Self::Receive { .. }, Self::Receive { .. }) => self.node_id().cmp(&other.node_id()),
            (
                Self::Error {
                    name: self_name, ..
                },
                Self::Error {
                    name: other_name, ..
                },
            )
            | (
                Self::Event {
                    name: self_name, ..
                },
                Self::Event {
                    name: other_name, ..
                },
            )
            | (
                Self::Function {
                    name: self_name, ..
                },
                Self::Function {
                    name: other_name, ..
                },
            ) => match self_name.cmp(other_name) {
                Ordering::Equal => self.node_id().cmp(&other.node_id()),
                name_ordering => name_ordering,
            },

            (Self::Constructor { .. }, _) => Ordering::Less,
            (_, Self::Constructor { .. }) => Ordering::Greater,
            (Self::Error { .. }, _) => Ordering::Less,
            (_, Self::Error { .. }) => Ordering::Greater,
            (Self::Event { .. }, _) => Ordering::Less,
            (_, Self::Event { .. }) => Ordering::Greater,
            (Self::Fallback { .. }, _) => Ordering::Less,
            (_, Self::Fallback { .. }) => Ordering::Greater,
            (Self::Function { .. }, _) => Ordering::Less,
            (_, Self::Function { .. }) => Ordering::Greater,
        }
    }
}

impl PartialOrd for AbiEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterComponent {
    pub name: String,
    pub r#type: String,
    pub components: Vec<ParameterComponent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AbiParameter {
    pub node_id: Option<NodeId>, // will be `None` if the function is a generated getter
    pub name: Option<String>,
    pub r#type: String,
    pub components: Vec<ParameterComponent>,
    pub indexed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageItem {
    pub node_id: NodeId,
    pub label: String,
    pub slot: usize,
    pub offset: usize,
    pub r#type: String,
}

pub fn selector_from_signature(signature: &str) -> u32 {
    let mut hasher = Keccak256::new();
    hasher.update(signature.as_bytes());
    let result = hasher.finalize();

    let selector_bytes: [u8; 4] = result[0..4].try_into().unwrap();
    u32::from_be_bytes(selector_bytes)
}

pub(crate) fn extract_function_type_parameters_abi(
    semantic: &Rc<SemanticContext>,
    type_id: TypeId,
) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
    let Type::Function(function_type) = semantic.types().get_type_by_id(type_id) else {
        return None;
    };
    let mut inputs = Vec::new();
    for parameter_type_id in &function_type.parameter_types {
        let (r#type, components) = type_as_abi_parameter(semantic, *parameter_type_id)?;
        inputs.push(AbiParameter {
            node_id: None,
            name: None,
            r#type,
            components,
            indexed: false,
        });
    }
    let (r#type, components) = type_as_abi_parameter(semantic, function_type.return_type)?;
    let outputs = vec![AbiParameter {
        node_id: None,
        name: None,
        r#type,
        components,
        indexed: false,
    }];
    Some((inputs, outputs))
}

pub(crate) fn type_as_abi_parameter(
    semantic: &Rc<SemanticContext>,
    type_id: TypeId,
) -> Option<(String, Vec<ParameterComponent>)> {
    match semantic.types().get_type_by_id(type_id) {
        Type::Array { element_type, .. } => {
            let (element_type_name, element_components) =
                type_as_abi_parameter(semantic, *element_type)?;
            Some((format!("{element_type_name}[]"), element_components))
        }
        Type::Struct { definition_id, .. } => {
            // We need to recursively expand the struct fields as components
            let Definition::Struct(definition) = semantic
                .binder()
                .find_definition_by_id(*definition_id)
                .expect("the definition of a type exists")
            else {
                unreachable!("the definition of a struct type is a struct");
            };
            let mut components = Vec::new();
            for member in &definition.ir_node.members {
                let name = member.name.unparse().to_string();
                let member_type_id = semantic.binder().node_typing(member.id()).as_type_id()?;
                let (r#type, subcomponents) = type_as_abi_parameter(semantic, member_type_id)?;
                components.push(ParameterComponent {
                    name,
                    r#type,
                    components: subcomponents,
                });
            }
            Some(("tuple".to_string(), components))
        }
        _ => Some((semantic.type_canonical_name(type_id)?, Vec::new())),
    }
}
