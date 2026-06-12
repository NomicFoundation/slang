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

    mod identifier_redeclaration {
        use super::*;

        #[test]
        fn constant_redefinition() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "constant_redefinition",
            )
        }

        #[test]
        fn constant_vs_function() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "constant_vs_function",
            )
        }

        #[test]
        fn cross_file_constant() -> Result<()> {
            run("resolution/identifier_redeclaration", "cross_file_constant")
        }

        #[test]
        fn default_import_struct_vs_contract() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "default_import_struct_vs_contract",
            )
        }

        #[test]
        fn default_import_vs_contract() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "default_import_vs_contract",
            )
        }

        #[test]
        fn duplicate_imported_symbol() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "duplicate_imported_symbol",
            )
        }

        #[test]
        fn duplicate_symbol_via_default_import() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "duplicate_symbol_via_default_import",
            )
        }

        #[test]
        fn enum_member() -> Result<()> {
            run("resolution/identifier_redeclaration", "enum_member")
        }

        #[test]
        fn error_no_overloading() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "error_no_overloading",
            )
        }

        #[test]
        fn error_vs_function() -> Result<()> {
            run("resolution/identifier_redeclaration", "error_vs_function")
        }

        #[test]
        fn free_function_vs_contract() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "free_function_vs_contract",
            )
        }

        #[test]
        fn function_event_clash() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "function_event_clash",
            )
        }

        #[test]
        fn imported_symbol_alias() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "imported_symbol_alias",
            )
        }

        #[test]
        fn local_variable() -> Result<()> {
            run("resolution/identifier_redeclaration", "local_variable")
        }

        #[test]
        fn local_variable_disjoint_scope() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "local_variable_disjoint_scope",
            )
        }

        #[test]
        fn path_import_alias_vs_contract() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "path_import_alias_vs_contract",
            )
        }

        #[test]
        fn state_variable() -> Result<()> {
            run("resolution/identifier_redeclaration", "state_variable")
        }

        #[test]
        fn struct_member() -> Result<()> {
            run("resolution/identifier_redeclaration", "struct_member")
        }

        #[test]
        fn top_level() -> Result<()> {
            run("resolution/identifier_redeclaration", "top_level")
        }

        #[test]
        fn transient_state_variable() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "transient_state_variable",
            )
        }

        #[test]
        fn transitive_default_import() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "transitive_default_import",
            )
        }

        #[test]
        fn user_defined_value_type() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "user_defined_value_type",
            )
        }

        #[test]
        fn variable_vs_function() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "variable_vs_function",
            )
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

    mod unexpected_terminal {
        use super::*;

        #[test]
        fn in_expression() -> Result<()> {
            run("syntax/unexpected_terminal", "in_expression")
        }

        #[test]
        fn trailing_token() -> Result<()> {
            run("syntax/unexpected_terminal", "trailing_token")
        }
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
