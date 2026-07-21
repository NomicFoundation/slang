use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::resolution::IdentifierRedeclaration;
use slang_solidity_v2_common::diagnostics::kinds::structure::{
    AbstractContractPublicConstructor, BreakOutsideLoop, ConstructorNotInContract,
    ContinueOutsideLoop, EmptyEnum, EmptyStruct, EnumWithTooManyMembers, FreeFunctionPayable,
    FreeFunctionVisibility, FunctionMustBeImplemented, FunctionNameMatchesContainer,
    InterfaceFunctionCannotBeImplemented, InterfaceFunctionNotExternal,
    InvalidUsingDirectiveContainer, LibraryPayableFunction, LibraryVirtualFunction,
    LibraryVirtualModifier, MissingFunctionVisibility, ModifierInInterface, MultipleConstructors,
    NonAbstractContractInternalConstructor, PayableInternalOrPrivateFunction,
    UnimplementedModifierMustBeVirtual, VirtualFreeFunction, VirtualPrivateFunction,
};
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use crate::binder::{
    AssemblyBlock, Binder, Definition, FileScope, ParametersScope, Scope, ScopeId,
};
use crate::context::SemanticFile;

mod conflicts;

/// In this pass all definitions are collected with their naming identifiers.
/// Also lexical (and other kinds of) scopes are identified and linked together,
/// and definitions are registered into them for later lookup. The pass
/// instantiates a `Binder` object which will store all this information as well
/// as references and typing information for the nodes, to be resolved in later
/// passes.
pub fn run(
    files: &[impl SemanticFile],
    binder: &mut Binder,
    language_version: LanguageVersion,
    diagnostics: &mut DiagnosticCollection,
) {
    for file in files {
        Pass::visit_file(file, binder, language_version, diagnostics);
    }

    // Once every file scope is populated, detect clashes involving the
    // symbols brought into each file's scope through default imports (which,
    // unlike aliased/deconstructed imports, don't register a local definition
    // and so can't be caught while visiting a single file).
    let file_ids = files.iter().map(|file| file.id());
    for (file_id, range) in conflicts::find_default_import_conflicts(binder, file_ids) {
        diagnostics.push(file_id, range, IdentifierRedeclaration);
    }

    // The default-import graph is now final. Precompute each file's transitive
    // import closure once, so later passes resolve file-scope symbols with a
    // flat scan instead of re-walking the graph on every lookup.
    binder.precompute_default_import_closures();
}

struct ScopeFrame {
    // Scope associated with the node that created the stack frame. This is
    // solely used for integrity validation when popping the current frame.
    structural_scope_id: ScopeId,
    // Scope to use when resolving a symbol.
    lexical_scope_id: ScopeId,
}

struct Pass<'a, F: SemanticFile> {
    current_file: &'a F,
    language_version: LanguageVersion,
    scope_stack: Vec<ScopeFrame>,
    // Number of enclosing loops (`for`, `while`, `do-while`) at the current
    // traversal point. Used to flag `break` statements that appear outside any
    // loop.
    loop_depth: usize,
    binder: &'a mut Binder,
    diagnostics: &'a mut DiagnosticCollection,
}

impl<'a, F: SemanticFile> Pass<'a, F> {
    fn visit_file(
        file: &'a F,
        binder: &'a mut Binder,
        language_version: LanguageVersion,
        diagnostics: &'a mut DiagnosticCollection,
    ) {
        let mut pass = Self {
            current_file: file,
            language_version,
            scope_stack: Vec::new(),
            loop_depth: 0,
            binder,
            diagnostics,
        };
        ir::visitor::accept_source_unit(file.ir_root(), &mut pass);
        assert!(pass.scope_stack.is_empty());
        assert_eq!(pass.loop_depth, 0);
    }

    fn enter_scope(&mut self, scope: Scope) -> ScopeId {
        let scope_id = self.binder.insert_scope(scope);
        self.scope_stack.push(ScopeFrame {
            structural_scope_id: scope_id,
            lexical_scope_id: scope_id,
        });
        scope_id
    }

