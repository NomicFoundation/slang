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

mod resolution {
    use super::*;

    mod built_in_redeclaration {
        use super::*;

        #[test]
        fn supersedes_external_shadowing() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "supersedes_external_shadowing",
            )
        }

        #[test]
        fn yul_function() -> Result<()> {
            run("resolution/built_in_redeclaration", "yul_function")
        }

        #[test]
        fn yul_function_parameter() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "yul_function_parameter",
            )
        }

        #[test]
        fn yul_function_return() -> Result<()> {
            run("resolution/built_in_redeclaration", "yul_function_return")
        }

        #[test]
        fn yul_variable() -> Result<()> {
            run("resolution/built_in_redeclaration", "yul_variable")
        }
    }

    mod external_declaration_shadowing {
        use super::*;

        #[test]
        fn yul_for_loop_variable_shadows_built_in() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_for_loop_variable_shadows_built_in",
            )
        }

        #[test]
        fn yul_shadow_constant() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_shadow_constant",
            )
        }

        #[test]
        fn yul_shadow_contract_name() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_shadow_contract_name",
            )
        }

        #[test]
        fn yul_shadow_function_name() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_shadow_function_name",
            )
        }

        #[test]
        fn yul_shadow_function_parameter() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_shadow_function_parameter",
            )
        }

        #[test]
        fn yul_shadow_import_alias() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_shadow_import_alias",
            )
        }

        #[test]
        fn yul_variable_shadows_built_in() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_variable_shadows_built_in",
            )
        }

        #[test]
        fn yul_variable_shadows_default_import() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_variable_shadows_default_import",
            )
        }

        #[test]
        fn yul_variable_shadows_external_function() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_variable_shadows_external_function",
            )
        }

        #[test]
        fn yul_variable_shadows_inherited_member() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_variable_shadows_inherited_member",
            )
        }

        #[test]
        fn yul_variable_vs_solidity_local() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_variable_vs_solidity_local",
            )
        }
    }

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
        fn constant_shadows_private_base() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "constant_shadows_private_base",
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
        fn inherited_constant() -> Result<()> {
            run("resolution/identifier_redeclaration", "inherited_constant")
        }

        #[test]
        fn inherited_cross_kind() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_cross_kind",
            )
        }

        #[test]
        fn inherited_diamond_redeclared() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_diamond_redeclared",
            )
        }

        #[test]
        fn inherited_diamond_sibling() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_diamond_sibling",
            )
        }

        #[test]
        fn inherited_error() -> Result<()> {
            run("resolution/identifier_redeclaration", "inherited_error")
        }

        #[test]
        fn inherited_event_function_clash() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_event_function_clash",
            )
        }

        #[test]
        fn inherited_event_overload() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_event_overload",
            )
        }

        #[test]
        fn inherited_function_error_clash() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_function_error_clash",
            )
        }

        #[test]
        fn inherited_function_modifier_clash() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_function_modifier_clash",
            )
        }

        #[test]
        fn inherited_function_overload() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_function_overload",
            )
        }

        #[test]
        fn inherited_function_override() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_function_override",
            )
        }

        #[test]
        fn inherited_grandparent() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_grandparent",
            )
        }

        #[test]
        fn inherited_in_interfaces() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_in_interfaces",
            )
        }

        #[test]
        fn inherited_modifier_variable_clash() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_modifier_variable_clash",
            )
        }

        #[test]
        fn inherited_private_member() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_private_member",
            )
        }

        #[test]
        fn inherited_state_variable() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_state_variable",
            )
        }

        #[test]
        fn inherited_struct() -> Result<()> {
            run("resolution/identifier_redeclaration", "inherited_struct")
        }

        #[test]
        fn inherited_via_interface() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "inherited_via_interface",
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
        fn modifier_overload() -> Result<()> {
            run("resolution/identifier_redeclaration", "modifier_overload")
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
        fn yul_function_parameter_shadows_built_in() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_parameter_shadows_built_in",
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
        fn yul_function_return_shadows_built_in() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_return_shadows_built_in",
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
        fn yul_function_shadows_built_in() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "yul_function_shadows_built_in",
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

    mod recursive_structs {
        use super::*;

        #[test]
        fn cycle_feeds_deep_chain() -> Result<()> {
            run("semantic/recursive_structs", "cycle_feeds_deep_chain")
        }

        #[test]
        fn depth_exhausted() -> Result<()> {
            run("semantic/recursive_structs", "depth_exhausted")
        }

        #[test]
        fn direct() -> Result<()> {
            run("semantic/recursive_structs", "direct")
        }

        #[test]
        fn direct_dynamic_array() -> Result<()> {
            run("semantic/recursive_structs", "direct_dynamic_array")
        }

        #[test]
        fn direct_fixed_array() -> Result<()> {
            run("semantic/recursive_structs", "direct_fixed_array")
        }

        #[test]
        fn file_level() -> Result<()> {
            run("semantic/recursive_structs", "file_level")
        }

        #[test]
        fn indirect() -> Result<()> {
            run("semantic/recursive_structs", "indirect")
        }

        #[test]
        fn indirect_complex() -> Result<()> {
            run("semantic/recursive_structs", "indirect_complex")
        }

        #[test]
        fn indirect_dynamic_array1() -> Result<()> {
            run("semantic/recursive_structs", "indirect_dynamic_array1")
        }

        #[test]
        fn indirect_dynamic_array2() -> Result<()> {
            run("semantic/recursive_structs", "indirect_dynamic_array2")
        }

        #[test]
        fn indirect_dynamic_array3() -> Result<()> {
            run("semantic/recursive_structs", "indirect_dynamic_array3")
        }

        #[test]
        fn indirect_dynamic_multi_array() -> Result<()> {
            run("semantic/recursive_structs", "indirect_dynamic_multi_array")
        }

        #[test]
        fn indirect_fixed_array1() -> Result<()> {
            run("semantic/recursive_structs", "indirect_fixed_array1")
        }

        #[test]
        fn indirect_fixed_array2() -> Result<()> {
            run("semantic/recursive_structs", "indirect_fixed_array2")
        }

        #[test]
        fn indirect_fixed_array3() -> Result<()> {
            run("semantic/recursive_structs", "indirect_fixed_array3")
        }

        #[test]
        fn indirect_fixed_multi_array() -> Result<()> {
            run("semantic/recursive_structs", "indirect_fixed_multi_array")
        }

        #[test]
        fn not_really_recursive() -> Result<()> {
            run("semantic/recursive_structs", "not_really_recursive")
        }

        #[test]
        fn not_really_recursive_array() -> Result<()> {
            run("semantic/recursive_structs", "not_really_recursive_array")
        }

        #[test]
        fn tail_struct() -> Result<()> {
            run("semantic/recursive_structs", "tail_struct")
        }

        #[test]
        fn via_mapping() -> Result<()> {
            run("semantic/recursive_structs", "via_mapping")
        }
    }
}

