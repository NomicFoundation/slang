use std::collections::{BTreeMap, BTreeSet};

use language_v2_definition::model::Identifier;

use super::builder::StructuredCstModelBuilder;
use super::model::{Choice, Collection, Sequence};

/// A nonterminal, to check whether it has a property locally (see [`compute_has_descendant`]).
pub(super) enum Nonterminal<'a> {
    Sequence(&'a Sequence),
    Choice(&'a Choice),
    Collection(&'a Collection),
}

/// Computes whether each nonterminal has a descendant (transitively, including itself) with a
/// property that `is_local` checks on a single nonterminal, without looking at its children.
///
/// Uses DFS with memoization and a fixed-point correction pass to handle cycles.
pub(super) fn compute_has_descendant(
    builder: &StructuredCstModelBuilder,
    is_local: impl Fn(Nonterminal<'_>) -> bool,
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
        initial_dfs(builder, &is_local, nonterminal, &mut cache, &mut visiting);
    }

    // Phase 2: Fixed-point correction. The DFS may have cached `false` for nodes
    // whose cycle-ancestor later resolved to `true`. Re-evaluate until stable
    // (values only go false→true, so this always converges).
    correct_results(builder, &is_local, &all_nonterminals, &mut cache);

    cache
}

fn initial_dfs(
    builder: &StructuredCstModelBuilder,
    is_local: &impl Fn(Nonterminal<'_>) -> bool,
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

    let result = check(builder, is_local, nonterminal, |child| {
        initial_dfs(builder, is_local, child, cache, visiting)
    });

    visiting.remove(nonterminal);
    cache.insert(nonterminal.clone(), result);
    result
}

fn correct_results(
    builder: &StructuredCstModelBuilder,
    is_local: &impl Fn(Nonterminal<'_>) -> bool,
    all_nonterminals: &Vec<Identifier>,
    cache: &mut BTreeMap<Identifier, bool>,
) {
    loop {
        let mut changed = false;

        for nonterminal in all_nonterminals {
            if !cache[nonterminal] && check(builder, is_local, nonterminal, |child| cache[child]) {
                cache.insert(nonterminal.clone(), true);
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }
}

/// Checks whether `nonterminal` has the property locally, or any of its children has it (delegating
/// child lookups to `child_has`).
fn check(
    builder: &StructuredCstModelBuilder,
    is_local: &impl Fn(Nonterminal<'_>) -> bool,
    nonterminal: &Identifier,
    mut child_has: impl FnMut(&Identifier) -> bool,
) -> bool {
    let mut check_child = |child_id: &Identifier| -> bool {
        !builder.terminals.contains(child_id) && child_has(child_id)
    };

    if let Some(seq) = builder.sequences.get(nonterminal) {
        is_local(Nonterminal::Sequence(seq))
            || seq
                .fields
                .iter()
                .any(|f| check_child(f.field_type.as_identifier()))
    } else if let Some(choice) = builder.choices.get(nonterminal) {
        is_local(Nonterminal::Choice(choice))
            || choice
                .variants
                .iter()
                .any(|v| check_child(v.variant_type.as_identifier()))
    } else if let Some(coll) = builder.collections.get(nonterminal) {
        is_local(Nonterminal::Collection(coll)) || check_child(coll.item_type.as_identifier())
    } else {
        false
    }
}