    fn replace_scope(&mut self, scope: Scope) -> ScopeId {
        let Some(ScopeFrame {
            structural_scope_id,
            ..
        }) = self.scope_stack.pop()
        else {
            unreachable!("scope stack cannot be empty");
        };

        let scope_id = self.binder.insert_scope(scope);
        self.scope_stack.push(ScopeFrame {
            structural_scope_id,
            lexical_scope_id: scope_id,
        });
        scope_id
    }

    fn leave_scope_for_node_id(&mut self, node_id: NodeId) {
        let Some(ScopeFrame {
            structural_scope_id,
            ..
        }) = self.scope_stack.pop()
        else {
            unreachable!("attempt to pop an empty scope stack");
        };
        assert_eq!(
            structural_scope_id,
            self.binder.scope_id_for_node_id(node_id).unwrap()
        );
    }

    fn current_scope_id(&self) -> ScopeId {
        let Some(ScopeFrame {
            lexical_scope_id, ..
        }) = self.scope_stack.last()
        else {
            unreachable!("empty scope stack");
        };
        *lexical_scope_id
    }

    fn current_scope(&self) -> &Scope {
        let scope_id = self.current_scope_id();
        self.binder.get_scope_by_id(scope_id)
    }

    /// Returns the current (enclosing) definition
    fn enclosing_definition(&self) -> Option<&Definition> {
        self.binder
            .find_definition_by_id(self.current_scope().node_id())
    }

    /// Reports a diagnostic against the text range of the given IR node.
    fn report(&mut self, node: &dyn ir::TextRange, kind: impl Into<DiagnosticKind>) {
        self.diagnostics.push(
            self.current_file.id().to_owned(),
            node.calculate_text_range()
                .expect("IR node is expected to have a range."),
            kind,
        );
    }

    /// Whether the current (enclosing) scope belongs to a library definition.
    fn current_scope_is_library(&self) -> bool {
        matches!(self.enclosing_definition(), Some(Definition::Library(_)))
    }

    /// Whether the current (enclosing) scope belongs to an interface definition.
    fn current_scope_is_interface(&self) -> bool {
        matches!(self.enclosing_definition(), Some(Definition::Interface(_)))
    }

    /// Whether the current (enclosing) scope is the file scope, i.e. the
    /// definition is a free (file-level) one.
    fn current_scope_is_file(&self) -> bool {
        matches!(
            self.binder.get_scope_by_id(self.current_scope_id()),
            Scope::File(_)
        )
    }

    fn current_file_scope(&mut self) -> &mut FileScope {
        let scope_id = self.current_scope_id();
        let Scope::File(file_scope) = self.binder.get_scope_mut(scope_id) else {
            unreachable!("current scope is not a file scope");
        };
        file_scope
    }

    fn insert_definition_in_current_scope(&mut self, definition: Definition) {
        self.insert_definition_in_scope(definition, self.current_scope_id());
    }

    // Registers `definition` under the given scope, first checking whether its
    // identifier collides with a pre-existing definition in that scope. If so,
    // an `IdentifierRedeclaration` diagnostic is emitted; the definition is
    // registered regardless, so later passes can still type this definition and
    // resolve references to it.
    fn insert_definition_in_scope(&mut self, definition: Definition, scope_id: ScopeId) {
        let symbol = definition.identifier().unparse();
        if conflicts::find_conflicting_solidity_definition(
            self.binder,
            scope_id,
            symbol,
            &definition,
        )
        .is_some()
        {
            self.report(definition.identifier(), IdentifierRedeclaration);
        }
        self.binder.insert_definition_in_scope(definition, scope_id);
    }

    fn resolve_import_path(&self, import_node_id: NodeId) -> Option<FileId> {
        self.current_file
            .resolved_import_by_node_id(import_node_id)
            .cloned()

        // TODO(validation) SDR[22]: emit an error/warning if the file cannot be resolved
    }

