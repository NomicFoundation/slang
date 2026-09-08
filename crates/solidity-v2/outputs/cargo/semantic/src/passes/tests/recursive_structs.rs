//! The structs the cycle searches mark recursive.

use super::support::{Analyse, Analysis};
use crate::binder::Definition;
use crate::passes::p8_code_analysis::MAX_DEPTH;

/// The chain is legal: solc's `recursive` annotation walks it without the
/// depth limit its `CycleDetector` gives up at, and so does the search here.
#[test]
fn an_array_chain_past_the_depth_limit_is_recursive_throughout() {
    let source: String = std::iter::once("pragma solidity *;\n".to_owned())
        .chain((0..MAX_DEPTH).map(|index| format!("struct S{index} {{ S{}[] m; }}\n", index + 1)))
        .chain(std::iter::once(format!(
            "struct S{MAX_DEPTH} {{ S{MAX_DEPTH}[] m; }}\n"
        )))
        .collect();

    let context = Analysis::of_source(&source)
        .run(Analyse::Context)
        .expect_no_diagnostics()
        .into_context();

    let structs: Vec<bool> = context
        .all_definitions()
        .filter_map(|definition| match definition {
            Definition::Struct(structure) => Some(structure.is_recursive),
            _ => None,
        })
        .collect();
    assert_eq!(structs.len(), MAX_DEPTH + 1);
    assert!(structs.iter().all(|recursive| *recursive));
}
