use std::fmt::Debug;
use std::rc::Rc;

use enum_dispatch::enum_dispatch;
use serde::Serialize;

pub trait Kind: Sized + Copy + Clone + PartialEq + Eq + Debug + Serialize {}
impl<T> Kind for T where T: Sized + Copy + Clone + PartialEq + Eq + Debug + Serialize {}
pub trait Payload: Sized + Clone + Debug {}
impl<T> Payload for T where T: Sized + Clone + Debug {}

pub trait TreeModel: Clone + PartialEq {
    type InternalNodeKind: Kind;
    type InternalNodePayload: Payload;
    type LeafNodeKind: Kind;
    type LeafNodePayload: Payload;
    type EdgeKind: Kind;
    type EdgePayload: Payload;

    fn new_internal_node(
        kind: Self::InternalNodeKind,
        edges: Vec<Edge<Self>>,
        payload: Self::InternalNodePayload,
    ) -> Node<Self> {
        Rc::new(InternalNode::<Self> {
            kind,
            edges,
            payload,
        })
        .into()
    }

    fn new_leaf_node(kind: Self::LeafNodeKind, payload: Self::LeafNodePayload) -> Node<Self> {
        Rc::new(LeafNode::<Self> { kind, payload }).into()
    }
}

#[enum_dispatch]
pub trait AbstractNode<T: TreeModel>: Sized {
    /// Returns a slice of the children (not all descendants) of this node.
    fn edges(&self) -> &[Edge<T>] {
        &[]
    }

    // /// Creates a [`Cursor`] that starts at the current node as the root and a given initial `text_offset`.
    // pub fn cursor_with_offset(&self, text_offset: TextIndex) -> Cursor<T> {
    //     Cursor::<T>::new(self.clone(), text_offset)
    // }

    fn into_internal_node(self) -> Option<Rc<InternalNode<T>>> {
        None
    }

    fn is_internal_node(&self) -> bool {
        self.as_internal_node().is_some()
    }

    fn as_internal_node(&self) -> Option<&Rc<InternalNode<T>>> {
        None
    }

    fn is_internal_node_with_kind(&self, kind: T::InternalNodeKind) -> bool {
        self.as_internal_node_with_kind(kind).is_some()
    }

    fn as_internal_node_with_kind(
        &self,
        kind: T::InternalNodeKind,
    ) -> Option<&Rc<InternalNode<T>>> {
        self.as_internal_node()
            .filter(|internal_node| internal_node.kind == kind)
    }

    fn is_internal_node_with_kinds(&self, kinds: &[T::InternalNodeKind]) -> bool {
        self.as_internal_node_with_kinds(kinds).is_some()
    }

    fn as_internal_node_with_kinds(
        &self,
        kinds: &[T::InternalNodeKind],
    ) -> Option<&Rc<InternalNode<T>>> {
        self.as_internal_node()
            .filter(|internal_node| kinds.contains(&internal_node.kind))
    }

    fn into_leaf_node(self) -> Option<Rc<LeafNode<T>>> {
        None
    }

    fn is_leaf_node(&self) -> bool {
        self.as_leaf_node().is_some()
    }

    fn as_leaf_node(&self) -> Option<&Rc<LeafNode<T>>> {
        None
    }

    fn is_leaf_node_with_kind(&self, kind: T::LeafNodeKind) -> bool {
        self.as_leaf_node_with_kind(kind).is_some()
    }

    fn as_leaf_node_with_kind(&self, kind: T::LeafNodeKind) -> Option<&Rc<LeafNode<T>>> {
        self.as_leaf_node()
            .filter(|leaf_node| leaf_node.kind == kind)
    }

    fn is_leaf_node_with_kinds(&self, kinds: &[T::LeafNodeKind]) -> bool {
        self.as_leaf_node_with_kinds(kinds).is_some()
    }

    fn as_leaf_node_with_kinds(&self, kinds: &[T::LeafNodeKind]) -> Option<&Rc<LeafNode<T>>> {
        self.as_leaf_node()
            .filter(|leaf_node| kinds.contains(&leaf_node.kind))
    }
}

#[derive(Clone, Debug)]
#[enum_dispatch(AbstractNode<T>)]
pub enum Node<T: TreeModel> {
    Internal(Rc<InternalNode<T>>),
    Leaf(Rc<LeafNode<T>>),
}

#[derive(Clone, Debug)]
pub struct InternalNode<T: TreeModel> {
    pub kind: T::InternalNodeKind,
    pub edges: Vec<Edge<T>>,
    pub payload: T::InternalNodePayload,
}

#[derive(Clone, Debug)]
pub struct LeafNode<T: TreeModel> {
    pub kind: T::LeafNodeKind,
    pub payload: T::LeafNodePayload,
}

#[derive(Clone, Debug)]
pub struct Edge<T: TreeModel> {
    pub kind: T::EdgeKind,
    pub target: Node<T>,
    pub payload: T::EdgePayload,
}

impl<T: TreeModel> AbstractNode<T> for Rc<InternalNode<T>> {
    fn edges(&self) -> &[Edge<T>] {
        &self.edges
    }

    fn into_internal_node(self) -> Option<Rc<InternalNode<T>>> {
        Some(self)
    }

    fn as_internal_node(&self) -> Option<&Rc<InternalNode<T>>> {
        Some(self)
    }
}

impl<T: TreeModel> AbstractNode<T> for Rc<LeafNode<T>> {
    fn into_leaf_node(self) -> Option<Rc<LeafNode<T>>> {
        Some(self)
    }

    fn as_leaf_node(&self) -> Option<&Rc<LeafNode<T>>> {
        Some(self)
    }
}

mod test {
    #![allow(dead_code)]

    use super::{AbstractNode, TreeModel};
    use crate::text_index::TextIndex;

    #[derive(Clone, PartialEq)]
    struct CSTTreeModel;
    impl TreeModel for CSTTreeModel {
        type InternalNodeKind = u32;
        type InternalNodePayload = TextIndex;
        type LeafNodeKind = u32;
        type LeafNodePayload = String;
        type EdgeKind = u32;
        type EdgePayload = ();
    }

    type CSTNode = super::Node<CSTTreeModel>;
    type CSTLeafNode = super::LeafNode<CSTTreeModel>;
    type CSTInternalNode = super::InternalNode<CSTTreeModel>;
    type CSTEdge = super::Edge<CSTTreeModel>;

    impl CSTTreeModel {
        fn internal_node(kind: u32, edges: Vec<CSTEdge>) -> CSTNode {
            let mut payload = TextIndex::ZERO;
            for edge in &edges {
                payload += edge.target.text_len();
            }
            Self::new_internal_node(kind, edges, payload)
        }
        fn leaf_node(kind: u32, payload: String) -> CSTNode {
            Self::new_leaf_node(kind, payload)
        }
    }

    impl CSTNode {
        fn text_len(&self) -> TextIndex {
            match self {
                Self::Internal(internal_node) => internal_node.payload,
                Self::Leaf(leaf_node) => TextIndex::from(&leaf_node.payload),
            }
        }
    }

    fn main() {
        let node = CSTTreeModel::internal_node(0, vec![]);
        node.is_internal_node();
    }
}
