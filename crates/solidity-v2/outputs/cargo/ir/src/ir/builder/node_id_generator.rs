use crate::ir::nodes::{NodeId, NodeKind};

/// A per-kind tally of the nodes allocated by a [`NodeIdGenerator`].
///
/// Backed by a fixed-size array indexed by the [`NodeKind`] discriminant, so
/// recording a node is a single array increment. It is populated for free as
/// the IR is built and lets downstream consumers (e.g. the binder) pre-size
/// their per-node collections instead of growing and rehashing them.
#[derive(Clone, Debug)]
pub struct NodeKindHistogram {
    counts: [u32; NodeKind::COUNT],
}

impl NodeKindHistogram {
    fn record(&mut self, kind: NodeKind) {
        self.counts[kind as usize] += 1;
    }

    /// The number of nodes of the given `kind` allocated so far.
    pub fn count(&self, kind: NodeKind) -> u32 {
        self.counts[kind as usize]
    }

    /// The total number of nodes recorded across all kinds.
    pub fn total(&self) -> u32 {
        self.counts.iter().sum()
    }

    /// The number of `Identifier` nodes. Every reference is an identifier, so
    /// this is a tight upper bound on the size of the binder's `references` map.
    pub fn identifier_count(&self) -> usize {
        self.count(NodeKind::Identifier) as usize
    }

    /// The number of expression nodes (those whose kind is in
    /// `NodeKind::EXPRESSION_KINDS`). Used to estimate the size of the binder's
    /// `node_typing` map.
    pub fn expression_count(&self) -> usize {
        NodeKind::EXPRESSION_KINDS
            .iter()
            .map(|kind| self.count(*kind) as usize)
            .sum()
    }
}

impl Default for NodeKindHistogram {
    fn default() -> Self {
        Self {
            counts: [0; NodeKind::COUNT],
        }
    }
}

/// A strictly monotonically increasing `NodeId` generator.
///
/// While allocating IDs it also accumulates a [`NodeKindHistogram`] of the
/// kinds it has been asked to allocate (see [`Self::histogram`]).
pub struct NodeIdGenerator {
    next_id: usize,
    histogram: NodeKindHistogram,
}

impl NodeIdGenerator {
    /// Returns a `NodeId` greater than any previously returned by this
    /// generator.
    /// The returned ID is unique and suitable for use as a total-order key.
    pub fn next_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        id.into()
    }

    /// Like [`Self::next_id`], but also records `kind` in the histogram.
    pub(super) fn next_id_of(&mut self, kind: NodeKind) -> NodeId {
        self.histogram.record(kind);
        self.next_id()
    }

    /// The total number of `NodeId`s allocated by this generator so far.
    pub fn allocated_count(&self) -> usize {
        self.next_id.saturating_sub(1)
    }

    /// The per-kind histogram of the nodes allocated so far.
    pub fn histogram(&self) -> &NodeKindHistogram {
        &self.histogram
    }
}

impl Default for NodeIdGenerator {
    fn default() -> Self {
        Self {
            next_id: 1usize,
            histogram: NodeKindHistogram::default(),
        }
    }
}
