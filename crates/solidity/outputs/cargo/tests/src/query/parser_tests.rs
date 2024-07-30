use slang_solidity::query::Query;

// See https://github.com/NomicFoundation/slang/issues/1042
#[test]
fn test_parsing_error() {
    let result = Query::parse(r#"[FunctionDefinition ... @name Name: [_] ...]"#);
    match result {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(
            e.message,
            "Parse error:\nMapRes at: Name: [_] ...]\nin section 'parsing edge label', at: Name: [_] ...]\n",
        ),
    }
}
