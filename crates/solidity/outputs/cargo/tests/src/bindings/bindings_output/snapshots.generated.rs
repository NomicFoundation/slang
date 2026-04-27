// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

mod arrays {
    use super::*;

    #[test]
    fn byte_array_indexing() -> Result<()> {
        run("arrays", "byte_array_indexing")
    }

    #[test]
    fn byte_length() -> Result<()> {
        run("arrays", "byte_length")
    }

    #[test]
    fn bytes_as_arrays() -> Result<()> {
        run("arrays", "bytes_as_arrays")
    }

    #[test]
    fn bytes_index_access() -> Result<()> {
        run("arrays", "bytes_index_access")
    }

    #[test]
    fn fixed_arrays() -> Result<()> {
        run("arrays", "fixed_arrays")
    }

    #[test]
    fn fixed_size_arrays() -> Result<()> {
        run("arrays", "fixed_size_arrays")
    }

    #[test]
    fn fixed_size_with_shadowed_constants() -> Result<()> {
        run("arrays", "fixed_size_with_shadowed_constants")
    }

    #[test]
    fn indexing() -> Result<()> {
        run("arrays", "indexing")
    }

    #[test]
    fn length() -> Result<()> {
        run("arrays", "length")
    }

    #[test]
    fn static_length() -> Result<()> {
        run("arrays", "static_length")
    }
}

mod built_ins {
    use super::*;

    #[test]
    fn abi_decode() -> Result<()> {
        run("built_ins", "abi_decode")
    }

    #[test]
    fn address() -> Result<()> {
        run("built_ins", "address")
    }

    #[test]
    fn address_payable() -> Result<()> {
        run("built_ins", "address_payable")
    }

    #[test]
    fn array_push() -> Result<()> {
        run("built_ins", "array_push")
    }

    #[test]
    fn arrays() -> Result<()> {
        run("built_ins", "arrays")
    }

    #[test]
    fn bytes() -> Result<()> {
        run("built_ins", "bytes")
    }

    #[test]
    fn function_type() -> Result<()> {
        run("built_ins", "function_type")
    }

    #[test]
    fn functions() -> Result<()> {
        run("built_ins", "functions")
    }

    #[test]
    fn global_properties() -> Result<()> {
        run("built_ins", "global_properties")
    }

    #[test]
    fn instance_as_address() -> Result<()> {
        run("built_ins", "instance_as_address")
    }

    #[test]
    fn msg_data() -> Result<()> {
        run("built_ins", "msg_data")
    }

    #[test]
    fn msg_sender() -> Result<()> {
        run("built_ins", "msg_sender")
    }

    #[test]
    fn now() -> Result<()> {
        run("built_ins", "now")
    }

    #[test]
    fn shadowing() -> Result<()> {
        run("built_ins", "shadowing")
    }

    #[test]
    fn this() -> Result<()> {
        run("built_ins", "this")
    }

    #[test]
    fn this_as_address() -> Result<()> {
        run("built_ins", "this_as_address")
    }

    #[test]
    fn type_expr() -> Result<()> {
        run("built_ins", "type_expr")
    }

    #[test]
    fn yul_built_in_doesn_t_bind() -> Result<()> {
        run("built_ins", "yul_built_in_doesn_t_bind")
    }
}

mod constants {
    use super::*;

    #[test]
    fn bind_to_type() -> Result<()> {
        run("constants", "bind_to_type")
    }

    #[test]
    fn in_contract() -> Result<()> {
        run("constants", "in_contract")
    }

    #[test]
    fn top_level() -> Result<()> {
        run("constants", "top_level")
    }
}

mod contracts {
    use super::*;

    #[test]
    fn constructor_call_parent() -> Result<()> {
        run("contracts", "constructor_call_parent")
    }

    #[test]
    fn constructor_invocation() -> Result<()> {
        run("contracts", "constructor_invocation")
    }

    #[test]
    fn constructor_modifier() -> Result<()> {
        run("contracts", "constructor_modifier")
    }

    #[test]
    fn constructors() -> Result<()> {
        run("contracts", "constructors")
    }

    #[test]
    fn cyclic_inheritance() -> Result<()> {
        run("contracts", "cyclic_inheritance")
    }

    #[test]
    fn diamond() -> Result<()> {
        run("contracts", "diamond")
    }

    #[test]
    fn fallback_receive() -> Result<()> {
        run("contracts", "fallback_receive")
    }

