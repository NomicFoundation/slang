#[cfg(test)]
use semver::Version;

#[cfg(test)]
use super::{parser::parse, Range};

#[test]
fn single_version_range() {
    let range = parse("1.2.3");

    test_range_match(&range, &vec![Version::new(1, 2, 3)]);
    test_range_match_fail(&range, &vec![Version::new(1, 2, 0), Version::new(1, 0, 0)]);
}

#[test]
fn less_than_version() {
    let range = parse("<1.3.0");

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
fn less_than_version_v() {
    let range = parse("<v1.3.0");

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
    let range = parse("<=1.3.0");

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
fn empty_version() {
    let range = parse("");

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
fn wildcard_version() {
    let range = parse("*");

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
    let range = parse(">=1.3");

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
    let range = parse(">=1.3.x");

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
    let range = parse(">1.3");

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
    let range = parse(">1.3.x");

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
    let range = parse("<=0.8");

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
    let range = parse("<=0.8.x");

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
    let range = parse("<0.8");

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
    let range = parse("<0.8.x");

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
    let range = parse("1.2");

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
    let range = parse(">=1 <1.8");

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
    let range = parse("1.2.x");

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
    let range = parse("1.X");

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
    let range = parse("^1.2");

    test_range_match(&range, &vec![Version::new(1, 9, 9), Version::new(1, 2, 0)]);
    test_range_match_fail(&range, &vec![Version::new(2, 0, 0), Version::new(1, 0, 0)]);
}

#[test]
fn tilde_range() {
    let range = parse("~1.10.1");

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
    let range = parse("1.2 - 1.5.1");

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
fn ignore_operators_in_hyphen_ranges() {
    let range = parse("^1.2 - ~1.5.1");

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
    let range = parse(">1.0.0 <=2.5.1");

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
    let range = parse("<1.5 || ^2.1");

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
fn allow_inner_quotes() {
    let target_range = parse("0.8");
    let target_version = Version::new(0, 8, 0);

    // "0.8" but with different combinations of quotes embeded within
    // solc currently ignores these by stringifying the whole version pragma tokens together before parsing.
    // In practice we should also be doing this, but that happens higher up in the code.
    // Down here in the parser, we're just going to ignore them when we see them.
    let examples = [
        "\"0.8\"",
        "\"0.\" 8",
        "\"0\" .8",
        "0  . \"8\"",
        "0 '.' 8",
        "'0.8'",
        "\"0\".\"8\"",
    ];

    let example_ranges: Vec<Range> = examples.iter().map(|e| parse(e)).collect();
    for r in &example_ranges {
        assert!(r == &target_range);
        assert!(r.matches(&target_version));
    }
}

#[test]
fn major_wildcard() {
    let wild = parse("*");
    let range = parse("x.1.0");

    assert_eq!(wild, range);
}

#[test]
fn major_wildcard_concat() {
    let range = parse("x.1.0 >0.5.0");

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
    let range = parse("0.8.0 .0");

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

#[cfg(test)]
fn test_range_match(range: &Range, tests: &Vec<Version>) {
    for t in tests {
        assert!(range.matches(t));
    }
}

#[cfg(test)]
fn test_range_match_fail(range: &Range, tests: &Vec<Version>) {
    for t in tests {
        assert!(!range.matches(t));
    }
}

#[cfg(test)]
fn solc_test_case(range_str: &str, version: Version, positive: bool) {
    let range = parse(range_str);

    if positive {
        assert!(range.matches(&version));
    } else {
        assert!(!range.matches(&version));
    }
}
