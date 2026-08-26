// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::diagnostics_output::runner::run;

mod compilation {
    use super::*;

    mod missing_imported_file {
        use super::*;

        #[test]
        fn among_other_imports() -> Result<()> {
            run("compilation/missing_imported_file", "among_other_imports")
        }

        #[test]
        fn escapes_root() -> Result<()> {
            run("compilation/missing_imported_file", "escapes_root")
        }

        #[test]
        fn simple() -> Result<()> {
            run("compilation/missing_imported_file", "simple")
        }
    }
}

mod resolution {
    use super::*;

    mod ambiguous_reference {
        use super::*;

        #[test]
        fn attached_function_clash() -> Result<()> {
            run("resolution/ambiguous_reference", "attached_function_clash")
        }

        #[test]
        fn member_call_not_unique() -> Result<()> {
            run("resolution/ambiguous_reference", "member_call_not_unique")
        }

        #[test]
        fn member_not_unique() -> Result<()> {
            run("resolution/ambiguous_reference", "member_not_unique")
        }

        #[test]
        fn member_via_contract_name() -> Result<()> {
            run("resolution/ambiguous_reference", "member_via_contract_name")
        }

        #[test]
        fn overloaded_call() -> Result<()> {
            run("resolution/ambiguous_reference", "overloaded_call")
        }

        #[test]
        fn overloaded_call_named_arguments() -> Result<()> {
            run(
                "resolution/ambiguous_reference",
                "overloaded_call_named_arguments",
            )
        }

        #[test]
        fn overloaded_call_parenthesised() -> Result<()> {
            run(
                "resolution/ambiguous_reference",
                "overloaded_call_parenthesised",
            )
        }

        #[test]
        fn overloaded_event() -> Result<()> {
            run("resolution/ambiguous_reference", "overloaded_event")
        }

        #[test]
        fn overloaded_event_selector() -> Result<()> {
            run(
                "resolution/ambiguous_reference",
                "overloaded_event_selector",
            )
        }

        #[test]
        fn overloaded_function_as_value() -> Result<()> {
            run(
                "resolution/ambiguous_reference",
                "overloaded_function_as_value",
            )
        }

        #[test]
        fn public_variable_shadows_base_function() -> Result<()> {
            run(
                "resolution/ambiguous_reference",
                "public_variable_shadows_base_function",
            )
        }
    }

    mod ambiguous_yul_reference {
        use super::*;

        #[test]
        fn overloaded_reference() -> Result<()> {
            run("resolution/ambiguous_yul_reference", "overloaded_reference")
        }
    }

    mod built_in_redeclaration {
        use super::*;

