use semver::Version;

use crate::parser::Parser;
use crate::utils::version::{parse_range, Range};
use crate::utils::LanguageFacts;

type Result<T> = std::result::Result<T, String>;

fn parse_string(s: &str) -> Result<Range> {
    let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();

    let output = parser.parse_nonterminal(crate::cst::NonterminalKind::VersionExpressionSets, s);

    parse_range(output.create_tree_cursor())
}

#[test]
fn single_version_range() {
    let range = parse_string("1.2.3").unwrap();

    test_range_match(&range, &vec![Version::new(1, 2, 3)]);
    test_range_match_fail(&range, &vec![Version::new(1, 2, 0), Version::new(1, 0, 0)]);
}

#[test]
fn less_than_version() {
    let range = parse_string("<1.3.0").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(1, 2, 100),
            Version::new(0, 8, 0),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 3, 5),
            Version::new(2, 0, 0),
        ],
    );
}

#[test]
fn less_than_equal_version() {
    let range = parse_string("<=1.3.0").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 2, 0),
            Version::new(1, 2, 100),
            Version::new(0, 8, 0),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(1, 3, 5), Version::new(2, 0, 0)]);
}

#[test]
fn wildcard_version() {
    let range = parse_string("*").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(0, 0, 0),
            Version::new(1, 212, 1),
            Version::new(20, 1, 10),
        ],
    );
}

#[test]
fn gteq_partial_version() {
    let range = parse_string(">=1.3").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 3, 5),
            Version::new(1, 4, 0),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(1, 2, 0), Version::new(0, 8, 0)]);
}

#[test]
fn gteq_wildcard_version() {
    let range = parse_string(">=1.3.x").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 3, 4),
            Version::new(1, 4, 0),
            Version::new(2, 3, 3),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(1, 2, 1), Version::new(0, 8, 0)]);
}

#[test]
fn gt_partial_version() {
    let range = parse_string(">1.3").unwrap();

    test_range_match(&range, &vec![Version::new(1, 4, 0), Version::new(2, 5, 3)]);
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 3, 5),
            Version::new(1, 2, 0),
            Version::new(0, 8, 0),
        ],
    );
}

#[test]
fn gt_wildcard_version() {
    let range = parse_string(">1.3.x").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 4, 0),
            Version::new(2, 1, 0),
            Version::new(1, 3, 5),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 3, 0),
            Version::new(1, 2, 0),
            Version::new(0, 8, 0),
        ],
    );
}

#[test]
fn lteq_partial_version() {
    let range = parse_string("<=0.8").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(0, 7, 8),
            Version::new(0, 0, 5),
            Version::new(0, 8, 0),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(0, 8, 5), Version::new(1, 2, 0)]);
}

#[test]
fn lteq_wildcard_version() {
    let range = parse_string("<=0.8.x").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(0, 7, 8),
            Version::new(0, 0, 5),
            Version::new(0, 8, 0),
            Version::new(0, 8, 5),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(1, 2, 0), Version::new(0, 9, 1)]);
}

#[test]
fn lt_partial_version() {
    let range = parse_string("<0.8").unwrap();

    test_range_match(&range, &vec![Version::new(0, 7, 8), Version::new(0, 0, 5)]);
    test_range_match_fail(
        &range,
        &vec![
            Version::new(0, 8, 0),
            Version::new(0, 8, 5),
            Version::new(1, 2, 0),
        ],
    );
}

#[test]
fn lt_wildcard_version() {
    let range = parse_string("<0.8.x").unwrap();

    test_range_match(&range, &vec![Version::new(0, 7, 8), Version::new(0, 0, 5)]);
    test_range_match_fail(
        &range,
        &vec![
            Version::new(0, 8, 0),
            Version::new(0, 8, 5),
            Version::new(1, 2, 0),
        ],
    );
}

#[test]
fn partial_version() {
    let range = parse_string("1.2").unwrap();

    test_range_match(&range, &vec![Version::new(1, 2, 0), Version::new(1, 2, 3)]);
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 3, 2),
            Version::new(2, 0, 0),
            Version::new(1, 0, 0),
        ],
    );
}

#[test]
fn combo_partial_version() {
    let range = parse_string(">=1 <1.8").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 0, 0),
            Version::new(1, 5, 0),
            Version::new(1, 7, 9),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(0, 9, 9),
            Version::new(3, 0, 0),
            Version::new(2, 8, 3),
        ],
    );
}

#[test]
fn x_patch_partial_version() {
    let range = parse_string("1.2.x").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(1, 2, 4),
            Version::new(1, 2, 9),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(1, 3, 0), Version::new(2, 0, 0)]);
}

