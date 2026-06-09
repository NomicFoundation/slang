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

mod resolution {
    use super::*;

    mod identifier_not_found {
        use super::*;

        #[test]
        fn unresolved_base() -> Result<()> {
            run("resolution/identifier_not_found", "unresolved_base")
        }
    }
}

mod structure {
    use super::*;

    mod function_name_matches_container {
        use super::*;

        #[test]
        fn contract() -> Result<()> {
            run("structure/function_name_matches_container", "contract")
        }

        #[test]
        fn interface() -> Result<()> {
            run("structure/function_name_matches_container", "interface")
        }

        #[test]
        fn library() -> Result<()> {
            run("structure/function_name_matches_container", "library")
        }
    }

    mod invalid_using_directive_container {
        use super::*;

        #[test]
        fn contract() -> Result<()> {
            run("structure/invalid_using_directive_container", "contract")
        }

        #[test]
        fn file_level() -> Result<()> {
            run("structure/invalid_using_directive_container", "file_level")
        }

        #[test]
        fn interface() -> Result<()> {
            run("structure/invalid_using_directive_container", "interface")
        }

        #[test]
        fn library() -> Result<()> {
            run("structure/invalid_using_directive_container", "library")
        }
    }

    #[test]
    fn multiple_constructors() -> Result<()> {
        run("structure", "multiple_constructors")
    }
}

mod syntax {
    use super::*;

    mod expected_array_length_expression {
        use super::*;

        #[test]
        fn array_types() -> Result<()> {
            run("syntax/expected_array_length_expression", "array_types")
        }

        #[test]
        fn state_variable() -> Result<()> {
            run("syntax/expected_array_length_expression", "state_variable")
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

    mod multiple_override_specifiers {
        use super::*;

        #[test]
        fn fallback_functions() -> Result<()> {
            run("syntax/multiple_override_specifiers", "fallback_functions")
        }

        #[test]
        fn functions() -> Result<()> {
            run("syntax/multiple_override_specifiers", "functions")
        }

        #[test]
        fn modifiers() -> Result<()> {
            run("syntax/multiple_override_specifiers", "modifiers")
        }

        #[test]
        fn receive_functions() -> Result<()> {
            run("syntax/multiple_override_specifiers", "receive_functions")
        }

        #[test]
        fn state_variables() -> Result<()> {
            run("syntax/multiple_override_specifiers", "state_variables")
        }
    }

    mod multiple_virtual_specifiers {
        use super::*;

        #[test]
        fn fallback_functions() -> Result<()> {
            run("syntax/multiple_virtual_specifiers", "fallback_functions")
        }

        #[test]
        fn functions() -> Result<()> {
            run("syntax/multiple_virtual_specifiers", "functions")
        }

        #[test]
        fn modifiers() -> Result<()> {
            run("syntax/multiple_virtual_specifiers", "modifiers")
        }

        #[test]
        fn receive_functions() -> Result<()> {
            run("syntax/multiple_virtual_specifiers", "receive_functions")
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

    mod invalid_base {
        use super::*;

        #[test]
        fn function() -> Result<()> {
            run("type_system/invalid_base", "function")
        }

        #[test]
        fn library() -> Result<()> {
            run("type_system/invalid_base", "library")
        }
    }
}
