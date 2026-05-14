// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::diagnostics_output::runner::run;

mod compilation {
    use super::*;

    #[test]
    fn missing_file() -> Result<()> {
        run("compilation", "missing_file")
    }

    #[test]
    fn unresolved_import() -> Result<()> {
        run("compilation", "unresolved_import")
    }
}

mod syntax {
    use super::*;

    mod invalid_constructor_visibility {
        use super::*;

        #[test]
        fn constructors() -> Result<()> {
            run("syntax/invalid_constructor_visibility", "constructors")
        }
    }

    mod invalid_fallback_visibility {
        use super::*;

        #[test]
        fn fallbacks() -> Result<()> {
            run("syntax/invalid_fallback_visibility", "fallbacks")
        }
    }

    mod invalid_function_visibility {
        use super::*;

        #[test]
        fn free_functions() -> Result<()> {
            run("syntax/invalid_function_visibility", "free_functions")
        }
    }

    mod invalid_receive_attributes {
        use super::*;

        #[test]
        fn receives() -> Result<()> {
            run("syntax/invalid_receive_attributes", "receives")
        }
    }

    mod multiple_mutability_specifiers {
        use super::*;

        #[test]
        fn constructors() -> Result<()> {
            run("syntax/multiple_mutability_specifiers", "constructors")
        }

        #[test]
        fn fallback_functions() -> Result<()> {
            run(
                "syntax/multiple_mutability_specifiers",
                "fallback_functions",
            )
        }

        #[test]
        fn function_types() -> Result<()> {
            run("syntax/multiple_mutability_specifiers", "function_types")
        }

        #[test]
        fn functions() -> Result<()> {
            run("syntax/multiple_mutability_specifiers", "functions")
        }

        #[test]
        fn receive_functions() -> Result<()> {
            run("syntax/multiple_mutability_specifiers", "receive_functions")
        }

        #[test]
        fn state_variables() -> Result<()> {
            run("syntax/multiple_mutability_specifiers", "state_variables")
        }
    }

    mod multiple_visibility_specifiers {
        use super::*;

        #[test]
        fn constructors() -> Result<()> {
            run("syntax/multiple_visibility_specifiers", "constructors")
        }

        #[test]
        fn free_functions() -> Result<()> {
            run("syntax/multiple_visibility_specifiers", "free_functions")
        }

        #[test]
        fn function_types() -> Result<()> {
            run("syntax/multiple_visibility_specifiers", "function_types")
        }

        #[test]
        fn functions() -> Result<()> {
            run("syntax/multiple_visibility_specifiers", "functions")
        }

        #[test]
        fn state_variables() -> Result<()> {
            run("syntax/multiple_visibility_specifiers", "state_variables")
        }
    }

    #[test]
    fn unexpected_eof() -> Result<()> {
        run("syntax", "unexpected_eof")
    }

    #[test]
    fn unexpected_terminal() -> Result<()> {
        run("syntax", "unexpected_terminal")
    }

    #[test]
    fn unsupported_syntax() -> Result<()> {
        run("syntax", "unsupported_syntax")
    }
}
