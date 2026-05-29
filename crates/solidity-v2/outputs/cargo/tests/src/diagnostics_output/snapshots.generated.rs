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

mod type_system {
    use super::*;

    mod comparison_operators {
        use super::*;

        #[test]
        fn booleans() -> Result<()> {
            run("type_system/comparison_operators", "booleans")
        }

        #[test]
        fn bytes() -> Result<()> {
            run("type_system/comparison_operators", "bytes")
        }

        #[test]
        fn compatible_passes() -> Result<()> {
            run("type_system/comparison_operators", "compatible_passes")
        }

        #[test]
        fn error() -> Result<()> {
            run("type_system/comparison_operators", "error")
        }

        #[test]
        fn event() -> Result<()> {
            run("type_system/comparison_operators", "event")
        }

        #[test]
        fn external_functions_with_call_options() -> Result<()> {
            run(
                "type_system/comparison_operators",
                "external_functions_with_call_options",
            )
        }

        #[test]
        fn function_types() -> Result<()> {
            run("type_system/comparison_operators", "function_types")
        }

        #[test]
        fn incomparable_types() -> Result<()> {
            run("type_system/comparison_operators", "incomparable_types")
        }

        #[test]
        fn integer_literal_too_big() -> Result<()> {
            run(
                "type_system/comparison_operators",
                "integer_literal_too_big",
            )
        }

        #[test]
        fn mapping_types() -> Result<()> {
            run("type_system/comparison_operators", "mapping_types")
        }

        #[test]
        fn struct_type() -> Result<()> {
            run("type_system/comparison_operators", "struct_type")
        }

        #[test]
        fn super_keyword() -> Result<()> {
            run("type_system/comparison_operators", "super_keyword")
        }

        #[test]
        fn this_keyword() -> Result<()> {
            run("type_system/comparison_operators", "this_keyword")
        }

        #[test]
        fn via_common_supertype() -> Result<()> {
            run("type_system/comparison_operators", "via_common_supertype")
        }
    }
}
