use slang_solidity_v2_ir::ir::NodeKindHistogram;

/// Up-front sizes for the binder's dominant per-node maps, derived from the IR
/// node-kind histogram (see `Binder::with_capacity`). These maps are keyed by
/// `NodeId` and end up roughly as large as the source, so reserving them once
/// avoids the repeated grow/rehash churn of filling them from empty.
#[derive(Default, Clone, Copy)]
pub(crate) struct BinderCapacities {
    /// Expected number of typed nodes (`node_typing`).
    pub node_typing: usize,
    /// Expected number of reference identifiers (`references`).
    pub references: usize,
}

impl From<&NodeKindHistogram> for BinderCapacities {
    /// Derives capacities from the IR node-kind histogram: `node_typing` tracks
    /// (roughly) the expression nodes, and `references` is keyed by the
    /// identifiers that resolve to a definition.
    fn from(histogram: &NodeKindHistogram) -> Self {
        Self {
            node_typing: histogram.expression_count(),
            references: histogram.identifier_count(),
        }
    }
}