mod structure {
    use super::*;

    #[test]
    fn abstract_contract_public_constructor() -> Result<()> {
        run("structure", "abstract_contract_public_constructor")
    }

    #[test]
    fn break_outside_loop() -> Result<()> {
        run("structure", "break_outside_loop")
    }

    #[test]
    fn constructor_in_interface() -> Result<()> {
        run("structure", "constructor_in_interface")
    }

    #[test]
    fn constructor_in_library() -> Result<()> {
        run("structure", "constructor_in_library")
    }

    #[test]
    fn continue_outside_loop() -> Result<()> {
        run("structure", "continue_outside_loop")
    }

    mod contract_should_be_abstract {
        use super::*;

        #[test]
        fn fully_implemented() -> Result<()> {
            run("structure/contract_should_be_abstract", "fully_implemented")
        }

        #[test]
        fn function_and_modifier() -> Result<()> {
            run(
                "structure/contract_should_be_abstract",
                "function_and_modifier",
            )
        }

        #[test]
        fn getter_implements_interface() -> Result<()> {
            run(
                "structure/contract_should_be_abstract",
                "getter_implements_interface",
            )
        }

        #[test]
        fn inherited_function() -> Result<()> {
            run(
                "structure/contract_should_be_abstract",
                "inherited_function",
            )
        }

