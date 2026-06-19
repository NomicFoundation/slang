use slang_solidity_v2_ir::ir::{NodeKind, NodeKindHistogram};

/// Total number of nodes across `kinds`, per the histogram.
fn count_kinds(kinds: &[NodeKind], histogram: &NodeKindHistogram) -> usize {
    kinds
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

/// IR node kinds that each open exactly one scope keyed by their own `NodeId`,
/// i.e. every entry in `scopes_by_node_id`. The mapping is fixed — these nodes
/// always open a scope — so the sum is the exact count. `FunctionDefinition`
/// opens either a function or a modifier scope (a modifier lowers to a
/// `FunctionDefinition`); `Block`/`ForStatement` both open block scopes; the
/// three contract-like kinds all open contract scopes.
const SCOPE_NODE_KINDS: &[NodeKind] = &[
    NodeKind::SourceUnit,                   // file scope
    NodeKind::ContractDefinition,           // contract scope
    NodeKind::InterfaceDefinition,          // contract scope
    NodeKind::LibraryDefinition,            // contract scope
    NodeKind::FunctionDefinition,           // function or modifier scope
    NodeKind::EnumDefinition,               // enum scope
    NodeKind::StructDefinition,             // struct scope
    NodeKind::VariableDeclarationStatement, // chained scope
    NodeKind::Block,                        // block scope
    NodeKind::ForStatement,                 // block scope (init clause)
    NodeKind::UsingDirective,               // using scope
    NodeKind::YulBlock,                     // yul block scope
    NodeKind::YulFunctionDefinition,        // yul function scope
];

/// IR node kinds that additionally open a *parameter* scope. These scopes have
/// no `NodeId`, so they only live in the `scopes` vec, not `scopes_by_node_id`.
/// `FunctionDefinition` overcounts by the number of modifiers (which collect
/// their parameters into the modifier scope rather than a dedicated one); that
/// only over-reserves the `scopes` vec, which is harmless.
const PARAMETER_SCOPE_NODE_KINDS: &[NodeKind] = &[
    NodeKind::FunctionDefinition,
    NodeKind::ErrorDefinition,
    NodeKind::EventDefinition,
];

/// IR node kinds that get linearised (one `linearisations` entry each).
/// Libraries are not inheritable and so are not linearised.
const LINEARISATION_KINDS: &[NodeKind] =
    &[NodeKind::ContractDefinition, NodeKind::InterfaceDefinition];

/// Up-front sizes for the binder's dominant per-node collections, derived from
/// the IR node-kind histogram (see `Binder::with_capacity`). These are keyed by
/// (or indexed alongside) `NodeId` and end up roughly as large as the source,
/// so reserving them once avoids the repeated grow/rehash churn of filling them
/// from empty.
#[derive(Default, Clone, Copy)]
pub(crate) struct BinderCapacities {
    /// Expected number of typed nodes (`node_typing`).
    pub node_typing: usize,
    /// Expected number of reference identifiers (`references`).
    pub references: usize,
    /// Expected number of definitions (`definitions` and, keyed by their naming
    /// identifier, `definitions_by_identifier`).
    pub definitions: usize,
    /// Exact number of scopes that are keyed by a `NodeId` (`scopes_by_node_id`).
    pub scopes_by_node_id: usize,
    /// Total number of scopes, including the parameter scopes that have no
    /// `NodeId` (the `scopes` vec).
    pub scopes: usize,
    /// Exact number of linearised contracts/interfaces (`linearisations`).
    pub linearisations: usize,
    /// Exact number of `assembly` statements (`assembly_blocks`).
    pub assembly_blocks: usize,
}

impl From<&NodeKindHistogram> for BinderCapacities {
    /// Derives capacities from the IR node-kind histogram: `node_typing` tracks
    /// (roughly) the expression nodes, `references` is keyed by the identifiers
    /// that resolve to a definition, `definitions` counts the node kinds that
    /// introduce a definition, and the scope counts come from the fixed set of
    /// scope-opening node kinds.
    fn from(histogram: &NodeKindHistogram) -> Self {
        let scopes_by_node_id = count_kinds(SCOPE_NODE_KINDS, histogram);
        Self {
            node_typing: count_kinds(EXPRESSION_KINDS, histogram),
            // Every reference is an identifier, so this is a tight upper bound.
            references: histogram.count(NodeKind::Identifier) as usize,
            definitions: count_kinds(DEFINITION_KINDS, histogram),
            scopes_by_node_id,
            scopes: scopes_by_node_id + count_kinds(PARAMETER_SCOPE_NODE_KINDS, histogram),
            linearisations: count_kinds(LINEARISATION_KINDS, histogram),
            assembly_blocks: histogram.count(NodeKind::AssemblyStatement) as usize,
        }
    }
}