    // Collects *all* the sequential parameters making and registering
    // definitions for named ones and return the constructed parameters scope ID
    // to link with the enclosing function definition
    fn collect_parameters(&mut self, parameters: &ir::Parameters) -> ScopeId {
        let mut scope = ParametersScope::new();
        for parameter in parameters {
            if let Some(name) = &parameter.name {
                // Parameters cannot overload, so any earlier parameter with
                // the same name is a redeclaration.
                if scope.lookup_definition(&name.text).is_some() {
                    self.report(name, IdentifierRedeclaration);
                }
                let definition = Definition::new_parameter(parameter);
                self.binder.insert_definition_no_scope(definition);
            }
            scope.add_parameter(parameter.name.as_ref().map(|id| &id.text), parameter.id());
        }
        self.binder.insert_scope(Scope::Parameters(scope))
    }

    // This is used to collect only named parameters and insert their
    // definitions into an existing scope. Used mostly for return parameters,
    // where position and types are not used for binding.
    fn collect_named_parameters_into_scope(
        &mut self,
        parameters: &ir::Parameters,
        scope_id: ScopeId,
    ) {
        for parameter in parameters {
            if parameter.name.is_some() {
                let definition = Definition::new_parameter(parameter);
                self.insert_definition_in_scope(definition, scope_id);
            }
        }
    }

    fn register_constructor(
        &mut self,
        node: &ir::FunctionDefinition,
        constructor_parameters_scope_id: ScopeId,
    ) {
        // Constructors are only valid inside contracts. When one appears in an
        // interface or library, flag it and skip tracking its parameter scope
        // (there's no contract to track it against).
        let current_scope_node_id = self.current_scope().node_id();
        match self.binder.get_definition_mut(current_scope_node_id) {
            Definition::Contract(contract_definition) => {
                if contract_definition
                    .constructor_parameters_scope_id
                    .is_some()
                {
                    self.report(node, MultipleConstructors);
                } else {
                    contract_definition.constructor_parameters_scope_id =
                        Some(constructor_parameters_scope_id);
                }
            }
            Definition::Interface(_) | Definition::Library(_) => {
                self.diagnostics.push(
                    self.current_file.id().to_owned(),
                    node.signature_text_range(),
                    ConstructorNotInContract,
                );
            }
            _ => unreachable!(
                "a constructor's enclosing scope must be a contract, interface or library"
            ),
        }
    }

    fn check_function_attributes(&mut self, node: &ir::FunctionDefinition) {
        if node.attributes.is_virtual {
            // A function declared in a library cannot be marked `virtual`.
            if self.current_scope_is_library() {
                self.report(node, LibraryVirtualFunction);

            // A free (file-level) function cannot be marked `virtual`.
            } else if self.current_scope_is_file() {
                self.report(node, VirtualFreeFunction);
            }

            // A `virtual` function cannot also be marked `private`.
            if node.attributes.visibility == ir::FunctionVisibility::Private {
                self.report(node, VirtualPrivateFunction);
            }
        }

        if self.current_scope_is_file() {
            // A free (file-level) function cannot specify a visibility modifier.
            if node.attributes.has_explicit_visibility {
                self.report(node, FreeFunctionVisibility);
            }

            // A free (file-level) function cannot be `payable`.
            if node.attributes.mutability == ir::FunctionMutability::Payable {
                self.report(node, FreeFunctionPayable);
            }
        } else if node.kind == ir::FunctionKind::Regular {
            // The remaining checks only concern regular (named) functions.
            // Constructors are handled separately and fallback/receive functions
            // have their required attributes enforced during IR construction.

            // A regular function inside a contract, interface or library must
            // specify a visibility.
            if !node.attributes.has_explicit_visibility {
                let suggested_visibility = if self.current_scope_is_interface() {
                    "external"
                } else {
                    "public"
                };
                self.report(
                    node,
                    MissingFunctionVisibility {
                        suggested_visibility: suggested_visibility.to_owned(),
                    },
                );
            }

            // An `internal` or `private` function cannot be `payable`. This only
            // applies to an explicitly-declared visibility (an unspecified one
            // defaults to `internal` but is reported as a missing-visibility error
            // instead, matching solc which does not additionally flag it here).
            if node.attributes.has_explicit_visibility
                && node.attributes.mutability == ir::FunctionMutability::Payable
                && matches!(
                    node.attributes.visibility,
                    ir::FunctionVisibility::Internal | ir::FunctionVisibility::Private
                )
            {
                self.report(node, PayableInternalOrPrivateFunction);
            }

            // A function declared in an interface must be `external`. This also
            // fires when no visibility is specified (which defaults to non-external),
            // matching solc's behavior of reporting it alongside the missing-visibility
            // diagnostic above.
            if self.current_scope_is_interface()
                && node.attributes.visibility != ir::FunctionVisibility::External
            {
                self.report(node, InterfaceFunctionNotExternal);
            }

            // A function declared in a library cannot be `payable`.
            if node.attributes.mutability == ir::FunctionMutability::Payable
                && self.current_scope_is_library()
            {
                self.report(node, LibraryPayableFunction);
            }
        }
    }