    #[test]
    fn implicit_conversion_to_address() -> Result<()> {
        run("contracts", "implicit_conversion_to_address")
    }

    #[test]
    fn inheritance() -> Result<()> {
        run("contracts", "inheritance")
    }

    #[test]
    fn inheritance_arguments_with_inner_constant() -> Result<()> {
        run("contracts", "inheritance_arguments_with_inner_constant")
    }

    #[test]
    fn inheritance_types() -> Result<()> {
        run("contracts", "inheritance_types")
    }

    #[test]
    fn inheritance_with_arguments() -> Result<()> {
        run("contracts", "inheritance_with_arguments")
    }

    #[test]
    fn inherited_legacy_constructor() -> Result<()> {
        run("contracts", "inherited_legacy_constructor")
    }

    #[test]
    fn inherited_state_vars() -> Result<()> {
        run("contracts", "inherited_state_vars")
    }

    #[test]
    fn internal_visibility() -> Result<()> {
        run("contracts", "internal_visibility")
    }

    #[test]
    fn legacy_constructors() -> Result<()> {
        run("contracts", "legacy_constructors")
    }

    #[test]
    fn legacy_function_options() -> Result<()> {
        run("contracts", "legacy_function_options")
    }

    #[test]
    fn multi_inheritance() -> Result<()> {
        run("contracts", "multi_inheritance")
    }

    #[test]
    fn overload_disambiguation_with_this() -> Result<()> {
        run("contracts", "overload_disambiguation_with_this")
    }

    #[test]
    fn private_member_not_visible() -> Result<()> {
        run("contracts", "private_member_not_visible")
    }

    #[test]
    fn public_array_getters() -> Result<()> {
        run("contracts", "public_array_getters")
    }

    #[test]
    fn public_getter_functions() -> Result<()> {
        run("contracts", "public_getter_functions")
    }

    #[test]
    fn public_getter_members() -> Result<()> {
        run("contracts", "public_getter_members")
    }

    #[test]
    fn public_getter_selector() -> Result<()> {
        run("contracts", "public_getter_selector")
    }

    #[test]
    fn public_getters() -> Result<()> {
        run("contracts", "public_getters")
    }

    #[test]
    fn public_inherited_getter() -> Result<()> {
        run("contracts", "public_inherited_getter")
    }

    #[test]
    fn public_mapping_getters() -> Result<()> {
        run("contracts", "public_mapping_getters")
    }

    #[test]
    fn public_nested_mapping_getters() -> Result<()> {
        run("contracts", "public_nested_mapping_getters")
    }

    #[test]
    fn public_struct_getter() -> Result<()> {
        run("contracts", "public_struct_getter")
    }

    #[test]
    fn qualified_inherited() -> Result<()> {
        run("contracts", "qualified_inherited")
    }

    #[test]
    fn qualified_parent_call() -> Result<()> {
        run("contracts", "qualified_parent_call")
    }

    #[test]
    fn qualified_self() -> Result<()> {
        run("contracts", "qualified_self")
    }

    #[test]
    fn state_var_override() -> Result<()> {
        run("contracts", "state_var_override")
    }

    #[test]
    fn storage_layout_constant() -> Result<()> {
        run("contracts", "storage_layout_constant")
    }

    #[test]
    fn storage_layout_constant_number() -> Result<()> {
        run("contracts", "storage_layout_constant_number")
    }

    #[test]
    fn storage_layout_inner_constant() -> Result<()> {
        run("contracts", "storage_layout_inner_constant")
    }

    #[test]
    fn super_deep() -> Result<()> {
        run("contracts", "super_deep")
    }

    #[test]
    fn super_linearisation() -> Result<()> {
        run("contracts", "super_linearisation")
    }

    #[test]
    fn super_scope() -> Result<()> {
        run("contracts", "super_scope")
    }

    #[test]
    fn this_scope() -> Result<()> {
        run("contracts", "this_scope")
    }

    #[test]
    fn unnamed_function() -> Result<()> {
        run("contracts", "unnamed_function")
    }

    #[test]
    fn virtual_lookup() -> Result<()> {
        run("contracts", "virtual_lookup")
    }

    #[test]
    fn virtual_methods() -> Result<()> {
        run("contracts", "virtual_methods")
    }

    #[test]
    fn visibility() -> Result<()> {
        run("contracts", "visibility")
    }
}

