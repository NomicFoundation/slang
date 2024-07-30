use slang_solidity::query::Query;

// See https://github.com/NomicFoundation/slang/issues/1042
#[test]
fn test_parsing_error_with_invalid_edge_label() {
    let result = Query::parse(r#"[FunctionDefinition ... @name Name: [_] ...]"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(
            e.message,
            "Parse error:\nMapRes at: Name: [_] ...]\nin section 'parsing edge label', at: Name: [_] ...]\n",
        ),
    }
}

#[test]
fn test_parsing_error_with_invalid_node_kind() {
    let result = Query::parse(r#"[FunctionDefinition ... [identifier] ...]"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(
            e.message,
            "Parse error:\nMapRes at: identifier] ...]\nin section 'parsing node kind', at: identifier] ...]\n",
        ),
    }
}
