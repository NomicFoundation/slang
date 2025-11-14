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

    #[test]
    fn calculator() {
        let lexer = Lexer::new(ContextKind::Solidity, "22", Version::new(0, 4, 26));

        assert_eq!(calculator::NumParser::new().parse(lexer), Ok(33));
    }
}
