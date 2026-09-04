//! Tests for the version-matching model a `pragma solidity` directive is built
//! into. Each case is written the way it appears in a pragma and driven through
//! the real parser and IR builder, so the grammar, the builder's reading of the
//! literals, and the matching all get exercised together.

use std::sync::Arc;

use semver::Version as SemverVersion;
use slang_solidity_v2_common::diagnostics::DiagnosticExtensions;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::ir;
use crate::ir::VersionPragmaSpecifierExtensions;

/// Builds the `pragma solidity <expression>;` the cases below describe, and
/// returns the pragma along with the diagnostics the IR builder reported.
fn build_pragma(expression: &str) -> (ir::VersionPragma, Vec<String>) {
    let file_id = "test.sol".into();
    let contents = format!("pragma solidity {expression};");

    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&file_id, &contents, LanguageVersion::LATEST);

    assert!(
        diagnostics.is_empty(),
        "`{expression}` failed to parse: {diagnostics:?}"
    );

    let mut id_generator = ir::NodeIdGenerator::default();
    let ir::BuildOutput {
        ir_root,
        diagnostics,
    } = ir::build(
        &file_id,
        &source_unit.unwrap(),
        &contents.as_str(),
        LanguageVersion::LATEST,
        &mut id_generator,
    );

    let directive = expect_variant!(&ir_root.members[0], ir::SourceUnitMember::PragmaDirective);
    let pragma = expect_variant!(&directive.pragma, ir::Pragma::VersionPragma);

    let codes = diagnostics
        .into_iter()
        .map(|diagnostic| diagnostic.code().to_owned())
        .collect();

    (Arc::clone(pragma), codes)
}

/// The specifier of the single comparator `expression` builds into.
fn specifier_of(expression: &str) -> ir::VersionPragmaSpecifier {
    let (pragma, _) = build_pragma(expression);

    Arc::clone(&pragma.sets[0][0].specifier)
}

/// Asserts that `expression` admits exactly the versions in `matching` and none
/// of the versions in `rejected`.
#[track_caller]
fn assert_matches(expression: &str, matching: &[SemverVersion], rejected: &[SemverVersion]) {
    let (pragma, _) = build_pragma(expression);

    for version in matching {
        assert!(
            pragma.matches_version(version.clone()),
            "`{expression}` should admit `{version}`"
        );
    }

    for version in rejected {
        assert!(
            !pragma.matches_version(version.clone()),
            "`{expression}` should not admit `{version}`"
        );
    }
}

/// Asserts that `expression` admits `version` — the shape solc's own matcher
/// tests are written in.
#[track_caller]
fn assert_admits(expression: &str, version: SemverVersion) {
    assert_matches(expression, &[version], &[]);
}

/// Asserts that `expression` does not admit `version`.
#[track_caller]
fn assert_rejects(expression: &str, version: SemverVersion) {
    assert_matches(expression, &[], &[version]);
}

fn v(major: u64, minor: u64, patch: u64) -> SemverVersion {
    SemverVersion::new(major, minor, patch)
}

//
// Operators
//

#[test]
fn exact_version() {
    assert_matches(
        "1.2.3",
        &[v(1, 2, 3)],
        &[v(1, 2, 0), v(1, 2, 4), v(1, 0, 0)],
    );
    assert_matches("=1.2.3", &[v(1, 2, 3)], &[v(1, 2, 4)]);
}

#[test]
fn less_than() {
    assert_matches(
        "<1.3.0",
        &[v(1, 2, 0), v(1, 2, 100), v(0, 8, 0)],
        &[v(1, 3, 0), v(1, 3, 5), v(2, 0, 0)],
    );
}

#[test]
fn less_than_equal() {
    assert_matches(
        "<=1.3.0",
        &[v(1, 2, 0), v(1, 3, 0)],
        &[v(1, 3, 1), v(2, 0, 0)],
    );
}

#[test]
fn greater_than() {
    assert_matches(
        "> 1.0.0",
        &[v(1, 0, 1), v(1, 1, 0)],
        &[v(1, 0, 0), v(0, 9, 9)],
    );
}

