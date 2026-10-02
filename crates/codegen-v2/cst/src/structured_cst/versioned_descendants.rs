use std::collections::BTreeMap;

use language_v2_definition::model::{Identifier, VersionSpecifier};

use super::builder::StructuredCstModelBuilder;
use super::descendants::{Nonterminal, compute_has_descendant};

/// Computes whether each nonterminal has any version-gated descendants (transitively).
///
/// A node "has versioned descendants" if:
/// - It has a non-`Always` `enabled` specifier, or
/// - Any of its fields/variants have a non-`Always` `enabled`, or
/// - Any of its children (transitively) have versioned descendants.
pub(super) fn compute_has_versioned_descendant(
    builder: &StructuredCstModelBuilder,
) -> BTreeMap<Identifier, bool> {
    let is_versioned = |spec: &VersionSpecifier| !matches!(spec, VersionSpecifier::Always);

    compute_has_descendant(builder, |nonterminal| match nonterminal {
        Nonterminal::Sequence(seq) => {
            is_versioned(&seq.enabled) || seq.fields.iter().any(|f| is_versioned(&f.enabled))
        }
        Nonterminal::Choice(choice) => {
            is_versioned(&choice.enabled)
                || choice.variants.iter().any(|v| is_versioned(&v.enabled))
        }
        Nonterminal::Collection(coll) => is_versioned(&coll.enabled),
    })
}
