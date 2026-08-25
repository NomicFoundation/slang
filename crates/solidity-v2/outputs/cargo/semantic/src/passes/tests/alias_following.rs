//! The declarations [`Binder::follow_symbol_aliases`] yields when a resolution
//! names import aliases, and the order it yields them in.

use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Resolution};
use crate::passes::tests::{Analyse, Analysis};

/// Resolves `symbol` at the file scope of `file_name`.
fn resolve_at_file_scope(binder: &Binder, file_name: &str, symbol: &str) -> Resolution {
    let scope_id = binder
        .scope_id_for_file_id(&file_name.into())
        .unwrap_or_else(|| panic!("no file scope for {file_name}"));

    binder.resolve_in_scope(scope_id, symbol)
}

/// The single definition `file_name` declares under `symbol`.
fn definition_at_file_scope(binder: &Binder, file_name: &str, symbol: &str) -> NodeId {
    match resolve_at_file_scope(binder, file_name, symbol) {
        Resolution::Definition(definition_id) => definition_id,
        other => panic!("expected a single `{symbol}` in {file_name}, got {other:?}"),
    }
}

/// The same declaration can be reached twice within one ambiguous resolution:
/// directly, and again through an import alias naming it. Following the aliases
/// drops the repeat, keeping the *first* occurrence, so the surviving
/// declarations stay in the order they were first resolved in.
///
/// The two rules only differ when a distinct declaration sits between the two
/// occurrences: for `[C_b, C_d, alias -> C_b]`, keeping the first yields
/// `[C_b, C_d]` where keeping the last would yield `[C_d, C_b]`.
#[test]
fn test_follow_aliases_keeps_a_repeated_definition_in_its_first_position() {
    // `a.sol` sees `C` three times over: declared by `b.sol`, declared by
    // `d.sol`, and re-exported by `c.sol` as an alias of `b.sol`'s. The import
    // order puts `d.sol`'s declaration between the two that name `b.sol`'s.
    let analysis = Analysis::builder()
        .file(
            "a.sol",
            r#"
            import "b.sol";
            import "d.sol";
            import "c.sol";
            "#,
        )
        .file("b.sol", "contract C {}")
        .file("d.sol", "contract C {}")
        .file("c.sol", r#"import {C} from "b.sol";"#)
        .run(Analyse::Definitions);
    let binder = analysis.binder();

    let c_in_b = definition_at_file_scope(binder, "b.sol", "C");
    let c_in_d = definition_at_file_scope(binder, "d.sol", "C");

    let resolution = resolve_at_file_scope(binder, "a.sol", "C");
    let Resolution::Ambiguous(definition_ids) = &resolution else {
        panic!("expected `C` to resolve ambiguously in a.sol, got {resolution:?}");
    };
    assert_eq!(
        &definition_ids[..],
        &[
            c_in_b,
            c_in_d,
            definition_at_file_scope(binder, "c.sol", "C")
        ],
        "all three should be visible, with the alias re-exported by c.sol last"
    );

    assert_eq!(
        binder.follow_symbol_aliases(resolution),
        Resolution::Ambiguous([c_in_b, c_in_d].into()),
        "the repeated declaration should keep the position of its first occurrence"
    );
}

/// An alias whose target is not otherwise visible contributes that target in
/// the alias' own position, leaving the surrounding declarations in place.
#[test]
fn test_follow_aliases_substitutes_a_target_in_place() {
    let analysis = Analysis::builder()
        .file(
            "a.sol",
            r#"
            import "c.sol";
            import "d.sol";
            "#,
        )
        .file("b.sol", "contract C {}")
        .file("c.sol", r#"import {C} from "b.sol";"#)
        .file("d.sol", "contract C {}")
        .run(Analyse::Definitions);
    let binder = analysis.binder();

    let c_in_b = definition_at_file_scope(binder, "b.sol", "C");
    let c_in_d = definition_at_file_scope(binder, "d.sol", "C");

    assert_eq!(
        binder.follow_symbol_aliases(resolve_at_file_scope(binder, "a.sol", "C")),
        Resolution::Ambiguous([c_in_b, c_in_d].into()),
    );
}
