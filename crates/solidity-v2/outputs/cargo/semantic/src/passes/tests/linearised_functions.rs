//! Tests for the function list `linearised_functions` computes for a
//! contract's hierarchy, and which inherited functions a more-derived one
//! drops from it.

use slang_solidity_v2_common::versions::LanguageVersion;

use super::support::{Analyse, Analysis, find_function};

#[test]
fn location_change_on_internal_override_replaces_the_base_function() {
    // Valid before 0.8.14. `D.f` takes `B.f`'s slot even though the parameter
    // location differs, so D's list holds D's function alone.
    let analysis = Analysis::of_source(
        r#"
        pragma solidity *;
        contract B {
            function f(uint256[] memory a) internal virtual returns (uint256) {
                return a.length;
            }
        }
        contract D is B {
            function f(uint256[] calldata a) internal override returns (uint256) {
                return a.length + 1;
            }
        }
        "#,
    )
    .version(LanguageVersion::V0_8_13)
    .run(Analyse::Context)
    .expect_no_diagnostics();

    let d = analysis.find_contract("D");
    let own_f = find_function(analysis.find_members("D"), "f").expect("D declares f");
    let listed: Vec<_> = analysis
        .context()
        .linearised_functions(d.id())
        .iter()
        .filter(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|name| name.unparse() == "f")
        })
        .collect();

    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id(), own_f.id());
}
