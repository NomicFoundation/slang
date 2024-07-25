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
fn test_adjacency() {
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
        Ok(_) => panic!("Expected parse failure"),
        Err(e) => assert_eq!(e.message, "Parse error:\nexpected ']' at: \nAlt at: [_\n"),
    }
}

#[test]
fn test_fails_parsing_ellipsis() {
    let result = Query::parse(r#"[_ ...]"#);
    match result {
        Ok(_) => panic!("Expected parse failure"),
        Err(e) => assert_eq!(
            e.message,
            "Parse error:\nFail at: ...]\nin section 'The ellipsis `...` operator is deprecated, and replaced with a new adjacency `.` operator. For more information, check the [Tree Query Language](https://nomicfoundation.github.io/slang/user-guide/tree-query-language/) guide.', at: ...]\n",
        ),
    }
}

#[test]
fn test_fails_consecutive_adjacency_operators() {
    let result = Query::parse(r#"[_ [DelimitedIdentifier] . .]"#);
    match result {
        Ok(_) => panic!("Expected parse failure"),
        Err(e) => assert_eq!(e.message, "Parse error:\nNoneOf at: .]\n"),
    }
}

#[test]
fn test_fails_sole_adjacency() {
    let result = Query::parse(r#"[_ .]"#);
    match result {
        Ok(_) => panic!("Expected parse failure"),
        Err(e) => assert_eq!(
            e.message,
            "Parse error:\nexpected ']' at: .]\nAlt at: [_ .]\n"
        ),
    }
}

#[test]
fn test_fails_adjacency_at_edge_of_alt_option() {
    let result = Query::parse(r#"([TreeNode] | . [DelimitedIdentifier])+"#);
    assert!(result.is_err(), "Expected parse failure");
}

#[test]
fn test_fails_parsing_trivia_node_selector() {
    let result = Query::parse(r#"[EndOfLine]"#);
    match result {
        Ok(_) => panic!("Expected parse failure"),
        Err(e) => assert_eq!(
            e.message,
            "Parse error:\nFail at: EndOfLine]\nin section 'Matching trivia nodes directly is forbidden. For more information, check the [Tree Query Language](https://nomicfoundation.github.io/slang/user-guide/tree-query-language/) guide.', at: EndOfLine]\n"
        ),
    }
}