#[test]
fn greater_than_equal() {
    assert_matches(">=1.0.0", &[v(1, 0, 0), v(1, 0, 1)], &[v(0, 9, 9)]);
}

#[test]
fn caret_allows_up_to_the_next_significant_component() {
    // A non-zero major pins the major.
    assert_matches(
        "^1.2.3",
        &[v(1, 2, 3), v(1, 8, 1)],
        &[v(1, 2, 2), v(2, 0, 0)],
    );
    // A zero major pins the minor instead.
    assert_matches(
        "^0.8.1",
        &[v(0, 8, 1), v(0, 8, 9)],
        &[v(0, 8, 0), v(0, 9, 0)],
    );
    // A zero minor pins the minor all the same — unlike npm's semver, solc never
    // narrows `^` down to the patch component.
    assert_matches(
        "^0.0.3",
        &[v(0, 0, 3), v(0, 0, 4)],
        &[v(0, 0, 2), v(0, 1, 0)],
    );
    // A component left out widens the bound it would have pinned.
    assert_matches("^0.8", &[v(0, 8, 0), v(0, 8, 9)], &[v(0, 9, 0)]);
    assert_matches("^0", &[v(0, 0, 0), v(0, 5, 1)], &[v(1, 0, 0)]);
}

#[test]
fn tilde_allows_up_to_the_next_written_component() {
    assert_matches(
        "~1.10.1",
        &[v(1, 10, 1), v(1, 10, 9)],
        &[v(1, 10, 0), v(1, 11, 0)],
    );
    assert_matches("~2.4", &[v(2, 4, 0), v(2, 4, 5)], &[v(2, 3, 9), v(2, 5, 0)]);
    assert_matches("~1", &[v(1, 0, 0), v(1, 2, 3)], &[v(0, 2, 3), v(2, 0, 0)]);
}

//
// Hyphen ranges
//

#[test]
fn hyphen_range() {
    // A complete end is inclusive.
    assert_matches(
        "1.2.0 - 1.5.1",
        &[v(1, 2, 0), v(1, 5, 1)],
        &[v(1, 1, 9), v(1, 5, 2)],
    );
    // A partial end runs to the end of what it leaves open.
    assert_matches("1.2 - 1.5", &[v(1, 2, 0), v(1, 5, 9)], &[v(1, 6, 0)]);
    // Each end compares only the components it wrote, independently of the
    // other: a partial end does not widen a complete start.
    assert_matches("0.8.0 - 0.8", &[v(0, 8, 0), v(0, 8, 36)], &[v(0, 7, 9)]);
    assert_matches("0.8.1 - 0.8", &[v(0, 8, 1), v(0, 8, 36)], &[v(0, 8, 0)]);
    // Whitespace around the hyphen is optional.
    assert_matches("1.0.0-2.0.0", &[v(1, 2, 3)], &[v(2, 2, 3)]);
}

/// A range flattens into the pair of comparators it stands for, and each one
/// keeps the source range of the endpoint it came from rather than the range's.
#[test]
fn a_range_flattens_into_the_conjunction_around_it() {
    let expression = ">=0.8.0 0.8.1 - 0.8.9";
    let (pragma, _) = build_pragma(expression);

    let comparators = &pragma.sets[0];
    assert_eq!(
        comparators
            .iter()
            .map(|comparator| comparator.operator)
            .collect::<Vec<_>>(),
        [
            ir::VersionPragmaOperator::GreaterThanEqual,
            ir::VersionPragmaOperator::GreaterThanEqual,
            ir::VersionPragmaOperator::LessThanEqual,
        ]
    );

    let source = format!("pragma solidity {expression};");
    let spans: Vec<&str> = comparators
        .iter()
        .map(|comparator| &source[comparator.range.clone()])
        .collect();
    assert_eq!(spans, [">=0.8.0", "0.8.1", "0.8.9"]);
}

//
// Partial versions and wildcards
//

