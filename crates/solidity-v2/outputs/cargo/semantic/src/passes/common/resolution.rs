use std::ops::Range;
use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::resolution::{
    AmbiguousReference, IdentifierNotFound, IdentifierNotFunctionOrNotUnique,
    IdentifierNotLibraryName, IdentifierNotNamespace, NamespaceMemberNotFound,
    NonFreeOrLibraryFunctionInUsingDirective,
};
use slang_solidity_v2_common::diagnostics::kinds::semantic::OverridableKind;
use slang_solidity_v2_common::diagnostics::kinds::type_system::InvalidBase;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition, DefinitionIds, Reference, Resolution, ScopeId, Typing};
use crate::types::{FunctionType, Type, TypeId, TypeRegistry};

/// What an `IdentifierPath` is written to name. Looking a name up does not
/// depend on it: the nearest declaration wins, whatever its kind. The context
/// then decides whether the declaration the path ends at is acceptable.
#[derive(Clone, Copy)]
pub(crate) enum PathContext {
    /// A type name: a contract, interface, library, struct, enum or
    /// user-defined value type.
    Type,
    /// A base in an inheritance list.
    Base,
    /// An entry of an `override` list.
    Override,
    /// A modifier invocation: a modifier, or a base whose constructor is
    /// invoked.
    ModifierInvocation,
    /// The library of a `using L for` directive.
    UsingLibrary,
    /// A function of a `using {f} for` directive.
    UsingFunction,
    /// The event of an `emit`.
    Event,
    /// The error of a `revert`.
    Error,
}

impl PathContext {
    /// How an identifier after the first is looked up in the namespace the
    /// previous one names. An event or error is named as a member of a type
    /// name used as a value, like `C.E` in an expression.
    fn member_resolver(self) -> fn(&Binder, ScopeId, &str) -> Resolution {
        match self {
            Self::Event | Self::Error => Binder::resolve_in_scope_as_expression_member,
            Self::Type
            | Self::Base
            | Self::Override
            | Self::ModifierInvocation
            | Self::UsingLibrary
            | Self::UsingFunction => Binder::resolve_in_scope_as_namespace,
        }
    }

    /// Whether the path may end at several declarations, which are narrowed
    /// down later: the overloads of an event, by the arguments of the `emit`,
    /// and the overrides of a virtual modifier, by the contract it runs in.
    fn accepts_set(self, binder: &Binder, definition_ids: &[NodeId]) -> bool {
        match self {
            Self::Event => true,
            Self::ModifierInvocation => definition_ids.iter().all(|definition_id| {
                matches!(
                    binder.find_definition_by_id(*definition_id),
                    Some(Definition::Modifier(_))
                )
            }),
            Self::Type
            | Self::Base
            | Self::Override
            | Self::UsingLibrary
            | Self::UsingFunction
            | Self::Error => false,
        }
    }

    /// Checks what the path ends at, returning the diagnostic to report when
    /// this context cannot take it. Several declarations that are not a set
    /// the context accepts are a redeclaration, reported on its own, but a
    /// `using` directive cannot attach any of them either.
    fn check_resolution(
        self,
        binder: &Binder,
        resolution: &Resolution,
    ) -> Result<(), DiagnosticKind> {
        match resolution {
            Resolution::Definition(definition_id) => self.check_definition(binder, *definition_id),
            Resolution::Ambiguous(_) => self.unusable_name_diagnostic().map_or(Ok(()), Err),
            Resolution::BuiltIn(_) | Resolution::Unresolved => Ok(()),
        }
    }

