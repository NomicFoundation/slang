mod node_extensions;
mod serialize;
mod storage_layout;

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
use crate::ast::Definition;

/// Serializes as the JSON ABI, the array of its entries.
pub struct ContractAbi {
    node_id: NodeId,
    name: String,
    file_id: FileId,
    entries: Vec<AbiEntry>,
    semantic: Arc<SemanticContext>,
}

impl ContractAbi {
    /// Builds the ABI with its entries in the order [`Self::entries`] describes.
    pub(crate) fn new(
        node_id: NodeId,
        name: String,
        file_id: FileId,
        mut entries: Vec<AbiEntry>,
        semantic: &Arc<SemanticContext>,
    ) -> Self {
        entries.sort_by(|this, other| this.sort_key().cmp(&other.sort_key()));
        order_overloads_by_selector(&mut entries, semantic);
        Self {
            node_id,
            name,
            file_id,
            entries,
            semantic: Arc::clone(semantic),
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

    /// The entries as the JSON lists them: by `type`, then by name, with a function's overloads in
    /// ascending selector order.
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

    /// Sorts by `type` and then name, as solc lists entries; the node id breaks ties, until
    /// [`ContractAbi::new`] puts overloads in selector order.
    fn sort_key(&self) -> (u8, Option<&str>, NodeId) {
        match self {
            AbiEntry::Constructor(inner) => (0, None, inner.node_id),
            AbiEntry::Error(inner) => (1, Some(&inner.name), inner.node_id),
            AbiEntry::Event(inner) => (2, Some(&inner.name), inner.node_id),
            AbiEntry::Fallback(inner) => (3, None, inner.node_id),
            AbiEntry::Function(inner) => (4, Some(&inner.name), inner.node_id),
            AbiEntry::Receive(inner) => (5, None, inner.node_id),
        }
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

/// A parameter of an ABI entry. Its type's spellings are written from the interned `TypeId` when
/// the ABI is serialized, rather than stored.
#[derive(Clone, Debug)]
pub struct AbiParameter {
    name: Option<String>,
    type_id: TypeId,
    indexed: bool,
}

impl AbiParameter {
    /// `None` when the type has no ABI representation, e.g. a mapping or a storage-only struct.
    pub(crate) fn new(
        name: Option<String>,
        type_id: TypeId,
        indexed: bool,
        semantic: &SemanticContext,
    ) -> Option<Self> {
        if !semantic.has_abi_type(type_id) {
            return None;
        }
        Some(Self {
            name,
            type_id,
            indexed,
        })
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn indexed(&self) -> bool {
        self.indexed
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
