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
//! [`Map`]/[`Set`] iteration order is arbitrary and shifts whenever the key
//! set does.
//!
//! Use of the underlying types directly is rejected by the
//! `clippy::disallowed_types` lint, configured in `crates/solidity-v2/clippy.toml`.
//!
//! NOTE: prefer constructing the hash-based aliases with `::default()` rather
//! than `::new()`, so that the backing hasher can be swapped without touching
//! call sites.

// The only place in the v2 crates allowed to name the underlying types:
#[allow(clippy::disallowed_types)]
mod aliases {
    pub type Map<K, V> = std::collections::HashMap<K, V>;

    pub type Set<T> = std::collections::HashSet<T>;

    pub type OrderedMap<K, V> = indexmap::IndexMap<K, V>;

    pub type OrderedSet<T> = indexmap::IndexSet<T>;

    pub type SortedMap<K, V> = std::collections::BTreeMap<K, V>;

    pub type SortedSet<T> = std::collections::BTreeSet<T>;
}

pub use aliases::*;
