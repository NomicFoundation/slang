#![allow(clippy::needless_raw_string_hashes)]

use slang_testlang::cst::Query;

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
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(e.message, "Parse error:\nexpected ']' at: \nAlt at: [_\n");
            assert_eq!((e.row, e.column), (0, 8));
        }
    }
}

// See https://github.com/NomicFoundation/slang/issues/1042
#[test]
fn test_parsing_error_with_invalid_edge_label() {
    let result = Query::parse(r#"[Tree @name Name: [_]]"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(
                e.message,
                "Parse error:\n'Name' is not a valid edge label at: Name: [_]]\n",
            );
            assert_eq!((e.row, e.column), (0, 12));
        }
    }
}

#[test]
fn test_parsing_error_with_invalid_node_kind() {
    let result = Query::parse(r#"[Tree [tree_node]]"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(
                e.message,
                "Parse error:\n'tree_node' is not a valid node kind at: tree_node]]\n",
            );
            assert_eq!((e.row, e.column), (0, 7));
        }
    }
}

#[test]
fn test_parsing_error_with_kind_beginning_with_underscore() {
    let result = Query::parse(r#"[Tree [_tree_node]]"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => {
            assert_eq!(
                e.message,
                "Parse error:\n'_tree_node' is not a valid node kind at: _tree_node]]\n",
            );
            assert_eq!((e.row, e.column), (0, 7));
        }
    }
}

#[test]
fn test_fails_parsing_ellipsis() {
    let result = Query::parse(r#"[_ ...]"#);
    match result {
        Ok(_) => panic!("Expected parse failure"),
        Err(e) => assert_eq!(
            e.message,
            "Parse error:\nThe ellipsis `...` operator is deprecated, and replaced with a new adjacency `.` operator. For more information, check the Tree Query Language guide: https://nomicfoundation.github.io/slang/latest/user-guide/tree-query-language/ at: ...]\n",
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
            "Parse error:\nMatching trivia nodes directly is forbidden. For more information, check the Tree Query Language guide: https://nomicfoundation.github.io/slang/latest/user-guide/tree-query-language/ at: EndOfLine]\n"
        ),
    }
}
