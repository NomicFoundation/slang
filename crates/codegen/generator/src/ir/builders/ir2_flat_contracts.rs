use crate::ir::{IrModel, IrModelMutator, ModelWithTransformer};

pub(super) fn build_from(structured_ast_model: &IrModel) -> ModelWithTransformer {
    let mut mutator = IrModelMutator::create_from(structured_ast_model);

    flatten_contract_specifiers(&mut mutator);
    unify_function_types(&mut mutator);
    flatten_function_attributes(&mut mutator);
    flatten_state_variable_attributes(&mut mutator);
    transmute_constant_state_variables(&mut mutator);
    collapse_redundant_node_types(&mut mutator);
    simplify_string_literals(&mut mutator);
    simplify_imports(&mut mutator);
    simplify_variable_declarations(&mut mutator);
    simplify_parameters(&mut mutator);
    simplify_mapping_type_parameters(&mut mutator);

    mutator.into()
}

fn flatten_contract_specifiers(mutator: &mut IrModelMutator) {
    // Flatten contract specifiers and bring the inherited types and storage
    // layout to the contract definition itself.
    mutator.remove_type("ContractSpecifiers");
    mutator.remove_type("ContractSpecifier");
    mutator.collapse_sequence("InheritanceSpecifier");
    mutator.collapse_sequence("StorageLayoutSpecifier");
    mutator.add_sequence_field(
        "ContractDefinition",
        "inheritance_types",
        "InheritanceTypes",
        false,
    );
    mutator.add_sequence_field("ContractDefinition", "storage_layout", "Expression", true);
}

fn unify_function_types(mutator: &mut IrModelMutator) {
    // Unifiy function definition types
    mutator.add_enum_type(
        "FunctionKind",
        &[
            "Regular",
            "Constructor",
            "Unnamed",
            "Fallback",
            "Receive",
            "Modifier",
        ],
    );

    // Add the kind to the FunctionDefinition type, which will now hold all kinds
    mutator.add_sequence_field("FunctionDefinition", "kind", "FunctionKind", false);

    // Then remove other specific function types and related attributes
    mutator.remove_type("ConstructorDefinition");
    mutator.remove_type("ConstructorAttributes");
    mutator.remove_type("ConstructorAttribute");

    mutator.remove_type("UnnamedFunctionDefinition");
    mutator.remove_type("UnnamedFunctionAttributes");
    mutator.remove_type("UnnamedFunctionAttribute");

    mutator.remove_type("FallbackFunctionDefinition");
    mutator.remove_type("FallbackFunctionAttributes");
    mutator.remove_type("FallbackFunctionAttribute");

    mutator.remove_type("ReceiveFunctionDefinition");
    mutator.remove_type("ReceiveFunctionAttributes");
    mutator.remove_type("ReceiveFunctionAttribute");

    mutator.remove_type("ModifierDefinition");
    mutator.remove_type("ModifierAttributes");
    mutator.remove_type("ModifierAttribute");

    // This also requires modifying the name and body fields
    mutator.remove_sequence_field("FunctionDefinition", "name");
    mutator.add_sequence_field("FunctionDefinition", "name", "Identifier", true);
    mutator.remove_sequence_field("FunctionDefinition", "body");
    mutator.add_sequence_field("FunctionDefinition", "body", "Block", true);

    // We don't need FunctionName or FunctionBody anymore
    mutator.remove_type("FunctionName");
    mutator.remove_type("FunctionBody");
}

fn flatten_function_attributes(mutator: &mut IrModelMutator) {
    // Function visibility, computed from a subset of the attributes
    mutator.add_enum_type(
        "FunctionVisibility",
        &["Public", "Private", "Internal", "External"],
    );

    // Function mutability, computed from a subset of the attributes
    mutator.add_enum_type(
        "FunctionMutability",
        &["Pure", "View", "NonPayable", "Payable"],
    );

    mutator.add_sequence_field(
        "FunctionDefinition",
        "visibility",
        "FunctionVisibility",
        false,
    );
    mutator.add_sequence_field(
        "FunctionDefinition",
        "mutability",
        "FunctionMutability",
        false,
    );
    // We use an optional unique terminal to effectively have a boolean
    mutator.add_sequence_field(
        "FunctionDefinition",
        "virtual_keyword",
        "VirtualKeyword",
        true,
    );

    // Flatten list of override specifiers and modifier invocations
    mutator.add_sequence_field(
        "FunctionDefinition",
        "override_specifier",
        "OverridePaths",
        true,
    );
    mutator.add_collection_type("ModifierInvocations", "ModifierInvocation");
    mutator.add_sequence_field(
        "FunctionDefinition",
        "modifier_invocations",
        "ModifierInvocations",
        false,
    );

    // And remove the list of attributes
    mutator.remove_type("FunctionAttributes");
    mutator.remove_type("FunctionAttribute");

    // For `FunctionType` we need visibility and mutability
    mutator.add_sequence_field("FunctionType", "visibility", "FunctionVisibility", false);
    mutator.add_sequence_field("FunctionType", "mutability", "FunctionMutability", false);
    mutator.remove_type("FunctionTypeAttributes");
    mutator.remove_type("FunctionTypeAttribute");
}