#[test]
fn partial_versions_span_what_they_leave_open() {
    assert_matches("1.2", &[v(1, 2, 0), v(1, 2, 9)], &[v(1, 1, 9), v(1, 3, 0)]);
    assert_matches("1", &[v(1, 0, 0), v(1, 9, 9)], &[v(0, 9, 9), v(2, 0, 0)]);
}

#[test]
fn wildcards_stand_in_for_a_component() {
    assert_matches("1.2.x", &[v(1, 2, 0), v(1, 2, 9)], &[v(1, 3, 3)]);
    assert_matches("1.X", &[v(1, 0, 0), v(1, 9, 9)], &[v(2, 0, 0)]);
    assert_matches("2.*.*", &[v(2, 1, 3)], &[v(1, 1, 3), v(3, 1, 3)]);
}

#[test]
fn a_lone_wildcard_admits_everything() {
    for expression in ["*", "x", "X", ">=*", "**", "xx"] {
        assert_matches(expression, &[v(0, 0, 0), v(1, 2, 3), v(20, 1, 10)], &[]);
    }
}

/// A wildcard frees its own component without freeing the ones after it, so a
/// wildcard written before a concrete component still constrains the version.
#[test]
fn a_wildcard_only_frees_its_own_component() {
    assert_matches(
        "x.8.36",
        &[v(0, 8, 36), v(9, 8, 36)],
        &[v(0, 8, 35), v(0, 9, 36)],
    );
    assert_matches("0.x.36", &[v(0, 8, 36), v(0, 9, 36)], &[v(0, 8, 35)]);
    assert_matches(
        "x.1.0",
        &[v(0, 1, 0), v(5, 1, 0)],
        &[v(0, 8, 36), v(0, 1, 1)],
    );
    assert_matches(
        "0.x.1",
        &[v(0, 0, 1), v(0, 5, 1)],
        &[v(0, 8, 36), v(1, 0, 1)],
    );
}

/// Comparison walks only the components the literal wrote and stops at the first
/// one the two versions disagree on. A trailing wildcard therefore reads the same
/// as leaving the component out.
#[test]
fn a_trailing_wildcard_reads_like_an_unwritten_component() {
    for (unwritten, wildcard) in [("0.8", "0.8.x"), ("1", "1.x"), ("1", "1.x.x")] {
        for version in [v(0, 8, 0), v(0, 8, 5), v(0, 9, 0), v(1, 0, 0), v(1, 2, 3)] {
            for operator in ["=", "<", "<=", ">", ">=", "^", "~"] {
                let (unwritten_pragma, _) = build_pragma(&format!("{operator}{unwritten}"));
                let (wildcard_pragma, _) = build_pragma(&format!("{operator}{wildcard}"));

                assert_eq!(
                    unwritten_pragma.matches_version(version.clone()),
                    wildcard_pragma.matches_version(version.clone()),
                    "`{operator}{unwritten}` and `{operator}{wildcard}` disagree on `{version}`"
                );
            }
        }
    }
}

/// A range's bounds only compare the components they wrote, so an unwritten one
/// leaves the comparison equal rather than pinning it to `0`.
#[test]
fn partial_bounds_compare_only_what_they_wrote() {
    assert_matches("<=0.8", &[v(0, 8, 0), v(0, 8, 5)], &[v(0, 9, 0)]);
    assert_matches(">0.8", &[v(0, 9, 0)], &[v(0, 8, 0), v(0, 8, 5)]);
    assert_matches("<0.8", &[v(0, 7, 9)], &[v(0, 8, 0), v(0, 8, 5)]);
    assert_matches(">=0.8", &[v(0, 8, 0), v(0, 8, 5)], &[v(0, 7, 9)]);
}

/// A pre-release sorts below the release it leads up to.
#[test]
fn pre_releases_sort_below_their_release() {
    let pre_release = [SemverVersion::parse("0.8.36-alpha").unwrap()];

    assert_matches("0.8.36", &[], &pre_release);
    assert_matches("<0.8.36", &pre_release, &[]);
    // Nothing was compared, so there is no release for it to sort below.
    assert_matches("*", &pre_release, &[]);
}

//
// Combining expressions
//

