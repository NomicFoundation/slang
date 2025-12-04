mod big;
mod contract;
mod tuple;

use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;
use crate::parser::grammar;

fn test(source: &str, _expected: &str) {
    let kind = ContextKind::Solidity;
    let version = LanguageVersion::V0_8_30;
    {
        // This is just to check for now
        let lexer = Lexer::new(kind, source, version);
        for l in lexer {
            println!("{:?}", l);
        }
        println!("Finished lexing");
    }

    let lexer = Lexer::new(kind, source, version);

    let result = grammar::SourceUnitParser::new().parse(&source, lexer);
    if result.is_err() {
        println!("{result:?}");
    }
    // Only checking if it's ok for now, at least until the representation of the AST is fixed
    assert!(result.is_ok());
    // assert_eq!(result, expected);
    println!("{:#?}", result.unwrap());
}
