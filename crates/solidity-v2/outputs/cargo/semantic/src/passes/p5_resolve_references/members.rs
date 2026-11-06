use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::{Definition, DefinitionIds, Resolution, ResolveOptions, Typing};
use crate::built_ins::InternalBuiltIn;
use crate::passes::common::{
    filter_overridden_definitions, filter_overridden_definitions_by_selector,
    find_definition_namespace_scope_id,
};
use crate::types::{
    CallMode, ContractType, FunctionType, InterfaceType, StructType, Type, TypeId, UserMetaType,
};

/// What the operand of a member access denotes. It decides which members the
/// access can name, and how a member named through it is typed.
#[derive(Clone, Copy)]
pub(super) enum MemberContext {
    /// Nothing to look members up in: an operand that did not resolve, or a
    /// `new` expression, which has no members from 0.8.0.
    None,
    /// A built-in, like `abi` or `msg`.
    BuiltIn(InternalBuiltIn),
    /// `super`, written in `contract_id`: the implemented functions of the
    /// bases after it, as they are declared.
    Super { contract_id: NodeId },
    /// `this` in a contract, or any other value of a contract or interface
    /// type: its externally visible members, typed as external functions and
    /// getters.
    ContractValue { type_id: TypeId, is_this: bool },
    /// `this` in a library, which reaches its externally visible functions
    /// but calls none of them externally.
    LibraryThis { type_id: TypeId },
    /// The name of a type or of an import, used as a value: the members it
    /// declares itself. `is_foreign` when it names a contract or interface
    /// outside the hierarchy of the code naming it, which reaches fewer of
    /// them.
    TypeName {
        type_id: TypeId,
        definition_id: NodeId,
        is_foreign: bool,
    },
    /// A value of any other type: the members of a struct, and the functions
    /// attached to the type.
    Value { type_id: TypeId },
}

impl MemberContext {
    /// The type of the operand, if it has one.
    #[inline]
    fn operand_type_id(self) -> Option<TypeId> {
        match self {
            Self::ContractValue { type_id, .. }
            | Self::TypeName { type_id, .. }
            | Self::LibraryThis { type_id }
            | Self::Value { type_id } => Some(type_id),
            Self::None | Self::BuiltIn(_) | Self::Super { .. } => None,
        }
    }
}

