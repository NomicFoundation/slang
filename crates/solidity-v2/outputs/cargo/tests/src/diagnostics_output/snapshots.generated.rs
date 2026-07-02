// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::diagnostics_output::runner::run;

mod compilation {
    use super::*;

    mod missing_imported_file {
        use super::*;

        #[test]
        fn simple() -> Result<()> {
            run("compilation/missing_imported_file", "simple")
        }
    }

    #[test]
    fn unresolved_import() -> Result<()> {
        run("compilation", "unresolved_import")
    }
}

mod passing {
    use super::*;

    #[test]
    fn function_external_delete_storage() -> Result<()> {
        run("passing", "function_external_delete_storage")
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
        fn function_parameter() -> Result<()> {
            run("resolution/identifier_redeclaration", "function_parameter")
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
        fn parameter_vs_return_parameter() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "parameter_vs_return_parameter",
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
        fn path_import_alias_vs_default_import() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "path_import_alias_vs_default_import",
            )
        }

        #[test]
        fn return_parameter() -> Result<()> {
            run("resolution/identifier_redeclaration", "return_parameter")
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

        #[test]
        fn yul_for_loop_variable() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_for_loop_variable",
            )
        }

        #[test]
        fn yul_function() -> Result<()> {
            run("resolution/identifier_redeclaration", "yul_function")
        }

        #[test]
        fn yul_function_parameter() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_parameter",
            )
        }

        #[test]
        fn yul_function_parameter_shadows_local() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_parameter_shadows_local",
            )
        }

        #[test]
        fn yul_function_parameter_shadows_parameter() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_parameter_shadows_parameter",
            )
        }

        #[test]
        fn yul_function_parameter_vs_return() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_parameter_vs_return",
            )
        }

        #[test]
        fn yul_function_return_shadows_local() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_return_shadows_local",
            )
        }

        #[test]
        fn yul_function_return_vs_outer_variable() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_return_vs_outer_variable",
            )
        }

        #[test]
        fn yul_function_shadows_function_with_state_access() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_shadows_function_with_state_access",
            )
        }

        #[test]
        fn yul_function_shadows_solidity() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_shadows_solidity",
            )
        }

        #[test]
        fn yul_function_signature_shadows_function() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_signature_shadows_function",
            )
        }

        #[test]
        fn yul_function_signature_shadows_state_var() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_signature_shadows_state_var",
            )
        }

        #[test]
        fn yul_function_vs_subscope_function() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_vs_subscope_function",
            )
        }

        #[test]
        fn yul_function_vs_subscope_variable() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_vs_subscope_variable",
            )
        }

        #[test]
        fn yul_multi_variable_declaration() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_multi_variable_declaration",
            )
        }

        #[test]
        fn yul_nested_function_redefinition() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_nested_function_redefinition",
            )
        }

        #[test]
        fn yul_shadow_constant() -> Result<()> {
            run("resolution/identifier_redeclaration", "yul_shadow_constant")
        }

        #[test]
        fn yul_shadow_contract_name() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_shadow_contract_name",
            )
        }

        #[test]
        fn yul_shadow_function_name() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_shadow_function_name",
            )
        }

        #[test]
        fn yul_shadow_function_parameter() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_shadow_function_parameter",
            )
        }

        #[test]
        fn yul_shadow_import_alias() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_shadow_import_alias",
            )
        }

        #[test]
        fn yul_variable() -> Result<()> {
            run("resolution/identifier_redeclaration", "yul_variable")
        }

        #[test]
        fn yul_variable_nested_block() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_variable_nested_block",
            )
        }

        #[test]
        fn yul_variable_shadows_default_import() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_variable_shadows_default_import",
            )
        }

        #[test]
        fn yul_variable_shadows_external_function() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_variable_shadows_external_function",
            )
        }

        #[test]
        fn yul_variable_shadows_inherited_member() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_variable_shadows_inherited_member",
            )
        }

        #[test]
        fn yul_variable_shadows_state_variable() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_variable_shadows_state_variable",
            )
        }

        #[test]
        fn yul_variable_vs_enclosing_yul_function() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_variable_vs_enclosing_yul_function",
            )
        }

        #[test]
        fn yul_variable_vs_solidity_local() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_variable_vs_solidity_local",
            )
        }

        #[test]
        fn yul_variable_vs_yul_function() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_variable_vs_yul_function",
            )
        }
    }

    mod incompatible_built_in_target {
        use super::*;

        #[test]
        fn blobhash() -> Result<()> {
            run("resolution/incompatible_built_in_target", "blobhash")
        }
    }

    mod incompatible_built_in_version {
        use super::*;

        #[test]
        fn event_selector() -> Result<()> {
            run("resolution/incompatible_built_in_version", "event_selector")
        }
    }
}

mod semantic {
    use super::*;

    mod constant_cycles {
        use super::*;

        mod array_size {
            use super::*;

            #[test]
            fn cyclic() -> Result<()> {
                run("semantic/constant_cycles/array_size", "cyclic")
            }

            #[test]
            fn evaluation_depth_exceeded() -> Result<()> {
                run(
                    "semantic/constant_cycles/array_size",
                    "evaluation_depth_exceeded",
                )
            }
        }

        mod constants {
            use super::*;

            #[test]
            fn approach_declared_first_exceeds_depth_limit_a1_b1() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "approach_declared_first_exceeds_depth_limit_a1_b1",
                )
            }