fn flatten_state_variable_attributes(mutator: &mut IrModelMutator) {
    // State variable visibility, computed from a subset of the attributes
    mutator.add_enum_type(
        "StateVariableVisibility",
        &["Public", "Private", "Internal"],
    );

    // State variable mutability, computed from a subset of the attributes.
    // NB. Even though most constant declarations are transformed into
    // `ConstantDefinition`, those with `public` visibility need to generate a
    // getter and thus we still need to represent them using a
    // `StateVariableDefinition`.
    mutator.add_enum_type(
        "StateVariableMutability",
        &["Mutable", "Constant", "Immutable", "Transient"],
    );

    mutator.add_sequence_field(
        "StateVariableDefinition",
        "visibility",
        "StateVariableVisibility",
        false,
    );
    mutator.add_sequence_field(
        "StateVariableDefinition",
        "mutability",
        "StateVariableMutability",
        false,
    );
    mutator.add_sequence_field(
        "StateVariableDefinition",
        "override_specifier",
        "OverridePaths",
        true,
    );

    // And remove the list of attributes
    mutator.remove_type("StateVariableAttributes");
    mutator.remove_type("StateVariableAttribute");
}

fn transmute_constant_state_variables(mutator: &mut IrModelMutator) {
    // State variables that are marked constant and are not public will be
    // transformed into `ConstantDefinition` (ie. the type used for top-level
    // constant definitions). Public constant state variables *cannot* be
    // transformed because they generate a getter, so it makes more sense to
    // keep them as `StateVariableDefinition`
    mutator.add_choice_variant("ContractMember", "ConstantDefinition");

    // Modify `ConstantDefinition` to accomodate for constant state variables
    mutator.add_sequence_field(
        "ConstantDefinition",
        "visibility",
        "StateVariableVisibility",
        true,
    );

    // ...and make the value optional because state variables may not define it
    // NOTE: this is not valid Solidity, but we still want to support the
    // representation until we have robust validation
    mutator.remove_sequence_field("ConstantDefinition", "value");
    mutator.add_sequence_field("ConstantDefinition", "value", "Expression", true);
}

fn collapse_redundant_node_types(mutator: &mut IrModelMutator) {
    // Collapse redundant node types
    mutator.collapse_sequence("ParametersDeclaration");
    mutator.collapse_sequence("ReturnsDeclaration");
    mutator.collapse_sequence("YulParametersDeclaration");
    mutator.collapse_sequence("YulReturnsDeclaration");
    mutator.collapse_sequence("ImportAlias");
    mutator.collapse_sequence("ElseBranch");
    mutator.collapse_sequence("UsingAlias");
    mutator.collapse_sequence("StateVariableDefinitionValue");
    mutator.collapse_sequence("OverridePathsDeclaration");
    mutator.collapse_sequence("VariableDeclarationValue");
    mutator.collapse_sequence("NamedArgumentGroup");

    // Collapse IndexAccessEnd manually (requires code in the transformer
    // implementation) because it's an optional containing an optional, and that
    // complicates automatic code generation in the transformer.
    mutator.remove_type("IndexAccessEnd");
    mutator.add_sequence_field("IndexAccessExpression", "end", "Expression", true);

    // Collapse the middle node in ArgumentsDeclaration
    mutator.remove_type("PositionalArgumentsDeclaration");
    mutator.remove_type("NamedArgumentsDeclaration");
    mutator.add_choice_variant("ArgumentsDeclaration", "PositionalArguments");
    mutator.add_choice_variant("ArgumentsDeclaration", "NamedArguments");
}

