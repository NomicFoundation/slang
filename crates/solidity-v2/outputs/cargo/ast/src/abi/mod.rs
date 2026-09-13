mod node_extensions;
mod serialize;
mod types;

use std::cmp::Ordering;
use std::sync::{Arc, OnceLock};

use itertools::Itertools;
use ruint::aliases::U256;
use sha3::{Digest, Keccak256};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{FunctionTypeMutability, TypeId};

pub use self::serialize::JsonAbi;
pub use self::types::{AbiType, NotAnAbiType, TupleComponent};

pub struct ContractAbi {
    node_id: NodeId,
    name: String,
    file_id: FileId,
    entries: Vec<AbiEntry>,
    storage_layout: Vec<StorageItem>,
    transient_storage_layout: Vec<StorageItem>,
    // `internalType` is rendered from the semantic types at serialization, see `JsonAbi`.
    semantic: Arc<SemanticContext>,
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

    /// The entries as solc's JSON ABI: `serde_json::to_value(abi.json())` is the `abi` array of
    /// solc's standard JSON output.
    pub fn json(&self) -> JsonAbi<'_> {
        JsonAbi(self)
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
    // Hashed on first use: only overloads, which sort by selector, ever need it.
    selector: OnceLock<u32>,
}

impl AbiFunction {
    pub(crate) fn new(
        node_id: NodeId,
        name: String,
        inputs: Vec<AbiParameter>,
        outputs: Vec<AbiParameter>,
        state_mutability: AbiMutability,
    ) -> Self {
        Self {
            node_id,
            name,
            inputs,
            outputs,
            state_mutability,
            selector: OnceLock::new(),
        }
    }

    fn hash_selector(name: &str, inputs: &[AbiParameter]) -> u32 {
        let parameters = inputs.iter().map(|input| &*input.abi_type).join(",");
        selector_from_signature(&format!("{name}({parameters})"))
    }

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

    /// The 4-byte selector, hashed from the canonical signature the inputs spell. Library
    /// members hash the library form instead (`FunctionDefinition::compute_selector`), so this
    /// stays crate-private: it only orders overloads.
    pub(crate) fn selector(&self) -> u32 {
        *self
            .selector
            .get_or_init(|| Self::hash_selector(&self.name, &self.inputs))
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
// same as `solc`'s. Overloaded functions follow in ascending selector order,
// which is where solc's interface function map puts them; other equal names
// use the `node_id` as the tie breaker to keep consistency with the
// `PartialEq` implementation.
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
                    Ordering::Equal => self_inner
                        .selector()
                        .cmp(&other_inner.selector())
                        .then_with(|| self.node_id().cmp(&other.node_id())),
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
    abi_type: Arc<AbiType>,
    type_id: TypeId,
    indexed: bool,
}

impl AbiParameter {
    pub fn node_id(&self) -> Option<NodeId> {
        self.node_id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn abi_type(&self) -> &AbiType {
        &self.abi_type
    }

    /// The semantic type behind [`Self::abi_type`]. `SemanticContext::type_abi_internal_name`
    /// spells it the way solc's JSON-ABI `internalType` field does.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// The parameter's type rendered as its canonical-signature spelling — e.g.
    /// `uint256`, `uint256[]`, or `(uint256,uint256)` for a struct. This is the
    /// form used for selector/signature hashing, **not** the JSON-ABI `"type"`
    /// field (which spells structs as `tuple`/`tuple[]`).
    ///
    /// Allocates a fresh `String` on each call; prefer [`AbiParameter::abi_type`]
    /// for allocation-free, structured access.
    pub fn type_name(&self) -> String {
        self.abi_type.to_string()
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

pub fn hash_from_signature(signature: &str) -> [u8; 32] {
    Keccak256::digest(signature).into()
}

pub fn selector_from_signature(signature: &str) -> u32 {
    let selector_bytes: [u8; 4] = hash_from_signature(signature)[0..4].try_into().unwrap();
    u32::from_be_bytes(selector_bytes)
}
