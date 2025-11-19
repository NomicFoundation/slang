#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

mod lexer;
mod parser;

pub use lexer::temp_sourcify;

#[cfg(test)]
mod tests {
    use semver::Version;

    use super::lexer::definition::Lexer;
    use super::parser::calculator;
    use crate::lexer::contexts::ContextKind;
    use crate::lexer::lexemes::LexemeKind;

    #[test]
    fn calculator() {
        let source = "22";
        let lexer = Lexer::new(ContextKind::Solidity, source, Version::new(0, 4, 26));

        assert_eq!(calculator::NumParser::new().parse(source, lexer), Ok(22));
    }

    #[test]
    fn pragma() {
        let source = "pragma solidity *.*.*;";
        let lexer = Lexer::new(ContextKind::Pragma, source, Version::new(0, 4, 26));

        let mut lexer2 = Lexer::new(ContextKind::Pragma, source, Version::new(0, 4, 26));
        while let Some(lexeme) = lexer2.next_lexeme() {
            println!("{lexeme:?}");
        }

        assert_eq!(
            calculator::PragmaDirectiveParser::new().parse(
                source,
                lexer.filter(|t| t
                    .as_ref()
                    .is_ok_and(|t| !matches!(t.1, LexemeKind::Whitespace)))
            ),
            Ok("aa".to_string())
        );
    }
}
