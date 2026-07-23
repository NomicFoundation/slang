//! Up-front capacities for the binder's `NodeId`-keyed collections, derived
//! from the IR node-kind histogram (see `Binder::with_capacity`).
//!
//! # Bucket sizing: why estimates only need to be *roughly* right
//!
//! The hash maps (`Map`) round capacity up to a power-of-two bucket array and
//! grow once they pass 7/8 load. So the only thing that actually matters is
//! *which power of two* an estimate lands in, not its exact value — and the two
//! directions of error are very much not symmetric:
//!
//! - **Over-estimating is the expensive mistake.** An estimate that pushes even
//!   slightly past a 7/8 boundary allocates the *next* power of two — up to ~2x
//!   the memory the map ever needs — and that doubled bucket array stays live
//!   for the whole analysis, inflating peak memory.
//! - **Under-estimating is cheap.** If the estimate is a little low the map
//!   grows once during fill and ends at exactly the size it would have reached
//!   growing from empty: one rehash, no extra peak.
//!
//! We therefore deliberately bias the map estimates *low*. Landing in the
//! natural bucket (or rehashing once into it) is strictly better than
//! overshooting into the next one. The `scopes` `Vec` is different: `Vec`
//! allocates exactly the requested capacity with no power-of-two rounding, so
//! there an exact (or slightly high) count is fine and never wastes a doubling.
//!
//! # The "minus definitions" heuristic
//!
//! `node_typing` and `references` are both keyed off identifier-ish nodes, and
//! `Identifier` is by far the most common node kind. But a large fraction of
//! identifiers are *definition names* (the `x` in `uint x;`): those never get a
//! `node_typing` entry and never resolve to a `references` entry. Counting all
//! identifiers therefore overshoots both maps — frequently past a bucket
//! boundary, which is exactly the doubling we want to avoid.
//!
//! Almost every identifier is either a definition name or a reference, so
//! `identifiers - definitions ≈ references`; and the same subtraction removes
//! those untyped definition-name identifiers from the `node_typing` estimate.
//! We use [`DEFINITION_KINDS`] as the proxy for
//! "definition-name identifiers". It slightly *overcounts* definitions (see its
//! docs), so the subtraction errs toward removing a few too many — which is the
//! safe (under-estimating) direction described above.

use slang_solidity_v2_ir::ir::{NodeKind, NodeKindHistogram};

/// Total number of nodes across `kinds`, per the histogram.
fn count_kinds(kinds: &[NodeKind], histogram: &NodeKindHistogram) -> usize {
    kinds
        .iter()
        .map(|kind| histogram.count(*kind) as usize)
        .sum()
}

/// IR node kinds that carry typing information, summed to seed the `node_typing`
/// estimate. These are (roughly) the concrete variants of the `Expression` IR
/// choice — `ElementaryType`/`StringExpression` are nested choices that own no
/// `NodeId`, so their concrete children are counted under their own kinds.
///
/// This is an over-approximation of the typed-node set in one direction and an
/// under-approximation in another, both small: `node_typing` is *also* set on a
/// few non-expression nodes (parameters, declarations, type names) that aren't
/// listed here, while many of the `Identifier` nodes counted here are
/// definition names that are never typed. The "minus definitions" correction in
/// [`BinderCapacities::from`] removes the latter; see the module docs.
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
/// counts sizes `definitions`/`definitions_by_identifier`, and also drives the
/// "minus definitions" correction for `node_typing`/`references`.
///
/// Expected accuracy: a slight **over**-count. `Parameter` also counts mapping
/// key/value types and other unnamed parameters that don't become definitions,
/// and `FunctionDefinition` covers modifiers. Pulling the other way, definitions
/// whose definiens is a shared node kind are omitted because the histogram can't
/// attribute them — `EnumMember`, `YulParameter`, and `YulVariable` all key on
/// `Identifier`, most of whose occurrences are *not* definitions. The net is a
/// small overshoot, which lands `definitions` on the natural bucket in practice
/// and makes the subtraction above conservative. Keep roughly in sync with the
/// `Definition` variants (`__SLANG_DEFINITION_TYPES__`).
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
/// only over-reserves the `scopes` vec, which is harmless (see module docs).
const PARAMETER_SCOPE_NODE_KINDS: &[NodeKind] = &[
    NodeKind::FunctionDefinition,
    NodeKind::ErrorDefinition,
    NodeKind::EventDefinition,
];