            #[test]
            fn approach_declared_first_exceeds_depth_limit_b1_a1() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "approach_declared_first_exceeds_depth_limit_b1_a1",
                )
            }

            #[test]
            fn cycle_through_call_arguments() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "cycle_through_call_arguments",
                )
            }

            #[test]
            fn dependency_depth_boundary() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "dependency_depth_boundary",
                )
            }

            #[test]
            fn dependency_depth_exceeded() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "dependency_depth_exceeded",
                )
            }

            #[test]
            fn imported_cycle() -> Result<()> {
                run("semantic/constant_cycles/constants", "imported_cycle")
            }

            #[test]
            fn indirect_cycle() -> Result<()> {
                run("semantic/constant_cycles/constants", "indirect_cycle")
            }

            #[test]
            fn member_access_cycle() -> Result<()> {
                run("semantic/constant_cycles/constants", "member_access_cycle")
            }

            #[test]
            fn module_member_access_cycle() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "module_member_access_cycle",
                )
            }

            #[test]
            fn parenthesized_member_access_cycle() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "parenthesized_member_access_cycle",
                )
            }

            #[test]
            fn qualified_self_reference_cycle() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "qualified_self_reference_cycle",
                )
            }

            #[test]
            fn shared_dependency_past_depth_limit() -> Result<()> {
                run(
                    "semantic/constant_cycles/constants",
                    "shared_dependency_past_depth_limit",
                )
            }

            #[test]
            fn standalone_cycle() -> Result<()> {
                run("semantic/constant_cycles/constants", "standalone_cycle")
            }
        }

        mod storage_base_slot {
            use super::*;

            #[test]
            fn cyclic() -> Result<()> {
                run("semantic/constant_cycles/storage_base_slot", "cyclic")
            }

            #[test]
            fn evaluation_depth_exceeded() -> Result<()> {
                run(
                    "semantic/constant_cycles/storage_base_slot",
                    "evaluation_depth_exceeded",
                )
            }
        }
    }

    mod cyclic_inheritance {
        use super::*;

        #[test]
        fn cross_file() -> Result<()> {
            run("semantic/cyclic_inheritance", "cross_file")
        }

        #[test]
        fn deep() -> Result<()> {
            run("semantic/cyclic_inheritance", "deep")
        }

        #[test]
        fn mutual() -> Result<()> {
            run("semantic/cyclic_inheritance", "mutual")
        }

        #[test]
        fn self_inheritance() -> Result<()> {
            run("semantic/cyclic_inheritance", "self_inheritance")
        }
    }

    mod linearisation_impossible {
        use super::*;

        #[test]
        fn contract() -> Result<()> {
            run("semantic/linearisation_impossible", "contract")
        }

        #[test]
        fn interface() -> Result<()> {
            run("semantic/linearisation_impossible", "interface")
        }
    }
}

mod structure {
    use super::*;

    #[test]
    fn empty_enum() -> Result<()> {
        run("structure", "empty_enum")
    }

    #[test]
    fn enum_with_too_many_members() -> Result<()> {
        run("structure", "enum_with_too_many_members")
    }

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
    fn library_fallback_function() -> Result<()> {
        run("structure", "library_fallback_function")
    }

    #[test]
    fn library_receive_function() -> Result<()> {
        run("structure", "library_receive_function")
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

    mod incompatible_syntax_version {
        use super::*;

        #[test]
        fn error_definition() -> Result<()> {
            run("syntax/incompatible_syntax_version", "error_definition")
        }

        #[test]
        fn storage_layout_specifier() -> Result<()> {
            run(
                "syntax/incompatible_syntax_version",
                "storage_layout_specifier",
            )
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
}

mod type_system {
    use super::*;

    #[test]
    fn fallback_function_mutability() -> Result<()> {
        run("type_system", "fallback_function_mutability")
    }

    #[test]
    fn fallback_function_signature() -> Result<()> {
        run("type_system", "fallback_function_signature")
    }

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

    mod invalid_function_type_visibility {
        use super::*;

        #[test]
        fn local_variable() -> Result<()> {
            run(
                "type_system/invalid_function_type_visibility",
                "local_variable",
            )
        }

        #[test]
        fn nested() -> Result<()> {
            run("type_system/invalid_function_type_visibility", "nested")
        }

        #[test]
        fn parameter() -> Result<()> {
            run("type_system/invalid_function_type_visibility", "parameter")
        }

        #[test]
        fn return_type() -> Result<()> {
            run(
                "type_system/invalid_function_type_visibility",
                "return_type",
            )
        }

        #[test]
        fn state_variable() -> Result<()> {
            run(
                "type_system/invalid_function_type_visibility",
                "state_variable",
            )
        }

        #[test]
        fn struct_field() -> Result<()> {
            run(
                "type_system/invalid_function_type_visibility",
                "struct_field",
            )
        }

        #[test]
        fn valid_visibilities() -> Result<()> {
            run(
                "type_system/invalid_function_type_visibility",
                "valid_visibilities",
            )
        }
    }

    mod receive_function_parameters {
        use super::*;

        #[test]
        fn contract() -> Result<()> {
            run("type_system/receive_function_parameters", "contract")
        }

        #[test]
        fn interface() -> Result<()> {
            run("type_system/receive_function_parameters", "interface")
        }
    }
}
