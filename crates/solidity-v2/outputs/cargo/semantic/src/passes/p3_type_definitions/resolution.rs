use num_traits::{Signed, ToPrimitive};
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    ArrayLengthFractional, ArrayLengthNegative, ArrayLengthNotConstant, ArrayLengthTooLarge,
    ArrayLengthZero, InvalidFunctionTypeVisibility,
};
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::{NodeIdentity, TextRange};

use super::Pass;
use crate::binder::{Definition, Resolution, ScopeId};
use crate::passes::common::constant_evaluator::{
    evaluate_compile_time_constant, ConstantResolver, EvaluationError,
};
use crate::passes::common::{node_location, resolve_identifier_path_in_scope};
use crate::types::{
    ArrayType, DataLocation, FixedSizeArrayType, FunctionType, MappingType, TupleType, Type, TypeId,
};

impl Pass<'_> {
    // Resolves an IdentifierPath starting at the "contract" scope level, or at
    // the file level if there's no contract scope open.
    // Returns the resolution of the last reference.
    pub(super) fn resolve_identifier_path(
        &mut self,
        identifier_path: &ir::IdentifierPath,
    ) -> Resolution {
        // start resolution from the current contract (or file if there's no
        // contract scope open)
        let starting_scope_id = self.current_contract_or_file_scope_id();
        resolve_identifier_path_in_scope(self.binder, identifier_path, starting_scope_id)
    }

    pub(super) fn resolve_parameter_types(
        &mut self,
        parameters: &ir::Parameters,
    ) -> Option<Vec<TypeId>> {
        let mut parameter_types = Vec::new();
        for parameter in parameters {
            let parameter_type_id = self.binder.node_typing(parameter.id()).as_type_id()?;
            parameter_types.push(parameter_type_id);
        }
        Some(parameter_types)
    }

    /// Resolves a fixed-size array length expression, and emits diagnostics
    /// if errors are found. On error, returns `usize::default()` so type
    /// construction can proceed.
    /// TODO: Change return to u256, when `FixedSizeArrayType` size field
    /// is changed to u256.
    fn resolve_array_length(&mut self, size_expression: &ir::Expression) -> usize {
        let (file_id, range) = node_location(size_expression, self.file_node_mapper);
        let value = match evaluate_compile_time_constant(
            size_expression,
            self.current_contract_or_file_scope_id(),
            self.types,
            &ConstantResolver {
                binder: self.binder,
                use_site: Some((&file_id, range.start)),
            },
        ) {
            Ok(value) => value,
            Err(EvaluationError::Diagnostic { kind, expression }) => {
                // Report at the failing operation, falling back to the length
                // expression when the evaluator didn't attach one.
                self.push_diagnostic(expression.as_ref().unwrap_or(size_expression), kind);
                return usize::default();
            }
            Err(EvaluationError::CouldNotEvaluate) => {
                self.push_diagnostic(size_expression, ArrayLengthNotConstant);
                return usize::default();
            }
        };

        // Classify the value: zero, then fractional, then negative and then too large.
        if value.is_zero() {
            self.push_diagnostic(size_expression, ArrayLengthZero);
            return usize::default();
        }
        let Some(integer) = value.as_integer() else {
            self.push_diagnostic(size_expression, ArrayLengthFractional);
            return usize::default();
        };
        if integer.is_negative() {
            self.push_diagnostic(size_expression, ArrayLengthNegative);
            return usize::default();
        }
        // TODO: Accept u256, when `FixedSizeArrayType` size field is changed to u256.
        let Some(size) = integer.to_usize() else {
            self.push_diagnostic(size_expression, ArrayLengthTooLarge);
            return usize::default();
        };
        size
    }

    /// Emits `kind` located at `node`.
    fn push_diagnostic(
        &mut self,
        node: &(impl NodeIdentity + TextRange),
        kind: impl Into<DiagnosticKind>,
    ) {
        let (file_id, range) = node_location(node, self.file_node_mapper);
        self.diagnostics.push(file_id, range, kind);
    }

    pub(super) fn resolve_type_name(
        &mut self,
        type_name: &ir::TypeName,
        data_location: Option<DataLocation>,
    ) -> Option<TypeId> {
        match type_name {
            ir::TypeName::ArrayTypeName(array_type_name) => {
                data_location.and_then(|data_location| {
                    self.resolve_type_name(&array_type_name.operand, Some(data_location))
                        .map(|element_type| {
                            if let Some(size_expression) = &array_type_name.index {
                                let size = self.resolve_array_length(size_expression);
                                self.types
                                    .register_type(Type::FixedSizeArray(FixedSizeArrayType {
                                        element_type,
                                        size,
                                        location: data_location,
                                    }))
                            } else {
                                self.types.register_type(Type::Array(ArrayType {
                                    element_type,
                                    location: data_location,
                                }))
                            }
                        })
                })
            }
            ir::TypeName::FunctionType(function_type) => {
                // Function types can only be internal or external, so emit a
                // diagnostic if the visibility is not one of those.
                if !matches!(
                    function_type.visibility,
                    ir::FunctionVisibility::Internal | ir::FunctionVisibility::External
                ) {
                    self.push_diagnostic(function_type, InvalidFunctionTypeVisibility);
                }

                // NOTE: Keep in sync with `type_of_function_definition`
                let parameter_types = self.resolve_parameter_types(&function_type.parameters)?;
                let return_type = if let Some(returns) = &function_type.returns {
                    let return_types = self.resolve_parameter_types(returns)?;
                    if return_types.len() == 1 {
                        return_types.first().copied().unwrap()
                    } else {
                        self.types.register_type(Type::Tuple(TupleType {
                            types: return_types,
                        }))
                    }
                } else {
                    self.types.void()
                };
                Some(self.types.register_type(Type::Function(FunctionType {
                    definition_id: None,
                    implicit_receiver_type: None,
                    parameter_types,
                    return_type,
                    visibility: (&function_type.visibility).into(),
                    mutability: (&function_type.mutability).into(),
                    partially_applied: false,
                })))
            }
            ir::TypeName::MappingType(mapping_type) => {
                let data_location = Some(DataLocation::Storage);
                let key_type_id =
                    self.resolve_type_name(&mapping_type.key_type.type_name, data_location)?;
                let value_type_id =
                    self.resolve_type_name(&mapping_type.value_type.type_name, data_location)?;
                Some(self.types.register_type(Type::Mapping(MappingType {
                    key_type_id,
                    value_type_id,
                })))
            }
            ir::TypeName::ElementaryType(elementary_type) => {
                self.type_of_elementary_type(elementary_type, data_location)
            }
            ir::TypeName::IdentifierPath(identifier_path) => {
                self.type_of_identifier_path(identifier_path, data_location)
            }
        }
    }

    pub(super) fn find_library_scope_id_for_identifier_path(
        &self,
        identifier_path: &ir::IdentifierPath,
    ) -> Option<ScopeId> {
        let definition_id = identifier_path
            .last()
            .and_then(|identifier| {
                self.binder
                    .find_reference_by_identifier_node_id(identifier.id())
            })
            .and_then(|reference| {
                // reference may resolve to an imported library, so we need to
                // follow aliases
                self.binder
                    .follow_symbol_aliases(reference.resolution.clone())
                    .as_definition_id()
            })?;

        let Some(Definition::Library(_)) = self.binder.find_definition_by_id(definition_id) else {
            // the referenced definition is not a library
            return None;
        };
        self.binder.scope_id_for_node_id(definition_id)
    }
}