#[test]
fn x_minor_partial_version() {
    let range = parse_string("1.X").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 0, 0),
            Version::new(1, 2, 0),
            Version::new(1, 4, 2),
            Version::new(1, 8, 0),
        ],
    );
    test_range_match_fail(&range, &vec![Version::new(2, 0, 0), Version::new(0, 7, 0)]);
}

#[test]
fn caret_range() {
    let range = parse_string("^1.2").unwrap();

    test_range_match(&range, &vec![Version::new(1, 9, 9), Version::new(1, 2, 0)]);
    test_range_match_fail(&range, &vec![Version::new(2, 0, 0), Version::new(1, 0, 0)]);
}

#[test]
fn tilde_range() {
    let range = parse_string("~1.10.1").unwrap();

    test_range_match(
        &range,
        &vec![Version::new(1, 10, 1), Version::new(1, 10, 6)],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(2, 0, 0),
            Version::new(1, 9, 0),
            Version::new(1, 11, 0),
        ],
    );
}

#[test]
fn hyphen_range() {
    let range = parse_string("1.2 - 1.5.1").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 2, 0),
            Version::new(1, 5, 1),
            Version::new(1, 3, 17),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 6, 0),
            Version::new(1, 0, 0),
            Version::new(2, 0, 0),
            Version::new(1, 5, 5),
        ],
    );
}

#[test]
fn concat_comparators() {
    let range = parse_string(">1.0.0 <=2.5.1").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 0, 1),
            Version::new(2, 0, 0),
            Version::new(2, 5, 1),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 0, 0),
            Version::new(2, 5, 2),
            Version::new(0, 5, 0),
            Version::new(3, 0, 0),
        ],
    );
}

#[test]
fn comparator_union() {
    let range = parse_string("<1.5 || ^2.1").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(1, 1, 0),
            Version::new(2, 2, 2),
            Version::new(0, 8, 1),
            Version::new(2, 9, 0),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(1, 5, 0),
            Version::new(1, 7, 0),
            Version::new(3, 0, 0),
        ],
    );
}

#[test]
fn major_wildcard() {
    let wild = parse_string("*").unwrap();
    let range = parse_string("x.1.0").unwrap();

    assert_eq!(wild, range);
}

#[test]
fn major_wildcard_concat() {
    let range = parse_string("x.1.0 >0.5.0").unwrap();

    test_range_match(
        &range,
        &vec![
            Version::new(0, 5, 5),
            Version::new(1, 0, 0),
            Version::new(0, 7, 0),
        ],
    );
    test_range_match_fail(
        &range,
        &vec![
            Version::new(0, 5, 0),
            Version::new(0, 0, 0),
            Version::new(0, 1, 0),
        ],
    );
}

#[test]
fn missing_major_version() {
    // Inspired by a contract that caused some problems
    // ".0" is not a valid semver, but we have to be able to parse it (and discard it) without fail
    let range = parse_string("0.8.0 .0").unwrap();

    test_range_match(&range, &vec![Version::new(0, 8, 0)]);
    test_range_match_fail(
        &range,
        &vec![
            Version::new(0, 0, 0),
            Version::new(0, 8, 1),
            Version::new(1, 0, 0),
        ],
    );
}