#[test]
fn juxtaposed_expressions_all_have_to_hold() {
    assert_matches(
        ">1.0.0 <=2.5.1",
        &[v(1, 0, 1), v(2, 5, 1)],
        &[v(1, 0, 0), v(2, 5, 2)],
    );
    assert_matches(">=1 <1.8", &[v(1, 0, 0), v(1, 7, 9)], &[v(1, 8, 0)]);
    // A lone wildcard adds no constraint of its own.
    assert_matches("x >0.5.0", &[v(0, 5, 1)], &[v(0, 5, 0)]);
    // Contradictions admit nothing at all.
    assert_matches("^0^1", &[], &[v(0, 0, 0), v(1, 0, 0)]);
    assert_matches("0.7.0 0.8.0", &[], &[v(0, 7, 0), v(0, 8, 0)]);
}

#[test]
fn alternatives_only_need_one_to_hold() {
    assert_matches(
        "<1.5 || ^2.1",
        &[v(1, 4, 9), v(2, 1, 0), v(2, 9, 9)],
        &[v(1, 5, 0), v(3, 0, 0)],
    );
    assert_matches(
        "0.5.0 || 0.6.0 || ^0.7.0",
        &[v(0, 5, 0), v(0, 6, 0), v(0, 7, 3)],
        &[v(0, 5, 1), v(0, 8, 0)],
    );
}

//
// Literal forms
//

