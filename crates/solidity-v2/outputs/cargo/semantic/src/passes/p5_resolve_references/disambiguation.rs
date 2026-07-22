use slang_solidity_v2_common::nodes::NodeId;

use super::Pass;
use crate::binder::{Definition, ParameterDefinition, Scope, Typing};
use crate::types::{FunctionType, Type, TypeId, UserMetaType};

/// Disambiguation functions that require typing (aka overload resolution)
impl Pass<'_> {
    fn get_function_definition_parameters(
        &self,
        definition_id: Option<NodeId>,
    ) -> Option<&[ParameterDefinition]> {
        let Some(Definition::Function(function_definition)) =
            self.binder.find_definition_by_id(definition_id?)
        else {
            return None;
        };

        let Scope::Parameters(parameters_scope) = self
            .binder
            .get_scope_by_id(function_definition.parameters_scope_id)
        else {
            unreachable!("incorrect scope kind, expected parameters");
        };
        Some(&parameters_scope.parameters)
    }

    /// Fetch the `FunctionType` defined by `type_id`. It can be a direct
    /// function type `Type::Function` or a function declaration through a
    /// member access of a contract/interface, ie. `C.foo`, which is
    /// represented via `Type::UserMetaType`.
    fn candidate_function_type(&self, type_id: TypeId) -> Option<&FunctionType> {
        match self.types.get_type_by_id(type_id) {
            Type::Function(function_type) => Some(function_type),
            Type::UserMetaType(UserMetaType { definition_id }) => {
                if !matches!(
                    self.binder.find_definition_by_id(*definition_id),
                    Some(Definition::Function(_))
                ) {
                    return None;
                }
                let function_type_id = self.binder.node_typing(*definition_id).as_type_id()?;
                if let Type::Function(function_type) = self.types.get_type_by_id(function_type_id) {
                    Some(function_type)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns the definition an overload candidate refers to, both for
    /// function values and for function declarations reached through a
    /// contract/interface type name. Expects a candidate already selected by
    /// one of the `lookup_function_matching_*` functions, so a user meta type
    /// can only be a function's.
    pub(super) fn candidate_definition_id(&self, type_id: TypeId) -> Option<NodeId> {
        match self.types.get_type_by_id(type_id) {
            Type::Function(FunctionType { definition_id, .. }) => *definition_id,
            Type::UserMetaType(UserMetaType { definition_id }) => {
                let is_function = matches!(
                    self.binder.find_definition_by_id(*definition_id),
                    Some(Definition::Function(_))
                );
                debug_assert!(
                    is_function,
                    "user meta type overload candidates must refer to a function definition"
                );
                is_function.then_some(*definition_id)
            }
            _ => None,
        }
    }

    /// Finds the first overload candidate in `type_ids` whose parameter list
    /// is compatible with the call's arguments. The receiver/arity handling is
    /// shared between call styles; `parameters_match` checks the (possibly
    /// receiver-adjusted) parameter list against the actual arguments, given
    /// whether the call crosses an external boundary.
    // TODO(validation) SDR[1108]: solc requires the compatible overload to be *unique*;
    // this selects the first compatible candidate instead of erroring on
    // ambiguous calls.
    fn lookup_function_matching_arguments(
        &self,
        type_ids: &[TypeId],
        argument_count: usize,
        receiver_type_id: Option<TypeId>,
        parameters_match: impl Fn(&[ParameterDefinition], bool) -> bool,
    ) -> Option<TypeId> {
        type_ids.iter().copied().find(|type_id| {
            let Some(function_type) = self.candidate_function_type(*type_id) else {
                return false;
            };
            let Some(parameters) =
                self.get_function_definition_parameters(function_type.definition_id)
            else {
                return false;
            };
            let external_call = function_type.is_externally_visible();
            if let Some(receiver_type_id) = receiver_type_id {
                // we have a receiver type, so either check for an implicit
                // receiver type, or the first parameter type
                // against it and then the rest, if the counts match
                if parameters.len() == argument_count
                    && function_type.implicit_receiver_type.is_some_and(
                        |implicit_receiver_type_id| {
                            self.types.implicitly_convertible_to(
                                receiver_type_id,
                                implicit_receiver_type_id,
                            )
                        },
                    )
                {
                    // matches "method" functions of a compatible receiver type
                    parameters_match(parameters, external_call)
                } else if parameters.len() == argument_count + 1
                    && function_type.implicit_receiver_type.is_none()
                    && parameters.first().is_some_and(|parameter| {
                        parameter.type_id.is_some_and(|type_id| {
                            self.types
                                .implicitly_convertible_to(receiver_type_id, type_id)
                        })
                    })
                {
                    // matches attached functions (these can only be
                    // free-functions or library functions) with a compatible
                    // first argument
                    parameters_match(&parameters[1..], external_call)
                } else {
                    false
                }
            } else if parameters.len() == argument_count {
                // argument count matches, check that all argument types are compatible
                parameters_match(parameters, external_call)
            } else {
                false
            }
        })
    }

    pub(super) fn lookup_function_matching_positional_arguments(
        &self,
        type_ids: &[TypeId],
        argument_typings: &[Typing],
        receiver_type_id: Option<TypeId>,
    ) -> Option<TypeId> {
        self.lookup_function_matching_arguments(
            type_ids,
            argument_typings.len(),
            receiver_type_id,
            |parameters, external_call| {
                self.parameters_match_positional_arguments(
                    parameters,
                    argument_typings,
                    external_call,
                )
            },
        )
    }

    fn parameters_match_positional_arguments(
        &self,
        parameters: &[ParameterDefinition],
        argument_typings: &[Typing],
        external_call: bool,
    ) -> bool {
        parameters
            .iter()
            .zip(argument_typings)
            .all(|(parameter, argument_typing)| {
                parameter.type_id.is_some_and(|type_id| {
                    self.parameter_type_matches_argument_typing(
                        type_id,
                        argument_typing,
                        external_call,
                    )
                })
            })
    }

    fn parameter_type_matches_argument_typing(
        &self,
        parameter_type: TypeId,
        argument_typing: &Typing,
        external_call: bool,
    ) -> bool {
        let Some(type_id) = argument_typing.as_type_id() else {
            return false;
        };
        if external_call {
            self.types
                .implicitly_convertible_to_for_external_call(type_id, parameter_type)
        } else {
            self.types
                .implicitly_convertible_to(type_id, parameter_type)
        }
    }

    pub(super) fn lookup_function_matching_named_arguments(
        &self,
        type_ids: &[TypeId],
        argument_typings: &[(String, Typing)],
        receiver_type_id: Option<TypeId>,
    ) -> Option<TypeId> {
        self.lookup_function_matching_arguments(
            type_ids,
            argument_typings.len(),
            receiver_type_id,
            |parameters, external_call| {
                if parameters.iter().any(|parameter| parameter.name.is_none()) {
                    // function has an unnamed parameter and we cannot use it
                    // for matching named arguments
                    return false;
                }
                self.parameters_match_named_arguments(parameters, argument_typings, external_call)
            },
        )
    }

    fn parameters_match_named_arguments(
        &self,
        parameters: &[ParameterDefinition],
        argument_typings: &[(String, Typing)],
        external_call: bool,
    ) -> bool {
        argument_typings
            .iter()
            .all(|(argument_name, argument_typing)| {
                let Some(parameter) = parameters.iter().find(|parameter| {
                    parameter
                        .name
                        .as_ref()
                        .is_some_and(|name| name == argument_name)
                }) else {
                    return false;
                };
                parameter.type_id.is_some_and(|type_id| {
                    self.parameter_type_matches_argument_typing(
                        type_id,
                        argument_typing,
                        external_call,
                    )
                })
            })
    }

    fn get_event_definition_parameters(
        &self,
        definition_id: NodeId,
    ) -> Option<&[ParameterDefinition]> {
        let Some(Definition::Event(event_definition)) =
            self.binder.find_definition_by_id(definition_id)
        else {
            return None;
        };

        let Scope::Parameters(parameters_scope) = self
            .binder
            .get_scope_by_id(event_definition.parameters_scope_id)
        else {
            unreachable!("incorrect scope kind, expected parameters");
        };
        Some(&parameters_scope.parameters)
    }

    pub(super) fn lookup_event_matching_positional_arguments(
        &self,
        definition_ids: &[NodeId],
        argument_typings: &[Typing],
    ) -> Option<NodeId> {
        for definition_id in definition_ids {
            let Some(parameters) = self.get_event_definition_parameters(*definition_id) else {
                continue;
            };
            if parameters.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                if self.parameters_match_positional_arguments(parameters, argument_typings, false) {
                    return Some(*definition_id);
                }
            }
        }
        None
    }

    pub(super) fn lookup_event_matching_named_arguments(
        &self,
        definition_ids: &[NodeId],
        argument_typings: &[(String, Typing)],
    ) -> Option<NodeId> {
        for definition_id in definition_ids {
            let Some(parameters) = self.get_event_definition_parameters(*definition_id) else {
                continue;
            };

            if parameters.iter().any(|parameter| parameter.name.is_none()) {
                // cannot match if any parameter is unnamed
                continue;
            }

            if parameters.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                if self.parameters_match_named_arguments(parameters, argument_typings, false) {
                    return Some(*definition_id);
                }
            }
        }
        None
    }
}