    /// Checks a single declaration the path ends at.
    fn check_definition(
        self,
        binder: &Binder,
        definition_id: NodeId,
    ) -> Result<(), DiagnosticKind> {
        let definition = binder
            .find_definition_by_id(definition_id)
            .expect("a resolution names a registered definition");
        match self {
            Self::Base => match definition {
                Definition::Contract(_) | Definition::Interface(_) => Ok(()),
                _ => Err(InvalidBase.into()),
            },
            Self::UsingLibrary => match definition {
                Definition::Library(_) => Ok(()),
                _ => Err(IdentifierNotLibraryName.into()),
            },
            Self::UsingFunction => match definition {
                Definition::Function(_) if is_free_or_library_function(binder, definition_id) => {
                    Ok(())
                }
                Definition::Function(_) => Err(NonFreeOrLibraryFunctionInUsingDirective.into()),
                _ => Err(IdentifierNotFunctionOrNotUnique.into()),
            },
            Self::Type | Self::Override | Self::ModifierInvocation | Self::Event | Self::Error => {
                Ok(())
            }
        }
    }

    /// The diagnostic a `using` directive reports for a name it cannot attach,
    /// in place of saying why the name failed to resolve.
    fn unusable_name_diagnostic(self) -> Option<DiagnosticKind> {
        match self {
            Self::UsingLibrary => Some(IdentifierNotLibraryName.into()),
            Self::UsingFunction => Some(IdentifierNotFunctionOrNotUnique.into()),
            Self::Type
            | Self::Base
            | Self::Override
            | Self::ModifierInvocation
            | Self::Event
            | Self::Error => None,
        }
    }
}

/// Whether the function `definition_id` is free, ie. has no enclosing
/// definition, or is declared in a library.
fn is_free_or_library_function(binder: &Binder, definition_id: NodeId) -> bool {
    binder
        .enclosing_definition_node_id(definition_id)
        .is_none_or(|enclosing_id| {
            matches!(
                binder.find_definition_by_id(enclosing_id),
                Some(Definition::Library(_))
            )
        })
}

/// The first identifier of an `IdentifierPath` that fails to resolve to a
/// declaration the path can use. The identifiers after it cannot resolve and
/// are not considered.
pub(crate) enum PathResolutionFailure {
    /// Every identifier resolves to a declaration the path can use.
    None,
    /// The identifier at this index resolves to nothing, or is reached
    /// through an alias that leads nowhere.
    Unresolved(usize),
    /// The identifier at this index names an overload set, or the overrides
    /// of a virtual modifier, that its position cannot take. A redeclaration
    /// also names several declarations, but is reported on its own.
    Ambiguous(usize),
    /// The identifier at this index is a qualifier naming a declaration that
    /// has no members, like a variable or a struct.
    NotANamespace(usize),
    /// The last identifier names a declaration of a kind the context cannot
    /// take, reported with this diagnostic. Its reference is left unresolved.
    WrongKind(DiagnosticKind),
}

/// The outcome of resolving an `IdentifierPath`.
pub(crate) struct PathResolution {
    /// The resolution of the last identifier, with symbol aliases followed.
    pub(crate) resolution: Resolution,
    pub(crate) failure: PathResolutionFailure,
    context: PathContext,
}