mod control {
    use super::*;

    #[test]
    fn do_while() -> Result<()> {
        run("control", "do_while")
    }

    #[test]
    fn emit_event() -> Result<()> {
        run("control", "emit_event")
    }

    #[test]
    fn for_empty_clauses() -> Result<()> {
        run("control", "for_empty_clauses")
    }

    #[test]
    fn for_empty_cond() -> Result<()> {
        run("control", "for_empty_cond")
    }

    #[test]
    fn for_empty_init() -> Result<()> {
        run("control", "for_empty_init")
    }

    #[test]
    fn for_expr_init() -> Result<()> {
        run("control", "for_expr_init")
    }

    #[test]
    fn for_stmt() -> Result<()> {
        run("control", "for_stmt")
    }

    #[test]
    fn for_tuple_init() -> Result<()> {
        run("control", "for_tuple_init")
    }

    #[test]
    fn if_else() -> Result<()> {
        run("control", "if_else")
    }

    #[test]
    fn return_stmt() -> Result<()> {
        run("control", "return_stmt")
    }

    #[test]
    fn try_catch() -> Result<()> {
        run("control", "try_catch")
    }

    #[test]
    fn try_stmt() -> Result<()> {
        run("control", "try_stmt")
    }

    #[test]
    fn unchecked() -> Result<()> {
        run("control", "unchecked")
    }

    #[test]
    fn while_stmt() -> Result<()> {
        run("control", "while_stmt")
    }
}

mod enums {
    use super::*;

    #[test]
    fn decls() -> Result<()> {
        run("enums", "decls")
    }

    #[test]
    fn in_params() -> Result<()> {
        run("enums", "in_params")
    }

    #[test]
    fn in_state_vars() -> Result<()> {
        run("enums", "in_state_vars")
    }

    #[test]
    fn sample() -> Result<()> {
        run("enums", "sample")
    }
}

mod errors {
    use super::*;

    #[test]
    fn custom_types() -> Result<()> {
        run("errors", "custom_types")
    }

    #[test]
    fn definitions() -> Result<()> {
        run("errors", "definitions")
    }

    #[test]
    fn named_args() -> Result<()> {
        run("errors", "named_args")
    }

    #[test]
    fn revert_as_function() -> Result<()> {
        run("errors", "revert_as_function")
    }

    #[test]
    fn revert_stmt() -> Result<()> {
        run("errors", "revert_stmt")
    }

    #[test]
    fn selector() -> Result<()> {
        run("errors", "selector")
    }
}

mod events {
    use super::*;

    #[test]
    fn called_as_functions() -> Result<()> {
        run("events", "called_as_functions")
    }

    #[test]
    fn custom_types() -> Result<()> {
        run("events", "custom_types")
    }

    #[test]
    fn definitions() -> Result<()> {
        run("events", "definitions")
    }

    #[test]
    fn emit_stmt() -> Result<()> {
        run("events", "emit_stmt")
    }

    #[test]
    fn named_args() -> Result<()> {
        run("events", "named_args")
    }

    #[test]
    fn overload_selection() -> Result<()> {
        run("events", "overload_selection")
    }

    #[test]
    fn selector() -> Result<()> {
        run("events", "selector")
    }
}

mod expressions {
    use super::*;

    #[test]
    fn basic() -> Result<()> {
        run("expressions", "basic")
    }

    #[test]
    fn binary_operators() -> Result<()> {
        run("expressions", "binary_operators")
    }

    #[test]
    fn call_options() -> Result<()> {
        run("expressions", "call_options")
    }

    #[test]
    fn conditional() -> Result<()> {
        run("expressions", "conditional")
    }

    #[test]
    fn elementary_casting() -> Result<()> {
        run("expressions", "elementary_casting")
    }

    #[test]
    fn emit_named_args() -> Result<()> {
        run("expressions", "emit_named_args")
    }

    #[test]
    fn funcalls() -> Result<()> {
        run("expressions", "funcalls")
    }

    #[test]
    fn funcalls_ambiguous_overload() -> Result<()> {
        run("expressions", "funcalls_ambiguous_overload")
    }

    #[test]
    fn funcalls_named_args() -> Result<()> {
        run("expressions", "funcalls_named_args")
    }

    #[test]
    fn funcalls_named_overloads() -> Result<()> {
        run("expressions", "funcalls_named_overloads")
    }