/// IR node kinds that get linearised (one `linearisations` entry each).
/// Libraries are not inheritable and so are not linearised.
const LINEARISATION_KINDS: &[NodeKind] =
    &[NodeKind::ContractDefinition, NodeKind::InterfaceDefinition];

/// Comparison operators (one `common_operand_typing` entry each). Their
/// operands reconcile to a common type that is recorded apart from the
/// operator's boolean result — the only binary operators where the two differ,
/// so the only ones needing an entry (see `BinaryOperatorExpression`).
const COMPARISON_KINDS: &[NodeKind] =
    &[NodeKind::EqualityExpression, NodeKind::InequalityExpression];

/// Up-front sizes for the binder's dominant per-node collections, derived from
/// the IR node-kind histogram (see `Binder::with_capacity`). These are keyed
/// by (or indexed alongside) `NodeId` and end up roughly as large as the source,
/// so reserving them once avoids the repeated grow/rehash churn of filling them
/// from empty. See the module docs for the bucket-sizing model that motivates
/// the bias toward under-estimating.
#[derive(Default, Clone, Copy)]
pub(crate) struct BinderCapacities {
    /// Typed nodes (`node_typing`). Estimate: expression nodes minus definition
    /// names. Tends to **under**-shoot slightly (a few non-expression nodes are
    /// also typed, and the subtraction is conservative) → at worst one rehash.
    pub node_typing: usize,
    /// Reconciled comparison operand types (`common_operand_typing`). One entry
    /// per comparison expression. **Exact.**
    pub common_operand_typing: usize,
    /// Reference identifiers (`references`). Estimate: identifiers minus
    /// definition names. Tends to **under**-shoot slightly → at worst one
    /// rehash.
    pub references: usize,
    /// Definitions (`definitions`, and `definitions_by_identifier` keyed by the
    /// naming identifier). Estimate: [`DEFINITION_KINDS`], a slight **over**-count.
    pub definitions: usize,
    /// Scopes keyed by a `NodeId` (`scopes_by_node_id`). **Exact.**
    pub scopes_by_node_id: usize,
    /// Total scopes including the no-`NodeId` parameter scopes (the `scopes`
    /// vec). Slight **over**-count (modifier parameter scopes); harmless for a
    /// `Vec`, which reserves exactly.
    pub scopes: usize,
    /// Linearised contracts/interfaces (`linearisations`). **Exact.**
    pub linearisations: usize,
    /// `assembly` statements (`assembly_blocks`). **Exact.**
    pub assembly_blocks: usize,
}

impl From<&NodeKindHistogram> for BinderCapacities {
    /// Derives capacities from the IR node-kind histogram. The scope,
    /// linearisation, and assembly counts are exact (fixed node→collection
    /// mappings); `node_typing` and `references` apply the "minus definitions"
    /// heuristic documented at the module level. All map estimates are biased to
    /// under-shoot rather than overshoot a power-of-two bucket boundary.
    fn from(histogram: &NodeKindHistogram) -> Self {
        let identifiers = histogram.count(NodeKind::Identifier) as usize;
        let definitions = count_kinds(DEFINITION_KINDS, histogram);
        let scopes_by_node_id = count_kinds(SCOPE_NODE_KINDS, histogram);

        Self {
            // Expression nodes, minus the definition-name identifiers counted
            // under `Identifier` that are never typed.
            node_typing: count_kinds(EXPRESSION_KINDS, histogram).saturating_sub(definitions),
            common_operand_typing: count_kinds(COMPARISON_KINDS, histogram),
            // Identifiers that aren't definition names ≈ resolved references.
            references: identifiers.saturating_sub(definitions),
            definitions,
            scopes_by_node_id,
            scopes: scopes_by_node_id + count_kinds(PARAMETER_SCOPE_NODE_KINDS, histogram),
            linearisations: count_kinds(LINEARISATION_KINDS, histogram),
            assembly_blocks: histogram.count(NodeKind::AssemblyStatement) as usize,
        }
    }
}