#[test]
fn solc_positive_tests() {
    solc_test_case("*", Version::new(1, 2, 3), true);
    solc_test_case("1.0.0 - 2.0.0", Version::new(1, 2, 3), true);
    solc_test_case("1.0.0", Version::new(1, 0, 0), true);
    solc_test_case("1.0", Version::new(1, 0, 0), true);
    solc_test_case("1", Version::new(1, 0, 0), true);
    solc_test_case(">=*", Version::new(0, 2, 4), true);
    solc_test_case("*", Version::new(1, 2, 3), true);
    solc_test_case(">=1.0.0", Version::new(1, 0, 0), true);
    solc_test_case(">=1.0.0", Version::new(1, 0, 1), true);
    solc_test_case(">=1.0.0", Version::new(1, 1, 0), true);
    solc_test_case(">1.0.0", Version::new(1, 0, 1), true);
    solc_test_case(">1.0.0", Version::new(1, 1, 0), true);
    solc_test_case("<=2.0.0", Version::new(2, 0, 0), true);
    solc_test_case("<=2.0.0", Version::new(1, 9999, 9999), true);
    solc_test_case("<=2.0.0", Version::new(0, 2, 9), true);
    solc_test_case("<2.0.0", Version::new(1, 9999, 9999), true);
    solc_test_case("<2.0.0", Version::new(0, 2, 9), true);
    solc_test_case(">= 1.0.0", Version::new(1, 0, 0), true);
    solc_test_case(">=  1.0.0", Version::new(1, 0, 1), true);
    solc_test_case(">=   1.0.0", Version::new(1, 1, 0), true);
    solc_test_case("> 1.0.0", Version::new(1, 0, 1), true);
    solc_test_case(">  1.0.0", Version::new(1, 1, 0), true);
    solc_test_case("<=   2.0.0", Version::new(2, 0, 0), true);
    solc_test_case("<= 2.0.0", Version::new(1, 9999, 9999), true);
    solc_test_case("<=  2.0.0", Version::new(0, 2, 9), true);
    solc_test_case("<    2.0.0", Version::new(1, 9999, 9999), true);
    solc_test_case("<\t2.0.0", Version::new(0, 2, 9), true);
    solc_test_case(">=0.1.97", Version::new(0, 1, 97), true);
    solc_test_case("0.1.20 || 1.2.4", Version::new(1, 2, 4), true);
    solc_test_case(">=0.2.3 || <0.0.1", Version::new(0, 0, 0), true);
    solc_test_case(">=0.2.3 || <0.0.1", Version::new(0, 2, 3), true);
    solc_test_case(">=0.2.3 || <0.0.1", Version::new(0, 2, 4), true);
    solc_test_case("\"2.x.x\"", Version::new(2, 1, 3), true);
    solc_test_case("1.2.x", Version::new(1, 2, 3), true);
    solc_test_case("\"1.2.x\" || \"2.x\"", Version::new(2, 1, 3), true);
    solc_test_case("\"1.2.x\" || \"2.x\"", Version::new(1, 2, 3), true);
    solc_test_case("x", Version::new(1, 2, 3), true);
    solc_test_case("2.*.*", Version::new(2, 1, 3), true);
    solc_test_case("1.2.*", Version::new(1, 2, 3), true);
    solc_test_case("1.2.* || 2.*", Version::new(2, 1, 3), true);
    solc_test_case("1.2.* || 2.*", Version::new(1, 2, 3), true);
    solc_test_case("*", Version::new(1, 2, 3), true);
    solc_test_case("2", Version::new(2, 1, 2), true);
    solc_test_case("2.3", Version::new(2, 3, 1), true);
    solc_test_case("~2.4", Version::new(2, 4, 0), true);
    solc_test_case("~2.4", Version::new(2, 4, 5), true);
    solc_test_case("~1", Version::new(1, 2, 3), true);
    solc_test_case("~1.0", Version::new(1, 0, 2), true);
    solc_test_case("~ 1.0", Version::new(1, 0, 2), true);
    solc_test_case("~ 1.0.3", Version::new(1, 0, 12), true);
    solc_test_case(">=1", Version::new(1, 0, 0), true);
    solc_test_case(">= 1", Version::new(1, 0, 0), true);
    solc_test_case("<1.2", Version::new(1, 1, 1), true);
    solc_test_case("< 1.2", Version::new(1, 1, 1), true);
    solc_test_case("=0.7.x", Version::new(0, 7, 2), true);
    solc_test_case("<=0.7.x", Version::new(0, 7, 2), true);
    solc_test_case(">=0.7.x", Version::new(0, 7, 2), true);
    solc_test_case("<=0.7.x", Version::new(0, 6, 2), true);
    solc_test_case("~1.2.1 >=1.2.3", Version::new(1, 2, 3), true);
    solc_test_case("~1.2.1 =1.2.3", Version::new(1, 2, 3), true);
    solc_test_case("~1.2.1 1.2.3", Version::new(1, 2, 3), true);
    solc_test_case("~1.2.1 >=1.2.3 1.2.3", Version::new(1, 2, 3), true);
    solc_test_case("~1.2.1 1.2.3 >=1.2.3", Version::new(1, 2, 3), true);
    solc_test_case(">=\"1.2.1\" 1.2.3", Version::new(1, 2, 3), true);
    solc_test_case("1.2.3 >=1.2.1", Version::new(1, 2, 3), true);
    solc_test_case(">=1.2.3 >=1.2.1", Version::new(1, 2, 3), true);
    solc_test_case(">=1.2.1 >=1.2.3", Version::new(1, 2, 3), true);
    solc_test_case(">=1.2", Version::new(1, 2, 8), true);
    solc_test_case("^1.2.3", Version::new(1, 8, 1), true);
    solc_test_case("^0.1.2", Version::new(0, 1, 2), true);
    solc_test_case("^0.1", Version::new(0, 1, 2), true);
    solc_test_case("^1.2", Version::new(1, 4, 2), true);
    solc_test_case("^1.2", Version::new(1, 2, 0), true);
    solc_test_case("^1", Version::new(1, 2, 0), true);
    solc_test_case("<=1.2.3", Version::new(1, 2, 3), true);
    solc_test_case(">1.2", Version::new(1, 3, 0), true);
    solc_test_case("^1.2 ^1", Version::new(1, 4, 2), true);
    solc_test_case("^0", Version::new(0, 5, 1), true);
    solc_test_case("^0", Version::new(0, 1, 1), true);
}

