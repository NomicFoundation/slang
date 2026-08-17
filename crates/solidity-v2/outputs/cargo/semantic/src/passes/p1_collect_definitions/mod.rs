use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::resolution::IdentifierRedeclaration;
use slang_solidity_v2_common::diagnostics::kinds::structure::{
    ConstructorNotInContract, MultipleConstructors,
};
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, FileScope, ParametersScope, Scope, ScopeId};
use crate::context::SemanticFile;

mod conflicts;
mod structural_checks;
mod visitor;

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
    // Import aliases are noted as they are declared, so that the alias
    // resolution below doesn't have to look for them among all definitions.
    let mut imported_symbol_ids = Vec::new();
    for file in files {
        Pass::visit_file(
            file,
            binder,
            language_version,
            diagnostics,
            &mut imported_symbol_ids,
        );
    }

    // The default-import graph is now final. Precompute each file's transitive
    // import closure once, so later passes resolve file-scope symbols with a
    // flat scan instead of re-walking the graph on every lookup.
    binder.precompute_default_import_closures();

    // Every file scope is populated, so each import alias can now be followed
    // to the declaration(s) it names. Resolve them all once here, so the later
    // passes (and the redeclaration checks just below) read the targets off the
    // alias instead of re-walking the chain on every query.
    binder.precompute_imported_symbol_definitions(&imported_symbol_ids);

    // Once every file scope is populated and the import closures are in place,
    // detect redeclaration clashes at file scope. These are handled here
    // (rather than while visiting each file) because resolving them correctly
    // may require following import aliases to declarations in other files,
    // which are only guaranteed to be registered once all files have been
    // visited. It runs after the closures are precomputed because following an
    // alias resolves symbols in file scopes, which relies on them.
    let file_ids = files.iter().map(|file| file.id());
    for (file_id, range) in conflicts::find_file_scope_conflicts(binder, file_ids) {
        diagnostics.push(file_id, range, IdentifierRedeclaration);
    }
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
    scope_stack: Vec<ScopeFrame>,
    // Number of enclosing loops (`for`, `while`, `do-while`) at the current
    // traversal point. Used to flag `break` statements that appear outside any
    // loop.
    loop_depth: usize,
    // Number of enclosing `unchecked` blocks at the current traversal point.
    // Used to flag `unchecked` blocks nested inside another one.
    unchecked_depth: usize,
    // While traversing the body of an implemented modifier, holds whether a
    // placeholder statement (`_`) has been encountered so far. `None` when not
    // inside such a body. Used to flag implemented modifiers whose body lacks a
    // placeholder.
    modifier_placeholder_found: Option<bool>,
    binder: &'a mut Binder,
    language_version: LanguageVersion,
    diagnostics: &'a mut DiagnosticCollection,
    // Accumulates the node ids of the import aliases declared while visiting,
    // across all the files of the compilation unit, for `run` to resolve once
    // every file scope is populated.
    imported_symbol_ids: &'a mut Vec<NodeId>,
}

impl<'a, F: SemanticFile> Pass<'a, F> {
    fn visit_file(
        file: &'a F,
        binder: &'a mut Binder,
        language_version: LanguageVersion,
        diagnostics: &'a mut DiagnosticCollection,
        imported_symbol_ids: &'a mut Vec<NodeId>,
    ) {
        let mut pass = Self {
            current_file: file,
            scope_stack: Vec::new(),
            loop_depth: 0,
            unchecked_depth: 0,
            modifier_placeholder_found: None,
            binder,
            language_version,
            diagnostics,
            imported_symbol_ids,
        };
        ir::visitor::accept_source_unit(file.ir_root(), &mut pass);
        assert!(pass.scope_stack.is_empty());
        assert_eq!(pass.loop_depth, 0);
        assert_eq!(pass.unchecked_depth, 0);
        assert!(pass.modifier_placeholder_found.is_none());
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
    //
    // Conflicts at *file* scope are not checked here: they may involve imported
    // symbols whose target declarations live in files that haven't been visited
    // yet, so following an alias to decide whether two names actually clash is
    // only reliable once every file scope is populated. They are detected in a
    // second step instead (see `conflicts::find_file_scope_conflicts`).
    fn insert_definition_in_scope(&mut self, definition: Definition, scope_id: ScopeId) {
        if !matches!(self.binder.get_scope_by_id(scope_id), Scope::File(_)) {
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
        for parameter in parameters.iter() {
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
        for parameter in parameters.iter() {
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

    /// Reports a diagnostic against the text range of the given IR node.
    fn report(&mut self, node: &dyn ir::TextRange, kind: impl Into<DiagnosticKind>) {
        self.diagnostics.push(
            self.current_file.id().to_owned(),
            node.calculate_text_range()
                .expect("IR node is expected to have a range."),
            kind,
        );
    }
}
