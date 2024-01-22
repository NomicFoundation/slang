use super::query_model::Query;

fn run_parser_test(input: &str, result: &str) {
    assert_eq!(Query::parse(input).unwrap().unparse(), result);
}

#[test]
fn test_parsing() {
    run_parser_test(
        r#"@root [_ ... [Identifier] ["abc\\\"\n\r\b\t\u{01abcd}"] ...]"#,
        r#"@root [_ ... [Identifier] ["abc\\\"\n\r\b\t\u{1abcd}"] ...]"#,
    );
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
