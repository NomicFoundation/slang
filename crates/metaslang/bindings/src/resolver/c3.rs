use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Produces a linearisation of a hierarchy of items using the C3 linearisation
/// algorithm. Given an item A with parents (B1, B2) in that order, the
/// linearisation of A can be defined as:
/// L(A) = [A] + merge(L(B1), L(B2), [B1, B2])
/// The merge operation (defined below) will order the items of the vectors by
/// taking candidates that can only appear in the first position in all of the
/// vectors.
/// NOTE: parents are considered left-to-right as in the traditional C3
/// linearisation algorithm, ie. Python style.
pub(crate) fn linearise<Item: Clone + Debug + Display + Eq + Hash + PartialEq>(
    target: &Item,
    parents: &HashMap<Item, Vec<Item>>,
) -> Option<Vec<Item>> {
    let mut linearisations: HashMap<Item, Vec<Item>> = HashMap::new();

    // Keeps a running queue of pending linearisations.
    let mut queue: VecDeque<Item> = VecDeque::new();
    queue.push_back(target.clone());

    // When an item cannot be resolved, we save it here along the number of
    // items already linearised. If we fail to linearise a second time, we use
    // this to check if progress was made to avoid infinite loops with cycles.
    let mut checkpoint = None;

    while let Some(item) = queue.pop_front() {
        if linearisations.contains_key(&item) {
            continue;
        }
        let item_parents = &parents[&item];
        let mut merge_set = Vec::new();
        for parent in item_parents {
            if let Some(parent_linearisation) = linearisations.get(parent) {
                merge_set.push(parent_linearisation.clone());
            } else {
                // Queue the parent with missing linearisation at the front to
                // resolve it first (unless it's already queued)
                if !queue.iter().any(|queued| queued == parent) {
                    queue.push_front(parent.clone());
                }
            }
        }
        if merge_set.len() == item_parents.len() {
            merge_set.push(item_parents.clone());
            let Some(merge_result) = merge(merge_set) else {
                // Failed to linearise the current item; linearisation is not possible.
                eprintln!("Linearisation of {item} failed");
                return None;
            };

            let mut result = Vec::new();
            result.push(item.clone());
            result.extend(merge_result);

            // Clear checkpoint if we just linearised it.
            if matches!(checkpoint, Some((ref check_item, _)) if *check_item == item) {
                checkpoint = None;
            }

            linearisations.insert(item, result);
        } else {
            // We're missing linearisations of some parents, so re-enqueue the
            // current item at the end and try again later, after hopefully
            // recursively resolving the linearisation of the parents.
            if let Some((ref check_item, items_linearised)) = checkpoint {
                if *check_item == item {
                    if items_linearised == linearisations.len() {
                        // no progress since last checkpoint; this indicates a cycle
                        eprintln!("Linearisation of {item} failed: cycle detected");
                        return None;
                    }
                    // Update progress and re-try
                    checkpoint = Some((item.clone(), linearisations.len()));
                }
            } else {
                // Create a checkpoint on the item we couldn't yet linearise.
                checkpoint = Some((item.clone(), linearisations.len()));
            }
            queue.push_back(item);
        }
    }

    linearisations.remove(target)
}

/// Merges the items in the set in C3 linearisation order. Returns None if
/// linearisation is not possible.
/// NOTE: Because we're dealing with vectors, we will be taking candidates from
/// the tails with `pop()` as it's more convenient, we will reverse all the
/// inputs. Both the inputs and the result are returned in natural order.
fn merge<Item: Clone + Debug + PartialEq>(mut set: Vec<Vec<Item>>) -> Option<Vec<Item>> {
    set = set
        .into_iter()
        .filter_map(|mut subset| {
            subset.reverse();
            if subset.is_empty() {
                None
            } else {
                Some(subset)
            }
        })
        .collect();
    if set.is_empty() {
        // Nothing to merge; this happens when linearising an item without parents
        return Some(Vec::new());
    }

    let mut result = Vec::new();
    loop {
        let found = find_candidate(&set);
        if let Some(found) = found {
            set = remove_candidate_from_set(set, &found);
            result.push(found);
        } else {
            // No more candidates found
            break;
        }
    }
    // If set is empty, we successfully merged the set.
    // Otherwise, linearisation is not possible.
    if set.is_empty() {
        Some(result)
    } else {
        None
    }
}

// Find a candidate in a set. A valid candidate can only appear in the last
// position in any vector in the set.
fn find_candidate<Item: Clone + PartialEq>(set: &Vec<Vec<Item>>) -> Option<Item> {
    for subset in set {
        let Some(candidate) = subset.last() else {
            continue;
        };
        if set.iter().all(|subset| {
            if let Some(position) = subset.iter().position(|item| item == candidate) {
                position == subset.len() - 1
            } else {
                true
            }
        }) {
            return Some(candidate.clone());
        }
    }
    None
}

// Removes a candidate from the set. It should be a valid candidate, so it can
// only appear in the last position in each vector in the set. Removes empty
// vectors and returns the updated set.
fn remove_candidate_from_set<Item: PartialEq>(
    set: Vec<Vec<Item>>,
    element: &Item,
) -> Vec<Vec<Item>> {
    set.into_iter()
        .filter_map(|mut subset| {
            if subset.last().expect("every vector in the set is not empty") == element {
                subset.pop();
            }
            if subset.is_empty() {
                None
            } else {
                Some(subset)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_no_elements() {
        assert_eq!(Some(vec![]), merge::<()>(vec![vec![]]));
    }

    #[test]
    fn test_merge_single_parent() {
        assert_eq!(Some(vec!['A']), merge(vec![vec!['A'], vec!['A']]));
    }

    #[test]
    fn test_merge_two_parents() {
        assert_eq!(
            Some(vec!['A', 'B']),
            merge(vec![vec!['A'], vec!['B'], vec!['A', 'B']])
        );
    }

    #[test]
    fn test_merge_not_linearisable() {
        assert_eq!(None, merge(vec![vec!['A', 'X'], vec![], vec!['X', 'A']]));
    }

    #[test]
    fn test_linearise() {
        let mut parents = HashMap::new();
        parents.insert('A', vec![]);
        parents.insert('B', vec!['A']);
        parents.insert('C', vec!['A']);
        parents.insert('D', vec!['B', 'C']);
        parents.insert('E', vec!['C', 'B']);

        assert_eq!(Some(vec!['D', 'B', 'C', 'A']), linearise(&'D', &parents));
        assert_eq!(Some(vec!['E', 'C', 'B', 'A']), linearise(&'E', &parents));
    }

    #[test]
    fn test_linearise_not_linearisable() {
        let mut parents = HashMap::new();
        parents.insert('X', vec![]);
        parents.insert('A', vec!['X']);
        parents.insert('C', vec!['X', 'A']);

        assert_eq!(None, linearise(&'C', &parents));
    }

    #[test]
    fn test_linearise_with_shallow_cycles() {
        let mut parents = HashMap::new();
        parents.insert('B', vec!['A']);
        parents.insert('A', vec!['B']);

        assert_eq!(None, linearise(&'A', &parents));
        assert_eq!(None, linearise(&'B', &parents));
    }

    #[test]
    fn test_linearise_with_deep_cycles() {
        let mut parents = HashMap::new();
        parents.insert('X', vec!['Y']);
        parents.insert('Y', vec!['Z']);
        parents.insert('Z', vec!['X']);

        assert_eq!(None, linearise(&'X', &parents));
        assert_eq!(None, linearise(&'Y', &parents));
        assert_eq!(None, linearise(&'Z', &parents));
    }
}