#[test]
fn solc_negative_tests() {
    solc_test_case("^0^1", Version::new(0, 0, 0), false);
    solc_test_case("^0^1", Version::new(1, 0, 0), false);
    solc_test_case("1.0.0 - 2.0.0", Version::new(2, 2, 3), false);
    solc_test_case("1.0.0", Version::new(1, 0, 1), false);
    solc_test_case(">=1.0.0", Version::new(0, 0, 0), false);
    solc_test_case(">=1.0.0", Version::new(0, 0, 1), false);
    solc_test_case(">=1.0.0", Version::new(0, 1, 0), false);
    solc_test_case(">1.0.0", Version::new(0, 0, 1), false);
    solc_test_case(">1.0.0", Version::new(0, 1, 0), false);
    solc_test_case("<=2.0.0", Version::new(3, 0, 0), false);
    solc_test_case("<=2.0.0", Version::new(2, 9999, 9999), false);
    solc_test_case("<=2.0.0", Version::new(2, 2, 9), false);
    solc_test_case("<2.0.0", Version::new(2, 9999, 9999), false);
    solc_test_case("<2.0.0", Version::new(2, 2, 9), false);
    solc_test_case(">=0.1.97", Version::new(0, 1, 93), false);
    solc_test_case("0.1.20 || 1.2.4", Version::new(1, 2, 3), false);
    solc_test_case(">=0.2.3 || <0.0.1", Version::new(0, 0, 3), false);
    solc_test_case(">=0.2.3 || <0.0.1", Version::new(0, 2, 2), false);
    solc_test_case("\"2.x.x\"", Version::new(1, 1, 3), false);
    solc_test_case("\"2.x.x\"", Version::new(3, 1, 3), false);
    solc_test_case("1.2.x", Version::new(1, 3, 3), false);
    solc_test_case("\"1.2.x\" || \"2.x\"", Version::new(3, 1, 3), false);
    solc_test_case("\"1.2.x\" || \"2.x\"", Version::new(1, 1, 3), false);
    solc_test_case("2.*.*", Version::new(1, 1, 3), false);
    solc_test_case("2.*.*", Version::new(3, 1, 3), false);
    solc_test_case("1.2.*", Version::new(1, 3, 3), false);
    solc_test_case("1.2.* || 2.*", Version::new(3, 1, 3), false);
    solc_test_case("1.2.* || 2.*", Version::new(1, 1, 3), false);
    solc_test_case("2", Version::new(1, 1, 2), false);
    solc_test_case("2.3", Version::new(2, 4, 1), false);
    solc_test_case("~2.4", Version::new(2, 5, 0), false);
    solc_test_case("~2.4", Version::new(2, 3, 9), false);
    solc_test_case("~1", Version::new(0, 2, 3), false);
    solc_test_case("~1.0", Version::new(1, 1, 0), false);
    solc_test_case("<1", Version::new(1, 0, 0), false);
    solc_test_case(">=1.2", Version::new(1, 1, 1), false);
    solc_test_case("=0.7.x", Version::new(0, 8, 2), false);
    solc_test_case(">=0.7.x", Version::new(0, 6, 2), false);
    solc_test_case("<0.7.x", Version::new(0, 7, 2), false);
    solc_test_case(">1.2", Version::new(1, 2, 8), false);
    solc_test_case("^1.2.3", Version::new(2, 0, 0), false);
    solc_test_case("^1.2.3", Version::new(1, 2, 2), false);
    solc_test_case("^1.2", Version::new(1, 1, 9), false);
    solc_test_case("^0", Version::new(1, 0, 0), false);
}

fn test_range_match(range: &Range, tests: &Vec<Version>) {
    for t in tests {
        assert!(range.matches(t));
    }
}

fn test_range_match_fail(range: &Range, tests: &Vec<Version>) {
    for t in tests {
        assert!(!range.matches(t));
    }
}

// Preferring to pass by value just to reduce noise in callers
#[allow(clippy::needless_pass_by_value)]
fn solc_test_case(range_str: &str, version: Version, positive: bool) {
    let range = parse_string(range_str).unwrap();

    if positive {
        assert!(range.matches(&version));
    } else {
        assert!(!range.matches(&version));
    }
}
