use semver::Version;
use slang_solidity_v2_common::versions::LanguageVersionSpecifier;

use crate::utils::{LanguageVersion, infer_language_versions};

/// Infers the list of supported versions by both the source file and Slang
fn infer(source: &str) -> Vec<LanguageVersion> {
    infer_language_versions(source)
        .supported_versions()
        .collect()
}

/// Whether `source` admits each of `versions`, which need not be supported by Slang.
fn matches(source: &str, versions: &[&str]) -> Vec<bool> {
    let constraint = infer_language_versions(source);

    versions
        .iter()
        .map(|version| constraint.matches(Version::parse(version).unwrap()))
        .collect()
}

/// Every supported version from `from` (included) to `till` (excluded).
fn range(from: LanguageVersion, till: LanguageVersion) -> Vec<LanguageVersion> {
    expand_versions(&LanguageVersionSpecifier::range(from, till))
}

/// Every supported version from `from` (included) onwards.
fn from(from: LanguageVersion) -> Vec<LanguageVersion> {
    expand_versions(&LanguageVersionSpecifier::from(from))
}

fn expand_versions(specifier: &LanguageVersionSpecifier) -> Vec<LanguageVersion> {
    LanguageVersion::ALL
        .iter()
        .copied()
        .filter(|&version| specifier.contains(version))
        .collect()
}

/// Every supported version.
fn all_versions() -> Vec<LanguageVersion> {
    LanguageVersion::ALL.to_vec()
}

#[test]
fn no_pragma_admits_every_version() {
    assert_eq!(infer("contract A {}"), all_versions());
    assert_eq!(infer(""), all_versions());
}

// Operators

#[test]
fn exact_version() {
    let expected = vec![LanguageVersion::V0_8_19];

    assert_eq!(infer("pragma solidity 0.8.19;"), expected);
    assert_eq!(infer("pragma solidity =0.8.19;"), expected);
}

#[test]
fn caret() {
    assert_eq!(
        infer("pragma solidity ^0.8.20;"),
        from(LanguageVersion::V0_8_20),
    );
}

#[test]
fn tilde() {
    assert_eq!(
        infer("pragma solidity ~0.8.30;"),
        from(LanguageVersion::V0_8_30),
    );
}

#[test]
fn comparators_in_a_set_all_hold() {
    assert_eq!(
        infer("pragma solidity >=0.8.4 <0.8.10;"),
        range(LanguageVersion::V0_8_4, LanguageVersion::V0_8_10),
    );
    assert_eq!(
        infer("pragma solidity >0.8.4 <=0.8.10;"),
        // `till` is excluded, so this is up to 0.8.10 included.
        range(LanguageVersion::V0_8_5, LanguageVersion::V0_8_11),
    );
}

#[test]
fn comparators_need_no_space_between_them() {
    assert_eq!(
        infer("pragma solidity>=0.8.4<0.8.10;"),
        range(LanguageVersion::V0_8_4, LanguageVersion::V0_8_10),
    );
}

#[test]
fn hyphen_range_includes_both_ends() {
    assert_eq!(
        infer("pragma solidity 0.8.2 - 0.8.5;"),
        // `till` is excluded, so this is up to 0.8.5 included.
        range(LanguageVersion::V0_8_2, LanguageVersion::V0_8_6),
    );
}

#[test]
fn alternatives_can_leave_gaps() {
    let mut expected = vec![LanguageVersion::V0_8_1];
    expected.extend(from(LanguageVersion::V0_8_30));

    assert_eq!(infer("pragma solidity 0.8.1 || >=0.8.30;"), expected);
}

#[test]
fn wildcards() {
    assert_eq!(infer("pragma solidity 0.8.x;"), all_versions());
    assert_eq!(infer("pragma solidity 0.8.*;"), all_versions());
    assert_eq!(infer("pragma solidity 0.8;"), all_versions());
    assert_eq!(infer("pragma solidity *;"), all_versions());
    // A wildcard frees its own component, but not the ones after it.
    assert_eq!(
        infer("pragma solidity 0.x.36;"),
        vec![LanguageVersion::V0_8_36]
    );
}