impl PathResolution {
    /// The identifier that failed to resolve, with the diagnostic to report
    /// for it: the head of the path is undeclared, any later identifier is a
    /// member that the preceding one does not provide, a qualifier without
    /// members is not a namespace, an identifier naming several declarations
    /// is ambiguous, and the last one may name the wrong kind of declaration.
    pub(crate) fn diagnostic<'a>(
        &self,
        identifier_path: &'a ir::IdentifierPath,
    ) -> Option<(&'a ir::Identifier, DiagnosticKind)> {
        let last = identifier_path
            .last()
            .expect("an identifier path has at least one identifier");
        if let PathResolutionFailure::WrongKind(kind) = &self.failure {
            return Some((last, kind.clone()));
        }
        let (identifier, kind) = self.resolution_diagnostic(identifier_path)?;
        Some(match self.context.unusable_name_diagnostic() {
            Some(kind) => (last, kind),
            None => (identifier, kind),
        })
    }

    /// The diagnostic for an identifier that does not resolve to a single
    /// declaration.
    fn resolution_diagnostic<'a>(
        &self,
        identifier_path: &'a ir::IdentifierPath,
    ) -> Option<(&'a ir::Identifier, DiagnosticKind)> {
        match &self.failure {
            PathResolutionFailure::None | PathResolutionFailure::WrongKind(_) => None,
            PathResolutionFailure::Unresolved(index) => {
                let index = *index;
                let identifier = &identifier_path[index];
                let kind = if index == 0 {
                    IdentifierNotFound.into()
                } else {
                    namespace_member_not_found(identifier_path, index).into()
                };
                Some((identifier, kind))
            }
            PathResolutionFailure::Ambiguous(index) => {
                let identifier = &identifier_path[*index];
                let kind = AmbiguousReference {
                    name: identifier.unparse().to_owned(),
                }
                .into();
                Some((identifier, kind))
            }
            PathResolutionFailure::NotANamespace(index) => {
                let identifier = &identifier_path[*index];
                let kind = IdentifierNotNamespace {
                    name: identifier.unparse().to_owned(),
                }
                .into();
                Some((identifier, kind))
            }
        }
    }
}

/// The diagnostic for the identifier at `index` of `identifier_path` not being
/// provided by the identifiers before it.
pub(crate) fn namespace_member_not_found(
    identifier_path: &ir::IdentifierPath,
    index: usize,
) -> NamespaceMemberNotFound {
    NamespaceMemberNotFound {
        name: identifier_path[index].unparse().to_owned(),
        namespace: identifier_path[..index]
            .iter()
            .map(|qualifier| qualifier.unparse())
            .collect::<Vec<_>>()
            .join("."),
    }
}

/// Resolves an `IdentifierPath` written in `context`, starting from the given
/// scope, and creates `Reference`s for all its elements. The first identifier
/// is looked up lexically, and each later one as a member of the namespace
/// (contract, interface, library or import) the previous one names.
pub(crate) fn resolve_identifier_path(
    binder: &mut Binder,
    identifier_path: &ir::IdentifierPath,
    starting_scope_id: ScopeId,
    context: PathContext,
) -> PathResolution {
    let resolve_member = context.member_resolver();
    let last_index = identifier_path.len() - 1;
    let mut scope_id = Some(starting_scope_id);
    let mut resolution = Resolution::Unresolved;
    let mut failure = PathResolutionFailure::None;
    let mut is_reportable = true;

    for (index, identifier) in identifier_path.iter().enumerate() {
        let symbol = identifier.unparse();
        resolution = if let Some(scope_id) = scope_id {
            if index == 0 {
                binder.resolve_in_scope(scope_id, symbol)
            } else {
                resolve_member(binder, scope_id, symbol)
            }
        } else {
            Resolution::Unresolved
        };
        let is_unresolved = matches!(resolution, Resolution::Unresolved);

        let reference = Reference::new(Arc::clone(identifier), resolution.clone());
        binder.insert_reference(reference);

        resolution = binder.follow_symbol_aliases(resolution);

        // A set the context narrows down later names the first declaration
        // for now: the most derived override of a virtual modifier.
        if index == last_index
            && let Resolution::Ambiguous(definition_ids) = &resolution
            && context.accepts_set(binder, definition_ids)
        {
            if matches!(context, PathContext::ModifierInvocation) {
                resolution = Resolution::Definition(definition_ids[0]);
                binder.fixup_reference(identifier.id(), resolution.clone());
            }
            break;
        }

        // Only the first identifier not resolving to a single declaration is
        // reported. An alias leading nowhere is reported where it is
        // imported, so ambiguity is judged after following aliases but
        // unresolution before.
        if is_reportable {
            if is_unresolved {
                failure = PathResolutionFailure::Unresolved(index);
            } else if let Resolution::Ambiguous(definition_ids) = &resolution
                && is_overload_or_override_set(binder, definition_ids)
            {
                failure = PathResolutionFailure::Ambiguous(index);
            }
            is_reportable &= matches!(resolution, Resolution::Definition(_));
        }

        // Change the resolution scope to be that of the last resolved
        // definition, so we can resolve the next identifier in the path.
        scope_id = resolution
            .as_definition_id()
            .and_then(|definition_id| find_definition_namespace_scope_id(binder, definition_id));

        // A qualifier with no scope to look its member up in. An import of a
        // file that could not be resolved is reported where it is imported.
        if is_reportable && scope_id.is_none() && index < last_index {
            let is_unresolved_import = resolution.as_definition_id().is_some_and(|definition_id| {
                matches!(
                    binder.find_definition_by_id(definition_id),
                    Some(Definition::Import(_))
                )
            });
            if !is_unresolved_import {
                failure = PathResolutionFailure::NotANamespace(index);
            }
            is_reportable = false;
        }
    }

    // A path that resolves names declarations of some kind, which the
    // context may not take. Its reference is then left unresolved.
    if matches!(failure, PathResolutionFailure::None)
        && let Err(kind) = context.check_resolution(binder, &resolution)
    {
        let last = &identifier_path[last_index];
        binder.fixup_reference(last.id(), Resolution::Unresolved);
        resolution = Resolution::Unresolved;
        failure = PathResolutionFailure::WrongKind(kind);
    }

    PathResolution {
        resolution,
        failure,
        context,
    }
}