    /// Check constructor attributes, which are already constrained by the
    /// grammar.  The visibility must be consistent with the contract's
    /// abstract-ness. This only applies when an explicit visibility is given (a
    /// constructor with no visibility is always fine). Only `public` and
    /// `internal` are grammatically valid on constructors.
    fn check_constructor_attributes(&mut self, node: &ir::FunctionDefinition) {
        if node.kind != ir::FunctionKind::Constructor || !node.attributes.has_explicit_visibility {
            return;
        }

        let Some(Definition::Contract(contract_definition)) = self.enclosing_definition() else {
            return;
        };

        match (
            node.attributes.visibility,
            contract_definition.ir_node.is_abstract,
        ) {
            // An abstract contract cannot expose a `public` constructor.
            (ir::FunctionVisibility::Public, true) => {
                self.report(node, AbstractContractPublicConstructor);
            }
            // A non-abstract contract cannot have an `internal` constructor.
            (ir::FunctionVisibility::Internal, false) => {
                self.report(node, NonAbstractContractInternalConstructor);
            }
            _ => {}
        }
    }

    /// Check modifier attributes, also constrained by the grammar.
    fn check_modifier_attributes(&mut self, node: &ir::FunctionDefinition) {
        // A modifier cannot be defined or declared in an interface. solc only
        // began rejecting this in 0.8.8 (error 6408); earlier versions accept
        // it, so gate the diagnostic accordingly.
        if self.language_version >= LanguageVersion::V0_8_8 && self.current_scope_is_interface() {
            self.report(node, ModifierInInterface);
        }

        // A modifier without an implementation body must be marked `virtual`.
        if node.body.is_none() && !node.attributes.is_virtual {
            self.report(node, UnimplementedModifierMustBeVirtual);
        }

        // A modifier declared in a library cannot be marked `virtual`.
        if node.attributes.is_virtual && self.current_scope_is_library() {
            self.report(node, LibraryVirtualModifier);
        }
    }
}