fn simplify_string_literals(mutator: &mut IrModelMutator) {
    // Remove all existing types, as we will simplify them to 3 variants
    mutator.remove_type("StringLiterals");
    mutator.remove_type("StringLiteral");
    mutator.remove_type("HexStringLiterals");
    mutator.remove_type("HexStringLiteral");
    mutator.remove_type("UnicodeStringLiterals");
    mutator.remove_type("UnicodeStringLiteral");

    // Re-declare `StringLiteral`, `HexStringLiteral` and `UnicodeStringLiteral`
    // as non-unique terminals
    mutator.add_non_unique_terminal("StringLiteral");
    mutator.add_non_unique_terminal("HexStringLiteral");
    mutator.add_non_unique_terminal("UnicodeStringLiteral");

    // Create the collection types using the double-quoted variants.
    // The choice is irrelevant because we only care that it's a non-unique
    // terminal, which is represented by an `Rc<TerminalNode>` anyway.
    mutator.add_collection_type("Strings", "StringLiteral");
    mutator.add_collection_type("HexStrings", "HexStringLiteral");
    mutator.add_collection_type("UnicodeStrings", "UnicodeStringLiteral");

    // Now we add the variants to the expression type
    mutator.add_choice_variant("StringExpression", "Strings");
    mutator.add_choice_variant("StringExpression", "HexStrings");
    mutator.add_choice_variant("StringExpression", "UnicodeStrings");

    // Update other uses of StringLiteral
    mutator.add_sequence_field("PathImport", "path", "StringLiteral", false);
    mutator.add_sequence_field("NamedImport", "path", "StringLiteral", false);
    mutator.add_sequence_field("ImportDeconstruction", "path", "StringLiteral", false);
    mutator.add_choice_variant("ExperimentalFeature", "StringLiteral");
    mutator.add_choice_variant("YulLiteral", "StringLiteral");
    mutator.add_choice_variant("YulLiteral", "HexStringLiteral");

    // For `AssemblyFlags`, also remove the enclosing declaration structure
    mutator.remove_type("AssemblyFlagsDeclaration");
    mutator.add_collection_type("AssemblyFlags", "StringLiteral");
    mutator.add_sequence_field("AssemblyStatement", "flags", "AssemblyFlags", false);
    mutator.add_sequence_field("AssemblyStatement", "label", "StringLiteral", true);
}

fn simplify_imports(mutator: &mut IrModelMutator) {
    // Collapse `ImportDirective` which is only a container for the `ImportClause`
    mutator.collapse_sequence("ImportDirective");
    // Remove `NamedImport`, since it can be converted to the equivalent `PathImport`
    mutator.remove_type("NamedImport");
}

fn simplify_variable_declarations(mutator: &mut IrModelMutator) {
    // Collapse the `VariableDeclarationType` into the parent `VariableDeclarationStatement`
    mutator.remove_type("VariableDeclarationType");
    mutator.add_sequence_field(
        "VariableDeclarationStatement",
        "type_name",
        "TypeName",
        true,
    );

    // Re-use `VariableDeclarationStatement` for variable declarations in tuple
    // deconstruction expressions. Remove `TupleMember` first.
    mutator.remove_type("UntypedTupleMember");
    mutator.remove_type("TypedTupleMember");
    mutator.remove_type("TupleMember");
    mutator.remove_type("TupleDeconstructionElements");
    mutator.remove_type("TupleDeconstructionElement");

    // Create a `TupleDeconstructionMember` initially as an enum so that the
    // `None` variant has no child
    mutator.add_enum_type("TupleDeconstructionMember", &["None"]);
    mutator.add_choice_variant("TupleDeconstructionMember", "Identifier");
    mutator.add_choice_variant("TupleDeconstructionMember", "VariableDeclarationStatement");
    mutator.add_collection_type("TupleDeconstructionMembers", "TupleDeconstructionMember");
    mutator.add_sequence_field(
        "TupleDeconstructionStatement",
        "members",
        "TupleDeconstructionMembers",
        false,
    );
    // This refactor also means we don't need the `var_keyword` field in the
    // `TupleDeconstructionStatement` anymore
    mutator.remove_sequence_field("TupleDeconstructionStatement", "var_keyword");
}

fn simplify_parameters(mutator: &mut IrModelMutator) {
    // Replace `EventParameter` and `ErrorParameter` with `Parameter`. This
    // requires adding an `indexed` attribute (required for event parameters).
    mutator.add_sequence_field("Parameter", "indexed", "IndexedKeyword", true);

    mutator.remove_type("EventParametersDeclaration");
    mutator.remove_type("EventParameters");
    mutator.remove_type("EventParameter");
    mutator.add_sequence_field("EventDefinition", "parameters", "Parameters", false);

    mutator.remove_type("ErrorParametersDeclaration");
    mutator.remove_type("ErrorParameters");
    mutator.remove_type("ErrorParameter");
    mutator.add_sequence_field("ErrorDefinition", "parameters", "Parameters", false);
}

fn simplify_mapping_type_parameters(mutator: &mut IrModelMutator) {
    // Replace `MappingKey` and `MappingValue` with regular `Parameter` types.
    // `MappingKeyType` is a subset of `TypeName` and can be removed as well.
    mutator.remove_type("MappingKeyType");
    mutator.remove_type("MappingKey");
    mutator.remove_type("MappingValue");

    mutator.add_sequence_field("MappingType", "key_type", "Parameter", false);
    mutator.add_sequence_field("MappingType", "value_type", "Parameter", false);
}
