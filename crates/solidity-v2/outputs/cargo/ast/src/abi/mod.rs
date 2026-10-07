mod node_extensions;
mod serialize;
mod storage_layout;

use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;

use sha3::{Digest, Keccak256};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::{AbiTypeSpelling, SemanticContext};
use slang_solidity_v2_semantic::types::{FunctionTypeMutability, TypeId};

pub(crate) use self::storage_layout::StorageKind;
pub use self::storage_layout::{
    StorageItem, StorageLayout, StorageMember, StoragePosition, StorageSize, StorageType,
    StorageTypeKind,
};
use crate::ast::{Definition, Type};

/// Serializes as the JSON ABI, the array of its entries.
pub struct ContractAbi {
    node_id: NodeId,
    name: String,
    file_id: FileId,
    entries: Vec<AbiEntry>,
}

impl ContractAbi {
    /// Builds the ABI with its entries sorted, see [`AbiEntry`]'s `Ord`.
    pub(crate) fn new(
        node_id: NodeId,
        name: String,
        file_id: FileId,
        mut entries: Vec<AbiEntry>,
        semantic: &Arc<SemanticContext>,
    ) -> Self {
        entries.sort();
        order_overloads_by_selector(&mut entries, semantic);
        Self {
            node_id,
            name,
            file_id,
            entries,
        }
    }

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
    /// How the JSON spells the parameters' `type`.
    type_spelling: AbiTypeSpelling,
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

/// How the JSON spells the parameter `type`s of a function declared in `enclosing_definition`:
/// by name in a library, as its selectors do.
pub(crate) fn json_type_spelling(enclosing_definition: Option<&Definition>) -> AbiTypeSpelling {
    match enclosing_definition {
        Some(Definition::Library(_)) => AbiTypeSpelling::LibraryJson,
        _ => AbiTypeSpelling::Json,
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
// keep consistency with the `PartialEq` implementation; `ContractAbi::new` then
// puts overloaded functions in ascending selector order.
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

/// Reorders each run of same-named functions in sorted `entries` by selector, hashing a selector
/// only for a function that has overloads.
fn order_overloads_by_selector(entries: &mut [AbiEntry], semantic: &Arc<SemanticContext>) {
    for run in entries.chunk_by_mut(same_function_name) {
        if run.len() > 1 {
            run.sort_by_cached_key(|entry| overload_selector(entry, semantic));
        }
    }
}

fn same_function_name(this: &AbiEntry, other: &AbiEntry) -> bool {
    match (this, other) {
        (AbiEntry::Function(this), AbiEntry::Function(other)) => this.name() == other.name(),
        _ => false,
    }
}

fn overload_selector(entry: &AbiEntry, semantic: &Arc<SemanticContext>) -> u32 {
    let AbiEntry::Function(function) = entry else {
        unreachable!("only functions share a name");
    };
    let selector = match Definition::try_create(function.node_id(), semantic) {
        Some(Definition::Function(definition)) => definition.compute_selector(),
        // A public state variable's getter can overload an inherited function.
        Some(Definition::StateVariable(definition)) => definition.compute_selector(),
        _ => unreachable!("an ABI function is a function or a state variable's getter"),
    };
    selector.expect("a function in the ABI is externally visible")
}

/// A parameter of an ABI entry: a view over its semantic type, the way [`Type`] is. The
/// spellings are rendered from the interned `TypeId` on request rather than stored.
#[derive(Clone)]
pub struct AbiParameter {
    node_id: Option<NodeId>, // will be `None` if the function is a generated getter
    name: Option<String>,
    type_id: TypeId,
    indexed: bool,
    semantic: Arc<SemanticContext>,
}

impl AbiParameter {
    /// `None` when the type has no ABI representation, e.g. a mapping or a storage-only struct.
    pub(crate) fn new(
        node_id: Option<NodeId>,
        name: Option<String>,
        type_id: TypeId,
        indexed: bool,
        semantic: &Arc<SemanticContext>,
    ) -> Option<Self> {
        if !semantic.has_abi_type(type_id) {
            return None;
        }
        Some(Self {
            node_id,
            name,
            type_id,
            indexed,
            semantic: Arc::clone(semantic),
        })
    }

    pub fn node_id(&self) -> Option<NodeId> {
        self.node_id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// The Solidity type behind the parameter.
    pub fn get_type(&self) -> Type {
        Type::create(self.type_id, &self.semantic)
    }

    /// The parameter's type rendered as its canonical-signature spelling — e.g.
    /// `uint256`, `uint256[]`, or `(uint256,uint256)` for a struct. This is the
    /// form used for selector/signature hashing, **not** the JSON-ABI `"type"`
    /// field (structs there are `tuple`/`tuple[]`, see [`ContractAbi`]'s `Serialize`), nor
    /// its `"internalType"` field, which is [`Self::internal_type`].
    pub fn type_name(&self) -> String {
        let mut name = String::new();
        self.semantic
            .write_type_abi_name(self.type_id, AbiTypeSpelling::Selector, &mut name)
            .expect(
                "the type was checked to have an ABI representation when the parameter was built",
            );
        name
    }

    /// The type as solc's JSON-ABI `internalType` spells it: `struct C.S[]`, `enum C.E`,
    /// `contract I`, `address payable`, a user-defined value type by name.
    pub fn internal_type(&self) -> String {
        self.semantic.type_abi_internal_name(self.type_id)
    }

    pub fn indexed(&self) -> bool {
        self.indexed
    }
}

impl fmt::Debug for AbiParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AbiParameter")
            .field("node_id", &self.node_id)
            .field("name", &self.name)
            .field("type_id", &self.type_id)
            .field("indexed", &self.indexed)
            .finish_non_exhaustive()
    }
}

pub fn hash_from_signature(signature: &str) -> [u8; 32] {
    Keccak256::digest(signature).into()
}

/// Keccak-256 over a signature written piece by piece, so a selector needs no signature string.
#[derive(Default)]
pub(crate) struct SignatureHasher(Keccak256);

impl fmt::Write for SignatureHasher {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.update(s.as_bytes());
        Ok(())
    }
}

impl SignatureHasher {
    pub(crate) fn selector(self) -> u32 {
        let hash: [u8; 32] = self.0.finalize().into();
        u32::from_be_bytes(hash[0..4].try_into().unwrap())
    }
}

pub fn selector_from_signature(signature: &str) -> u32 {
    let mut hasher = SignatureHasher::default();
    hasher.0.update(signature.as_bytes());
    hasher.selector()
}
