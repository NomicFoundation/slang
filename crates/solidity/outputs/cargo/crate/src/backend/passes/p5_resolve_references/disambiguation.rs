use super::Pass;
use crate::backend::binder::{Definition, Typing};
use crate::backend::types::{FunctionType, Type, TypeId};
use crate::cst::NodeId;

/// Disambiguation functions that require typing (aka overload resolution)
impl Pass {
    pub(super) fn lookup_function_matching_positional_arguments<'a>(
        &'a self,
        type_ids: &[TypeId],
        argument_typings: &[Typing],
        receiver_type_id: Option<TypeId>,
    ) -> Option<&'a FunctionType> {
        // get types and filter non-function types
        let mut function_types = type_ids.iter().filter_map(|type_id| {
            if let Type::Function(function_type) = self.types.get_type_by_id(*type_id) {
                Some(function_type)
            } else {
                None
            }
        });
        function_types.find(|function_type| {
            let parameter_types = &function_type.parameter_types;
            if let Some(receiver_type_id) = receiver_type_id {
                // we have a receiver type, so either check for an implicit
                // receiver type, or the first parameter type
                // against it and then the rest, if the counts match
                if parameter_types.len() == argument_typings.len()
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
                    self.matches_positional_arguments(
                        parameter_types,
                        argument_typings,
                        function_type.external,
                    )
                } else if parameter_types.len() == argument_typings.len() + 1
                    && function_type.implicit_receiver_type.is_none()
                    && parameter_types.first().is_some_and(|type_id| {
                        self.types
                            .implicitly_convertible_to(receiver_type_id, *type_id)
                    })
                {
                    // matches attached functions (these can only be
                    // free-functions or library functions) with a compatible
                    // first argument
                    self.matches_positional_arguments(
                        &parameter_types[1..],
                        argument_typings,
                        function_type.external,
                    )
                } else {
                    false
                }
            } else if parameter_types.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                self.matches_positional_arguments(
                    parameter_types,
                    argument_typings,
                    function_type.external,
                )
            } else {
                false
            }
        })
    }

    fn matches_positional_arguments(
        &self,
        parameter_types: &[TypeId],
        argument_typings: &[Typing],
        external_call: bool,
    ) -> bool {
        parameter_types
            .iter()
            .zip(argument_typings)
            .all(|(parameter_type, argument_typing)| {
                self.parameter_type_matches_argument_typing(
                    *parameter_type,
                    argument_typing,
                    external_call,
                )
            })
    }

    fn parameter_type_matches_argument_typing(
        &self,
        parameter_type: TypeId,
        argument_typing: &Typing,
        external_call: bool,
    ) -> bool {
        match argument_typing {
            Typing::Resolved(type_id) => {
                if external_call {
                    self.types
                        .implicitly_convertible_to_for_external_call(*type_id, parameter_type)
                } else {
                    self.types
                        .implicitly_convertible_to(*type_id, parameter_type)
                }
            }
            Typing::This => self
                .types
                .implicitly_convertible_to(self.types.address(), parameter_type),
            _ => false,
        }
    }

    pub(super) fn lookup_function_matching_named_arguments<'a>(
        &'a self,
        type_ids: &[TypeId],
        argument_typings: &[(String, Typing)],
        receiver_type_id: Option<TypeId>,
    ) -> Option<&'a FunctionType> {
        // get types and filter non-function types
        let mut function_types = type_ids.iter().filter_map(|type_id| {
            if let Type::Function(function_type) = self.types.get_type_by_id(*type_id) {
                Some(function_type)
            } else {
                None
            }
        });
        function_types.find(|function_type| {
            let Some(definition_id) = function_type.definition_id else {
                return false;
            };
            let definition = self.binder.find_definition_by_id(definition_id).unwrap();
            let Definition::Function(function_definition) = definition else {
                unreachable!("the definition associated to a function type is not a function");
            };
            let parameter_types = &function_type.parameter_types;
            let parameter_names = &function_definition.parameter_names;
            assert_eq!(
                parameter_types.len(),
                parameter_names.len(),
                "length of types and names of parameters should match"
            );
            if parameter_names.iter().any(|name| name.is_none()) {
                // function has an unnamed parameter and we cannot use it for
                // matching named arguments
                return false;
            }

            if parameter_types.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                self.matches_named_arguments(
                    parameter_names,
                    parameter_types,
                    argument_typings,
                    function_type.external,
                )
            } else if let Some(receiver_type_id) = receiver_type_id {
                // we have a receiver type, so check the first parameter type
                // against it and then the rest, if the counts match
                if parameter_types.len() == argument_typings.len() + 1
                    && parameter_types.first().is_some_and(|type_id| {
                        self.types
                            .implicitly_convertible_to(receiver_type_id, *type_id)
                    })
                {
                    self.matches_named_arguments(
                        &parameter_names[1..],
                        &parameter_types[1..],
                        argument_typings,
                        function_type.external,
                    )
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    fn matches_named_arguments(
        &self,
        parameter_names: &[Option<String>],
        parameter_types: &[TypeId],
        argument_typings: &[(String, Typing)],
        external_call: bool,
    ) -> bool {
        argument_typings
            .iter()
            .all(|(argument_name, argument_typing)| {
                let Some(index) = parameter_names
                    .iter()
                    .position(|name| name.as_ref().is_some_and(|name| name == argument_name))
                else {
                    return false;
                };
                let parameter_type = parameter_types[index];
                self.parameter_type_matches_argument_typing(
                    parameter_type,
                    argument_typing,
                    external_call,
                )
            })
    }

    pub(super) fn lookup_event_matching_positional_arguments(
        &self,
        definition_ids: &[NodeId],
        argument_typings: &[Typing],
    ) -> Option<NodeId> {
        for definition_id in definition_ids {
            let Some(Definition::Event(event_definition)) =
                self.binder.find_definition_by_id(*definition_id)
            else {
                continue;
            };

            let parameter_types = &event_definition.parameter_types;
            if parameter_types.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                if self.matches_positional_arguments(parameter_types, argument_typings, false) {
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
            let Some(Definition::Event(event_definition)) =
                self.binder.find_definition_by_id(*definition_id)
            else {
                continue;
            };

            let parameter_types = &event_definition.parameter_types;
            let parameter_names = &event_definition.parameter_names;
            assert_eq!(
                parameter_types.len(),
                parameter_names.len(),
                "length of types and names of parameters should match"
            );
            if parameter_names.iter().any(|name| name.is_none()) {
                // cannot match if any parameter is unnamed
                continue;
            }

            if parameter_types.len() == argument_typings.len() {
                // argument count matches, check that all types are implicitly convertible
                if self.matches_named_arguments(
                    parameter_names,
                    parameter_types,
                    argument_typings,
                    false,
                ) {
                    return Some(*definition_id);
                }
            }
        }
        None
    }
}
