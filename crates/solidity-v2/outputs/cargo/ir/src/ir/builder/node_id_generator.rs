use std::iter::FusedIterator;

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

    /// Adds the per-kind counts of `other` into this histogram.
    pub fn absorb(&mut self, other: &NodeKindHistogram) {
        for (total, count) in self.counts.iter_mut().zip(other.counts.iter()) {
            *total += *count;
        }
    }
}

impl Default for NodeKindHistogram {
    fn default() -> Self {
        Self {
            counts: [0; NodeKind::COUNT],
        }
    }
}

/// The node-id space, partitioned into equally sized, disjoint groups.
///
/// Iterating yields one group at a time, as the [`NodeIdGenerator`] that
/// allocates within it.
#[derive(Default)]
pub struct NodeIdGroups {
    /// Using `u64` so that the very last group is still yielded before the
    /// group is exhausted.
    next_group: u64,
}

impl Iterator for NodeIdGroups {
    type Item = NodeIdGenerator;

    /// The generator for the next group, or `None` once the space is exhausted.
    fn next(&mut self) -> Option<NodeIdGenerator> {
        let group = u32::try_from(self.next_group).ok()?;
        self.next_group += 1;

        Some(NodeIdGenerator::for_group(group))
    }
}

impl FusedIterator for NodeIdGroups {}

/// A strictly monotonically increasing `NodeId` generator for one group's
/// range.
///
/// While allocating IDs it also accumulates a [`NodeKindHistogram`] of the
/// kinds it has been asked to allocate (see [`Self::histogram`]).
pub struct NodeIdGenerator {
    base: u64,
    next_id: u64,
    histogram: NodeKindHistogram,
}

impl NodeIdGenerator {
    /// Bit width of a group's id range.
    const GROUP_SHIFT: u8 = 32;

    /// Creates the generator for the group at index `group`.
    fn for_group(group: u32) -> Self {
        let base = u64::from(group) << Self::GROUP_SHIFT;

        Self {
            base,
            // Id 0 is never allocated.
            next_id: base + 1,
            histogram: NodeKindHistogram::default(),
        }
    }

    /// Returns the next `NodeId` in this generator's group range, greater than
    /// any it has previously returned, and records `kind` in the histogram.
    ///
    /// Ids of distinct groups never collide.
    pub fn next_id_of(&mut self, kind: NodeKind) -> NodeId {
        self.histogram.record(kind);
        let id = self.next_id;
        debug_assert!(
            id >> Self::GROUP_SHIFT == self.base >> Self::GROUP_SHIFT,
            "group exhausted its node-id range"
        );
        self.next_id += 1;
        id.into()
    }

    /// The total number of `NodeId`s allocated by this generator so far.
    pub fn allocated_count(&self) -> u64 {
        self.next_id - self.base - 1
    }

    /// The per-kind histogram of the nodes allocated so far.
    pub fn histogram(&self) -> &NodeKindHistogram {
        &self.histogram
    }
}
