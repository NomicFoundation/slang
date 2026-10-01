use std::collections::BTreeMap;

use language_v2_definition::model::Identifier;

use super::builder::StructuredCstModelBuilder;
use super::descendants::{Nonterminal, compute_has_descendant};

/// Computes whether each nonterminal has any documentable descendants (transitively), so that
/// attaching `NatSpec` comments only walks those.
///
/// A node "has documentable descendants" if:
/// - It is documentable itself, or
/// - Any of its children (transitively) have documentable descendants.
pub(super) fn compute_has_documentable_descendant(
    builder: &StructuredCstModelBuilder,
) -> BTreeMap<Identifier, bool> {
    compute_has_descendant(
        builder,
        |nonterminal| matches!(nonterminal, Nonterminal::Sequence(seq) if seq.is_documentable),
    )
}
