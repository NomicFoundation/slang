mod contract;

use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;
use crate::parser::grammar;

fn test(source: &str, expected: &str) {
    let kind = ContextKind::Solidity;
    let version = LanguageVersion::V0_8_30;
    {
        // This is just to check for now
        let mut lexer = Lexer::new(kind, source.clone(), version);
        for l in lexer {
            println!("{:?}", l);
        }
        println!("Finished lexing");
    }

    let mut lexer = Lexer::new(kind, source.clone(), version);

    let result = grammar::SourceUnitParser::new().parse(&source, lexer);
    // Only checking if it's ok for now, at least until the representation of the AST is fixed
    assert!(result.is_ok());
    println!("{}", result.unwrap());
    // assert_eq!(result, expected);
}