        #[test]
        fn unimplemented_function() -> Result<()> {
            run(
                "structure/contract_should_be_abstract",
                "unimplemented_function",
            )
        }

        #[test]
        fn unimplemented_interface() -> Result<()> {
            run(
                "structure/contract_should_be_abstract",
                "unimplemented_interface",
            )
        }

        #[test]
        fn unimplemented_modifier() -> Result<()> {
            run(
                "structure/contract_should_be_abstract",
                "unimplemented_modifier",
            )
        }
    }

    #[test]
    fn empty_enum() -> Result<()> {
        run("structure", "empty_enum")
    }

    #[test]
    fn empty_struct() -> Result<()> {
        run("structure", "empty_struct")
    }

    #[test]
    fn enum_with_too_many_members() -> Result<()> {
        run("structure", "enum_with_too_many_members")
    }

    #[test]
    fn free_function_payable() -> Result<()> {
        run("structure", "free_function_payable")
    }

    #[test]
    fn free_function_visibility() -> Result<()> {
        run("structure", "free_function_visibility")
    }

    #[test]
    fn free_function_without_body() -> Result<()> {
        run("structure", "free_function_without_body")
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

    #[test]
    fn interface_function_cannot_be_implemented() -> Result<()> {
        run("structure", "interface_function_cannot_be_implemented")
    }

    #[test]
    fn interface_function_not_external() -> Result<()> {
        run("structure", "interface_function_not_external")
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
    fn library_function_without_body() -> Result<()> {
        run("structure", "library_function_without_body")
    }

    #[test]
    fn library_payable_function() -> Result<()> {
        run("structure", "library_payable_function")
    }

    #[test]
    fn library_receive_function() -> Result<()> {
        run("structure", "library_receive_function")
    }

    #[test]
    fn library_virtual_function() -> Result<()> {
        run("structure", "library_virtual_function")
    }

    #[test]
    fn library_virtual_modifier() -> Result<()> {
        run("structure", "library_virtual_modifier")
    }

    #[test]
    fn missing_function_visibility() -> Result<()> {
        run("structure", "missing_function_visibility")
    }

    #[test]
    fn modifier_in_interface() -> Result<()> {
        run("structure", "modifier_in_interface")
    }

    #[test]
    fn multiple_constructors() -> Result<()> {
        run("structure", "multiple_constructors")
    }

    #[test]
    fn non_abstract_contract_internal_constructor() -> Result<()> {
        run("structure", "non_abstract_contract_internal_constructor")
    }

    mod payable_function_type_must_be_external {
        use super::*;

        #[test]
        fn default_visibility() -> Result<()> {
            run(
                "structure/payable_function_type_must_be_external",
                "default_visibility",
            )
        }

        #[test]
        fn explicit_internal() -> Result<()> {
            run(
                "structure/payable_function_type_must_be_external",
                "explicit_internal",
            )
        }

        #[test]
        fn external_allowed() -> Result<()> {
            run(
                "structure/payable_function_type_must_be_external",
                "external_allowed",
            )
        }

        #[test]
        fn non_payable() -> Result<()> {
            run(
                "structure/payable_function_type_must_be_external",
                "non_payable",
            )
        }
    }

    #[test]
    fn payable_internal_or_private_function() -> Result<()> {
        run("structure", "payable_internal_or_private_function")
    }

    #[test]
    fn storage_layout_for_abstract_contract() -> Result<()> {
        run("structure", "storage_layout_for_abstract_contract")
    }

    #[test]
    fn unimplemented_modifier_must_be_virtual() -> Result<()> {
        run("structure", "unimplemented_modifier_must_be_virtual")
    }

    #[test]
    fn uninitialized_constant() -> Result<()> {
        run("structure", "uninitialized_constant")
    }

    #[test]
    fn uninitialized_file_level_constant() -> Result<()> {
        run("structure", "uninitialized_file_level_constant")
    }

    #[test]
    fn virtual_free_function() -> Result<()> {
        run("structure", "virtual_free_function")
    }

    #[test]
    fn virtual_private_function() -> Result<()> {
        run("structure", "virtual_private_function")
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

    mod invalid_mutability {
        use super::*;

        #[test]
        fn receive_function() -> Result<()> {
            run("syntax/invalid_mutability", "receive_function")
        }
    }

    mod invalid_visibility {
        use super::*;

        #[test]
        fn fallback_function() -> Result<()> {
            run("syntax/invalid_visibility", "fallback_function")
        }

        mod function_type {
            use super::*;

            #[test]
            fn external() -> Result<()> {
                run("syntax/invalid_visibility/function_type", "external")
            }

            #[test]
            fn local_variable() -> Result<()> {
                run("syntax/invalid_visibility/function_type", "local_variable")
            }

            #[test]
            fn nested() -> Result<()> {
                run("syntax/invalid_visibility/function_type", "nested")
            }

            #[test]
            fn parameter() -> Result<()> {
                run("syntax/invalid_visibility/function_type", "parameter")
            }

            #[test]
            fn return_type() -> Result<()> {
                run("syntax/invalid_visibility/function_type", "return_type")
            }

            #[test]
            fn state_variable() -> Result<()> {
                run("syntax/invalid_visibility/function_type", "state_variable")
            }

            #[test]
            fn struct_field() -> Result<()> {
                run("syntax/invalid_visibility/function_type", "struct_field")
            }
        }

        #[test]
        fn receive_function() -> Result<()> {
            run("syntax/invalid_visibility", "receive_function")
        }
    }

    #[test]
    fn more_than_one_inheritance_list() -> Result<()> {
        run("syntax", "more_than_one_inheritance_list")
    }

    #[test]
    fn more_than_one_storage_layout() -> Result<()> {
        run("syntax", "more_than_one_storage_layout")
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

    mod multiple_visibility_specifiers {
        use super::*;

        #[test]
        fn constructors() -> Result<()> {
            run("syntax/multiple_visibility_specifiers", "constructors")
        }

        #[test]
        fn fallback_functions() -> Result<()> {
            run(
                "syntax/multiple_visibility_specifiers",
                "fallback_functions",
            )
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
        fn receive_functions() -> Result<()> {
            run("syntax/multiple_visibility_specifiers", "receive_functions")
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

    mod array_length {
        use super::*;

        #[test]
        fn address_constant_length() -> Result<()> {
            run("type_system/array_length", "address_constant_length")
        }

        #[test]
        fn arithmetic_overflow() -> Result<()> {
            run("type_system/array_length", "arithmetic_overflow")
        }

        #[test]
        fn arithmetic_overflow_binary() -> Result<()> {
            run("type_system/array_length", "arithmetic_overflow_binary")
        }

        #[test]
        fn cast() -> Result<()> {
            run("type_system/array_length", "cast")
        }

        #[test]
        fn compound_expression() -> Result<()> {
            run("type_system/array_length", "compound_expression")
        }

        #[test]
        fn division_by_zero() -> Result<()> {
            run("type_system/array_length", "division_by_zero")
        }

        #[test]
        fn forward_reference() -> Result<()> {
            run("type_system/array_length", "forward_reference")
        }

        #[test]
        fn fractional() -> Result<()> {
            run("type_system/array_length", "fractional")
        }

        #[test]
        fn function_call_constant() -> Result<()> {
            run("type_system/array_length", "function_call_constant")
        }

        #[test]
        fn function_value() -> Result<()> {
            run("type_system/array_length", "function_value")
        }

        #[test]
        fn huge_scientific_literal() -> Result<()> {
            run("type_system/array_length", "huge_scientific_literal")
        }

        #[test]
        fn incompatible_operator() -> Result<()> {
            run("type_system/array_length", "incompatible_operator")
        }

        #[test]
        fn incompatible_operator_rational() -> Result<()> {
            run("type_system/array_length", "incompatible_operator_rational")
        }

        #[test]
        fn innermost_operation() -> Result<()> {
            run("type_system/array_length", "innermost_operation")
        }

        #[test]
        fn literal_fractional_division() -> Result<()> {
            run("type_system/array_length", "literal_fractional_division")
        }

        #[test]
        fn negative() -> Result<()> {
            run("type_system/array_length", "negative")
        }

        #[test]
        fn negative_exponent() -> Result<()> {
            run("type_system/array_length", "negative_exponent")
        }

        #[test]
        fn non_integer_value() -> Result<()> {
            run("type_system/array_length", "non_integer_value")
        }

        #[test]
        fn not_constant() -> Result<()> {
            run("type_system/array_length", "not_constant")
        }

        #[test]
        fn too_large() -> Result<()> {
            run("type_system/array_length", "too_large")
        }

        #[test]
        fn valid() -> Result<()> {
            run("type_system/array_length", "valid")
        }

        #[test]
        fn zero() -> Result<()> {
            run("type_system/array_length", "zero")
        }
    }

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

    mod storage_layout_base_slot {
        use super::*;

        #[test]
        fn address_constant() -> Result<()> {
            run("type_system/storage_layout_base_slot", "address_constant")
        }

        #[test]
        fn arithmetic_overflow() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "arithmetic_overflow",
            )
        }

        #[test]
        fn bitwise_negation_after_cast() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "bitwise_negation_after_cast",
            )
        }

        #[test]
        fn bitwise_negation_literal() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "bitwise_negation_literal",
            )
        }

        #[test]
        fn bytes_constant() -> Result<()> {
            run("type_system/storage_layout_base_slot", "bytes_constant")
        }

        #[test]
        fn cast() -> Result<()> {
            run("type_system/storage_layout_base_slot", "cast")
        }

        #[test]
        fn constant_initialized_from_cast() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "constant_initialized_from_cast",
            )
        }