    #[test]
    fn funcalls_nested_overload() -> Result<()> {
        run("expressions", "funcalls_nested_overload")
    }

    #[test]
    fn funcalls_output() -> Result<()> {
        run("expressions", "funcalls_output")
    }

    #[test]
    fn funcalls_overload() -> Result<()> {
        run("expressions", "funcalls_overload")
    }

    #[test]
    fn funcalls_overload_indexed() -> Result<()> {
        run("expressions", "funcalls_overload_indexed")
    }

    #[test]
    fn funcalls_overload_member_access() -> Result<()> {
        run("expressions", "funcalls_overload_member_access")
    }

    #[test]
    fn funcalls_overload_options() -> Result<()> {
        run("expressions", "funcalls_overload_options")
    }

    #[test]
    fn funcalls_overload_this() -> Result<()> {
        run("expressions", "funcalls_overload_this")
    }

    #[test]
    fn funcalls_overload_using_for() -> Result<()> {
        run("expressions", "funcalls_overload_using_for")
    }

    #[test]
    fn incomplete_member_access() -> Result<()> {
        run("expressions", "incomplete_member_access")
    }

    #[test]
    fn legacy_call_options() -> Result<()> {
        run("expressions", "legacy_call_options")
    }

    #[test]
    fn literal_address() -> Result<()> {
        run("expressions", "literal_address")
    }

    #[test]
    fn literal_arithmetic() -> Result<()> {
        run("expressions", "literal_arithmetic")
    }

    #[test]
    fn literal_booleans() -> Result<()> {
        run("expressions", "literal_booleans")
    }

    #[test]
    fn literal_implicit_conversion() -> Result<()> {
        run("expressions", "literal_implicit_conversion")
    }

    #[test]
    fn literal_integers() -> Result<()> {
        run("expressions", "literal_integers")
    }

    #[test]
    fn member_inherits_data_location() -> Result<()> {
        run("expressions", "member_inherits_data_location")
    }

    #[test]
    fn new_array() -> Result<()> {
        run("expressions", "new_array")
    }

    #[test]
    fn new_output() -> Result<()> {
        run("expressions", "new_output")
    }

    #[test]
    fn new_with_legacy_call_options() -> Result<()> {
        run("expressions", "new_with_legacy_call_options")
    }

    #[test]
    fn revert_named_args() -> Result<()> {
        run("expressions", "revert_named_args")
    }

    #[test]
    fn type_expr() -> Result<()> {
        run("expressions", "type_expr")
    }

    #[test]
    fn type_expr_integers() -> Result<()> {
        run("expressions", "type_expr_integers")
    }

    #[test]
    fn type_expr_minmax() -> Result<()> {
        run("expressions", "type_expr_minmax")
    }

    #[test]
    fn uint160_address_casting() -> Result<()> {
        run("expressions", "uint160_address_casting")
    }
}

mod function_types {
    use super::*;

    #[test]
    fn args_return_types() -> Result<()> {
        run("function_types", "args_return_types")
    }

    #[test]
    fn call() -> Result<()> {
        run("function_types", "call")
    }

    #[test]
    fn externals() -> Result<()> {
        run("function_types", "externals")
    }

    #[test]
    fn reference() -> Result<()> {
        run("function_types", "reference")
    }
}

mod functions {
    use super::*;

    #[test]
    fn definition_of_parameters() -> Result<()> {
        run("functions", "definition_of_parameters")
    }

    #[test]
    fn external_calls_implicit_conversions() -> Result<()> {
        run("functions", "external_calls_implicit_conversions")
    }

    #[test]
    fn implicit_location_conversion() -> Result<()> {
        run("functions", "implicit_location_conversion")
    }

    #[test]
    fn overrides_interface_method() -> Result<()> {
        run("functions", "overrides_interface_method")
    }
}

mod imports {
    use super::*;

    #[test]
    fn alias_import() -> Result<()> {
        run("imports", "alias_import")
    }

    #[test]
    fn aliased_path_import() -> Result<()> {
        run("imports", "aliased_path_import")
    }

    #[test]
    fn deconstruction() -> Result<()> {
        run("imports", "deconstruction")
    }

    #[test]
    fn default() -> Result<()> {
        run("imports", "default")
    }

    #[test]
    fn default_deep() -> Result<()> {
        run("imports", "default_deep")
    }

    #[test]
    fn named() -> Result<()> {
        run("imports", "named")
    }