#[test]
fn quoted_literals_read_the_same_as_bare_ones() {
    assert_matches("\"1.2.3\"", &[v(1, 2, 3)], &[v(1, 2, 4)]);
    assert_matches("'1.2.3'", &[v(1, 2, 3)], &[v(1, 2, 4)]);
    assert_matches("^\"1.2.3\"", &[v(1, 8, 0)], &[v(2, 0, 0)]);
    // A range may even mix the two quote styles.
    assert_matches(
        "\"4.5.6\"-'7.8.9'",
        &[v(5, 0, 0)],
        &[v(4, 5, 5), v(7, 9, 0)],
    );
    // Escape sequences stand for the characters they name, so this is `0.8.36`.
    assert_matches(r#""\x30.8.\x33\x36""#, &[v(0, 8, 36)], &[v(0, 8, 35)]);
}

#[test]
fn trivia_is_allowed_between_the_components_of_a_literal() {
    assert_matches(
        "/* comments */ 0 /* are */ . /* allowed */ 8 . /* here */ 1",
        &[v(0, 8, 1)],
        &[v(0, 8, 2)],
    );
}

//
// Reading literals into versions
//

#[test]
fn a_component_names_a_number_or_a_wildcard() {
    use ir::VersionPragmaComponent::{Number, Wildcard};

    assert_eq!(*specifier_of("0.8.1"), [Number(0), Number(8), Number(1)]);
    // A literal holds as many components as it wrote, whether that is fewer than
    // a version has or more.
    assert_eq!(*specifier_of("0.8"), [Number(0), Number(8)]);
    assert_eq!(specifier_of("0.8.36.0").len(), 4);
    // A run of wildcard characters collapses into a single wildcard, the way
    // solc's character-by-character reading of a literal leaves it.
    for wildcard in ["*", "x", "X", "xx", "**", "x*", "XX"] {
        assert_eq!(*specifier_of(wildcard), [Wildcard], "{wildcard}");
    }
    // Leading zeros carry no meaning of their own, as semver intends.
    for (written, meant) in [("08", "8"), ("007", "7"), ("00", "0")] {
        assert_eq!(specifier_of(written), specifier_of(meant), "{written}");
    }
    // A number far larger than any release is still a number.
    assert_eq!(*specifier_of("4294967296"), [Number(4_294_967_296)]);
}

/// A component that names neither a number nor a wildcard is kept as
/// [`ir::VersionPragmaComponent::Unrecognized`], so the specifier records what
/// was written.
#[test]
fn a_component_that_names_neither_is_unrecognized() {
    use ir::VersionPragmaComponent::{Number, Unrecognized};

    // The grammar lets a single unquoted component mix digits and wildcards.
    for malformed in ["1x", "x1", "0x1", "1*"] {
        assert_eq!(*specifier_of(malformed), [Unrecognized], "{malformed}");
    }
    // A quoted literal can hold letters, symbols, or nothing at all.
    for malformed in [
        "", " ", "Y", "beta", "latest", "alpha1", "3-alpha", "8+build", "@", "$", "8 ", "-1",
    ] {
        assert_eq!(
            *specifier_of(&format!("\"{malformed}\"")),
            [Unrecognized],
            "{malformed}"
        );
    }
    // A component that names neither leaves its neighbours alone: they still
    // read as the numbers they wrote.
    assert_eq!(
        *specifier_of(r#""0.8.beta""#),
        [Number(0), Number(8), Unrecognized]
    );
    // A number too large to hold is not a number Slang can compare.
    assert_eq!(
        *specifier_of("99999999999999999999999999999999999999999"),
        [Unrecognized]
    );
}

#[test]
fn a_specifier_holding_an_unrecognized_component_is_invalid() {
    assert!(specifier_of("0.8.1").is_valid());
    assert!(specifier_of("x").is_valid());
    // More components than a version has is still a valid specifier — it just
    // matches nothing.
    assert!(specifier_of("0.8.36.0").is_valid());

    assert!(!specifier_of(r#""0.8.beta""#).is_valid());
    assert!(!specifier_of(r#""""#).is_valid());
    // Nothing was written at all, which no literal can express: even an empty
    // quoted one holds the single component between its quotes.
    let empty: ir::VersionPragmaSpecifier = Vec::new().into();
    assert!(!empty.is_valid());
}

//
// Versions Slang cannot compile, which are well-formed all the same
//

#[test]
fn a_literal_with_extra_components_matches_nothing() {
    // A version has three components, so a literal that wrote a fourth runs past
    // the end of every version and leaves the version the lesser of the two.
    assert_matches("0.8.36.0", &[], &[v(0, 8, 36), v(0, 8, 0)]);
    assert_matches("1.2.3.4", &[], &[v(1, 2, 3), v(1, 2, 4)]);
    // Which the operator then judges as it would any other disagreement.
    assert_matches("<0.8.36.0", &[v(0, 8, 36)], &[v(0, 8, 37)]);
    // A trailing wildcard constrains nothing, so it reaches no further.
    assert_matches("0.8.36.x", &[v(0, 8, 36)], &[v(0, 8, 35)]);
}

#[test]
fn a_literal_larger_than_any_release_matches_nothing() {
    assert_matches("4294967296", &[], &[v(0, 8, 36), v(1, 0, 0)]);
    // ...but it compares like the number it is.
    assert_matches("<4294967296", &[v(0, 8, 36), v(1, 0, 0)], &[]);
}

//
// Diagnostics
//

/// Letters and symbols only reach a version literal through a quoted string: an
/// unquoted specifier can hold nothing but digits and wildcard characters, so the
/// parser rejects `pragma solidity 0.8.beta;` outright.
#[test]
fn unrecognized_components_are_reported_and_kept_as_written() {
    let (pragma, codes) = build_pragma(r#""0.8.beta""#);

    assert_eq!(codes, ["syntax/invalid-version-specifier"]);

    let comparator = &pragma.sets[0][0];
    assert_eq!(comparator.operator, ir::VersionPragmaOperator::Equal);
    assert_eq!(
        *comparator.specifier,
        [
            ir::VersionPragmaComponent::Number(0),
            ir::VersionPragmaComponent::Number(8),
            ir::VersionPragmaComponent::Unrecognized,
        ]
    );
}

/// Every shape of unexpected character in a quoted literal is reported, and the
/// comparator it lands in admits nothing whatever operator it is paired with.
#[test]
fn unexpected_characters_in_a_quoted_literal_are_reported() {
    for literal in [
        r#""Y""#,
        r#""beta""#,
        r#""latest""#,
        r#""0.8.beta""#,
        r#""1.2.3-alpha""#,
        r#""1.2.3+build""#,
        r#""0.8.@""#,
        r#""0.8.$""#,
        r#""0.8. ""#,
        r#""-1.2.3""#,
        r#""""#,
        r#""0.8.""#,
    ] {
        let (_, codes) = build_pragma(literal);
        assert_eq!(
            codes,
            ["syntax/invalid-version-specifier"],
            "{literal} should be reported"
        );

        // Already reported above, so it admits everything rather than also
        // reporting the pragma as incompatible.
        for operator in ["=", "<", "<=", ">", ">=", "^", "~"] {
            let (pragma, codes) = build_pragma(&format!("{operator}{literal}"));
            assert!(
                pragma.matches_version(v(0, 8, 36)),
                "`{operator}{literal}` should admit everything"
            );
            assert_eq!(
                codes,
                ["syntax/invalid-version-specifier"],
                "`{operator}{literal}` should only be reported as invalid"
            );
        }
    }
}

/// The parser rejects letters and symbols in an unquoted literal, so they never
/// reach the IR that way.
#[test]
fn unexpected_characters_outside_a_quoted_literal_do_not_parse() {
    for expression in ["0.8.beta", "0.8.@", "1.2.3-alpha", "Y"] {
        let contents = format!("pragma solidity {expression};");
        let ParseOutput { diagnostics, .. } =
            Parser::parse(&"test.sol".into(), &contents, LanguageVersion::LATEST);

        assert!(!diagnostics.is_empty(), "`{expression}` should not parse");
    }
}

#[test]
fn an_unreadable_literal_suppresses_the_compatibility_check() {
    // The literal is reported rather than the pragma, so only one diagnostic
    // comes out even though it admits no version at all.
    let (_, codes) = build_pragma("1x");

    assert_eq!(codes, ["syntax/invalid-version-specifier"]);
}

#[test]
fn every_unreadable_literal_is_reported() {
    let (_, codes) = build_pragma("1x || 2x");

    assert_eq!(
        codes,
        [
            "syntax/invalid-version-specifier",
            "syntax/invalid-version-specifier"
        ]
    );
}

/// An unreadable specifier admits every version, so it neither reports an
/// incompatibility of its own nor hides one its neighbours are responsible for:
/// the readable half of this conjunction excludes the version on its own.
#[test]
fn an_unreadable_specifier_does_not_hide_a_neighbour_that_excludes_the_version() {
    let (_, codes) = build_pragma("1x 0.7.0");

    // Reported in source order, and the pragma as a whole starts before the
    // specifier inside it.
    assert_eq!(
        codes,
        [
            "syntax/incompatible-version-pragma",
            "syntax/invalid-version-specifier"
        ]
    );
}

/// Each pragma is judged on its own, so one holding an unrecognized component
/// does not suppress the compatibility check on the next.
#[test]
fn an_unreadable_pragma_does_not_suppress_the_check_on_another() {
    let file_id = "test.sol".into();
    let contents = "pragma solidity 1x;\npragma solidity ^0.7.0;";

    let ParseOutput {
        source_unit,
        diagnostics,
    } = Parser::parse(&file_id, contents, LanguageVersion::LATEST);
    assert!(diagnostics.is_empty(), "failed to parse: {diagnostics:?}");

    let mut id_generator = ir::NodeIdGenerator::default();
    let ir::BuildOutput { diagnostics, .. } = ir::build(
        &file_id,
        &source_unit.unwrap(),
        &contents,
        LanguageVersion::LATEST,
        &mut id_generator,
    );

    let codes: Vec<String> = diagnostics
        .into_iter()
        .map(|diagnostic| diagnostic.code().to_owned())
        .collect();

    assert_eq!(
        codes,
        [
            "syntax/invalid-version-specifier",
            "syntax/incompatible-version-pragma"
        ]
    );
}

//
// The operator table transplanted from solc's own matcher tests, at
// `test/libsolidity/SemVerMatcher.cpp` in the `ethereum/solidity` repository.
//

#[test]
fn solc_positive_cases() {
    assert_admits("*", v(1, 2, 3));
    assert_admits("1.0.0 - 2.0.0", v(1, 2, 3));
    assert_admits("1.0.0", v(1, 0, 0));
    assert_admits("1.0", v(1, 0, 0));
    assert_admits("1", v(1, 0, 0));
    assert_admits(">=*", v(0, 2, 4));
    assert_admits(">=1.0.0", v(1, 0, 0));
    assert_admits(">=1.0.0", v(1, 0, 1));
    assert_admits(">=1.0.0", v(1, 1, 0));
    assert_admits(">1.0.0", v(1, 0, 1));
    assert_admits(">1.0.0", v(1, 1, 0));
    assert_admits("<=2.0.0", v(2, 0, 0));
    assert_admits("<=2.0.0", v(1, 9999, 9999));
    assert_admits("<=2.0.0", v(0, 2, 9));
    assert_admits("<2.0.0", v(1, 9999, 9999));
    assert_admits("<2.0.0", v(0, 2, 9));
    assert_admits(">= 1.0.0", v(1, 0, 0));
    assert_admits(">=  1.0.0", v(1, 0, 1));
    assert_admits(">=   1.0.0", v(1, 1, 0));
    assert_admits("> 1.0.0", v(1, 0, 1));
    assert_admits(">  1.0.0", v(1, 1, 0));
    assert_admits("<=   2.0.0", v(2, 0, 0));
    assert_admits("<= 2.0.0", v(1, 9999, 9999));
    assert_admits("<=  2.0.0", v(0, 2, 9));
    assert_admits("<    2.0.0", v(1, 9999, 9999));
    assert_admits("<\t2.0.0", v(0, 2, 9));
    assert_admits(">=0.1.97", v(0, 1, 97));
    assert_admits("0.1.20 || 1.2.4", v(1, 2, 4));
    assert_admits(">=0.2.3 || <0.0.1", v(0, 0, 0));
    assert_admits(">=0.2.3 || <0.0.1", v(0, 2, 3));
    assert_admits(">=0.2.3 || <0.0.1", v(0, 2, 4));
    assert_admits("\"2.x.x\"", v(2, 1, 3));
    assert_admits("1.2.x", v(1, 2, 3));
    assert_admits("\"1.2.x\" || \"2.x\"", v(2, 1, 3));
    assert_admits("\"1.2.x\" || \"2.x\"", v(1, 2, 3));
    assert_admits("x", v(1, 2, 3));
    assert_admits("2.*.*", v(2, 1, 3));
    assert_admits("1.2.*", v(1, 2, 3));
    assert_admits("1.2.* || 2.*", v(2, 1, 3));
    assert_admits("1.2.* || 2.*", v(1, 2, 3));
    assert_admits("2", v(2, 1, 2));
    assert_admits("2.3", v(2, 3, 1));
    assert_admits("~2.4", v(2, 4, 0));
    assert_admits("~2.4", v(2, 4, 5));
    assert_admits("~1", v(1, 2, 3));
    assert_admits("~1.0", v(1, 0, 2));
    assert_admits("~ 1.0", v(1, 0, 2));
    assert_admits("~ 1.0.3", v(1, 0, 12));
    assert_admits(">=1", v(1, 0, 0));
    assert_admits(">= 1", v(1, 0, 0));
    assert_admits("<1.2", v(1, 1, 1));
    assert_admits("< 1.2", v(1, 1, 1));
    assert_admits("=0.7.x", v(0, 7, 2));
    assert_admits("<=0.7.x", v(0, 7, 2));
    assert_admits(">=0.7.x", v(0, 7, 2));
    assert_admits("<=0.7.x", v(0, 6, 2));
    assert_admits("~1.2.1 >=1.2.3", v(1, 2, 3));
    assert_admits("~1.2.1 =1.2.3", v(1, 2, 3));
    assert_admits("~1.2.1 1.2.3", v(1, 2, 3));
    assert_admits("~1.2.1 >=1.2.3 1.2.3", v(1, 2, 3));
    assert_admits("~1.2.1 1.2.3 >=1.2.3", v(1, 2, 3));
    assert_admits(">=\"1.2.1\" 1.2.3", v(1, 2, 3));
    assert_admits("1.2.3 >=1.2.1", v(1, 2, 3));
    assert_admits(">=1.2.3 >=1.2.1", v(1, 2, 3));
    assert_admits(">=1.2.1 >=1.2.3", v(1, 2, 3));
    assert_admits(">=1.2", v(1, 2, 8));
    assert_admits("^1.2.3", v(1, 8, 1));
    assert_admits("^0.1.2", v(0, 1, 2));
    assert_admits("^0.1", v(0, 1, 2));
    assert_admits("^1.2", v(1, 4, 2));
    assert_admits("^1.2", v(1, 2, 0));
    assert_admits("^1", v(1, 2, 0));
    assert_admits("<=1.2.3", v(1, 2, 3));
    assert_admits(">1.2", v(1, 3, 0));
    assert_admits("^1.2 ^1", v(1, 4, 2));
    assert_admits("^0", v(0, 5, 1));
    assert_admits("^0", v(0, 1, 1));
}

#[test]
fn solc_negative_cases() {
    assert_rejects("^0^1", v(0, 0, 0));
    assert_rejects("^0^1", v(1, 0, 0));
    assert_rejects("1.0.0 - 2.0.0", v(2, 2, 3));
    assert_rejects("1.0.0", v(1, 0, 1));
    assert_rejects(">=1.0.0", v(0, 0, 0));
    assert_rejects(">=1.0.0", v(0, 0, 1));
    assert_rejects(">=1.0.0", v(0, 1, 0));
    assert_rejects(">1.0.0", v(0, 0, 1));
    assert_rejects(">1.0.0", v(0, 1, 0));
    assert_rejects("<=2.0.0", v(3, 0, 0));
    assert_rejects("<=2.0.0", v(2, 9999, 9999));
    assert_rejects("<=2.0.0", v(2, 2, 9));
    assert_rejects("<2.0.0", v(2, 9999, 9999));
    assert_rejects("<2.0.0", v(2, 2, 9));
    assert_rejects(">=0.1.97", v(0, 1, 93));
    assert_rejects("0.1.20 || 1.2.4", v(1, 2, 3));
    assert_rejects(">=0.2.3 || <0.0.1", v(0, 0, 3));
    assert_rejects(">=0.2.3 || <0.0.1", v(0, 2, 2));
    assert_rejects("\"2.x.x\"", v(1, 1, 3));
    assert_rejects("\"2.x.x\"", v(3, 1, 3));
    assert_rejects("1.2.x", v(1, 3, 3));
    assert_rejects("\"1.2.x\" || \"2.x\"", v(3, 1, 3));
    assert_rejects("\"1.2.x\" || \"2.x\"", v(1, 1, 3));
    assert_rejects("2.*.*", v(1, 1, 3));
    assert_rejects("2.*.*", v(3, 1, 3));
    assert_rejects("1.2.*", v(1, 3, 3));
    assert_rejects("1.2.* || 2.*", v(3, 1, 3));
    assert_rejects("1.2.* || 2.*", v(1, 1, 3));
    assert_rejects("2", v(1, 1, 2));
    assert_rejects("2.3", v(2, 4, 1));
    assert_rejects("~2.4", v(2, 5, 0));
    assert_rejects("~2.4", v(2, 3, 9));
    assert_rejects("~1", v(0, 2, 3));
    assert_rejects("~1.0", v(1, 1, 0));
    assert_rejects("<1", v(1, 0, 0));
    assert_rejects(">=1.2", v(1, 1, 1));
    assert_rejects("=0.7.x", v(0, 8, 2));
    assert_rejects(">=0.7.x", v(0, 6, 2));
    assert_rejects("<0.7.x", v(0, 7, 2));
    assert_rejects(">1.2", v(1, 2, 8));
    assert_rejects("^1.2.3", v(2, 0, 0));
    assert_rejects("^1.2.3", v(1, 2, 2));
    assert_rejects("^1.2", v(1, 1, 9));
    assert_rejects("^0", v(1, 0, 0));
}
