mod node_extensions;
mod types;

use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;

use ruint::aliases::U256;
use sha3::{Digest, Keccak256};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::SemanticContext;
pub use slang_solidity_v2_semantic::context::{
    StorageMember, StoragePosition, StorageSize, StorageTypeKind,
};
use slang_solidity_v2_semantic::types::{FunctionTypeMutability, TypeId};

pub use self::types::{AbiType, NotAnAbiType, TupleComponent};
use crate::abi::types::{is_abi_type, type_as_abi_type};
use crate::ast::Type;

pub struct ContractAbi {
    node_id: NodeId,
    name: String,
    file_id: FileId,
    entries: Vec<AbiEntry>,
    storage_layout: StorageLayout,
    transient_storage_layout: StorageLayout,
}

impl ContractAbi {
    /// Builds the ABI with its entries sorted, see [`AbiEntry`]'s `Ord`.
    pub(crate) fn new(
        node_id: NodeId,
        name: String,
        file_id: FileId,
        mut entries: Vec<AbiEntry>,
        storage_layout: StorageLayout,
        transient_storage_layout: StorageLayout,
    ) -> Self {
        entries.sort();
        Self {
            node_id,
            name,
            file_id,
            entries,
            storage_layout,
            transient_storage_layout,
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

    pub fn storage_layout(&self) -> &StorageLayout {
        &self.storage_layout
    }

    pub fn transient_storage_layout(&self) -> &StorageLayout {
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

/// A parameter of an ABI entry: a view over its semantic type, the way [`Type`] is. The ABI
/// shape ([`Self::abi_type`]) and the spellings are rendered from the interned `TypeId` on
/// request rather than stored.
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
        if !is_abi_type(semantic, type_id) {
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

    pub fn abi_type(&self) -> AbiType {
        type_as_abi_type(&self.semantic, self.type_id).expect(
            "the type was checked to have an ABI representation when the parameter was built",
        )
    }

    /// The parameter's type rendered as its canonical-signature spelling — e.g.
    /// `uint256`, `uint256[]`, or `(uint256,uint256)` for a struct. This is the
    /// form used for selector/signature hashing, **not** the JSON-ABI `"type"`
    /// field (structs there are `tuple`/`tuple[]`, which nothing renders yet), nor
    /// its `"internalType"` field, which is [`Self::internal_type`].
    pub fn type_name(&self) -> String {
        self.abi_type().to_string()
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

/// Which storage a layout describes: the persistent storage, or the transient
/// storage that is cleared at the end of each transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageKind {
    Persistent,
    Transient,
}

/// The state variables of one kind of storage, and the types they are laid
/// out with.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StorageLayout {
    items: Vec<StorageItem>,
    types: SortedMap<TypeId, StorageType>,
}

impl StorageLayout {
    pub(crate) fn new(items: Vec<StorageItem>, types: SortedMap<TypeId, StorageType>) -> Self {
        Self { items, types }
    }

    /// The state variables, in the order they are laid out.
    pub fn items(&self) -> &[StorageItem] {
        &self.items
    }

    /// Every type the items refer to, directly or through another type,
    /// ordered by [`TypeId`].
    pub fn types(&self) -> impl ExactSizeIterator<Item = &StorageType> {
        self.types.values()
    }

    /// The entry of [`Self::types`] for `type_id`, as named by a
    /// [`StorageItem`] or a [`StorageTypeKind`] of this layout.
    pub fn storage_type(&self, type_id: TypeId) -> Option<&StorageType> {
        self.types.get(&type_id)
    }
}

/// A state variable, and where it is stored.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageItem {
    node_id: NodeId,
    name: String,
    slot: U256,
    offset: usize,
    type_id: TypeId,
}

impl StorageItem {
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// The state variable's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn slot(&self) -> U256 {
        self.slot
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    /// The key of the item's type in [`StorageLayout::types`], which also
    /// holds its name.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}

/// How a type is laid out in storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageType {
    type_id: TypeId,
    label: String,
    size: StorageSize,
    kind: StorageTypeKind,
}

impl StorageType {
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// The type's name as a storage layout spells it, e.g. `struct C.S` or
    /// `mapping(address => uint256)`.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// How much storage the type occupies: whole slots, or a width in bytes
    /// for a value type that can share a slot with its neighbours.
    pub fn size(&self) -> StorageSize {
        self.size
    }

    pub fn kind(&self) -> &StorageTypeKind {
        &self.kind
    }
}

pub fn hash_from_signature(signature: &str) -> [u8; 32] {
    Keccak256::digest(signature).into()
}

pub fn selector_from_signature(signature: &str) -> u32 {
    let selector_bytes: [u8; 4] = hash_from_signature(signature)[0..4].try_into().unwrap();
    u32::from_be_bytes(selector_bytes)
}
