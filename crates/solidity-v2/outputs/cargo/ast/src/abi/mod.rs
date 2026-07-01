mod node_extensions;
mod types;

use std::cmp::Ordering;
use std::sync::Arc;

use itertools::Either;
use ruint::aliases::U256;
use sha3::{Digest, Keccak256};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{FunctionTypeMutability, TupleType, Type, TypeId};

pub use self::types::TupleComponent;
use crate::abi::types::{type_as_abi_type, AbiType};

pub struct ContractAbi {
    node_id: NodeId,
    name: String,
    file_id: FileId,
    entries: Vec<AbiEntry>,
    storage_layout: Vec<StorageItem>,
    transient_storage_layout: Vec<StorageItem>,
}

impl ContractAbi {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_id(&self) -> &FileId {
        &self.file_id
    }

    pub fn entries(&self) -> &[AbiEntry] {
        &self.entries
    }

    pub fn storage_layout(&self) -> &[StorageItem] {
        &self.storage_layout
    }

    pub fn transient_storage_layout(&self) -> &[StorageItem] {
        &self.transient_storage_layout
    }
}

pub type AbiMutability = FunctionTypeMutability;

#[derive(Clone, Debug)]
pub struct AbiConstructor {
    node_id: NodeId,
    inputs: Vec<AbiParameter>,
    state_mutability: AbiMutability,
}

impl AbiConstructor {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn inputs(&self) -> &[AbiParameter] {
        &self.inputs
    }

    pub fn state_mutability(&self) -> &AbiMutability {
        &self.state_mutability
    }
}

#[derive(Clone, Debug)]
pub struct AbiError {
    node_id: NodeId,
    name: String,
    inputs: Vec<AbiParameter>,
}

impl AbiError {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inputs(&self) -> &[AbiParameter] {
        &self.inputs
    }
}

#[derive(Clone, Debug)]
pub struct AbiEvent {
    node_id: NodeId,
    name: String,
    inputs: Vec<AbiParameter>,
    anonymous: bool,
}

impl AbiEvent {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inputs(&self) -> &[AbiParameter] {
        &self.inputs
    }

    pub fn anonymous(&self) -> bool {
        self.anonymous
    }
}

#[derive(Clone, Debug)]
pub struct AbiFallback {
    node_id: NodeId,
    state_mutability: AbiMutability,
}

impl AbiFallback {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn state_mutability(&self) -> &AbiMutability {
        &self.state_mutability
    }
}

#[derive(Clone, Debug)]
pub struct AbiFunction {
    node_id: NodeId,
    name: String,
    inputs: Vec<AbiParameter>,
    outputs: Vec<AbiParameter>,
    state_mutability: AbiMutability,
}

impl AbiFunction {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn inputs(&self) -> &[AbiParameter] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[AbiParameter] {
        &self.outputs
    }

    pub fn state_mutability(&self) -> &AbiMutability {
        &self.state_mutability
    }
}

#[derive(Clone, Debug)]
pub struct AbiReceive {
    node_id: NodeId,
    state_mutability: AbiMutability,
}

impl AbiReceive {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn state_mutability(&self) -> &AbiMutability {
        &self.state_mutability
    }
}

#[derive(Clone, Debug)]
pub enum AbiEntry {
    Constructor(AbiConstructor),
    Error(AbiError),
    Event(AbiEvent),
    Fallback(AbiFallback),
    Function(AbiFunction),
    Receive(AbiReceive),
}

impl AbiEntry {
    pub fn node_id(&self) -> NodeId {
        match self {
            AbiEntry::Constructor(inner) => inner.node_id(),
            AbiEntry::Error(inner) => inner.node_id(),
            AbiEntry::Event(inner) => inner.node_id(),
            AbiEntry::Fallback(inner) => inner.node_id(),
            AbiEntry::Function(inner) => inner.node_id(),
            AbiEntry::Receive(inner) => inner.node_id(),
        }
    }
}

impl PartialEq for AbiEntry {
    fn eq(&self, other: &Self) -> bool {
        self.node_id() == other.node_id()
    }
}

impl Eq for AbiEntry {}

