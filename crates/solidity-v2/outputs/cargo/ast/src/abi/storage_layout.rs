use ruint::aliases::U256;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::context::StorageTypeTable;
pub use slang_solidity_v2_semantic::context::{
    StorageMember, StoragePosition, StorageSize, StorageTypeKind,
};
use slang_solidity_v2_semantic::types::TypeId;

/// Which storage a layout describes: the persistent storage, or the transient
/// storage that is cleared at the end of each transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StorageKind {
    Persistent,
    Transient,
}

/// The state variables of one kind of storage, and the types they are laid
/// out with.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StorageLayout {
    items: Vec<StorageItem>,
    types: StorageTypeTable<StorageType>,
}

impl StorageLayout {
    pub(crate) fn new(items: Vec<StorageItem>, types: StorageTypeTable<StorageType>) -> Self {
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
    pub(super) node_id: NodeId,
    pub(super) name: String,
    pub(super) slot: U256,
    pub(super) offset: usize,
    pub(super) type_id: TypeId,
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
    pub(super) type_id: TypeId,
    pub(super) label: String,
    pub(super) size: StorageSize,
    pub(super) kind: StorageTypeKind,
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
