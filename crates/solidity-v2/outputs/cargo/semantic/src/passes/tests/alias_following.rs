//! The declarations [`Binder::follow_symbol_aliases`] yields when a resolution
//! names import aliases, and the order it yields them in.

use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;

use super::build_files;
use crate::binder::{Binder, Resolution};
use crate::passes::p1_collect_definitions;

/// Collects definitions over `sources` (see [`build_files`]), which is enough
/// to resolve file-scope symbols and to follow import aliases: `p1` computes
/// the default-import closures and the aliases' targets before it returns.
fn collect_definitions(sources: &[(&str, &str)]) -> Binder {
    let files = build_files(sources, LanguageVersion::LATEST);
    let mut binder = Binder::default();
    let mut diagnostics = DiagnosticCollection::default();

    p1_collect_definitions::run(
        &files,
        &mut binder,
        LanguageVersion::LATEST,
        &mut diagnostics,
    );

    binder
}

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
    let binder = collect_definitions(&[
        (
            "a.sol",
            r#"
import "b.sol";
import "d.sol";
import "c.sol";
            "#,
        ),
        ("b.sol", "contract C {}"),
        ("d.sol", "contract C {}"),
        ("c.sol", r#"import {C} from "b.sol";"#),
    ]);

    let c_in_b = definition_at_file_scope(&binder, "b.sol", "C");
    let c_in_d = definition_at_file_scope(&binder, "d.sol", "C");

    let resolution = resolve_at_file_scope(&binder, "a.sol", "C");
    let Resolution::Ambiguous(definition_ids) = &resolution else {
        panic!("expected `C` to resolve ambiguously in a.sol, got {resolution:?}");
    };
    assert_eq!(
        &definition_ids[..],
        &[
            c_in_b,
            c_in_d,
            definition_at_file_scope(&binder, "c.sol", "C")
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
    let binder = collect_definitions(&[
        (
            "a.sol",
            r#"
import "c.sol";
import "d.sol";
            "#,
        ),
        ("b.sol", "contract C {}"),
        ("c.sol", r#"import {C} from "b.sol";"#),
        ("d.sol", "contract C {}"),
    ]);

    let c_in_b = definition_at_file_scope(&binder, "b.sol", "C");
    let c_in_d = definition_at_file_scope(&binder, "d.sol", "C");

    assert_eq!(
        binder.follow_symbol_aliases(resolve_at_file_scope(&binder, "a.sol", "C")),
        Resolution::Ambiguous([c_in_b, c_in_d].into()),
    );
}
