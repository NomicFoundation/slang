use slang_solidity_v2_ir::ir::{NodeKind, NodeKindHistogram};

/// IR node kinds that map one-to-one to a binder `Definition` (each occurrence
/// in the source produces exactly one definition). Summing their histogram
/// counts sizes the `definitions` and `definitions_by_identifier` maps.
///
/// Definitions whose definiens is a shared node kind we can't attribute from
/// the histogram alone are deliberately omitted: `EnumMember`, `YulParameter`,
/// and `YulVariable` all key on `Identifier` (of which most occurrences are
/// *not* definitions). This only slightly undersizes the maps. Keep roughly in
/// sync with the `Definition` variants (`__SLANG_DEFINITION_TYPES__`).
const DEFINITION_KINDS: &[NodeKind] = &[
    NodeKind::ConstantDefinition,
    NodeKind::ContractDefinition,
    NodeKind::EnumDefinition,
    NodeKind::ErrorDefinition,
    NodeKind::EventDefinition,
    NodeKind::FunctionDefinition, // also covers modifiers
    NodeKind::ImportDeconstructionSymbol,
    NodeKind::InterfaceDefinition,
    NodeKind::LibraryDefinition,
    NodeKind::Parameter, // also covers type parameters
    NodeKind::PathImport,
    NodeKind::StateVariableDefinition,
    NodeKind::StructDefinition,
    NodeKind::StructMember,
    NodeKind::UserDefinedValueTypeDefinition,
    NodeKind::VariableDeclaration,
    NodeKind::YulFunctionDefinition,
];

fn definition_count(histogram: &NodeKindHistogram) -> usize {
    DEFINITION_KINDS
        .iter()
        .map(|kind| histogram.count(*kind) as usize)
        .sum()
}

/// IR node kinds that carry typing information, used to pre-size the
/// `node_typing` map. These are (roughly) the concrete variants of the
/// `Expression` IR choice — `ElementaryType`/`StringExpression` are nested
/// choices that own no `NodeId`, so their concrete children are counted under
/// their own kinds. It's only an estimate: `node_typing` is also set on some
/// declarations/parameters not listed here, so this can be tweaked to tighten
/// the bound. Under-sizing only costs an occasional grow.
const EXPRESSION_KINDS: &[NodeKind] = &[
    NodeKind::AssignmentExpression,
    NodeKind::ConditionalExpression,
    NodeKind::OrExpression,
    NodeKind::AndExpression,
    NodeKind::EqualityExpression,
    NodeKind::InequalityExpression,
    NodeKind::BitwiseOrExpression,
    NodeKind::BitwiseXorExpression,
    NodeKind::BitwiseAndExpression,
    NodeKind::ShiftExpression,
    NodeKind::AdditiveExpression,
    NodeKind::MultiplicativeExpression,
    NodeKind::ExponentiationExpression,
    NodeKind::PostfixExpression,
    NodeKind::PrefixExpression,
    NodeKind::FunctionCallExpression,
    NodeKind::CallOptionsExpression,
    NodeKind::MemberAccessExpression,
    NodeKind::IndexAccessExpression,
    NodeKind::NewExpression,
    NodeKind::TupleExpression,
    NodeKind::TypeExpression,
    NodeKind::ArrayExpression,
    NodeKind::HexNumberExpression,
    NodeKind::DecimalNumberExpression,
    NodeKind::PayableKeyword,
    NodeKind::ThisKeyword,
    NodeKind::SuperKeyword,
    NodeKind::TrueKeyword,
    NodeKind::FalseKeyword,
    NodeKind::Identifier,
];

fn expression_count(histogram: &NodeKindHistogram) -> usize {
    EXPRESSION_KINDS
        .iter()
        .map(|kind| histogram.count(*kind) as usize)
        .sum()
}

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
    /// Expected number of definitions (`definitions` and, keyed by their naming
    /// identifier, `definitions_by_identifier`).
    pub definitions: usize,
}

impl From<&NodeKindHistogram> for BinderCapacities {
    /// Derives capacities from the IR node-kind histogram: `node_typing` tracks
    /// (roughly) the expression nodes, `references` is keyed by the identifiers
    /// that resolve to a definition, and `definitions` counts the node kinds
    /// that introduce a definition.
    fn from(histogram: &NodeKindHistogram) -> Self {
        Self {
            node_typing: expression_count(histogram),
            // Every reference is an identifier, so this is a tight upper bound.
            references: histogram.count(NodeKind::Identifier) as usize,
            definitions: definition_count(histogram),
        }
    }
}