impl Pass<'_> {
    /// The context a member access whose operand types as `operand_typing`
    /// resolves its member in. An overload set operand is reported and sunk
    /// before getting here.
    #[inline]
    pub(super) fn member_context(&self, operand_typing: &Typing) -> MemberContext {
        match operand_typing {
            Typing::Unresolved | Typing::Undetermined(_) | Typing::NewExpression(_) => {
                MemberContext::None
            }
            Typing::BuiltIn(built_in) => MemberContext::BuiltIn(*built_in),
            Typing::Super(contract_id) => MemberContext::Super {
                contract_id: *contract_id,
            },
            Typing::This(type_id) => match self.types.get_type_by_id(*type_id) {
                Type::Contract(_) | Type::Interface(_) => MemberContext::ContractValue {
                    type_id: *type_id,
                    is_this: true,
                },
                _ => MemberContext::LibraryThis { type_id: *type_id },
            },
            Typing::Resolved(type_id) => match self.types.get_type_by_id(*type_id) {
                Type::Contract(_) | Type::Interface(_) => MemberContext::ContractValue {
                    type_id: *type_id,
                    is_this: false,
                },
                Type::UserMetaType(UserMetaType { definition_id }) => MemberContext::TypeName {
                    type_id: *type_id,
                    definition_id: *definition_id,
                    is_foreign: self.is_foreign_contract_or_interface(*definition_id),
                },
                _ => MemberContext::Value { type_id: *type_id },
            },
        }
    }

    /// Whether `operand_typing` reaches members as a contract value, ie. is
    /// `this` in a contract or a value of a contract or interface type. This
    /// is what makes `Self::member_context` a `MemberContext::ContractValue`,
    /// without classifying any other operand.
    #[inline]
    pub(super) fn is_contract_value(&self, operand_typing: &Typing) -> bool {
        match operand_typing {
            Typing::Resolved(type_id) | Typing::This(type_id) => matches!(
                self.types.get_type_by_id(*type_id),
                Type::Contract(_) | Type::Interface(_)
            ),
            _ => false,
        }
    }

    /// Resolves `symbol` as a member reached in `context`. A function
    /// overridden by another candidate is dropped: `super` compares the
    /// functions as declared, and every other context by selector.
    pub(super) fn resolve_member(&mut self, context: MemberContext, symbol: &str) -> Resolution {
        let resolution = self.lookup_member(context, symbol);
        if matches!(context, MemberContext::Super { .. }) {
            filter_overridden_definitions(self.binder, self.types, resolution)
        } else {
            filter_overridden_definitions_by_selector(self.binder, self.types, resolution)
        }
    }

    fn lookup_member(&mut self, context: MemberContext, symbol: &str) -> Resolution {
        match context {
            MemberContext::None => Resolution::Unresolved,
            MemberContext::BuiltIn(built_in) => self
                .built_ins_resolver()
                .lookup_member_of(&built_in, symbol)
                .into(),
            // This resolves `super` against the linearisation of the contract
            // it is written in; which function runs depends on the contract
            // being compiled, see `SemanticContext::resolve_super`.
            MemberContext::Super { contract_id } => {
                let scope_id = self
                    .binder
                    .scope_id_for_node_id(contract_id)
                    .expect("a contract has a scope");
                self.binder.resolve_in_contract_scope(
                    scope_id,
                    symbol,
                    ResolveOptions::Super(contract_id),
                )
            }
            MemberContext::ContractValue {
                type_id,
                is_this: true,
            } => {
                let mut definition_ids = self.lookup_member_of_this(symbol).get_definition_ids();
                self.add_attached_functions_for_type(type_id, symbol, &mut definition_ids);
                Resolution::from(definition_ids)
            }
            MemberContext::LibraryThis { .. } => self.lookup_member_of_this(symbol),
            MemberContext::ContractValue {
                type_id,
                is_this: false,
            } => {
                let (Type::Contract(ContractType { definition_id })
                | Type::Interface(InterfaceType { definition_id })) =
                    self.types.get_type_by_id(type_id)
                else {
                    unreachable!("a contract value has a contract or interface type");
                };
                let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                let own_members = self.binder.resolve_in_contract_scope(
                    scope_id,
                    symbol,
                    ResolveOptions::External,
                );
                self.with_attached_functions(type_id, symbol, &own_members)
            }
            MemberContext::TypeName {
                type_id,
                definition_id,
                is_foreign,
            } => {
                let resolution = find_definition_namespace_scope_id(self.binder, definition_id)
                    .map_or(Resolution::Unresolved, |scope_id| {
                        self.binder
                            .resolve_in_scope_as_expression_member(scope_id, symbol)
                    });
                let own_members = if is_foreign {
                    let visible: DefinitionIds = resolution
                        .get_definition_ids()
                        .into_iter()
                        .filter(|id| self.is_reachable_from_outside_hierarchy(*id))
                        .collect();
                    Resolution::from(visible)
                } else {
                    resolution
                };
                self.with_attached_functions(type_id, symbol, &own_members)
            }
            MemberContext::Value { type_id } => {
                let own_members = if let Type::Struct(StructType { definition_id, .. }) =
                    self.types.get_type_by_id(type_id)
                {
                    let scope_id = self.binder.scope_id_for_node_id(*definition_id).unwrap();
                    self.binder.resolve_in_scope_as_namespace(scope_id, symbol)
                } else {
                    Resolution::Unresolved
                };
                self.with_attached_functions(type_id, symbol, &own_members)
            }
        }
    }

    /// The externally visible members of the contract or library `this`
    /// names, starting at it in the linearisation.
    // TODO(validation) SDR[34]: for `this` resolutions we need to check that
    // the returned definitions are externally available (ie. either
    // `external` or `public`)
    fn lookup_member_of_this(&self, symbol: &str) -> Resolution {
        let Some(scope_id) = self.current_contract_scope_id() else {
            return Resolution::Unresolved;
        };
        let node_id = self.binder.get_scope_by_id(scope_id).node_id();
        self.binder
            .resolve_in_contract_scope(scope_id, symbol, ResolveOptions::This(node_id))
    }

    /// Adds to `own_members` the functions attached to `type_id` by the
    /// active `using` directives, falling back to a built-in member of the
    /// type when there are none.
    #[inline]
    fn with_attached_functions(
        &mut self,
        type_id: TypeId,
        symbol: &str,
        own_members: &Resolution,
    ) -> Resolution {
        let current_contract_id = self.current_contract_node_id();
        let mut definition_ids = own_members.get_definition_ids();
        self.add_attached_functions_for_type(type_id, symbol, &mut definition_ids);
        Resolution::from(definition_ids).or_else(|| {
            self.built_ins_resolver()
                .lookup_member_of_type_id(type_id, symbol, current_contract_id)
                .into()
        })
    }

    /// Types a member reached in `context`, which resolves to `resolution`.
    pub(super) fn typing_of_member(
        &mut self,
        context: MemberContext,
        resolution: &Resolution,
    ) -> Typing {
        let typing = if matches!(context, MemberContext::ContractValue { .. }) {
            self.typing_of_resolution_as_contract_member(resolution)
        } else {
            self.typing_of_resolution(resolution)
        };
        // See `adjust_member_type_for_context`.
        match typing {
            Typing::Resolved(type_id) => {
                Typing::Resolved(self.adjust_member_type_for_context(type_id, context))
            }
            Typing::This(type_id) => {
                Typing::This(self.adjust_member_type_for_context(type_id, context))
            }
            Typing::Undetermined(type_ids) => Typing::Undetermined(
                type_ids
                    .into_iter()
                    .map(|type_id| self.adjust_member_type_for_context(type_id, context))
                    .collect(),
            ),
            Typing::Unresolved
            | Typing::BuiltIn(_)
            | Typing::NewExpression(_)
            | Typing::Super(_) => typing,
        }
    }

    fn typing_of_resolution_as_contract_member(&mut self, resolution: &Resolution) -> Typing {
        match resolution {
            Resolution::Definition(definition_id) => self.typing_of_contract_member(*definition_id),
            // Each overload is accessed externally, as a single member is.
            Resolution::Ambiguous(definition_ids) => Typing::Undetermined(
                definition_ids
                    .iter()
                    .filter_map(|definition_id| {
                        self.typing_of_contract_member(*definition_id).as_type_id()
                    })
                    .collect(),
            ),
            Resolution::Unresolved | Resolution::BuiltIn(_) => {
                self.typing_of_resolution(resolution)
            }
        }
    }

    /// The typing of the member `definition_id` reached through `this` or a
    /// contract reference, which accesses it externally.
    fn typing_of_contract_member(&mut self, definition_id: NodeId) -> Typing {
        // Check if the target is a state variable with a getter or a function
        // with an externalized type; the member is accessed through that type.
        if let Some(member_type_id) =
            match self.binder.find_definition_by_id(definition_id).unwrap() {
                Definition::StateVariable(state_var_definition) => {
                    state_var_definition.getter_type_id()
                }
                Definition::Function(function_definition) => {
                    function_definition.externalized_type_id
                }
                _ => None,
            }
        {
            return Typing::Resolved(member_type_id);
        }

        let typing = self.binder.node_typing(definition_id).clone();

        // If the resolved type is a function and the operand is either
        // `this` or something of an address type, the function is being
        // used as an external function: change the expression typing to
        // indicate the external access.
        if let Some(type_id) = typing.as_type_id()
            && let Type::Function(function_type) = self.types.get_type_by_id(type_id)
            && self.is_externally_visible_function(function_type)
        {
            return Typing::Resolved(self.types.externalize_function_type(type_id));
        }

        typing
    }

    /// Whether a function value refers to an externally visible function: it
    /// is called externally, or it names a function declared `public` or
    /// `external` (or a public state variable's getter).
    fn is_externally_visible_function(&self, function_type: &FunctionType) -> bool {
        function_type.call_mode.is_external()
            || function_type
                .definition_id
                .and_then(|definition_id| self.binder.find_definition_by_id(definition_id))
                .is_some_and(|definition| {
                    matches!(
                        definition,
                        Definition::Function(_) | Definition::StateVariable(_)
                    ) && definition.is_externally_visible()
                })
    }

    /// Adjusts the type of a member reached in `context`:
    /// - reference types with an "inherited" data location take the operand's
    ///   location;
    /// - functions attached via `using for` bind the receiver as their first
    ///   argument, producing a partially applied function;
    /// - `public` and `external` library functions, reached through the library
    ///   or bound, are called by a delegate call;
    /// - function declarations that are not callable through the operand's
    ///   contract/interface type name become the user meta type of their
    ///   definition.
    fn adjust_member_type_for_context(
        &mut self,
        type_id: TypeId,
        context: MemberContext,
    ) -> TypeId {
        let Some(receiver_type_id) = context.operand_type_id() else {
            return type_id;
        };
        let type_ = self.types.get_type_by_id(type_id);

        if type_.is_inherited_location() {
            if let Some(operand_location) =
                self.types.get_type_by_id(receiver_type_id).data_location()
            {
                let type_ = type_.clone();
                return self
                    .types
                    .register_type_with_data_location(type_, operand_location);
            }
        } else if let Type::Function(function_type) = type_ {
            let is_delegate_called = function_type.definition_id.is_some_and(|definition_id| {
                self.is_delegate_called_library_function(definition_id)
            });

            // Only a function declaration can be attached to the receiver: a
            // function-typed value, eg. a struct field, keeps its call mode.
            if function_type.definition_id.is_some()
                && function_type.implicit_receiver_type.is_none()
                && function_type.parameter_types.first().is_some_and(|first| {
                    self.types.implicitly_convertible_to_for_call(
                        receiver_type_id,
                        *first,
                        is_delegate_called,
                    )
                })
            {
                let call_mode = if is_delegate_called {
                    CallMode::Delegate { bound: true }
                } else {
                    CallMode::Internal { bound: true }
                };
                return self
                    .types
                    .function_type_with_call_mode(function_type.clone(), call_mode);
            }

            if is_delegate_called && self.is_library_type_name(receiver_type_id) {
                return self.types.function_type_with_call_mode(
                    function_type.clone(),
                    CallMode::Delegate { bound: false },
                );
            }

            if let MemberContext::TypeName {
                definition_id,
                is_foreign,
                ..
            } = context
                && let Some(declaration_type_id) = self.as_foreign_function_declaration_type(
                    definition_id,
                    is_foreign,
                    function_type.definition_id,
                )
            {
                return declaration_type_id;
            }
        }
        type_id
    }

    /// Whether `type_id` is the type of a library's name, eg. `L` in `L.f`.
    fn is_library_type_name(&self, type_id: TypeId) -> bool {
        matches!(
            self.types.get_type_by_id(type_id),
            Type::UserMetaType(UserMetaType { definition_id })
                if matches!(
                    self.binder.find_definition_by_id(*definition_id),
                    Some(Definition::Library(_))
                )
        )
    }

    /// Returns the user meta type of a function's definition when reaching the
    /// function through a contract/interface *type name* (eg. `C.g`) makes it
    /// a non-callable declaration with no mobile type: external functions
    /// always, and public functions of a *foreign* contract. Internal/private
    /// functions reached the same way stay normal callables (qualified base
    /// calls), as do members of library type names (`L.f`).
    fn as_foreign_function_declaration_type(
        &mut self,
        type_name_id: NodeId,
        is_foreign: bool,
        function_definition_id: Option<NodeId>,
    ) -> Option<TypeId> {
        if !matches!(
            self.binder.find_definition_by_id(type_name_id),
            Some(Definition::Contract(_) | Definition::Interface(_))
        ) {
            return None;
        }

        let definition_id = function_definition_id?;
        let Some(Definition::Function(function_definition)) =
            self.binder.find_definition_by_id(definition_id)
        else {
            return None;
        };
        let is_foreign_and_visible = match function_definition.ir_node.attributes.visibility {
            ir::FunctionVisibility::External => true,
            ir::FunctionVisibility::Public => is_foreign,
            ir::FunctionVisibility::Internal | ir::FunctionVisibility::Private => false,
        };
        if !is_foreign_and_visible {
            return None;
        }

        Some(
            self.types
                .register_type(Type::UserMetaType(UserMetaType { definition_id })),
        )
    }

    /// Whether `definition_id` is a contract or interface named from outside
    /// its hierarchy.
    pub(super) fn is_foreign_contract_or_interface(&self, definition_id: NodeId) -> bool {
        matches!(
            self.binder.find_definition_by_id(definition_id),
            Some(Definition::Contract(_) | Definition::Interface(_))
        ) && self.is_foreign_contract(definition_id)
    }

    /// Whether a member of a contract or interface can be reached through its
    /// name from outside the contract's hierarchy: types, errors and
    /// externally visible functions can, events only from 0.8.21, and
    /// variables, modifiers and internal functions never.
    pub(super) fn is_reachable_from_outside_hierarchy(&self, definition_id: NodeId) -> bool {
        match self.binder.find_definition_by_id(definition_id) {
            Some(definition @ Definition::Function(_)) => definition.is_externally_visible(),
            Some(Definition::Event(_)) => self.types.language_version() >= LanguageVersion::V0_8_21,
            Some(
                Definition::StateVariable(_) | Definition::Constant(_) | Definition::Modifier(_),
            ) => false,
            _ => true,
        }
    }
}
