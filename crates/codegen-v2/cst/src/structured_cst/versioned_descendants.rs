use std::collections::{BTreeMap, BTreeSet};

use language_v2_definition::model::{Identifier, VersionSpecifier};

use super::builder::StructuredCstModelBuilder;

/// Computes whether each nonterminal has any version-gated descendants (transitively).
///
/// A node "has versioned descendants" if:
/// - It has a non-`Always` `enabled` specifier, or
/// - Any of its fields/variants have a non-`Always` `enabled`, or
/// - Any of its children (transitively) have versioned descendants.
///
/// Uses DFS with memoization and a fixed-point correction pass to handle cycles.
pub(super) fn compute_has_versioned_descendant(
    builder: &StructuredCstModelBuilder,
) -> BTreeMap<Identifier, bool> {
    let all_nonterminals: Vec<_> = builder
        .sequences
        .keys()
        .chain(builder.choices.keys())
        .chain(builder.collections.keys())
        .cloned()
        .collect();

    // Phase 1: DFS with cycle-breaking (conservatively returns `false` for back-edges).
    let mut cache = BTreeMap::new();
    let mut visiting = BTreeSet::new();

    for nonterminal in &all_nonterminals {
        initial_dfs(builder, nonterminal, &mut cache, &mut visiting);
    }

    // Phase 2: Fixed-point correction. The DFS may have cached `false` for nodes
    // whose cycle-ancestor later resolved to `true`. Re-evaluate until stable
    // (values only go false→true, so this always converges).
    correct_results(builder, &all_nonterminals, &mut cache);

    cache
}

fn initial_dfs(
    builder: &StructuredCstModelBuilder,
    nonterminal: &Identifier,
    cache: &mut BTreeMap<Identifier, bool>,
    visiting: &mut BTreeSet<Identifier>,
) -> bool {
    if let Some(&value) = cache.get(nonterminal) {
        return value;
    }

    if !visiting.insert(nonterminal.clone()) {
        // Back-edge in a cycle: conservatively assume false (corrected in phase 2).
        return false;
    }

    let result = check_local(builder, nonterminal, |child| {
        if builder.terminals.contains(child) {
            false
        } else {
            initial_dfs(builder, child, cache, visiting)
        }
    });

    visiting.remove(nonterminal);
    cache.insert(nonterminal.clone(), result);
    result
}

fn correct_results(
    builder: &StructuredCstModelBuilder,
    all_nonterminals: &Vec<Identifier>,
    cache: &mut BTreeMap<Identifier, bool>,
) {
    loop {
        let mut changed = false;

        for nonterminal in all_nonterminals {
            if !cache[nonterminal] && check_local(builder, nonterminal, |child| cache[child]) {
                cache.insert(nonterminal.clone(), true);
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }
}

/// Checks whether `nonterminal` has versioned descendants by inspecting its own `enabled`
/// specifiers and delegating child lookups to `child_has_versioned`.
fn check_local(
    builder: &StructuredCstModelBuilder,
    nonterminal: &Identifier,
    mut child_has_versioned: impl FnMut(&Identifier) -> bool,
) -> bool {
    let is_versioned = |spec: &VersionSpecifier| !matches!(spec, VersionSpecifier::Always);

    let mut check_child = |child_id: &Identifier| -> bool {
        !builder.terminals.contains(child_id) && child_has_versioned(child_id)
    };

    if let Some(seq) = builder.sequences.get(nonterminal) {
        is_versioned(&seq.enabled)
            || seq
                .fields
                .iter()
                .any(|f| is_versioned(&f.enabled) || check_child(f.field_type.as_identifier()))
    } else if let Some(choice) = builder.choices.get(nonterminal) {
        is_versioned(&choice.enabled)
            || choice
                .variants
                .iter()
                .any(|v| is_versioned(&v.enabled) || check_child(v.variant_type.as_identifier()))
    } else if let Some(coll) = builder.collections.get(nonterminal) {
        is_versioned(&coll.enabled) || check_child(coll.item_type.as_identifier())
    } else {
        false
    }
}