    #[test]
    fn named_import() -> Result<()> {
        run("imports", "named_import")
    }
}

mod interfaces {
    use super::*;

    #[test]
    fn implicit_conversions() -> Result<()> {
        run("interfaces", "implicit_conversions")
    }

    #[test]
    fn inheritance() -> Result<()> {
        run("interfaces", "inheritance")
    }

    #[test]
    fn modifiers() -> Result<()> {
        run("interfaces", "modifiers")
    }

    #[test]
    fn own_types_access() -> Result<()> {
        run("interfaces", "own_types_access")
    }

    #[test]
    fn simple() -> Result<()> {
        run("interfaces", "simple")
    }

    #[test]
    fn visibility() -> Result<()> {
        run("interfaces", "visibility")
    }
}

mod libraries {
    use super::*;

    #[test]
    fn constants() -> Result<()> {
        run("libraries", "constants")
    }

    #[test]
    fn modifiers() -> Result<()> {
        run("libraries", "modifiers")
    }

    #[test]
    fn modifiers_scope() -> Result<()> {
        run("libraries", "modifiers_scope")
    }

    #[test]
    fn propagate_dynamic_scope() -> Result<()> {
        run("libraries", "propagate_dynamic_scope")
    }

    #[test]
    fn visibility() -> Result<()> {
        run("libraries", "visibility")
    }
}

mod mappings {
    use super::*;

    #[test]
    fn custom_types() -> Result<()> {
        run("mappings", "custom_types")
    }

    #[test]
    fn elementary() -> Result<()> {
        run("mappings", "elementary")
    }

    #[test]
    fn indexing() -> Result<()> {
        run("mappings", "indexing")
    }

    #[test]
    fn named_parameters() -> Result<()> {
        run("mappings", "named_parameters")
    }

    #[test]
    fn named_params_in_variables() -> Result<()> {
        run("mappings", "named_params_in_variables")
    }

    #[test]
    fn nested() -> Result<()> {
        run("mappings", "nested")
    }

    #[test]
    fn nested_custom() -> Result<()> {
        run("mappings", "nested_custom")
    }

    #[test]
    fn nested_named_params() -> Result<()> {
        run("mappings", "nested_named_params")
    }
}

mod modifiers {
    use super::*;

    #[test]
    fn diamond() -> Result<()> {
        run("modifiers", "diamond")
    }

    #[test]
    fn function_named_underscore() -> Result<()> {
        run("modifiers", "function_named_underscore")
    }

    #[test]
    fn imported() -> Result<()> {
        run("modifiers", "imported")
    }

    #[test]
    fn inherited() -> Result<()> {
        run("modifiers", "inherited")
    }

    #[test]
    fn simple() -> Result<()> {
        run("modifiers", "simple")
    }

    #[test]
    fn virtual_modifier() -> Result<()> {
        run("modifiers", "virtual_modifier")
    }

    #[test]
    fn with_args() -> Result<()> {
        run("modifiers", "with_args")
    }
}

mod scoping {
    use super::*;

    #[test]
    fn c99_scopes() -> Result<()> {
        run("scoping", "c99_scopes")
    }

    #[test]
    fn functions() -> Result<()> {
        run("scoping", "functions")
    }

    #[test]
    fn hoisting_scopes() -> Result<()> {
        run("scoping", "hoisting_scopes")
    }

    #[test]
    fn private_variables() -> Result<()> {
        run("scoping", "private_variables")
    }

    #[test]
    fn shadowing() -> Result<()> {
        run("scoping", "shadowing")
    }

    #[test]
    fn statement_scope() -> Result<()> {
        run("scoping", "statement_scope")
    }
}

mod structs {
    use super::*;

    #[test]
    fn declaration() -> Result<()> {
        run("structs", "declaration")
    }

    #[test]
    fn function_call() -> Result<()> {
        run("structs", "function_call")
    }

    #[test]
    fn named_params_construction() -> Result<()> {
        run("structs", "named_params_construction")
    }

    #[test]
    fn nested() -> Result<()> {
        run("structs", "nested")
    }

    #[test]
    fn recursive() -> Result<()> {
        run("structs", "recursive")
    }

    #[test]
    fn sample() -> Result<()> {
        run("structs", "sample")
    }

    #[test]
    fn simple() -> Result<()> {
        run("structs", "simple")
    }
}

mod user_types {
    use super::*;

