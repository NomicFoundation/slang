#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

mod lexer;
mod parser;

pub use lexer::temp_sourcify;

#[cfg(test)]
mod tests {
    use semver::Version;

    use super::lexer::definition::Lexer;
    use super::parser::calculator;
    use crate::lexer::contexts::{ContextKind, PragmaContext, SolidityContext};
    use crate::lexer::lexemes::LexemeKind;

    // #[test]
    // fn calculator() {
    //     let source = "22";
    //     let lexer = Lexer::new(ContextKind::Solidity, source, Version::new(0, 4, 26));

    //     let l2 = lexer.clone();
    //     assert_eq!(
    //         calculator::NumParser::new().parse(source, &lexer, l2),
    //         Ok(22)
    //     );
    // }

    #[test]
    fn pragma() {
        let source = "pragma *.*.*;";
        let lexer = Lexer::new(ContextKind::Solidity, source, Version::new(0, 4, 26));

        // for i in lexer.clone() {
        //     println!("{i:?}");
        // }

        // dbg!(&lexer);

        let l2 = lexer.clone();

        // l2.switch_context(ContextKind::Pragma);
        // dbg!(&lexer);
        // l2.switch_context(ContextKind::Solidity);

        assert_eq!(
            calculator::PragmaDirectiveParser::new().parse(
                source,
                &lexer,
                l2.filter(|t| t
                    .as_ref()
                    .is_ok_and(|t| !matches!(t.1, LexemeKind::Whitespace)))
            ),
            Ok("aa".to_string())
        );
    }
}
