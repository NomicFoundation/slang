use slang_solidity_v2_common::diagnostics::kinds::structure::{
    AbstractContractPublicConstructor, FreeFunctionPayable, FreeFunctionVisibility,
    FreeFunctionWithModifiers, FreeFunctionWithOverride, FunctionMustBeImplemented,
    GlobalUsingForInsideContract, GlobalUsingForWildcard, InterfaceFunctionCannotBeImplemented,
    InterfaceFunctionNotExternal, InterfaceFunctionWithModifiers, InvalidUsingDirectiveContainer,
    LibraryNonConstantStateVariable, LibraryPayableFunction, LibraryVirtualFunction,
    LibraryVirtualModifier, MissingFunctionVisibility, ModifierInInterface,
    NonAbstractContractInternalConstructor, PayableInternalOrPrivateFunction,
    UncheckedBlockNotInRegularBlock, UnimplementedFunctionWithModifiers,
    UnimplementedModifierMustBeVirtual, UsingForWildcardAtFileLevel, VariableDeclarationNotInBlock,
    VariableInInterface, VirtualFreeFunction, VirtualPrivateFunction,
};
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::Definition;
use crate::context::SemanticFile;

impl<F: SemanticFile> Pass<'_, F> {
    pub(super) fn check_using_directive(&mut self, node: &ir::UsingDirective) {
        let current_scope_node_id = self.current_scope().node_id();
        let container = self.binder.find_definition_by_id(current_scope_node_id);
        // `None` means the directive is at the file level; any definition here
        // is a contract, library or interface (the only containers that can hold
        // a `using` directive).
        let at_file_level = container.is_none();
        let in_allowed_container = match container {
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

        let targets_wildcard = matches!(node.target, ir::UsingTarget::Asterisk(_));

        // The target type must be spelled out explicitly at the file level; the
        // wildcard `*` is only allowed inside a contract, library or interface.
        if at_file_level && targets_wildcard {
            self.report(node, UsingForWildcardAtFileLevel);
        }

        // `global` is only meaningful at the file level.
        if node.is_global && !at_file_level {
            self.report(node, GlobalUsingForInsideContract);
        }

        // `global` can only attach functions to a specific type, not to `*`.
        if node.is_global && targets_wildcard {
            self.report(node, GlobalUsingForWildcard);
        }
    }

    /// Reports a diagnostic if the given statement, used as the un-braced body
    /// of a control-flow statement, is one that is only allowed directly inside
    /// a block: a variable declaration or an `unchecked` block.
    pub(super) fn check_control_flow_body(&mut self, body: &ir::Statement) {
        match body {
            ir::Statement::VariableDeclarationStatement(declaration) => {
                self.report(declaration, VariableDeclarationNotInBlock);
            }
            ir::Statement::UncheckedBlock(unchecked_block) => {
                self.report(unchecked_block, UncheckedBlockNotInRegularBlock);
            }
            _ => {}
        }
    }

    pub(super) fn check_function_attributes(&mut self, node: &ir::FunctionDefinition) {
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

            // A free (file-level) function cannot have modifier invocations.
            if !node.attributes.modifier_invocations.is_empty() {
                self.report(node, FreeFunctionWithModifiers);
            }

            // A free (file-level) function cannot carry an `override` specifier.
            if node.attributes.override_specifier.is_some() {
                self.report(node, FreeFunctionWithOverride);
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

            // A function declared in an interface cannot have modifier
            // invocations (solc 5842). Otherwise, any function without an
            // implementation body (eg. an abstract function in a contract)
            // cannot have them either (solc 2668). solc reports only one of the
            // two, so mirror that with an `else if`.
            if !node.attributes.modifier_invocations.is_empty() {
                if self.current_scope_is_interface() {
                    self.report(node, InterfaceFunctionWithModifiers);
                } else if node.body.is_none() {
                    self.report(node, UnimplementedFunctionWithModifiers);
                }
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
    pub(super) fn check_constructor_attributes(&mut self, node: &ir::FunctionDefinition) {
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

    pub(super) fn check_constructor_or_function_body(&mut self, node: &ir::FunctionDefinition) {
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
    }

    /// Check modifier attributes, also constrained by the grammar.
    pub(super) fn check_modifier_attributes(&mut self, node: &ir::FunctionDefinition) {
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

    pub(super) fn check_state_variable_container(&mut self, node: &ir::StateVariableDefinition) {
        // Interfaces cannot declare any variables.
        if self.current_scope_is_interface() {
            self.report(node, VariableInInterface);
        }
        // Libraries can only declare `constant` state variables.
        else if self.current_scope_is_library()
            && node.attributes.mutability != ir::StateVariableMutability::Constant
        {
            self.report(node, LibraryNonConstantStateVariable);
        }
    }

    pub(super) fn check_constant_container(&mut self, node: &ir::ConstantDefinition) {
        // Interfaces cannot declare any variables, including `constant`s. Note
        // that a non-`public` `constant` state variable is lowered to a
        // `ConstantDefinition` in the IR, so this complements the check in
        // `enter_state_variable_definition` (which catches `public constant`s
        // and non-`constant` state variables).
        if self.current_scope_is_interface() {
            self.report(node, VariableInInterface);
        }
    }
}