impl<F: SemanticFile> Visitor for Pass<'_, F> {
    fn enter_source_unit(&mut self, node: &ir::SourceUnit) -> bool {
        let scope = Scope::new_file(node.id(), self.current_file.id());
        self.enter_scope(scope);

        true
    }

    fn leave_source_unit(&mut self, node: &ir::SourceUnit) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_contract_definition(&mut self, node: &ir::ContractDefinition) -> bool {
        let definition = Definition::new_contract(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_contract_definition(&mut self, node: &ir::ContractDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_library_definition(&mut self, node: &ir::LibraryDefinition) -> bool {
        let definition = Definition::new_library(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_library_definition(&mut self, node: &ir::LibraryDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_interface_definition(&mut self, node: &ir::InterfaceDefinition) -> bool {
        let definition = Definition::new_interface(node);
        self.insert_definition_in_current_scope(definition);

        let scope = Scope::new_contract(node.id(), self.current_scope_id());
        self.enter_scope(scope);

        true
    }

    fn leave_interface_definition(&mut self, node: &ir::InterfaceDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_using_directive(&mut self, node: &ir::UsingDirective) -> bool {
        let current_scope_node_id = self.current_scope().node_id();
        let in_allowed_container = match self.binder.find_definition_by_id(current_scope_node_id) {
            None => {
                true // allow in global (file-level) scope
            }
            Some(definition) => {
                matches!(definition, Definition::Contract(_) | Definition::Library(_))
            }
        };

        if !in_allowed_container {
            self.report(node, InvalidUsingDirectiveContainer);
        }

        true
    }

    fn enter_path_import(&mut self, node: &ir::PathImport) -> bool {
        let imported_file_id = self.resolve_import_path(node.id());

        if node.alias.is_some() {
            let definition = Definition::new_import(node, imported_file_id);
            self.insert_definition_in_current_scope(definition);
        } else if let Some(imported_file_id) = imported_file_id {
            self.current_file_scope()
                .add_default_import(imported_file_id, node.range.clone());
        }

        false
    }

    fn enter_import_deconstruction(&mut self, node: &ir::ImportDeconstruction) -> bool {
        let imported_file_id = self.resolve_import_path(node.id());

        for symbol in &node.symbols {
            let definition = Definition::new_imported_symbol(
                symbol,
                symbol.name.unparse().to_owned(),
                imported_file_id.clone(),
            );
            self.insert_definition_in_current_scope(definition);
        }

        false
    }

    fn enter_function_definition(&mut self, node: &ir::FunctionDefinition) -> bool {
        match node.kind {
            ir::FunctionKind::Regular
            | ir::FunctionKind::Constructor
            | ir::FunctionKind::Fallback
            | ir::FunctionKind::Receive => {
                if matches!(node.kind, ir::FunctionKind::Constructor) {
                    self.check_constructor_attributes(node);
                } else {
                    self.check_function_attributes(node);
                }

                let parameters_scope_id = self.collect_parameters(&node.parameters);

                if let Some(name) = &node.name {
                    let definition = Definition::new_function(node, parameters_scope_id);

                    let enclosing_definition = self.enclosing_definition();
                    let enclosing_container_name = enclosing_definition
                        .filter(|enclosing_definition| {
                            matches!(
                                enclosing_definition,
                                Definition::Contract(_)
                                    | Definition::Interface(_)
                                    | Definition::Library(_)
                            )
                        })
                        .map(|definition| definition.identifier().unparse());

                    if enclosing_container_name
                        .is_some_and(|container_name| container_name == name.unparse())
                    {
                        self.report(node, FunctionNameMatchesContainer);

                        // Skip registering the function symbol in the current scope
                        // to avoid interference with resolution.
                        self.binder.insert_definition_no_scope(definition);
                    } else {
                        self.insert_definition_in_current_scope(definition);
                    }
                } else if matches!(node.kind, ir::FunctionKind::Constructor) {
                    // Register the constructor to resolve named parameters when
                    // constructing this contract
                    self.register_constructor(node, parameters_scope_id);
                }

                if node.body.is_none() {
                    // A free (file-level) function or a function declared in a
                    // library must have an implementation body.
                    if self.current_scope_is_file() || self.current_scope_is_library() {
                        self.report(node, FunctionMustBeImplemented);
                    }
                } else {
                    // Conversely, a function declared in an interface cannot
                    // have an implementation body. Skip constructors explicitly
                    // as they cannot be in interfaces anyways.
                    if self.current_scope_is_interface()
                        && !matches!(node.kind, ir::FunctionKind::Constructor)
                    {
                        self.report(node, InterfaceFunctionCannotBeImplemented);
                    }
                }

                let function_scope =
                    Scope::new_function(node.id(), self.current_scope_id(), parameters_scope_id);
                let function_scope_id = self.enter_scope(function_scope);

                if let Some(returns) = &node.returns {
                    self.collect_named_parameters_into_scope(returns, function_scope_id);
                }
            }

            ir::FunctionKind::Modifier => {
                self.check_modifier_attributes(node);

                let definition = Definition::new_modifier(node);
                self.insert_definition_in_current_scope(definition);

                let modifier_scope = Scope::new_modifier(node.id(), self.current_scope_id());
                let modifier_scope_id = self.enter_scope(modifier_scope);
                self.collect_named_parameters_into_scope(&node.parameters, modifier_scope_id);
            }
        }
        true
    }

    fn leave_function_definition(&mut self, node: &ir::FunctionDefinition) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_enum_definition(&mut self, node: &ir::EnumDefinition) -> bool {
        let definition = Definition::new_enum(node);
        self.insert_definition_in_current_scope(definition);

        // An enum must declare at least one member, and at most 256 (its values
        // must fit in a single byte).
        if node.members.is_empty() {
            self.report(node, EmptyEnum);
        } else if node.members.len() > 256 {
            self.report(node, EnumWithTooManyMembers);
        }

        let enum_scope = Scope::new_enum(node.id());
        let enum_scope_id = self.binder.insert_scope(enum_scope);
        for member in &node.members {
            let definition = Definition::new_enum_member(member);
            self.insert_definition_in_scope(definition, enum_scope_id);
        }

        false
    }

    fn enter_struct_definition(&mut self, node: &ir::StructDefinition) -> bool {
        let definition = Definition::new_struct(node);
        self.insert_definition_in_current_scope(definition);

        // A struct must declare at least one member.
        if node.members.is_empty() {
            self.report(node, EmptyStruct);
        }

        let struct_scope = Scope::new_struct(node.id());
        let struct_scope_id = self.binder.insert_scope(struct_scope);
        for member in &node.members {
            let definition = Definition::new_struct_member(member);
            self.insert_definition_in_scope(definition, struct_scope_id);
        }

        true
    }

    fn enter_error_definition(&mut self, node: &ir::ErrorDefinition) -> bool {
        let parameters_scope_id = self.collect_parameters(&node.parameters);
        let definition = Definition::new_error(node, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_event_definition(&mut self, node: &ir::EventDefinition) -> bool {
        let parameters_scope_id = self.collect_parameters(&node.parameters);
        let definition = Definition::new_event(node, parameters_scope_id);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_state_variable_definition(&mut self, node: &ir::StateVariableDefinition) -> bool {
        let definition = Definition::new_state_variable(node);
        self.insert_definition_in_current_scope(definition);

        // there may be more definitions in the type of the state variable (eg.
        // key/value names in mappings)
        true
    }

    fn enter_constant_definition(&mut self, node: &ir::ConstantDefinition) -> bool {
        let definition = Definition::new_constant(node, self.current_scope_id());
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn enter_user_defined_value_type_definition(
        &mut self,
        node: &ir::UserDefinedValueTypeDefinition,
    ) -> bool {
        let definition = Definition::new_user_defined_value_type(node);
        self.insert_definition_in_current_scope(definition);

        false
    }

    fn leave_variable_declaration_statement(&mut self, node: &ir::VariableDeclarationStatement) {
        // Open a new scope that replaces but is linked to the current one so
        // definitions declared here are only available for statements after
        // this one. This is a "chained" scope that continues the parent's
        // lexical scope, not a new lexical scope of its own.
        let scope = Scope::new_chained(node.id(), self.current_scope_id());
        self.replace_scope(scope);

        match &node.target {
            ir::VariableDeclarationTarget::SingleTypedDeclaration(single) => {
                let definition = Definition::new_variable(&single.declaration);
                self.insert_definition_in_current_scope(definition);
            }
            ir::VariableDeclarationTarget::MultiTypedDeclaration(multi) => {
                for element in &multi.elements {
                    if let Some(member) = &element.member {
                        let definition = Definition::new_variable(member);
                        self.insert_definition_in_current_scope(definition);
                    }
                }
            }
        }
    }

    fn enter_block(&mut self, node: &ir::Block) -> bool {
        let scope = Scope::new_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);
        true
    }

    fn leave_block(&mut self, node: &ir::Block) {
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_for_statement(&mut self, node: &ir::ForStatement) -> bool {
        // Open a new block here to hold declarations in the initialization
        // clause. This is a new lexical scope.
        let scope = Scope::new_block(node.id(), self.current_scope_id());
        self.enter_scope(scope);
        self.loop_depth += 1;
        true
    }

    fn leave_for_statement(&mut self, node: &ir::ForStatement) {
        self.loop_depth -= 1;
        self.leave_scope_for_node_id(node.id());
    }

    fn enter_while_statement(&mut self, _node: &ir::WhileStatement) -> bool {
        self.loop_depth += 1;
        true
    }

    fn leave_while_statement(&mut self, _node: &ir::WhileStatement) {
        self.loop_depth -= 1;
    }

    fn enter_do_while_statement(&mut self, _node: &ir::DoWhileStatement) -> bool {
        self.loop_depth += 1;
        true
    }

    fn leave_do_while_statement(&mut self, _node: &ir::DoWhileStatement) {
        self.loop_depth -= 1;
    }

    fn enter_break_statement(&mut self, node: &ir::BreakStatement) -> bool {
        // A `break` statement is only valid inside a `for`, `while` or
        // `do-while` loop.
        if self.loop_depth == 0 {
            self.report(node, BreakOutsideLoop);
        }
        true
    }

    fn enter_continue_statement(&mut self, node: &ir::ContinueStatement) -> bool {
        // A `continue` statement is only valid inside a `for`, `while` or
        // `do-while` loop.
        if self.loop_depth == 0 {
            self.report(node, ContinueOutsideLoop);
        }
        true
    }

    fn leave_try_statement(&mut self, node: &ir::TryStatement) {
        if let Some(returns) = &node.returns {
            // Collect the parameters in the returns declaration of the try
            // statement and make them available in the body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.id()).unwrap();
            self.collect_named_parameters_into_scope(returns, body_scope_id);
        }
    }

    fn leave_catch_clause(&mut self, node: &ir::CatchClause) {
        if let Some(error) = &node.error {
            // Collect the parameters in the catch declaration and make them
            // available in the body block.
            let body_scope_id = self.binder.scope_id_for_node_id(node.body.id()).unwrap();
            self.collect_named_parameters_into_scope(&error.parameters, body_scope_id);
        }
    }

    fn enter_mapping_type(&mut self, node: &ir::MappingType) -> bool {
        if node.key_type.name.is_some() {
            let definition = Definition::new_type_parameter(&node.key_type);
            self.binder.insert_definition_no_scope(definition);
        }
        if node.value_type.name.is_some() {
            let definition = Definition::new_type_parameter(&node.value_type);
            self.binder.insert_definition_no_scope(definition);
        }

        true
    }

    fn enter_function_type(&mut self, node: &ir::FunctionType) -> bool {
        for parameter in &node.parameters {
            if parameter.name.is_some() {
                let definition = Definition::new_type_parameter(parameter);
                self.binder.insert_definition_no_scope(definition);
            }
        }
        if let Some(returns) = &node.returns {
            for parameter in returns {
                if parameter.name.is_some() {
                    let definition = Definition::new_type_parameter(parameter);
                    self.binder.insert_definition_no_scope(definition);
                }
            }
        }

        false
    }

    fn enter_assembly_statement(&mut self, node: &ir::AssemblyStatement) -> bool {
        // Record the assembly block (with the enclosing Solidity scope) so that
        // `p6_resolve_yul` can process only these branches instead of walking
        // the full IR tree, and so the backend has a per-block record of the
        // Solidity definitions it references (filled in by p6).
        self.binder.insert_assembly_block(AssemblyBlock {
            ir_node: Arc::clone(node),
            enclosing_scope_id: self.current_scope_id(),
            solidity_references: Vec::new(),
        });
        // Keep visiting the statement's label/flags; `enter_yul_block` below
        // still skips the Yul body.
        true
    }

    fn enter_yul_block(&mut self, _node: &ir::YulBlock) -> bool {
        // All Yul is collected and resolved in `p6_resolve_yul`, so there's
        // nothing to do here. Skip the assembly body entirely. (The enclosing
        // `AssemblyStatement`'s flags/label are still visited, since we don't
        // skip from `enter_assembly_statement`.)
        false
    }
}
