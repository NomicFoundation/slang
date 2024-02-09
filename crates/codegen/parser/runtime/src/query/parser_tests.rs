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
    run_parser_test(r#"@root [Token1]"#, r#"@root [Token1]"#);
}

#[test]
fn test_binding() {
    run_parser_test(r#"[Rule1 @b [Token1]]"#, r#"[Rule1 @b [Token1]]"#);
}

#[test]
fn test_zero_or_more_canonicalisation() {
    run_parser_test(r#"[Rule1 ([Rule2])*]"#, r#"[Rule1 (([Rule2])+)?]"#);
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