    #[test]
    fn in_contract() -> Result<()> {
        run("user_types", "in_contract")
    }

    #[test]
    fn in_library() -> Result<()> {
        run("user_types", "in_library")
    }

    #[test]
    fn top_level() -> Result<()> {
        run("user_types", "top_level")
    }

    #[test]
    fn wrap_unwrap() -> Result<()> {
        run("user_types", "wrap_unwrap")
    }
}

mod using {
    use super::*;

    #[test]
    fn address() -> Result<()> {
        run("using", "address")
    }

    #[test]
    fn attached_to_this() -> Result<()> {
        run("using", "attached_to_this")
    }

    #[test]
    fn binding_enum() -> Result<()> {
        run("using", "binding_enum")
    }

    #[test]
    fn binding_enum_member() -> Result<()> {
        run("using", "binding_enum_member")
    }

    #[test]
    fn built_ins_results() -> Result<()> {
        run("using", "built_ins_results")
    }

    #[test]
    fn can_override_built_ins() -> Result<()> {
        run("using", "can_override_built_ins")
    }

    #[test]
    fn casting() -> Result<()> {
        run("using", "casting")
    }

    #[test]
    fn chained_calls() -> Result<()> {
        run("using", "chained_calls")
    }

    #[test]
    fn deconstruction() -> Result<()> {
        run("using", "deconstruction")
    }

    #[test]
    fn disambiguate_with_interface_function() -> Result<()> {
        run("using", "disambiguate_with_interface_function")
    }

    #[test]
    fn duplicate_directives() -> Result<()> {
        run("using", "duplicate_directives")
    }

    #[test]
    fn elementary() -> Result<()> {
        run("using", "elementary")
    }

    #[test]
    fn elementary_arrays() -> Result<()> {
        run("using", "elementary_arrays")
    }

    #[test]
    fn for_fixed_arrays() -> Result<()> {
        run("using", "for_fixed_arrays")
    }

    #[test]
    fn function_types() -> Result<()> {
        run("using", "function_types")
    }

    #[test]
    fn global() -> Result<()> {
        run("using", "global")
    }

    #[test]
    fn global_and_local() -> Result<()> {
        run("using", "global_and_local")
    }

    #[test]
    fn global_multi_file() -> Result<()> {
        run("using", "global_multi_file")
    }

    #[test]
    fn in_contract() -> Result<()> {
        run("using", "in_contract")
    }

    #[test]
    fn in_interface() -> Result<()> {
        run("using", "in_interface")
    }

    #[test]
    fn in_library() -> Result<()> {
        run("using", "in_library")
    }

    #[test]
    fn inherit_extension() -> Result<()> {
        run("using", "inherit_extension")
    }

    #[test]
    fn inherited_types() -> Result<()> {
        run("using", "inherited_types")
    }

    #[test]
    fn mappings() -> Result<()> {
        run("using", "mappings")
    }

    #[test]
    fn named_args() -> Result<()> {
        run("using", "named_args")
    }

    #[test]
    fn on_interfaces_inherited() -> Result<()> {
        run("using", "on_interfaces_inherited")
    }

    #[test]
    fn on_parameters() -> Result<()> {
        run("using", "on_parameters")
    }

    #[test]
    fn on_state_var_initialization() -> Result<()> {
        run("using", "on_state_var_initialization")
    }

    #[test]
    fn on_super_calls() -> Result<()> {
        run("using", "on_super_calls")
    }

    #[test]
    fn overloaded_functions_attached() -> Result<()> {
        run("using", "overloaded_functions_attached")
    }

    #[test]
    fn overloaded_multi_arity() -> Result<()> {
        run("using", "overloaded_multi_arity")
    }

    #[test]
    fn qualified_type() -> Result<()> {
        run("using", "qualified_type")
    }

    #[test]
    fn receiver_implicit_conversion() -> Result<()> {
        run("using", "receiver_implicit_conversion")
    }

    #[test]
    fn star() -> Result<()> {
        run("using", "star")
    }

    #[test]
    fn star_in_library() -> Result<()> {
        run("using", "star_in_library")
    }

    #[test]
    fn star_inherited() -> Result<()> {
        run("using", "star_inherited")
    }

    #[test]
    fn top_level() -> Result<()> {
        run("using", "top_level")
    }

    #[test]
    fn uint_alias() -> Result<()> {
        run("using", "uint_alias")
    }

