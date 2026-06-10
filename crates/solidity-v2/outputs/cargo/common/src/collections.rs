//! Canonical collection types for Slang v2.
//!
//! All maps and sets in the v2 crates should be picked from this module,
//! based on the guarantees each use case needs:
//!
//! | Alias                           | Backed by               | Iteration order | Reach for it when…                                  |
//! |---------------------------------|-------------------------|-----------------|-----------------------------------------------------|
//! | [`Map`] / [`Set`]               | `HashMap` / `HashSet`   | unspecified     | order is never observed (lookups, seen-sets)        |
//! | [`OrderedMap`] / [`OrderedSet`] | `IndexMap` / `IndexSet` | insertion order | order is observable (API results, snapshots), or entries need stable indices (interning) |
//! | [`SortedMap`] / [`SortedSet`]   | `BTreeMap` / `BTreeSet` | sorted by key   | output should be sorted by key, or you need range queries |
//!
//! The hash-based aliases use the same `SipHash` algorithm as the `std`
//! default, but with a fixed seed instead of a random per-process one, so
//! hashing is fully deterministic from run to run (required by the
//! instruction-counting benchmarks).
//!
//! The fixed seed forfeits `std`'s `HashDoS` protection; that trade-off is fine for
//! certain compiler workloads (like `NodeId`s), but may be unsafe for external keys
//! (like string interning). If this becomes a problem we should consider another Map/Set
//! pair that guarantees `HashDoS` protection.
//!
//! Even with a deterministic hasher, [`Map`]/[`Set`] iteration order is
//! arbitrary and shifts whenever the key set does — never let it reach
//! user-visible output. Use [`OrderedMap`] or [`SortedMap`] there.
//!
//! Use of the underlying types directly is rejected by the
//! `clippy::disallowed_types` lint, configured in `crates/solidity-v2/clippy.toml`.
//!
//! NOTE: construct the hash-based aliases with `::default()`; `::new()` is
//! not available on collections with a non-`std` hasher.

// The only place in the v2 crates allowed to name the underlying types:
#[allow(clippy::disallowed_types)]
mod aliases {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasherDefault;

    /// `std`'s default `SipHash`, seeded with a fixed key instead of
    /// `RandomState`'s random per-process one.
    pub type DeterministicState = BuildHasherDefault<DefaultHasher>;

    pub type Map<K, V> = std::collections::HashMap<K, V, DeterministicState>;

    pub type Set<T> = std::collections::HashSet<T, DeterministicState>;

    pub type OrderedMap<K, V> = indexmap::IndexMap<K, V, DeterministicState>;

    pub type OrderedSet<T> = indexmap::IndexSet<T, DeterministicState>;

    pub type SortedMap<K, V> = std::collections::BTreeMap<K, V>;

    pub type SortedSet<T> = std::collections::BTreeSet<T>;
}

pub use aliases::*;