#[test]
fn quoted_version() {
    let expected = vec![LanguageVersion::V0_8_7];

    assert_eq!(infer(r#"pragma solidity "0.8.7";"#), expected);
    assert_eq!(infer("pragma solidity '0.8.7';"), expected);
}

#[test]
fn exact_versions_in_a_set_all_hold() {
    assert_eq!(
        matches("pragma solidity 0.7.0 0.8.0;", &["0.7.0", "0.8.0"]),
        [false, false],
    );
}

#[test]
fn a_hyphen_without_spaces_is_a_range() {
    // Not `2.0.0` with a `-3.0.0` pre-release.
    assert_eq!(
        matches(
            "pragma solidity ^1.0.0 || 2.0.0-3.0.0;",
            &["1.5.0", "2.0.0", "2.5.0", "3.0.0", "3.0.1"]
        ),
        [true, true, true, true, false],
    );
}

#[test]
fn operators_and_ranges_over_quoted_versions() {
    assert_eq!(
        matches(
            r#"pragma solidity ^"1.2.3" || "4.5.6"-'7.8.9';"#,
            &[
                "1.2.2", "1.2.3", "1.9.0", "2.0.0", "4.5.6", "7.8.9", "7.8.10"
            ]
        ),
        [false, true, true, false, true, true, false],
    );
}

#[test]
fn trivia_inside_a_version() {
    assert_eq!(
        infer("pragma solidity 0 /* a */ . 8 . /* b */ 3;"),
        vec![LanguageVersion::V0_8_3]
    );
}

#[test]
fn experimental_pragmas_that_look_like_versions_are_ignored() {
    for source in [
        "pragma experimental solidity;",
        r#"pragma experimental "solidity";"#,
        r#"pragma experimental "1.2.3";"#,
    ] {
        assert_eq!(
            matches(source, &["0.0.0", "1.2.3"]),
            [true, true],
            "{source}"
        );
    }
}

// Several pragmas

#[test]
fn several_pragmas_all_hold() {
    let source = "
        pragma solidity >=0.8.0;
        pragma solidity <0.8.5;
        contract A {}
    ";

    assert_eq!(
        infer(source),
        range(LanguageVersion::V0_8_0, LanguageVersion::V0_8_5),
    );
}

#[test]
fn contradictory_pragmas_admit_nothing() {
    let source = "
        pragma solidity ^0.8.20;
        pragma solidity <0.8.10;
    ";

    assert_eq!(infer(source), vec![]);
}

#[test]
fn pragmas_after_other_members_still_count() {
    let source = "
        contract A {
            function f() public pure returns (uint256 x) {
                assembly { x := add(1, 2) }
            }
        }
        pragma solidity 0.8.3;
    ";

    assert_eq!(infer(source), vec![LanguageVersion::V0_8_3]);
}

#[test]
fn other_pragmas_are_ignored() {
    let source = "
        pragma abicoder v2;
        pragma experimental ABIEncoderV2;
        pragma experimental SMTChecker;
        pragma experimental \"unknown\";
    ";
    assert_eq!(infer(source), all_versions());

    let source = "
        pragma abicoder v2;
        pragma solidity 0.8.10;
        pragma experimental ABIEncoderV2;
    ";
    assert_eq!(infer(source), vec![LanguageVersion::V0_8_10]);
}

// Resilience

#[test]
fn unsupported_versions_admit_no_supported_version() {
    assert_eq!(infer("pragma solidity ^0.6.0;"), vec![]);
    assert_eq!(infer("pragma solidity >=0.9.0;"), vec![]);
}

// Versions outside of the supported ones

#[test]
fn older_versions() {
    assert_eq!(
        matches(
            "pragma solidity ^0.6.2;",
            &["0.6.1", "0.6.2", "0.6.12", "0.7.0", "0.8.0"]
        ),
        [false, true, true, false, false],
    );
    assert_eq!(
        matches(
            "pragma solidity >=0.4.22 <0.6.0;",
            &["0.4.21", "0.4.22", "0.5.17", "0.6.0"]
        ),
        [false, true, true, false],
    );
}

#[test]
fn newer_versions() {
    assert_eq!(
        matches(
            "pragma solidity >=0.8.0;",
            &["0.8.38", "0.9.0", "1.0.0", "10.20.30"]
        ),
        [true, true, true, true],
    );
    assert_eq!(
        matches("pragma solidity ^0.8.0;", &["0.8.38", "0.8.99", "0.9.0"]),
        [true, true, false],
    );
    assert_eq!(
        matches("pragma solidity >=0.9.0;", &["0.8.38", "0.9.0", "1.0.0"]),
        [false, true, true],
    );
}

#[test]
fn a_wildcard_in_the_middle_leaves_gaps() {
    assert_eq!(
        matches(
            "pragma solidity 0.x.36;",
            &["0.7.36", "0.8.35", "0.8.36", "0.9.36", "1.0.36"]
        ),
        [true, false, true, true, false],
    );
}

#[test]
fn pre_releases_sort_below_their_release() {
    assert_eq!(
        matches(
            "pragma solidity >=0.8.20;",
            &[
                "0.8.20-nightly.2023.5.1",
                "0.8.20",
                "0.8.21-nightly.2023.6.1"
            ]
        ),
        [false, true, true],
    );
}

#[test]
fn supported_versions_agree_with_matches() {
    let constraint = infer_language_versions("pragma solidity >=0.8.3 <0.8.30 || 0.8.35;");

    let matching: Vec<_> = LanguageVersion::ALL
        .iter()
        .copied()
        .filter(|&version| constraint.matches(version))
        .collect();

    assert_eq!(
        constraint.supported_versions().collect::<Vec<_>>(),
        matching
    );
}

#[test]
fn old_syntax_does_not_get_in_the_way() {
    // 0.4 code that the 0.8 grammar rejects: `var`, `throw`, `constant`
    // functions, and a constructor named after its contract.
    let body = "
        contract Old {
            uint public value;
            function Old() public { value = 1; }
            function get() constant returns (uint) {
                var x = value;
                if (x == 0) throw;
                return x;
            }
            function () payable {}
        }
    ";

    assert_eq!(infer(&format!("pragma solidity ^0.4.24;\n{body}")), vec![]);
    assert_eq!(
        infer(&format!("pragma solidity >=0.4.22 <0.9.0;\n{body}")),
        all_versions(),
    );
}

#[test]
fn syntax_errors_around_the_pragma_do_not_hide_it() {
    let source = "
        contract { function ((( }
        pragma solidity 0.8.12;
        } } } struct ;;; §§ # @
    ";

    assert_eq!(infer(source), vec![LanguageVersion::V0_8_12]);
}

#[test]
fn an_unterminated_string_does_not_hide_a_later_pragma() {
    // A string literal cannot span lines, so the unclosed one is lexed as a
    // single unrecognized lexeme up to its line break, and the next line is
    // lexed as usual.
    let source = r#"
        string constant S = "abc;
        pragma solidity 0.8.12;
    "#;

    assert_eq!(infer(source), vec![LanguageVersion::V0_8_12]);
}

#[test]
fn assembly_blocks_do_not_hide_later_pragmas() {
    let source = "
        contract A { function f() public { assembly { let x := 1 } } }
        contract B { function g() public { assembly { } } }
        pragma solidity 0.8.14;
    ";

    assert_eq!(infer(source), vec![LanguageVersion::V0_8_14]);
}

#[test]
fn pragmas_in_comments_and_strings_do_not_count() {
    let source = r#"
        // pragma solidity 0.8.1;
        /* pragma solidity 0.8.2; */
        /// pragma solidity 0.8.3;
        /** pragma solidity 0.8.4; */
        contract A {
            string s = "pragma solidity 0.8.5;";
            string t = 'pragma solidity 0.8.6;';
            string u = unicode"pragma solidity 0.8.7; ☃";
            function f() public {
                assembly {
                    // pragma solidity 0.8.8;
                    let v := "pragma solidity 0.8.9;"
                }
            }
        }
        pragma solidity 0.8.10;
    "#;

    assert_eq!(infer(source), vec![LanguageVersion::V0_8_10]);
}

#[test]
fn trivia_inside_the_pragma() {
    let source = "pragma /* a */ solidity // b\r\n  >=0.8.4 /* c */\t<0.8.6 ;";

    assert_eq!(
        infer(source),
        range(LanguageVersion::V0_8_4, LanguageVersion::V0_8_6),
    );
}

// API

#[test]
fn versions_come_out_ascending_and_distinct() {
    let inferred = infer("pragma solidity >=0.8.5 || ^0.8.0;");

    assert_eq!(inferred, all_versions());
    assert!(inferred.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn the_result_does_not_borrow_the_source() {
    let source = String::from("pragma solidity 0.8.2;");
    let constraint = infer_language_versions(&source);
    drop(source);

    assert_eq!(
        constraint.supported_versions().collect::<Vec<_>>(),
        vec![LanguageVersion::V0_8_2]
    );
}

// Unexpected
//
// Pragmas that are not recognized, or that are recognized but maybe should not
// be. These pin down what happens, not what should.

#[test]
fn a_pragma_that_does_not_parse_is_ignored() {
    let source = "
        pragma solidity >=;
        pragma solidity 0.8.12;
    ";

    assert_eq!(infer(source), vec![LanguageVersion::V0_8_12]);
}

#[test]
fn a_version_that_is_not_spelled_out_is_ignored() {
    let source = r#"pragma solidity "0.8.beta";"#;

    assert_eq!(infer(source), all_versions());
}

#[test]
fn an_unterminated_pragma_is_ignored() {
    // Without its `;`, the directive runs to the end of the source, swallowing
    // the contract after it.
    let source = "
        pragma solidity 0.8.12
        contract A {}
    ";

    assert_eq!(infer(source), all_versions());
}

#[test]
fn an_unterminated_pragma_hides_the_next_one() {
    // Without its `;`, the directive runs up to the next directive's `;`, and
    // the two of them together do not parse, so neither counts.
    let source = "
        pragma abicoder v2
        pragma solidity 0.8.1;
        contract A {}
    ";

    assert_eq!(infer(source), all_versions());
}

#[test]
fn a_pragma_inside_a_contract_still_counts() {
    // Directives are only allowed at the top level, so solc rejects this
    // source. Lexing does not track nesting, so the directive is found anyway.
    let source = "
        contract A {
            pragma solidity 0.8.12;
        }
    ";

    assert_eq!(infer(source), vec![LanguageVersion::V0_8_12]);
}

#[test]
fn an_unterminated_comment_can_expose_pragma_text() {
    // The lexer does not recognize a `/*` that is never closed as a comment,
    // so the directive inside it counts, and contradicts the real one.
    let source = "
        pragma solidity ^0.8.0;
        /* old: pragma solidity 0.4.24;
        contract A {}
    ";

    assert_eq!(infer(source), vec![]);
}

#[test]
fn old_non_ascii_strings_can_expose_pragma_text() {
    // Before 0.7.0, plain string literals could hold non-ASCII characters, but
    // the source is lexed as the latest version, where they cannot. The quotes
    // then pair up wrongly until the end of the line, and the text inside the
    // next string on it reads as a directive.
    //
    // This is the only unexpected case that is a valid source file in old Solidity, we
    // may want to revisit it.
    let source = r#"
        pragma solidity ^0.6.0;
        contract A {
            string a = "é"; string b = "pragma solidity 0.4.0;";
        }
    "#;

    assert_eq!(matches(source, &["0.4.0", "0.6.12"]), [false, false]);
}