/// Whether the declarations sharing a name are overloads or overrides of each
/// other, rather than a redeclaration. Only functions and events can be
/// overloaded, and modifiers resolve to all of their overrides.
fn is_overload_or_override_set(binder: &Binder, definition_ids: &[NodeId]) -> bool {
    definition_ids.iter().all(|definition_id| {
        matches!(
            binder.find_definition_by_id(*definition_id),
            Some(Definition::Function(_) | Definition::Event(_) | Definition::Modifier(_))
        )
    })
}

/// When a symbol resolves to an ambiguous set of Solidity functions
/// (overloads/virtuals), drop the ones overridden by a previously seen
/// definition with the same declared parameter types. Two functions that differ in a data
/// location are two overloads here. This is how a bare name or a `super`
/// member is looked up. This reads function typing information, so it requires
/// `p3_type_definitions` to have run.
pub(crate) fn filter_overridden_definitions(
    binder: &Binder,
    types: &TypeRegistry,
    resolution: Resolution,
) -> Resolution {
    filter_overridden_functions(binder, types, resolution, |candidate, earlier| {
        candidate.parameter_types == earlier.parameter_types
    })
}

/// The same for the members of a contract value, reached through `this` or an
/// expression of a contract or interface type, and for the members of a type
/// name. Those collapse by selector, so `calldata` and `memory` count as the
/// same parameter and a base is overridden even when a data location differs.
/// A type name reaches only its contract's own members, none of which share a
/// selector.
pub(crate) fn filter_overridden_definitions_by_selector(
    binder: &Binder,
    types: &TypeRegistry,
    resolution: Resolution,
) -> Resolution {
    filter_overridden_functions(binder, types, resolution, |candidate, earlier| {
        types.parameter_lists_are_indistinguishable(
            &candidate.parameter_types,
            &earlier.parameter_types,
        )
    })
}