        #[test]
        fn not_reserved_before_own_version() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "not_reserved_before_own_version",
            )
        }

        #[test]
        fn not_yet_reserved_until_fork() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "not_yet_reserved_until_fork",
            )
        }

        #[test]
        fn reference_to_unavailable_built_in_name() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "reference_to_unavailable_built_in_name",
            )
        }

        #[test]
        fn reserved_after_own_fork() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "reserved_after_own_fork",
            )
        }

        #[test]
        fn reserved_before_own_fork() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "reserved_before_own_fork",
            )
        }

        #[test]
        fn supersedes_external_shadowing() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "supersedes_external_shadowing",
            )
        }

        #[test]
        fn undeclared_unavailable_built_in_reference() -> Result<()> {
            run(
                "resolution/built_in_redeclaration",
                "undeclared_unavailable_built_in_reference",
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

    mod duplicate_event_definition {
        use super::*;

        #[test]
        fn contract_members() -> Result<()> {
            run("resolution/duplicate_event_definition", "contract_members")
        }

        #[test]
        fn distinguishable_events() -> Result<()> {
            run(
                "resolution/duplicate_event_definition",
                "distinguishable_events",
            )
        }

        #[test]
        fn free_events() -> Result<()> {
            run("resolution/duplicate_event_definition", "free_events")
        }

        #[test]
        fn inherited() -> Result<()> {
            run("resolution/duplicate_event_definition", "inherited")
        }

        #[test]
        fn library_and_interface() -> Result<()> {
            run(
                "resolution/duplicate_event_definition",
                "library_and_interface",
            )
        }

        #[test]
        fn multiple_files() -> Result<()> {
            run("resolution/duplicate_event_definition", "multiple_files")
        }

        #[test]
        fn redeclared_and_duplicated() -> Result<()> {
            run(
                "resolution/duplicate_event_definition",
                "redeclared_and_duplicated",
            )
        }
    }

    mod duplicate_function_definition {
        use super::*;

        #[test]
        fn circular_imports() -> Result<()> {
            run(
                "resolution/duplicate_function_definition",
                "circular_imports",
            )
        }

        #[test]
        fn contract_members() -> Result<()> {
            run(
                "resolution/duplicate_function_definition",
                "contract_members",
            )
        }

        #[test]
        fn distinguishable_overloads() -> Result<()> {
            run(
                "resolution/duplicate_function_definition",
                "distinguishable_overloads",
            )
        }

        #[test]
        fn free_functions() -> Result<()> {
            run("resolution/duplicate_function_definition", "free_functions")
        }

        #[test]
        fn library_and_interface() -> Result<()> {
            run(
                "resolution/duplicate_function_definition",
                "library_and_interface",
            )
        }

        #[test]
        fn multiple() -> Result<()> {
            run("resolution/duplicate_function_definition", "multiple")
        }

        #[test]
        fn qualified_import() -> Result<()> {
            run(
                "resolution/duplicate_function_definition",
                "qualified_import",
            )
        }

        #[test]
        fn via_default_import() -> Result<()> {
            run(
                "resolution/duplicate_function_definition",
                "via_default_import",
            )
        }

        #[test]
        fn via_import_alias() -> Result<()> {
            run(
                "resolution/duplicate_function_definition",
                "via_import_alias",
            )
        }
    }

    mod expected_function_in_using_directive {
        use super::*;

        #[test]
        fn ambiguous_with_non_function() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "ambiguous_with_non_function",
            )
        }

        #[test]
        fn asterisk_not_a_library() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "asterisk_not_a_library",
            )
        }

        #[test]
        fn builtin_keccak256() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "builtin_keccak256",
            )
        }

        #[test]
        fn builtin_name_in_member_path() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "builtin_name_in_member_path",
            )
        }

        #[test]
        fn contract_member_function() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "contract_member_function",
            )
        }

        #[test]
        fn error() -> Result<()> {
            run("resolution/expected_function_in_using_directive", "error")
        }

        #[test]
        fn event() -> Result<()> {
            run("resolution/expected_function_in_using_directive", "event")
        }

        #[test]
        fn free_and_library_function() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "free_and_library_function",
            )
        }

        #[test]
        fn function_without_braces() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "function_without_braces",
            )
        }

        #[test]
        fn imported_alias() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "imported_alias",
            )
        }

        #[test]
        fn library_form() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "library_form",
            )
        }

        #[test]
        fn library_forward_reference() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "library_forward_reference",
            )
        }

        #[test]
        fn library_not_found() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "library_not_found",
            )
        }

        #[test]
        fn not_a_library() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "not_a_library",
            )
        }

        #[test]
        fn not_found() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "not_found",
            )
        }

        #[test]
        fn operator_non_function() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "operator_non_function",
            )
        }

        #[test]
        fn overloaded_functions() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "overloaded_functions",
            )
        }

        #[test]
        fn state_variable() -> Result<()> {
            run(
                "resolution/expected_function_in_using_directive",
                "state_variable",
            )
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
        fn yul_variable_shadows_blobhash() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_variable_shadows_blobhash",
            )
        }

        #[test]
        fn yul_variable_shadows_blobhash_before_introduction() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_variable_shadows_blobhash_before_introduction",
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
        fn yul_variable_shadows_erc7201() -> Result<()> {
            run(
                "resolution/external_declaration_shadowing",
                "yul_variable_shadows_erc7201",
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
        fn aliased_import_overload_set() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "aliased_import_overload_set",
            )
        }

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
        fn idempotent_default_imports() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "idempotent_default_imports",
            )
        }

        #[test]
        fn idempotent_transitive_import() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "idempotent_transitive_import",
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
        fn library_members() -> Result<()> {
            run("resolution/identifier_redeclaration", "library_members")
        }

        #[test]
        fn local_function_overloads_default_import() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "local_function_overloads_default_import",
            )
        }

        #[test]
        fn local_function_overloads_import() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "local_function_overloads_import",
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
        fn overloaded_and_idempotent_imports() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "overloaded_and_idempotent_imports",
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
        fn single_directive_duplicate_import() -> Result<()> {
            run(
                "resolution/identifier_redeclaration",
                "single_directive_duplicate_import",
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

    mod imported_declaration_not_found {
        use super::*;

        #[test]
        fn aliased_symbol() -> Result<()> {
            run(
                "resolution/imported_declaration_not_found",
                "aliased_symbol",
            )
        }

        #[test]
        fn chained_deconstruction() -> Result<()> {
            run(
                "resolution/imported_declaration_not_found",
                "chained_deconstruction",
            )
        }

        #[test]
        fn dead_end_chain() -> Result<()> {
            run(
                "resolution/imported_declaration_not_found",
                "dead_end_chain",
            )
        }

        #[test]
        fn missing_symbol() -> Result<()> {
            run(
                "resolution/imported_declaration_not_found",
                "missing_symbol",
            )
        }

        #[test]
        fn mutual_imports() -> Result<()> {
            run(
                "resolution/imported_declaration_not_found",
                "mutual_imports",
            )
        }

        #[test]
        fn unresolved_file() -> Result<()> {
            run(
                "resolution/imported_declaration_not_found",
                "unresolved_file",
            )
        }

        #[test]
        fn via_default_import() -> Result<()> {
            run(
                "resolution/imported_declaration_not_found",
                "via_default_import",
            )
        }
    }

    mod incompatible_built_in_target {
        use super::*;

        #[test]
        fn blobhash() -> Result<()> {
            run("resolution/incompatible_built_in_target", "blobhash")
        }

        #[test]
        fn block_difficulty_all_targets() -> Result<()> {
            run(
                "resolution/incompatible_built_in_target",
                "block_difficulty_all_targets",
            )
        }

        #[test]
        fn block_prevrandao_all_targets() -> Result<()> {
            run(
                "resolution/incompatible_built_in_target",
                "block_prevrandao_all_targets",
            )
        }

        #[test]
        fn block_prevrandao_pre_paris() -> Result<()> {
            run(
                "resolution/incompatible_built_in_target",
                "block_prevrandao_pre_paris",
            )
        }

        #[test]
        fn yul_difficulty_post_paris() -> Result<()> {
            run(
                "resolution/incompatible_built_in_target",
                "yul_difficulty_post_paris",
            )
        }

        #[test]
        fn yul_difficulty_pre_paris() -> Result<()> {
            run(
                "resolution/incompatible_built_in_target",
                "yul_difficulty_pre_paris",
            )
        }
    }

    mod incompatible_built_in_version {
        use super::*;

        #[test]
        fn event_selector() -> Result<()> {
            run("resolution/incompatible_built_in_version", "event_selector")
        }
    }

    mod member_not_found {
        use super::*;

        #[test]
        fn address_on_internal_function() -> Result<()> {
            run(
                "resolution/member_not_found",
                "address_on_internal_function",
            )
        }

        #[test]
        fn concat_on_bytes_value() -> Result<()> {
            run("resolution/member_not_found", "concat_on_bytes_value")
        }

        #[test]
        fn constant_via_super() -> Result<()> {
            run("resolution/member_not_found", "constant_via_super")
        }

        #[test]
        fn creation_code_on_abstract_contract() -> Result<()> {
            run(
                "resolution/member_not_found",
                "creation_code_on_abstract_contract",
            )
        }

        #[test]
        fn event_via_super() -> Result<()> {
            run("resolution/member_not_found", "event_via_super")
        }

        #[test]
        fn external_function_via_super() -> Result<()> {
            run("resolution/member_not_found", "external_function_via_super")
        }

        #[test]
        fn internal_function_on_contract_instance() -> Result<()> {
            run(
                "resolution/member_not_found",
                "internal_function_on_contract_instance",
            )
        }

        #[test]
        fn length_on_calldata_slice() -> Result<()> {
            run("resolution/member_not_found", "length_on_calldata_slice")
        }

        #[test]
        fn member_not_found_via_super() -> Result<()> {
            run("resolution/member_not_found", "member_not_found_via_super")
        }

        #[test]
        fn member_on_error_declaration() -> Result<()> {
            run("resolution/member_not_found", "member_on_error_declaration")
        }

        #[test]
        fn member_on_function_value() -> Result<()> {
            run("resolution/member_not_found", "member_on_function_value")
        }

        #[test]
        fn min_on_contract_meta_type() -> Result<()> {
            run("resolution/member_not_found", "min_on_contract_meta_type")
        }

        #[test]
        fn missing_function_in_library() -> Result<()> {
            run("resolution/member_not_found", "missing_function_in_library")
        }

        #[test]
        fn missing_member() -> Result<()> {
            run("resolution/member_not_found", "missing_member")
        }

        #[test]
        fn missing_member_in_module() -> Result<()> {
            run("resolution/member_not_found", "missing_member_in_module")
        }

        #[test]
        fn name_on_integer_meta_type() -> Result<()> {
            run("resolution/member_not_found", "name_on_integer_meta_type")
        }

        #[test]
        fn no_overload_accepts() -> Result<()> {
            run("resolution/member_not_found", "no_overload_accepts")
        }

        #[test]
        fn parameter_on_error_instantiation() -> Result<()> {
            run(
                "resolution/member_not_found",
                "parameter_on_error_instantiation",
            )
        }

        #[test]
        fn pop_on_calldata_slice() -> Result<()> {
            run("resolution/member_not_found", "pop_on_calldata_slice")
        }

        #[test]
        fn pop_on_fixed_size_array() -> Result<()> {
            run("resolution/member_not_found", "pop_on_fixed_size_array")
        }

        #[test]
        fn pop_on_memory_array() -> Result<()> {
            run("resolution/member_not_found", "pop_on_memory_array")
        }

        #[test]
        fn pop_on_string() -> Result<()> {
            run("resolution/member_not_found", "pop_on_string")
        }

        #[test]
        fn pop_on_value_type() -> Result<()> {
            run("resolution/member_not_found", "pop_on_value_type")
        }

        #[test]
        fn public_state_variable_via_super() -> Result<()> {
            run(
                "resolution/member_not_found",
                "public_state_variable_via_super",
            )
        }

        #[test]
        fn push_on_memory_array() -> Result<()> {
            run("resolution/member_not_found", "push_on_memory_array")
        }

        #[test]
        fn runtime_code_on_abstract_contract() -> Result<()> {
            run(
                "resolution/member_not_found",
                "runtime_code_on_abstract_contract",
            )
        }

        #[test]
        fn selector_on_error_instantiation() -> Result<()> {
            run(
                "resolution/member_not_found",
                "selector_on_error_instantiation",
            )
        }

        #[test]
        fn selector_on_error_instantiation_with_named_arguments() -> Result<()> {
            run(
                "resolution/member_not_found",
                "selector_on_error_instantiation_with_named_arguments",
            )
        }

        #[test]
        fn selector_on_event_invocation() -> Result<()> {
            run(
                "resolution/member_not_found",
                "selector_on_event_invocation",
            )
        }

        #[test]
        fn selector_on_internal_function() -> Result<()> {
            run(
                "resolution/member_not_found",
                "selector_on_internal_function",
            )
        }

        #[test]
        fn state_variable_via_super() -> Result<()> {
            run("resolution/member_not_found", "state_variable_via_super")
        }
    }

    mod no_matching_callable_declaration {
        use super::*;

        #[test]
        fn named_arguments_incompatible_types() -> Result<()> {
            run(
                "resolution/no_matching_callable_declaration",
                "named_arguments_incompatible_types",
            )
        }

        #[test]
        fn named_arguments_unknown_name() -> Result<()> {
            run(
                "resolution/no_matching_callable_declaration",
                "named_arguments_unknown_name",
            )
        }

        #[test]
        fn overloaded_event() -> Result<()> {
            run(
                "resolution/no_matching_callable_declaration",
                "overloaded_event",
            )
        }

        #[test]
        fn positional_arguments() -> Result<()> {
            run(
                "resolution/no_matching_callable_declaration",
                "positional_arguments",
            )
        }
    }
}

mod runner {
    use super::*;

    #[test]
    fn diagnostic_without_source_span() -> Result<()> {
        run("runner", "diagnostic_without_source_span")
    }
}

mod semantic {
    use super::*;

    mod bytecode_cycles {
        use super::*;

        #[test]
        fn abstract_contract_reaching_cycle() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "abstract_contract_reaching_cycle",
            )
        }

        #[test]
        fn attached_function_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "attached_function_cycle")
        }

        #[test]
        fn attached_function_on_contract_value() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "attached_function_on_contract_value",
            )
        }

        #[test]
        fn base_constructor_and_derived_initializer() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "base_constructor_and_derived_initializer",
            )
        }

        #[test]
        fn base_constructor_arguments() -> Result<()> {
            run("semantic/bytecode_cycles", "base_constructor_arguments")
        }

        #[test]
        fn base_constructor_modifier_arguments() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "base_constructor_modifier_arguments",
            )
        }

        #[test]
        fn chain_reaching_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "chain_reaching_cycle")
        }

        #[test]
        fn constant_getter_override_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "constant_getter_override_cycle")
        }

        #[test]
        fn constant_value_attribution() -> Result<()> {
            run("semantic/bytecode_cycles", "constant_value_attribution")
        }

        #[test]
        fn creation_code_of_base() -> Result<()> {
            run("semantic/bytecode_cycles", "creation_code_of_base")
        }

        #[test]
        fn creation_code_of_self_and_base() -> Result<()> {
            run("semantic/bytecode_cycles", "creation_code_of_self_and_base")
        }

        #[test]
        fn diamond_super_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "diamond_super_cycle")
        }

        #[test]
        fn external_call_no_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "external_call_no_cycle")
        }

        #[test]
        fn free_function_call_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "free_function_call_cycle")
        }

        #[test]
        fn function_and_fallback_attribution() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "function_and_fallback_attribution",
            )
        }

        #[test]
        fn function_pointer_reference() -> Result<()> {
            run("semantic/bytecode_cycles", "function_pointer_reference")
        }

        #[test]
        fn getter_override_no_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "getter_override_no_cycle")
        }

        #[test]
        fn import_alias_constant_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "import_alias_constant_cycle")
        }

        #[test]
        fn import_alias_function_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "import_alias_function_cycle")
        }

        #[test]
        fn imported_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "imported_cycle")
        }

        #[test]
        fn inherited_entry_point_attribution() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "inherited_entry_point_attribution",
            )
        }

        #[test]
        fn inherited_public_constant_getter() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "inherited_public_constant_getter",
            )
        }

        #[test]
        fn inherited_shared_site() -> Result<()> {
            run("semantic/bytecode_cycles", "inherited_shared_site")
        }

        #[test]
        fn internal_library_call_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "internal_library_call_cycle")
        }

        #[test]
        fn library_creation_code_of_self() -> Result<()> {
            run("semantic/bytecode_cycles", "library_creation_code_of_self")
        }

        #[test]
        fn library_public_constant_getter() -> Result<()> {
            run("semantic/bytecode_cycles", "library_public_constant_getter")
        }

        #[test]
        fn member_access_constant_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "member_access_constant_cycle")
        }

        #[test]
        fn member_access_public_constant_cycle() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "member_access_public_constant_cycle",
            )
        }

        #[test]
        fn modifier_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "modifier_cycle")
        }

        #[test]
        fn modifier_invocation_arguments() -> Result<()> {
            run("semantic/bytecode_cycles", "modifier_invocation_arguments")
        }

        #[test]
        fn mutual_state_variables() -> Result<()> {
            run("semantic/bytecode_cycles", "mutual_state_variables")
        }

        #[test]
        fn new_in_constructor() -> Result<()> {
            run("semantic/bytecode_cycles", "new_in_constructor")
        }

        #[test]
        fn private_library_function_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "private_library_function_cycle")
        }

        #[test]
        fn public_constant_getter() -> Result<()> {
            run("semantic/bytecode_cycles", "public_constant_getter")
        }

        #[test]
        fn public_constant_getter_of_self() -> Result<()> {
            run("semantic/bytecode_cycles", "public_constant_getter_of_self")
        }

        #[test]
        fn public_library_call_no_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "public_library_call_no_cycle")
        }

        #[test]
        fn qualified_modifier_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "qualified_modifier_cycle")
        }

        #[test]
        fn qualified_modifier_override_no_cycle() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "qualified_modifier_override_no_cycle",
            )
        }

        #[test]
        fn receive_and_fallback_attribution() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "receive_and_fallback_attribution",
            )
        }

        #[test]
        fn referenced_constant_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "referenced_constant_cycle")
        }

        #[test]
        fn runtime_code_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "runtime_code_cycle")
        }

        #[test]
        fn selector_access_no_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "selector_access_no_cycle")
        }

        #[test]
        fn shared_dependency_attribution() -> Result<()> {
            run("semantic/bytecode_cycles", "shared_dependency_attribution")
        }

        #[test]
        fn super_call_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "super_call_cycle")
        }

        #[test]
        fn super_unimplemented_override_cycle() -> Result<()> {
            run(
                "semantic/bytecode_cycles",
                "super_unimplemented_override_cycle",
            )
        }

        #[test]
        fn three_contract_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "three_contract_cycle")
        }

        #[test]
        fn unreferenced_constant_no_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "unreferenced_constant_no_cycle")
        }

        #[test]
        fn unused_overload_no_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "unused_overload_no_cycle")
        }

        #[test]
        fn user_defined_operator_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "user_defined_operator_cycle")
        }

        #[test]
        fn validator_exhausted() -> Result<()> {
            run("semantic/bytecode_cycles", "validator_exhausted")
        }

        #[test]
        fn virtual_override_no_cycle() -> Result<()> {
            run("semantic/bytecode_cycles", "virtual_override_no_cycle")
        }
    }

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
            fn public_mixed_cycle() -> Result<()> {
                run("semantic/constant_cycles/constants", "public_mixed_cycle")
            }

            #[test]
            fn public_self_cycle() -> Result<()> {
                run("semantic/constant_cycles/constants", "public_self_cycle")
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

    mod anonymous_event_with_too_many_indexed_parameters {
        use super::*;

        #[test]
        fn at_limit() -> Result<()> {
            run(
                "structure/anonymous_event_with_too_many_indexed_parameters",
                "at_limit",
            )
        }

        #[test]
        fn non_indexed_parameters() -> Result<()> {
            run(
                "structure/anonymous_event_with_too_many_indexed_parameters",
                "non_indexed_parameters",
            )
        }

        #[test]
        fn too_many() -> Result<()> {
            run(
                "structure/anonymous_event_with_too_many_indexed_parameters",
                "too_many",
            )
        }
    }

    #[test]
    fn break_outside_loop() -> Result<()> {
        run("structure", "break_outside_loop")
    }

    mod conflicting_mapping_parameter_name {
        use super::*;

        #[test]
        fn function_type_parameter() -> Result<()> {
            run(
                "structure/conflicting_mapping_parameter_name",
                "function_type_parameter",
            )
        }

        #[test]
        fn function_type_value() -> Result<()> {
            run(
                "structure/conflicting_mapping_parameter_name",
                "function_type_value",
            )
        }

        #[test]
        fn nested_multiple() -> Result<()> {
            run(
                "structure/conflicting_mapping_parameter_name",
                "nested_multiple",
            )
        }

        #[test]
        fn nested_single() -> Result<()> {
            run(
                "structure/conflicting_mapping_parameter_name",
                "nested_single",
            )
        }

        #[test]
        fn same_level() -> Result<()> {
            run("structure/conflicting_mapping_parameter_name", "same_level")
        }

        #[test]
        fn valid() -> Result<()> {
            run("structure/conflicting_mapping_parameter_name", "valid")
        }
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
        fn external_override_data_location() -> Result<()> {
            run(
                "structure/contract_should_be_abstract",
                "external_override_data_location",
            )
        }

        #[test]
        fn external_override_return_type_differs() -> Result<()> {
            run(
                "structure/contract_should_be_abstract",
                "external_override_return_type_differs",
            )
        }

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

    mod duplicate_abicoder_specifier {
        use super::*;

        #[test]
        fn abi_encoder_v2_keywords() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "abi_encoder_v2_keywords",
            )
        }

        #[test]
        fn abi_encoder_v2_mixed_forms() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "abi_encoder_v2_mixed_forms",
            )
        }

        #[test]
        fn abi_encoder_v2_strings() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "abi_encoder_v2_strings",
            )
        }

        #[test]
        fn abicoder_v1_then_experimental() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "abicoder_v1_then_experimental",
            )
        }

        #[test]
        fn abicoder_v1_then_v2() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "abicoder_v1_then_v2",
            )
        }

        #[test]
        fn abicoder_v2() -> Result<()> {
            run("structure/duplicate-abicoder-specifier", "abicoder_v2")
        }

        #[test]
        fn abicoder_v2_then_experimental() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "abicoder_v2_then_experimental",
            )
        }

        #[test]
        fn abicoder_v2_then_v1() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "abicoder_v2_then_v1",
            )
        }

        #[test]
        fn experimental_then_abicoder_v1() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "experimental_then_abicoder_v1",
            )
        }

        #[test]
        fn experimental_then_abicoder_v2() -> Result<()> {
            run(
                "structure/duplicate-abicoder-specifier",
                "experimental_then_abicoder_v2",
            )
        }
    }

    mod duplicate_assembly_flag {
        use super::*;

        #[test]
        fn fully_encoded_flag() -> Result<()> {
            run("structure/duplicate_assembly_flag", "fully_encoded_flag")
        }

        #[test]
        fn hex_escaped_flag() -> Result<()> {
            run("structure/duplicate_assembly_flag", "hex_escaped_flag")
        }

        #[test]
        fn line_continuation_in_flag() -> Result<()> {
            run(
                "structure/duplicate_assembly_flag",
                "line_continuation_in_flag",
            )
        }

        #[test]
        fn repeated() -> Result<()> {
            run("structure/duplicate_assembly_flag", "repeated")
        }

        #[test]
        fn repeated_three_times() -> Result<()> {
            run("structure/duplicate_assembly_flag", "repeated_three_times")
        }

        #[test]
        fn repeated_with_dialect() -> Result<()> {
            run("structure/duplicate_assembly_flag", "repeated_with_dialect")
        }

        #[test]
        fn single_flag() -> Result<()> {
            run("structure/duplicate_assembly_flag", "single_flag")
        }

        #[test]
        fn unicode_escaped_flag() -> Result<()> {
            run("structure/duplicate_assembly_flag", "unicode_escaped_flag")
        }
    }

    #[test]
    fn duplicate_default_case() -> Result<()> {
        run("structure", "duplicate_default_case")
    }

    #[test]
    fn duplicate_error_catch_clause() -> Result<()> {
        run("structure", "duplicate_error_catch_clause")
    }

    #[test]
    fn duplicate_low_level_catch_clause() -> Result<()> {
        run("structure", "duplicate_low_level_catch_clause")
    }

    mod duplicate_named_argument {
        use super::*;

        #[test]
        fn function_call() -> Result<()> {
            run("structure/duplicate_named_argument", "function_call")
        }

        #[test]
        fn state_variable_initializer() -> Result<()> {
            run(
                "structure/duplicate_named_argument",
                "state_variable_initializer",
            )
        }
    }

    #[test]
    fn duplicate_panic_catch_clause() -> Result<()> {
        run("structure", "duplicate_panic_catch_clause")
    }

    mod duplicate_yul_switch_case {
        use super::*;

        #[test]
        fn distinct_values() -> Result<()> {
            run("structure/duplicate_yul_switch_case", "distinct_values")
        }

        #[test]
        fn equivalent_values() -> Result<()> {
            run("structure/duplicate_yul_switch_case", "equivalent_values")
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

    mod empty_tuple_component {
        use super::*;

        #[test]
        fn assignment_rhs() -> Result<()> {
            run("structure/empty_tuple_component", "assignment_rhs")
        }

        #[test]
        fn read_statement() -> Result<()> {
            run("structure/empty_tuple_component", "read_statement")
        }

        #[test]
        fn unary_operand() -> Result<()> {
            run("structure/empty_tuple_component", "unary_operand")
        }

        #[test]
        fn valid() -> Result<()> {
            run("structure/empty_tuple_component", "valid")
        }
    }

    mod empty_tuple_on_lhs {
        use super::*;

        #[test]
        fn direct() -> Result<()> {
            run("structure/empty_tuple_on_lhs", "direct")
        }

        #[test]
        fn nested() -> Result<()> {
            run("structure/empty_tuple_on_lhs", "nested")
        }

        #[test]
        fn parenthesized() -> Result<()> {
            run("structure/empty_tuple_on_lhs", "parenthesized")
        }

        #[test]
        fn valid() -> Result<()> {
            run("structure/empty_tuple_on_lhs", "valid")
        }
    }

    #[test]
    fn enum_with_too_many_members() -> Result<()> {
        run("structure", "enum_with_too_many_members")
    }

    mod event_with_too_many_indexed_parameters {
        use super::*;

        #[test]
        fn at_limit() -> Result<()> {
            run(
                "structure/event_with_too_many_indexed_parameters",
                "at_limit",
            )
        }

        #[test]
        fn non_indexed_parameters() -> Result<()> {
            run(
                "structure/event_with_too_many_indexed_parameters",
                "non_indexed_parameters",
            )
        }

        #[test]
        fn too_many() -> Result<()> {
            run(
                "structure/event_with_too_many_indexed_parameters",
                "too_many",
            )
        }
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
    fn free_function_with_modifiers() -> Result<()> {
        run("structure", "free_function_with_modifiers")
    }

    #[test]
    fn free_function_with_override() -> Result<()> {
        run("structure", "free_function_with_override")
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
    fn global_using_for_inside_contract() -> Result<()> {
        run("structure", "global_using_for_inside_contract")
    }

    #[test]
    fn global_using_for_wildcard() -> Result<()> {
        run("structure", "global_using_for_wildcard")
    }

    #[test]
    fn interface_function_cannot_be_implemented() -> Result<()> {
        run("structure", "interface_function_cannot_be_implemented")
    }

    #[test]
    fn interface_function_not_external() -> Result<()> {
        run("structure", "interface_function_not_external")
    }

    #[test]
    fn interface_function_with_modifiers() -> Result<()> {
        run("structure", "interface_function_with_modifiers")
    }

    #[test]
    fn invalid_catch_clause_name() -> Result<()> {
        run("structure", "invalid_catch_clause_name")
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
    fn library_non_constant_state_variable() -> Result<()> {
        run("structure", "library_non_constant_state_variable")
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
    fn modifier_body_without_placeholder() -> Result<()> {
        run("structure", "modifier_body_without_placeholder")
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
    fn multiple_fallback_functions() -> Result<()> {
        run("structure", "multiple_fallback_functions")
    }

    #[test]
    fn multiple_receive_functions() -> Result<()> {
        run("structure", "multiple_receive_functions")
    }

    #[test]
    fn named_function_type_return_parameter() -> Result<()> {
        run("structure", "named_function_type_return_parameter")
    }

    #[test]
    fn nested_unchecked_block() -> Result<()> {
        run("structure", "nested_unchecked_block")
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
    fn placeholder_in_unchecked_block() -> Result<()> {
        run("structure", "placeholder_in_unchecked_block")
    }

    mod redefined_built_in_error {
        use super::*;

        #[test]
        fn error() -> Result<()> {
            run("structure/redefined_built_in_error", "error")
        }

        #[test]
        fn panic() -> Result<()> {
            run("structure/redefined_built_in_error", "panic")
        }
    }

    #[test]
    fn storage_layout_for_abstract_contract() -> Result<()> {
        run("structure", "storage_layout_for_abstract_contract")
    }

    #[test]
    fn trailing_non_default_case() -> Result<()> {
        run("structure", "trailing_non_default_case")
    }

    mod unchecked_block_not_in_regular_block {
        use super::*;

        #[test]
        fn do_while_statement() -> Result<()> {
            run(
                "structure/unchecked_block_not_in_regular_block",
                "do_while_statement",
            )
        }

        #[test]
        fn else_branch() -> Result<()> {
            run(
                "structure/unchecked_block_not_in_regular_block",
                "else_branch",
            )
        }

        #[test]
        fn for_statement() -> Result<()> {
            run(
                "structure/unchecked_block_not_in_regular_block",
                "for_statement",
            )
        }

        #[test]
        fn if_statement() -> Result<()> {
            run(
                "structure/unchecked_block_not_in_regular_block",
                "if_statement",
            )
        }

        #[test]
        fn while_statement() -> Result<()> {
            run(
                "structure/unchecked_block_not_in_regular_block",
                "while_statement",
            )
        }
    }

    #[test]
    fn unimplemented_function_with_modifiers() -> Result<()> {
        run("structure", "unimplemented_function_with_modifiers")
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

    mod using_for_functions_with_wildcard {
        use super::*;

        #[test]
        fn at_file_level() -> Result<()> {
            run(
                "structure/using_for_functions_with_wildcard",
                "at_file_level",
            )
        }

        #[test]
        fn functions_for_named_type() -> Result<()> {
            run(
                "structure/using_for_functions_with_wildcard",
                "functions_for_named_type",
            )
        }

        #[test]
        fn inside_contract() -> Result<()> {
            run(
                "structure/using_for_functions_with_wildcard",
                "inside_contract",
            )
        }

        #[test]
        fn library_for_wildcard() -> Result<()> {
            run(
                "structure/using_for_functions_with_wildcard",
                "library_for_wildcard",
            )
        }
    }

    #[test]
    fn using_for_wildcard_at_file_level() -> Result<()> {
        run("structure", "using_for_wildcard_at_file_level")
    }

    mod variable_declaration_not_in_block {
        use super::*;

        #[test]
        fn do_while_statement() -> Result<()> {
            run(
                "structure/variable_declaration_not_in_block",
                "do_while_statement",
            )
        }

        #[test]
        fn else_branch() -> Result<()> {
            run("structure/variable_declaration_not_in_block", "else_branch")
        }

        #[test]
        fn for_statement() -> Result<()> {
            run(
                "structure/variable_declaration_not_in_block",
                "for_statement",
            )
        }

        #[test]
        fn if_statement() -> Result<()> {
            run(
                "structure/variable_declaration_not_in_block",
                "if_statement",
            )
        }

        #[test]
        fn while_statement() -> Result<()> {
            run(
                "structure/variable_declaration_not_in_block",
                "while_statement",
            )
        }
    }

    #[test]
    fn variable_in_interface() -> Result<()> {
        run("structure", "variable_in_interface")
    }

    #[test]
    fn virtual_free_function() -> Result<()> {
        run("structure", "virtual_free_function")
    }

    #[test]
    fn virtual_private_function() -> Result<()> {
        run("structure", "virtual_private_function")
    }

    mod yul_break_continue {
        use super::*;

        #[test]
        fn for_loop_init() -> Result<()> {
            run("structure/yul_break_continue", "for_loop_init")
        }

        #[test]
        fn for_loop_post() -> Result<()> {
            run("structure/yul_break_continue", "for_loop_post")
        }

        #[test]
        fn nested_in_function() -> Result<()> {
            run("structure/yul_break_continue", "nested_in_function")
        }

        #[test]
        fn outside_for_loop() -> Result<()> {
            run("structure/yul_break_continue", "outside_for_loop")
        }

        #[test]
        fn valid() -> Result<()> {
            run("structure/yul_break_continue", "valid")
        }
    }

    mod yul_function_in_for_loop_init {
        use super::*;

        #[test]
        fn in_init_block() -> Result<()> {
            run("structure/yul_function_in_for_loop_init", "in_init_block")
        }

        #[test]
        fn nested_for_in_init_block() -> Result<()> {
            run(
                "structure/yul_function_in_for_loop_init",
                "nested_for_in_init_block",
            )
        }

        #[test]
        fn nested_for_in_post_block() -> Result<()> {
            run(
                "structure/yul_function_in_for_loop_init",
                "nested_for_in_post_block",
            )
        }

        #[test]
        fn valid() -> Result<()> {
            run("structure/yul_function_in_for_loop_init", "valid")
        }
    }

    mod yul_leave_outside_function {
        use super::*;

        #[test]
        fn in_for_loop_body() -> Result<()> {
            run("structure/yul_leave_outside_function", "in_for_loop_body")
        }

        #[test]
        fn outside_function() -> Result<()> {
            run("structure/yul_leave_outside_function", "outside_function")
        }

        #[test]
        fn valid() -> Result<()> {
            run("structure/yul_leave_outside_function", "valid")
        }
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

    mod incompatible_version_pragma {
        use super::*;

        #[test]
        fn alternatives() -> Result<()> {
            run("syntax/incompatible_version_pragma", "alternatives")
        }

        #[test]
        fn below_supported_versions() -> Result<()> {
            run(
                "syntax/incompatible_version_pragma",
                "below_supported_versions",
            )
        }

        #[test]
        fn bounded_range() -> Result<()> {
            run("syntax/incompatible_version_pragma", "bounded_range")
        }

        #[test]
        fn caret_excludes_earlier_versions() -> Result<()> {
            run(
                "syntax/incompatible_version_pragma",
                "caret_excludes_earlier_versions",
            )
        }

        #[test]
        fn four_components() -> Result<()> {
            run("syntax/incompatible_version_pragma", "four_components")
        }

        #[test]
        fn four_components_matching_the_version() -> Result<()> {
            run(
                "syntax/incompatible_version_pragma",
                "four_components_matching_the_version",
            )
        }

        #[test]
        fn hyphen_range() -> Result<()> {
            run("syntax/incompatible_version_pragma", "hyphen_range")
        }

        #[test]
        fn multiple_pragmas() -> Result<()> {
            run("syntax/incompatible_version_pragma", "multiple_pragmas")
        }

        #[test]
        fn partial_version() -> Result<()> {
            run("syntax/incompatible_version_pragma", "partial_version")
        }

        #[test]
        fn wildcard_before_a_concrete_component() -> Result<()> {
            run(
                "syntax/incompatible_version_pragma",
                "wildcard_before_a_concrete_component",
            )
        }
    }

    mod invalid_assembly_dialect {
        use super::*;

        #[test]
        fn dialect_with_whitespace() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "dialect_with_whitespace")
        }

        #[test]
        fn empty_dialect() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "empty_dialect")
        }

        #[test]
        fn encoded_unknown_dialect() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "encoded_unknown_dialect")
        }

        #[test]
        fn fully_encoded_dialect() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "fully_encoded_dialect")
        }

        #[test]
        fn hex_escaped_dialect() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "hex_escaped_dialect")
        }

        #[test]
        fn line_continuation_in_dialect() -> Result<()> {
            run(
                "syntax/invalid_assembly_dialect",
                "line_continuation_in_dialect",
            )
        }

        #[test]
        fn supported_dialect() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "supported_dialect")
        }

        #[test]
        fn unicode_escaped_dialect() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "unicode_escaped_dialect")
        }

        #[test]
        fn unknown_dialect() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "unknown_dialect")
        }

        #[test]
        fn uppercase_dialect() -> Result<()> {
            run("syntax/invalid_assembly_dialect", "uppercase_dialect")
        }
    }

    mod invalid_mutability {
        use super::*;

        #[test]
        fn receive_function() -> Result<()> {
            run("syntax/invalid_mutability", "receive_function")
        }
    }

    mod invalid_version_specifier {
        use super::*;

        #[test]
        fn digits_mixed_with_wildcard() -> Result<()> {
            run(
                "syntax/invalid_version_specifier",
                "digits_mixed_with_wildcard",
            )
        }

        #[test]
        fn empty_string() -> Result<()> {
            run("syntax/invalid_version_specifier", "empty_string")
        }

        #[test]
        fn number_too_large() -> Result<()> {
            run("syntax/invalid_version_specifier", "number_too_large")
        }

        #[test]
        fn range_endpoint() -> Result<()> {
            run("syntax/invalid_version_specifier", "range_endpoint")
        }

        #[test]
        fn reported_once_per_specifier() -> Result<()> {
            run(
                "syntax/invalid_version_specifier",
                "reported_once_per_specifier",
            )
        }

        #[test]
        fn string_is_not_a_version() -> Result<()> {
            run(
                "syntax/invalid_version_specifier",
                "string_is_not_a_version",
            )
        }

        #[test]
        fn string_with_letters() -> Result<()> {
            run("syntax/invalid_version_specifier", "string_with_letters")
        }

        #[test]
        fn string_with_prerelease() -> Result<()> {
            run("syntax/invalid_version_specifier", "string_with_prerelease")
        }

        #[test]
        fn string_with_symbol() -> Result<()> {
            run("syntax/invalid_version_specifier", "string_with_symbol")
        }

        #[test]
        fn string_with_trailing_period() -> Result<()> {
            run(
                "syntax/invalid_version_specifier",
                "string_with_trailing_period",
            )
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
    fn library_inheritance() -> Result<()> {
        run("syntax", "library_inheritance")
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

    mod non_address_state_mutability {
        use super::*;

        #[test]
        fn local_variable() -> Result<()> {
            run("syntax/non_address_state_mutability", "local_variable")
        }

        #[test]
        fn return_parameter() -> Result<()> {
            run("syntax/non_address_state_mutability", "return_parameter")
        }

        #[test]
        fn state_variable() -> Result<()> {
            run("syntax/non_address_state_mutability", "state_variable")
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

    mod unrecognized_experimental_feature {
        use super::*;

        #[test]
        fn distinct_unrecognized_features() -> Result<()> {
            run(
                "syntax/unrecognized_experimental_feature",
                "distinct_unrecognized_features",
            )
        }

        #[test]
        fn same_unrecognized_feature() -> Result<()> {
            run(
                "syntax/unrecognized_experimental_feature",
                "same_unrecognized_feature",
            )
        }
    }

    mod unsupported_abicoder_v_1 {
        use super::*;

        #[test]
        fn v1() -> Result<()> {
            run("syntax/unsupported_abicoder_v1", "v1")
        }

        #[test]
        fn v2() -> Result<()> {
            run("syntax/unsupported_abicoder_v1", "v2")
        }
    }

    mod unsupported_experimental_smt_checker {
        use super::*;

        #[test]
        fn duplicate_keywords() -> Result<()> {
            run(
                "syntax/unsupported_experimental_smt_checker",
                "duplicate_keywords",
            )
        }

        #[test]
        fn duplicate_mixed_forms() -> Result<()> {
            run(
                "syntax/unsupported_experimental_smt_checker",
                "duplicate_mixed_forms",
            )
        }

        #[test]
        fn keyword() -> Result<()> {
            run("syntax/unsupported_experimental_smt_checker", "keyword")
        }

        #[test]
        fn string() -> Result<()> {
            run("syntax/unsupported_experimental_smt_checker", "string")
        }
    }

    mod unsupported_experimental_solidity {
        use super::*;

        #[test]
        fn duplicate_keywords() -> Result<()> {
            run(
                "syntax/unsupported_experimental_solidity",
                "duplicate_keywords",
            )
        }

        #[test]
        fn duplicate_mixed_forms() -> Result<()> {
            run(
                "syntax/unsupported_experimental_solidity",
                "duplicate_mixed_forms",
            )
        }

        #[test]
        fn keyword() -> Result<()> {
            run("syntax/unsupported_experimental_solidity", "keyword")
        }

        #[test]
        fn string() -> Result<()> {
            run("syntax/unsupported_experimental_solidity", "string")
        }
    }

    mod version_pragma {
        use super::*;

        #[test]
        fn comparators_without_whitespace() -> Result<()> {
            run("syntax/version_pragma", "comparators_without_whitespace")
        }

        #[test]
        fn escaped_string_literal() -> Result<()> {
            run("syntax/version_pragma", "escaped_string_literal")
        }

        #[test]
        fn large_component() -> Result<()> {
            run("syntax/version_pragma", "large_component")
        }

        #[test]
        fn leading_zero() -> Result<()> {
            run("syntax/version_pragma", "leading_zero")
        }

        #[test]
        fn repeated_wildcard_characters() -> Result<()> {
            run("syntax/version_pragma", "repeated_wildcard_characters")
        }

        #[test]
        fn trivia_between_components() -> Result<()> {
            run("syntax/version_pragma", "trivia_between_components")
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
        fn public_constant_forward_reference() -> Result<()> {
            run(
                "type_system/array_length",
                "public_constant_forward_reference",
            )
        }

        #[test]
        fn public_constant_length() -> Result<()> {
            run("type_system/array_length", "public_constant_length")
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

    mod cannot_call_via_contract_type_name {
        use super::*;

        #[test]
        fn base_constructor_argument() -> Result<()> {
            run(
                "type_system/cannot_call_via_contract_type_name",
                "base_constructor_argument",
            )
        }

        #[test]
        fn external_base() -> Result<()> {
            run(
                "type_system/cannot_call_via_contract_type_name",
                "external_base",
            )
        }

        #[test]
        fn foreign_contract() -> Result<()> {
            run(
                "type_system/cannot_call_via_contract_type_name",
                "foreign_contract",
            )
        }

        #[test]
        fn free_function() -> Result<()> {
            run(
                "type_system/cannot_call_via_contract_type_name",
                "free_function",
            )
        }

        #[test]
        fn named_arguments() -> Result<()> {
            run(
                "type_system/cannot_call_via_contract_type_name",
                "named_arguments",
            )
        }

        #[test]
        fn overloaded() -> Result<()> {
            run(
                "type_system/cannot_call_via_contract_type_name",
                "overloaded",
            )
        }

        #[test]
        fn overloaded_internal_selected() -> Result<()> {
            run(
                "type_system/cannot_call_via_contract_type_name",
                "overloaded_internal_selected",
            )
        }

        #[test]
        fn overloaded_public_selected() -> Result<()> {
            run(
                "type_system/cannot_call_via_contract_type_name",
                "overloaded_public_selected",
            )
        }
    }

    mod expression_not_a_value {
        use super::*;

        #[test]
        fn array_literal_item() -> Result<()> {
            run("type_system/expression_not_a_value", "array_literal_item")
        }

        #[test]
        fn base_constructor_argument() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "base_constructor_argument",
            )
        }

        #[test]
        fn built_in_function() -> Result<()> {
            run("type_system/expression_not_a_value", "built_in_function")
        }

        #[test]
        fn built_in_namespace() -> Result<()> {
            run("type_system/expression_not_a_value", "built_in_namespace")
        }

        #[test]
        fn call_argument() -> Result<()> {
            run("type_system/expression_not_a_value", "call_argument")
        }

        #[test]
        fn call_option_value() -> Result<()> {
            run("type_system/expression_not_a_value", "call_option_value")
        }

        #[test]
        fn condition() -> Result<()> {
            run("type_system/expression_not_a_value", "condition")
        }

        #[test]
        fn conditional_elementary_type_name_branches() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "conditional_elementary_type_name_branches",
            )
        }

        #[test]
        fn conditional_tuple_of_type_names() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "conditional_tuple_of_type_names",
            )
        }

        #[test]
        fn conditional_type_name_branch() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "conditional_type_name_branch",
            )
        }

        #[test]
        fn constant_initializer() -> Result<()> {
            run("type_system/expression_not_a_value", "constant_initializer")
        }

        #[test]
        fn emit_argument() -> Result<()> {
            run("type_system/expression_not_a_value", "emit_argument")
        }

        #[test]
        fn index_access_index() -> Result<()> {
            run("type_system/expression_not_a_value", "index_access_index")
        }

        #[test]
        fn inheritance_specifier_argument() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "inheritance_specifier_argument",
            )
        }

        #[test]
        fn logical_operator_operand() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "logical_operator_operand",
            )
        }

        #[test]
        fn loop_condition() -> Result<()> {
            run("type_system/expression_not_a_value", "loop_condition")
        }

        #[test]
        fn modifier_invocation_argument() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "modifier_invocation_argument",
            )
        }

        #[test]
        fn modifier_placeholder() -> Result<()> {
            run("type_system/expression_not_a_value", "modifier_placeholder")
        }

        #[test]
        fn module_aliases() -> Result<()> {
            run("type_system/expression_not_a_value", "module_aliases")
        }

        #[test]
        fn named_argument() -> Result<()> {
            run("type_system/expression_not_a_value", "named_argument")
        }

        #[test]
        fn operator_operand() -> Result<()> {
            run("type_system/expression_not_a_value", "operator_operand")
        }

        #[test]
        fn revert_argument() -> Result<()> {
            run("type_system/expression_not_a_value", "revert_argument")
        }

        #[test]
        fn state_variable_initializer() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "state_variable_initializer",
            )
        }

        #[test]
        fn statement_address_transfer() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "statement_address_transfer",
            )
        }

        #[test]
        fn statement_array_pop() -> Result<()> {
            run("type_system/expression_not_a_value", "statement_array_pop")
        }

        #[test]
        fn statement_built_in_function() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "statement_built_in_function",
            )
        }

        #[test]
        fn statement_built_in_member() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "statement_built_in_member",
            )
        }

        #[test]
        fn statement_built_in_namespace() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "statement_built_in_namespace",
            )
        }

        #[test]
        fn statement_for_iterator() -> Result<()> {
            run(
                "type_system/expression_not_a_value",
                "statement_for_iterator",
            )
        }

        #[test]
        fn statement_super() -> Result<()> {
            run("type_system/expression_not_a_value", "statement_super")
        }

        #[test]
        fn super_reference() -> Result<()> {
            run("type_system/expression_not_a_value", "super_reference")
        }

        #[test]
        fn tuple_component() -> Result<()> {
            run("type_system/expression_not_a_value", "tuple_component")
        }

        #[test]
        fn uncalled_new() -> Result<()> {
            run("type_system/expression_not_a_value", "uncalled_new")
        }
    }

    mod expression_not_callable {
        use super::*;

        #[test]
        fn built_in_object() -> Result<()> {
            run("type_system/expression_not_callable", "built_in_object")
        }

        #[test]
        fn enum_value() -> Result<()> {
            run("type_system/expression_not_callable", "enum_value")
        }

        #[test]
        fn import_alias() -> Result<()> {
            run("type_system/expression_not_callable", "import_alias")
        }

        #[test]
        fn integer_literal() -> Result<()> {
            run("type_system/expression_not_callable", "integer_literal")
        }

        #[test]
        fn mapping() -> Result<()> {
            run("type_system/expression_not_callable", "mapping")
        }

        #[test]
        fn modifier() -> Result<()> {
            run("type_system/expression_not_callable", "modifier")
        }

        #[test]
        fn named_arguments() -> Result<()> {
            run("type_system/expression_not_callable", "named_arguments")
        }

        #[test]
        fn this_expression() -> Result<()> {
            run("type_system/expression_not_callable", "this_expression")
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

    mod incompatible_conditional_branches {
        use super::*;

        #[test]
        fn byte_array_and_hex_literal() -> Result<()> {
            run(
                "type_system/incompatible_conditional_branches",
                "byte_array_and_hex_literal",
            )
        }

        #[test]
        fn byte_array_and_literal_zero() -> Result<()> {
            run(
                "type_system/incompatible_conditional_branches",
                "byte_array_and_literal_zero",
            )
        }

        #[test]
        fn integer_sign_branches() -> Result<()> {
            run(
                "type_system/incompatible_conditional_branches",
                "integer_sign_branches",
            )
        }

        #[test]
        fn mapping_value_types() -> Result<()> {
            run(
                "type_system/incompatible_conditional_branches",
                "mapping_value_types",
            )
        }

        #[test]
        fn rational_and_integer_branches() -> Result<()> {
            run(
                "type_system/incompatible_conditional_branches",
                "rational_and_integer_branches",
            )
        }

        #[test]
        fn tuple_element_widths() -> Result<()> {
            run(
                "type_system/incompatible_conditional_branches",
                "tuple_element_widths",
            )
        }
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

    mod literal_too_large {
        use super::*;

        #[test]
        fn conditional_branch() -> Result<()> {
            run("type_system/literal_too_large", "conditional_branch")
        }
    }

    mod partially_applied_function_used_as_value {
        use super::*;

        #[test]
        fn conditional_branch() -> Result<()> {
            run(
                "type_system/partially_applied_function_used_as_value",
                "conditional_branch",
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