// The ordering defined by this implementation is alphabetical "type" + "name",
// same as `solc`'s. For equal names we use the `node_id` as the tie breaker to
// keep consistency with the `PartialEq` implementation.
impl Ord for AbiEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Constructor(_), Self::Constructor(_))
            | (Self::Fallback(_), Self::Fallback(_))
            | (Self::Receive(_), Self::Receive(_)) => self.node_id().cmp(&other.node_id()),
            (Self::Error(self_inner), Self::Error(other_inner)) => {
                match self_inner.name.cmp(&other_inner.name) {
                    Ordering::Equal => self.node_id().cmp(&other.node_id()),
                    name_ordering => name_ordering,
                }
            }
            (Self::Event(self_inner), Self::Event(other_inner)) => {
                match self_inner.name.cmp(&other_inner.name) {
                    Ordering::Equal => self.node_id().cmp(&other.node_id()),
                    name_ordering => name_ordering,
                }
            }
            (Self::Function(self_inner), Self::Function(other_inner)) => {
                match self_inner.name.cmp(&other_inner.name) {
                    Ordering::Equal => self.node_id().cmp(&other.node_id()),
                    name_ordering => name_ordering,
                }
            }

            (Self::Constructor(_), _) => Ordering::Less,
            (_, Self::Constructor(_)) => Ordering::Greater,
            (Self::Error(_), _) => Ordering::Less,
            (_, Self::Error(_)) => Ordering::Greater,
            (Self::Event(_), _) => Ordering::Less,
            (_, Self::Event(_)) => Ordering::Greater,
            (Self::Fallback(_), _) => Ordering::Less,
            (_, Self::Fallback(_)) => Ordering::Greater,
            (Self::Function(_), _) => Ordering::Less,
            (_, Self::Function(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for AbiEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AbiParameter {
    node_id: Option<NodeId>, // will be `None` if the function is a generated getter
    name: Option<String>,
    abi_type: AbiType,
    indexed: bool,
}

impl AbiParameter {
    pub fn node_id(&self) -> Option<NodeId> {
        self.node_id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn type_name(&self) -> String {
        self.abi_type.to_string()
    }

    pub fn components(&self) -> &[TupleComponent] {
        self.abi_type.components()
    }

    pub fn indexed(&self) -> bool {
        self.indexed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageItem {
    node_id: NodeId,
    label: String,
    slot: U256,
    offset: usize,
    type_name: String,
}

impl StorageItem {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn slot(&self) -> U256 {
        self.slot
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn type_name(&self) -> &str {
        &self.type_name
    }
}

pub fn selector_from_signature(signature: &str) -> u32 {
    let mut hasher = Keccak256::new();
    hasher.update(signature.as_bytes());
    let result = hasher.finalize();

    let selector_bytes: [u8; 4] = result[0..4].try_into().unwrap();
    u32::from_be_bytes(selector_bytes)
}

pub(crate) fn extract_function_type_parameters_abi(
    semantic: &Arc<SemanticContext>,
    type_id: TypeId,
) -> Option<(Vec<AbiParameter>, Vec<AbiParameter>)> {
    let Type::Function(function_type) = semantic.types().get_type_by_id(type_id) else {
        return None;
    };
    // TODO: our type system doesn't track parameter names for function types,
    // so we can't convey that information in the ABI. This is important for
    // getters where we should transfer that information from mapping or struct
    // types (eg. a getter that returns a struct should name its output
    // parameters from the struct members).
    let mut inputs = Vec::new();
    for parameter_type_id in &function_type.parameter_types {
        let abi_type = type_as_abi_type(semantic, *parameter_type_id)?;
        inputs.push(AbiParameter {
            node_id: None,
            name: None,
            abi_type,
            indexed: false,
        });
    }
    // A tuple as a return type from a function represents multiple return
    // values, so we need to flatten it
    let output_types = match semantic.types().get_type_by_id(function_type.return_type) {
        Type::Tuple(TupleType { types }) => Either::Left(types.iter()),
        _ => Either::Right(std::iter::once(&function_type.return_type)),
    };
    let outputs = output_types
        .map(|output_type_id| {
            let abi_type = type_as_abi_type(semantic, *output_type_id)?;
            Some(AbiParameter {
                node_id: None,
                name: None,
                abi_type,
                indexed: false,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some((inputs, outputs))
}