/// Walks the candidates most-derived first and drops each function that an
/// earlier candidate overrides according to `overridden_by`. Only a member
/// declared in a contract or interface carries an implicit receiver, and a
/// `using` directive does not give one, so free and library functions never
/// override each other, even when they share a signature. Two of them attached
/// to the same type by different directives are competing candidates, not an
/// override pair.
fn filter_overridden_functions(
    binder: &Binder,
    types: &TypeRegistry,
    resolution: Resolution,
    overridden_by: impl Fn(&FunctionType, &FunctionType) -> bool,
) -> Resolution {
    // TODO: it may be possible/desirable to use information procured in
    // `p4_compute_linearisations` here.
    let Resolution::Ambiguous(definition_ids) = resolution else {
        return resolution;
    };
    let mut seen_function_types: Vec<&FunctionType> = Vec::new();
    let mut filtered_definitions = DefinitionIds::new();
    for definition_id in definition_ids {
        match binder.find_definition_by_id(definition_id).unwrap() {
            Definition::Function(_) => {
                if let &Typing::Resolved(type_id) = binder.node_typing(definition_id) {
                    let Type::Function(function_type) = types.get_type_by_id(type_id) else {
                        unreachable!("type of function definition is not a function");
                    };
                    if function_type.implicit_receiver_type.is_some()
                        && seen_function_types.iter().any(|seen_function_type| {
                            seen_function_type.implicit_receiver_type.is_some()
                                && overridden_by(function_type, seen_function_type)
                        })
                    {
                        // the function is overridden by some other previously seen definition
                        continue;
                    }
                    seen_function_types.push(function_type);
                }
            }
            Definition::StateVariable(state_variable) => {
                // TODO(validation) SDR[36]: the state variable should have the
                // `override` attribute and the rest of the definitions should
                // be either functions with the correct signature, state
                // variables or private variables or constants.
                //
                // Remember the getter type if present to override functions in
                // bases.
                if let Some(getter_type_id) = state_variable.getter_type_id {
                    let Type::Function(getter_type) = types.get_type_by_id(getter_type_id) else {
                        unreachable!("getter function type is not a function")
                    };
                    seen_function_types.push(getter_type);
                }
            }
            _ => {}
        }
        filtered_definitions.push(definition_id);
    }
    Resolution::from(filtered_definitions)
}

