use logos::source;
use semver::Version;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;
use crate::parser::grammar;

#[test]
fn empty_contract() {
    let kind = ContextKind::Solidity;
    let source = "contract Foo {}";
    let version = LanguageVersion::V0_8_30;
    {
        let mut lexer = Lexer::new(kind, source.clone(), version);
        for l in lexer {
            println!("{:?}", l);
        }
        println!("Finished lexing");
    }

    let mut lexer = Lexer::new(kind, source.clone(), version);

    assert_eq!(
        grammar::SourceUnitParser::new().parse(&source, lexer),
        Ok("33".into())
    );
}