    #[test]
    fn user_types() -> Result<()> {
        run("using", "user_types")
    }
}

mod variables {
    use super::*;

    #[test]
    fn destructuring() -> Result<()> {
        run("variables", "destructuring")
    }

    #[test]
    fn incomplete_type_name() -> Result<()> {
        run("variables", "incomplete_type_name")
    }

    #[test]
    fn local_vars() -> Result<()> {
        run("variables", "local_vars")
    }

    #[test]
    fn params() -> Result<()> {
        run("variables", "params")
    }

    #[test]
    fn state_vars() -> Result<()> {
        run("variables", "state_vars")
    }

    #[test]
    fn tuple_assignment() -> Result<()> {
        run("variables", "tuple_assignment")
    }

    #[test]
    fn var_declaration() -> Result<()> {
        run("variables", "var_declaration")
    }

    #[test]
    fn var_tuple_declaration() -> Result<()> {
        run("variables", "var_tuple_declaration")
    }

    #[test]
    fn var_type_detection() -> Result<()> {
        run("variables", "var_type_detection")
    }
}

mod yul {
    use super::*;

    #[test]
    fn all_built_ins() -> Result<()> {
        run("yul", "all_built_ins")
    }

    #[test]
    fn blocks() -> Result<()> {
        run("yul", "blocks")
    }

    #[test]
    fn built_ins() -> Result<()> {
        run("yul", "built_ins")
    }

    #[test]
    fn catch_params() -> Result<()> {
        run("yul", "catch_params")
    }

    #[test]
    fn conditionals() -> Result<()> {
        run("yul", "conditionals")
    }

    #[test]
    fn constant_access_from_functions() -> Result<()> {
        run("yul", "constant_access_from_functions")
    }

    #[test]
    fn constructor_params() -> Result<()> {
        run("yul", "constructor_params")
    }

    #[test]
    fn external_variables() -> Result<()> {
        run("yul", "external_variables")
    }

    #[test]
    fn fallback_params() -> Result<()> {
        run("yul", "fallback_params")
    }

    #[test]
    fn for_init_variables() -> Result<()> {
        run("yul", "for_init_variables")
    }

    #[test]
    fn functions() -> Result<()> {
        run("yul", "functions")
    }

    #[test]
    fn identifiers_with_dots() -> Result<()> {
        run("yul", "identifiers_with_dots")
    }

    #[test]
    fn imported_constant() -> Result<()> {
        run("yul", "imported_constant")
    }

    #[test]
    fn imported_deconstructed_constants() -> Result<()> {
        run("yul", "imported_deconstructed_constants")
    }

    #[test]
    fn imported_undefined_symbol() -> Result<()> {
        run("yul", "imported_undefined_symbol")
    }

    #[test]
    fn inherited_constant() -> Result<()> {
        run("yul", "inherited_constant")
    }

    #[test]
    fn labels() -> Result<()> {
        run("yul", "labels")
    }

    #[test]
    fn legacy_built_ins() -> Result<()> {
        run("yul", "legacy_built_ins")
    }

    #[test]
    fn loops() -> Result<()> {
        run("yul", "loops")
    }

    #[test]
    fn modifier_params() -> Result<()> {
        run("yul", "modifier_params")
    }

    #[test]
    fn nested_functions() -> Result<()> {
        run("yul", "nested_functions")
    }

    #[test]
    fn non_functional_notation() -> Result<()> {
        run("yul", "non_functional_notation")
    }

    #[test]
    fn slot_of_public_var_override() -> Result<()> {
        run("yul", "slot_of_public_var_override")
    }

    #[test]
    fn slot_of_return_params() -> Result<()> {
        run("yul", "slot_of_return_params")
    }

    #[test]
    fn slot_offset_members() -> Result<()> {
        run("yul", "slot_offset_members")
    }

    #[test]
    fn slot_suffix() -> Result<()> {
        run("yul", "slot_suffix")
    }

    #[test]
    fn solidity_built_in_doesn_t_bind() -> Result<()> {
        run("yul", "solidity_built_in_doesn_t_bind")
    }

    #[test]
    fn stack_assign() -> Result<()> {
        run("yul", "stack_assign")
    }

    #[test]
    fn try_params() -> Result<()> {
        run("yul", "try_params")
    }

    #[test]
    fn variables() -> Result<()> {
        run("yul", "variables")
    }
}