        #[test]
        fn constant_member_access() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "constant_member_access",
            )
        }

        #[test]
        fn fractional() -> Result<()> {
            run("type_system/storage_layout_base_slot", "fractional")
        }

        #[test]
        fn int_constant_negative() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "int_constant_negative",
            )
        }

        #[test]
        fn integer_valued_rational() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "integer_valued_rational",
            )
        }

        #[test]
        fn negative() -> Result<()> {
            run("type_system/storage_layout_base_slot", "negative")
        }

        #[test]
        fn non_integer_type() -> Result<()> {
            run("type_system/storage_layout_base_slot", "non_integer_type")
        }

        #[test]
        fn not_constant() -> Result<()> {
            run("type_system/storage_layout_base_slot", "not_constant")
        }

        #[test]
        fn out_of_range() -> Result<()> {
            run("type_system/storage_layout_base_slot", "out_of_range")
        }

        #[test]
        fn out_of_range_expressions() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "out_of_range_expressions",
            )
        }

        #[test]
        fn type_max() -> Result<()> {
            run("type_system/storage_layout_base_slot", "type_max")
        }

        #[test]
        fn units() -> Result<()> {
            run("type_system/storage_layout_base_slot", "units")
        }

        #[test]
        fn unlimited_arithmetic() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "unlimited_arithmetic",
            )
        }

        #[test]
        fn user_defined_value_type() -> Result<()> {
            run(
                "type_system/storage_layout_base_slot",
                "user_defined_value_type",
            )
        }

        #[test]
        fn valid() -> Result<()> {
            run("type_system/storage_layout_base_slot", "valid")
        }
    }
}