/// A member that can take part in overriding. A function, modifier or public
/// state variable, viewed through the properties the override rules compare.
#[derive(Clone, Copy)]
pub(crate) enum Overridable<'a> {
    /// A regular function, a fallback or a receive.
    Function {
        definition: &'a ir::FunctionDefinition,
        /// Whether the member is declared in an interface. A function declared
        /// in an interface is implicitly `virtual`.
        in_interface: bool,
    },
    Modifier(&'a ir::ModifierDefinition),
    /// A `public` state variable.
    StateVariable(&'a ir::StateVariableDefinition),
}

impl<'a> Overridable<'a> {
    /// The overridable members of the contract or interface `type_id`.
    pub(crate) fn members_of(
        binder: &'a Binder,
        type_id: NodeId,
    ) -> impl Iterator<Item = Self> + 'a {
        let (members, in_interface) = match binder.find_definition_by_id(type_id) {
            Some(Definition::Contract(contract)) => (&contract.ir_node.members[..], false),
            Some(Definition::Interface(interface)) => (&interface.ir_node.members[..], true),
            _ => unreachable!("a type with bases is a contract or an interface"),
        };
        members
            .iter()
            .filter_map(move |member| Self::of(member, in_interface))
    }

    /// The overridable view of `member`, or `None` for members that don't take
    /// part in overriding.
    pub(crate) fn of(member: &'a ir::ContractMember, in_interface: bool) -> Option<Self> {
        match member {
            ir::ContractMember::FunctionDefinition(definition) => {
                Self::of_function(definition, in_interface)
            }
            ir::ContractMember::ModifierDefinition(definition) => Some(Self::Modifier(definition)),
            ir::ContractMember::StateVariableDefinition(state_variable)
                if matches!(
                    state_variable.attributes.visibility,
                    ir::StateVariableVisibility::Public
                ) =>
            {
                Some(Self::StateVariable(state_variable))
            }
            _ => None,
        }
    }

    /// The overridable view of a function definition, for callers that hold
    /// one directly rather than a contract member. `None` for a constructor,
    /// which takes no part in overriding.
    pub(crate) fn of_function(
        definition: &'a ir::FunctionDefinition,
        in_interface: bool,
    ) -> Option<Self> {
        match definition.kind {
            ir::FunctionKind::Regular | ir::FunctionKind::Fallback | ir::FunctionKind::Receive => {
                Some(Self::Function {
                    definition,
                    in_interface,
                })
            }
            ir::FunctionKind::Constructor => None,
        }
    }

    /// Whether this member and `other` occupy the same override slot. They
    /// match on name and kind (modifier, or function kind), and regular
    /// functions and getters also need indistinguishable parameter lists. The
    /// relation is symmetric, so which of the two overrides the other does not
    /// matter here.
    pub(crate) fn overrides(
        &self,
        binder: &'a Binder,
        types: &'a TypeRegistry,
        other: &Self,
    ) -> bool {
        if self.is_modifier() || other.is_modifier() {
            // Modifiers cannot be overloaded, so the name is enough.
            return self.is_modifier() && other.is_modifier() && self.name() == other.name();
        }
        if self.function_kind() != other.function_kind() || self.name() != other.name() {
            return false;
        }
        match self.function_kind() {
            // A fallback or receive has no selector, so name and kind are
            // enough.
            ir::FunctionKind::Fallback | ir::FunctionKind::Receive => true,
            // A member whose parameters aren't all typed is reported on its
            // own. Treating it as matching anything here would invent an
            // override.
            ir::FunctionKind::Regular => match (
                self.function_type(binder, types),
                other.function_type(binder, types),
            ) {
                (Some(self_type), Some(other_type)) => types.parameter_lists_are_indistinguishable(
                    &self_type.parameter_types,
                    &other_type.parameter_types,
                ),
                _ => false,
            },
            ir::FunctionKind::Constructor => {
                unreachable!("constructors take no part in overriding")
            }
        }
    }

    pub(crate) fn node_id(&self) -> NodeId {
        match self {
            Self::Function { definition, .. } => definition.id(),
            Self::Modifier(definition) => definition.id(),
            Self::StateVariable(state_variable) => state_variable.id(),
        }
    }

    /// The name, or `None` for a fallback or receive.
    pub(crate) fn name(&self) -> Option<&'a str> {
        match self {
            Self::Function { definition, .. } => {
                definition.name.as_ref().map(|name| name.unparse())
            }
            Self::Modifier(definition) => Some(definition.name.unparse()),
            Self::StateVariable(state_variable) => Some(state_variable.name.unparse()),
        }
    }

    pub(crate) fn kind(&self) -> OverridableKind {
        match self {
            Self::Function { .. } => OverridableKind::Function,
            Self::Modifier(_) => OverridableKind::Modifier,
            Self::StateVariable(_) => OverridableKind::PublicStateVariable,
        }
    }

    /// The function kind. A getter is a regular function.
    pub(crate) fn function_kind(&self) -> ir::FunctionKind {
        match self {
            Self::Function { definition, .. } => definition.kind,
            Self::Modifier(_) => unreachable!("modifiers have no function kind"),
            Self::StateVariable(_) => ir::FunctionKind::Regular,
        }
    }

    pub(crate) fn is_modifier(&self) -> bool {
        matches!(self, Self::Modifier(_))
    }

    pub(crate) fn is_function(&self) -> bool {
        matches!(self, Self::Function { .. })
    }

    pub(crate) fn is_interface_function(&self) -> bool {
        matches!(
            self,
            Self::Function {
                in_interface: true,
                ..
            }
        )
    }

    pub(crate) fn has_override_specifier(&self) -> bool {
        match self {
            Self::Function { definition, .. } => definition.attributes.override_specifier.is_some(),
            Self::Modifier(definition) => definition.attributes.override_specifier.is_some(),
            Self::StateVariable(state_variable) => {
                state_variable.attributes.override_specifier.is_some()
            }
        }
    }

    pub(crate) fn is_virtual(&self) -> bool {
        match self {
            Self::Function {
                definition,
                in_interface,
            } => definition.attributes.is_virtual || *in_interface,
            Self::Modifier(definition) => definition.attributes.is_virtual,
            Self::StateVariable(_) => false,
        }
    }

    pub(crate) fn visibility(&self) -> ir::FunctionVisibility {
        match self {
            Self::Function { definition, .. } => definition.attributes.visibility,
            Self::Modifier(_) => unreachable!("modifiers have no visibility"),
            Self::StateVariable(_) => ir::FunctionVisibility::External,
        }
    }

    pub(crate) fn mutability(&self) -> ir::FunctionMutability {
        match self {
            Self::Function { definition, .. } => definition.attributes.mutability,
            Self::Modifier(_) => unreachable!("modifiers have no state mutability"),
            Self::StateVariable(state_variable) => {
                if matches!(
                    state_variable.attributes.mutability,
                    ir::StateVariableMutability::Constant
                ) {
                    ir::FunctionMutability::Pure
                } else {
                    ir::FunctionMutability::View
                }
            }
        }
    }

    pub(crate) fn is_implemented(&self) -> bool {
        match self {
            Self::Function { definition, .. } => definition.body.is_some(),
            Self::Modifier(definition) => definition.body.is_some(),
            Self::StateVariable(_) => true,
        }
    }

    /// The declared function type, or the getter's type for a state variable.
    /// `None` when it couldn't be computed, which is reported elsewhere. A
    /// modifier has no type: see `modifier_parameter_types`.
    pub(crate) fn function_type(
        &self,
        binder: &'a Binder,
        types: &'a TypeRegistry,
    ) -> Option<&'a FunctionType> {
        let type_id = match self {
            Self::Function { definition, .. } => {
                binder.node_typing(definition.id()).as_type_id()?
            }
            Self::Modifier(_) => unreachable!("modifiers have no type"),
            Self::StateVariable(state_variable) => {
                match binder.find_definition_by_id(state_variable.id()) {
                    Some(Definition::StateVariable(definition)) => definition.getter_type_id?,
                    _ => unreachable!("state variable is not registered as a definition"),
                }
            }
        };
        let Type::Function(function_type) = types.get_type_by_id(type_id) else {
            unreachable!("type of a function or getter is not a function");
        };
        Some(function_type)
    }

    /// The parameter types of a modifier. `None` when one couldn't be
    /// computed, which is reported elsewhere.
    pub(crate) fn modifier_parameter_types(
        binder: &Binder,
        definition: &ir::ModifierDefinition,
    ) -> Option<Vec<TypeId>> {
        definition
            .parameters
            .iter()
            .map(|parameter| binder.node_typing(parameter.id()).as_type_id())
            .collect()
    }

    pub(crate) fn range(&self) -> Range<usize> {
        match self {
            Self::Function { definition, .. } => definition.signature_text_range(),
            Self::Modifier(definition) => definition.signature_text_range(),
            Self::StateVariable(state_variable) => state_variable.range.clone(),
        }
    }
}

/// Given a `Definition`'s `NodeId`, find the scope where we should resolve
/// symbols if the definition acts as a namespace. This is typically used when
/// resolving `MemberAccessExpression`s with a `UserMetatype` operand. Returns
/// `None` if the definition cannot be used as a namespace.
pub(crate) fn find_definition_namespace_scope_id(
    binder: &Binder,
    node_id: NodeId,
) -> Option<ScopeId> {
    match binder.find_definition_by_id(node_id)? {
        Definition::Import(import_definition) => import_definition
            .resolved_file_id
            .as_ref()
            .and_then(|file_id| binder.scope_id_for_file_id(file_id)),
        Definition::Contract(_)
        | Definition::Enum(_)
        | Definition::Interface(_)
        | Definition::Library(_) => {
            // this is a "namespace" lookup
            binder.scope_id_for_node_id(node_id)
        }
        _ => None,
    }
}
