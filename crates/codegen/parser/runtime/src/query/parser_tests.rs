use super::model::Query;

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
fn test_ellipsis() {
    run_parser_test(r#"[_ ...]"#, r#"[_ ...]"#);
}

#[test]
fn test_anonymous() {
    run_parser_test(r#"[_]"#, r#"[_]"#);
}

#[test]
fn test_root_binding() {
    run_parser_test(r#"@root [X]"#, r#"@root [X]"#);
}

#[test]
fn test_binding() {
    run_parser_test(r#"[A @b [X]]"#, r#"[A @b [X]]"#);
}

#[test]
fn test_zero_or_more_canonicalisation() {
    run_parser_test(r#"[A ([B])*]"#, r#"[A (([B])+)?]"#);
}

// Test the error message on parse failure
#[test]
fn test_parsing_error() {
    let result = Query::parse(r#"@root [_ ..."#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(
            e.to_string(),
            "Parse error:\nexpected '(' at: [_ ...\nAlt at: [_ ...\n"
        ),
    }
}
