use num_traits::{Signed, ToPrimitive};
use ruint::aliases::U256;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    ArrayLengthNegative, ArrayLengthNonInteger, ArrayLengthNotConstant, ArrayLengthTooLarge,
    ArrayLengthZero, InvalidFunctionTypeVisibility, StorageLayoutBaseNonInteger,
    StorageLayoutBaseNotConstant, StorageLayoutBaseOutOfRange,
};
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::TextRange;

use super::evaluator::{
    evaluate_compile_time_constant, ComptimeResolution, ConstantIdentifierResolver, EvaluationError,
};
use super::Pass;
use crate::binder::{Definition, Resolution, Scope, ScopeId};
use crate::built_ins::BuiltInsResolver;
use crate::passes::common::resolve_identifier_path_in_scope;
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

    /// Evaluates a fixed-size array length expression, and emits diagnostics
    /// if errors are found. On error, returns `usize::default()` so type
    /// construction can proceed.
    /// TODO: Change return to u256, when `FixedSizeArrayType` size field
    /// is changed to u256.
    fn evaluate_array_length(&mut self, size_expression: &ir::Expression) -> usize {
        let value = match evaluate_compile_time_constant(
            size_expression,
            self.current_contract_or_file_scope_id(),
            self,
        ) {
            Ok(value) => value,
            Err(EvaluationError::Diagnostic(kind)) => {
                self.push_expression_diagnostic(size_expression, kind);
                return usize::default();
            }
            Err(EvaluationError::CouldNotEvaluate) => {
                self.push_expression_diagnostic(size_expression, ArrayLengthNotConstant);
                return usize::default();
            }
        };

        // Classify the value: zero, then non-integer, then negative and then too large.
        if value.is_zero() {
            self.push_expression_diagnostic(size_expression, ArrayLengthZero);
            return usize::default();
        }
        let Some(integer) = value.as_integer() else {
            self.push_expression_diagnostic(size_expression, ArrayLengthNonInteger);
            return usize::default();
        };
        if integer.is_negative() {
            self.push_expression_diagnostic(size_expression, ArrayLengthNegative);
            return usize::default();
        }
        // TODO: Accept u256, when `FixedSizeArrayType` size field is changed to u256.
        let Some(size) = integer.to_usize() else {
            self.push_expression_diagnostic(size_expression, ArrayLengthTooLarge);
            return usize::default();
        };
        size
    }

    /// Evaluates a contract's storage-layout base slot expression, and emits
    /// diagnostics if errors are found. Returns `None` on any error.
    pub(super) fn evaluate_storage_layout_base_slot(
        &mut self,
        base_slot_expression: &ir::Expression,
    ) -> Option<U256> {
        let value = match evaluate_compile_time_constant(
            base_slot_expression,
            self.current_contract_or_file_scope_id(),
            self,
        ) {
            Ok(value) => value,
            Err(EvaluationError::Diagnostic(kind)) => {
                self.push_expression_diagnostic(base_slot_expression, kind);
                return None;
            }
            Err(EvaluationError::CouldNotEvaluate) => {
                self.push_expression_diagnostic(base_slot_expression, StorageLayoutBaseNotConstant);
                return None;
            }
        };

        // Classify the value: non-integer, then out of range (negative or too large).
        let Some(integer) = value.into_integer() else {
            self.push_expression_diagnostic(base_slot_expression, StorageLayoutBaseNonInteger);
            return None;
        };
        let Ok(base_slot) = U256::try_from(&integer) else {
            self.push_expression_diagnostic(
                base_slot_expression,
                StorageLayoutBaseOutOfRange {
                    value: integer.to_string(),
                },
            );
            return None;
        };
        Some(base_slot)
    }

    fn push_expression_diagnostic(
        &mut self,
        expression: &ir::Expression,
        kind: impl Into<DiagnosticKind>,
    ) {
        self.diagnostics.push(
            self.file_id.clone(),
            expression
                .calculate_text_range()
                .expect("expression has a text range"),
            kind,
        );
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
                                let size = self.evaluate_array_length(size_expression);
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
                    self.diagnostics.push(
                        self.file_id.clone(),
                        function_type.range.clone(),
                        InvalidFunctionTypeVisibility,
                    );
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
                    .follow_symbol_aliases(&reference.resolution)
                    .as_definition_id()
            })?;

        let Some(Definition::Library(_)) = self.binder.find_definition_by_id(definition_id) else {
            // the referenced definition is not a library
            return None;
        };
        self.binder.scope_id_for_node_id(definition_id)
    }
}

impl ConstantIdentifierResolver<ScopeId> for Pass<'_> {
    fn resolve_identifier_in_scope(
        &self,
        identifier: &str,
        scope_id: &ScopeId,
    ) -> ComptimeResolution<ScopeId> {
        match self.binder.resolve_in_scope(*scope_id, identifier) {
            Resolution::Definition(definition_id) => {
                match self
                    .binder
                    .find_definition_by_id(definition_id)
                    .expect("resolved definition is found")
                {
                    Definition::Constant(constant_definition) => {
                        if let Some(value) = &constant_definition.ir_node.value {
                            ComptimeResolution::Constant {
                                value: value.clone(),
                                target_scope: constant_definition.enclosing_scope_id,
                            }
                        } else {
                            // TODO(validation) SDR[1732]: this is a constant state
                            // variable for which the grammar allows an absent
                            // value expression but it's a semantic error
                            ComptimeResolution::Unresolved
                        }
                    }
                    _ => ComptimeResolution::Unresolved,
                }
            }
            Resolution::Ambiguous(_) => {
                // TODO(validation) SDR[1731]: multiple definitions found which is an error
                ComptimeResolution::Unresolved
            }
            Resolution::BuiltIn(_) => unreachable!("the binder doesn't resolve to built-ins"),

            Resolution::Unresolved => {
                // Try to resolve a built-in using the scope as context
                let built_in = match self.binder.get_scope_by_id(*scope_id) {
                    Scope::Block(_)
                    | Scope::Chained(_)
                    | Scope::Contract(_)
                    | Scope::File(_)
                    | Scope::Function(_)
                    | Scope::Modifier(_) => BuiltInsResolver::lookup_global(identifier),

                    Scope::Enum(_) | Scope::Parameters(_) | Scope::Struct(_) | Scope::Using(_) => {
                        None
                    }

                    Scope::YulBlock(_) | Scope::YulFunction(_) => {
                        BuiltInsResolver::lookup_yul_global(identifier)
                    }
                };
                built_in.map_or(ComptimeResolution::Unresolved, ComptimeResolution::BuiltIn)
            }
        }
    }
}
