#![allow(clippy::needless_raw_string_hashes)]

use slang_testlang::query::Query;

fn run_parser_test(input: &str, result: &str) {
    assert_eq!(Query::parse(input).unwrap().to_string(), result);
}

#[test]
fn test_text_escaping() {
    run_parser_test(
        r#"["abc\\\"\n\r\b\t\u{01abcd}\     
         "]"#,
        r#"["abc\\\"\n\r\b\t\u{1abcd}"]"#,
    );
}

#[test]
fn test_anchor() {
    run_parser_test(r#"[_ . [_]]"#, r#"[_ . [_]]"#);
}

#[test]
fn test_anonymous() {
    run_parser_test(r#"[_]"#, r#"[_]"#);
}

#[test]
fn test_root_capture() {
    run_parser_test(
        r#"@root [DelimitedIdentifier]"#,
        r#"@root [DelimitedIdentifier]"#,
    );
}

#[test]
fn test_capture() {
    run_parser_test(
        r#"[TreeNode @b [DelimitedIdentifier]]"#,
        r#"[TreeNode @b [DelimitedIdentifier]]"#,
    );
}

#[test]
fn test_zero_or_more_canonicalisation() {
    run_parser_test(
        r#"[TreeNode ([TreeNodeChild])*]"#,
        r#"[TreeNode (([TreeNodeChild])+)?]"#,
    );
}

// Test the error message on parse failure
#[test]
fn test_parsing_error() {
    let result = Query::parse(r#"@root [_"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.message, "Parse error:\nexpected ']' at: \nAlt at: [_\n"),
    }
}

#[test]
fn test_fails_parsing_ellipsis() {
    let result = Query::parse(r#"[_ ...]"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.message, "Parse error:\nNoneOf at: ..]\n"),
    }
}

#[test]
fn test_fails_consecutive_anchors() {
    let result = Query::parse(r#"[_ . .]"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.message, "Parse error:\nNoneOf at: .]\n"),
    }
}
