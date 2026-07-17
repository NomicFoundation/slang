use std::sync::Arc;

use slang_solidity_v2_common::diagnostics::kinds::structure::PayableInternalOrPrivateFunction;
use slang_solidity_v2_common::diagnostics::kinds::syntax::{
    InvalidMutability, InvalidVisibility, MultipleMutabilitySpecifiers, MultipleOverrideSpecifiers,
    MultipleVirtualSpecifiers, MultipleVisibilitySpecifiers,
};
use slang_solidity_v2_common::terminals::TerminalKind;
use slang_solidity_v2_cst::structured_cst::nodes as input;
use slang_solidity_v2_cst::structured_cst::text_range::TextRange;

use crate::ir::builder::{CstToIrBuilder, Source};
use crate::ir::{
    nodes as output, FunctionMutability, FunctionVisibility, StateVariableMutability,
    StateVariableVisibility,
};

impl<S: Source> CstToIrBuilder<'_, S> {
    pub(super) fn build_function_attributes(
        &mut self,
        attributes: &input::FunctionAttributes,
    ) -> output::FunctionAttributes {
        let id = self.next_id(output::NodeKind::FunctionAttributes);
        let range = attributes.calculate_text_range().unwrap_or_default();

        let mut visibility = None;
        let mut mutability = None;
        let mut is_virtual = false;
        let mut override_specifier = None;
        let mut modifier_invocations = output::ModifierInvocations::new();

        for attribute in &attributes.elements {
            match attribute {
                input::FunctionAttribute::PublicKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::Public);
                }
                input::FunctionAttribute::PrivateKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::Private);
                }
                input::FunctionAttribute::InternalKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::Internal);
                }
                input::FunctionAttribute::ExternalKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::External);
                }
                input::FunctionAttribute::PureKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::Pure);
                }
                input::FunctionAttribute::ViewKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::View);
                }
                input::FunctionAttribute::PayableKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::Payable);
                }
                input::FunctionAttribute::VirtualKeyword(node) => {
                    self.record_virtual(&mut is_virtual, node);
                }
                input::FunctionAttribute::OverrideSpecifier(node) => {
                    self.record_override(&mut override_specifier, node);
                }
                input::FunctionAttribute::ModifierInvocation(node) => {
                    modifier_invocations.push(self.build_modifier_invocation(node));
                }
            }
        }

        Arc::new(output::FunctionAttributesStruct {
            id,
            range,
            visibility: visibility.unwrap_or(FunctionVisibility::Internal),
            has_explicit_visibility: visibility.is_some(),
            mutability: mutability.unwrap_or(FunctionMutability::NonPayable),
            is_virtual,
            override_specifier,
            modifier_invocations,
        })
    }

    pub(super) fn build_constructor_attributes(
        &mut self,
        attributes: &input::ConstructorAttributes,
    ) -> output::FunctionAttributes {
        let id = self.next_id(output::NodeKind::FunctionAttributes);
        let range = attributes.calculate_text_range().unwrap_or_default();

        let mut visibility = None;
        let mut mutability = None;
        let mut modifier_invocations = output::ModifierInvocations::new();

        for attribute in &attributes.elements {
            match attribute {
                input::ConstructorAttribute::PublicKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::Public);
                }
                input::ConstructorAttribute::InternalKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::Internal);
                }
                input::ConstructorAttribute::PayableKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::Payable);
                }
                input::ConstructorAttribute::ModifierInvocation(node) => {
                    modifier_invocations.push(self.build_modifier_invocation(node));
                }
            }
        }

        Arc::new(output::FunctionAttributesStruct {
            id,
            range,
            visibility: visibility.unwrap_or(FunctionVisibility::Public),
            has_explicit_visibility: visibility.is_some(),
            mutability: mutability.unwrap_or(FunctionMutability::NonPayable),
            is_virtual: false,
            override_specifier: None,
            modifier_invocations,
        })
    }

    pub(super) fn build_fallback_function_attributes(
        &mut self,
        owner: &impl TextRange,
        attributes: &input::FallbackFunctionAttributes,
    ) -> output::FunctionAttributes {
        let id = self.next_id(output::NodeKind::FunctionAttributes);
        let range = attributes.calculate_text_range().unwrap_or_default();

        let mut visibility = None;
        let mut mutability = None;
        let mut is_virtual = false;
        let mut override_specifier = None;
        let mut modifier_invocations = output::ModifierInvocations::new();

        for attribute in &attributes.elements {
            match attribute {
                input::FallbackFunctionAttribute::ExternalKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::External);
                }
                input::FallbackFunctionAttribute::PureKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::Pure);
                }
                input::FallbackFunctionAttribute::ViewKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::View);
                }
                input::FallbackFunctionAttribute::PayableKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::Payable);
                }
                input::FallbackFunctionAttribute::VirtualKeyword(node) => {
                    self.record_virtual(&mut is_virtual, node);
                }
                input::FallbackFunctionAttribute::OverrideSpecifier(node) => {
                    self.record_override(&mut override_specifier, node);
                }
                input::FallbackFunctionAttribute::ModifierInvocation(node) => {
                    modifier_invocations.push(self.build_modifier_invocation(node));
                }
            }
        }

        let has_explicit_visibility = visibility.is_some();
        let visibility = visibility.unwrap_or_else(|| {
            self.report(
                owner,
                InvalidVisibility {
                    valid: vec![TerminalKind::ExternalKeyword],
                },
            );
            FunctionVisibility::External
        });

        Arc::new(output::FunctionAttributesStruct {
            id,
            range,
            visibility,
            has_explicit_visibility,
            mutability: mutability.unwrap_or(FunctionMutability::NonPayable),
            is_virtual,
            override_specifier,
            modifier_invocations,
        })
    }

    pub(super) fn build_receive_function_attributes(
        &mut self,
        owner: &impl TextRange,
        attributes: &input::ReceiveFunctionAttributes,
    ) -> output::FunctionAttributes {
        let id = self.next_id(output::NodeKind::FunctionAttributes);
        let range = attributes.calculate_text_range().unwrap_or_default();

        let mut visibility = None;
        let mut mutability = None;
        let mut is_virtual = false;
        let mut override_specifier = None;
        let mut modifier_invocations = output::ModifierInvocations::new();

        for attribute in &attributes.elements {
            match attribute {
                input::ReceiveFunctionAttribute::ExternalKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::External);
                }
                input::ReceiveFunctionAttribute::PayableKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::Payable);
                }
                input::ReceiveFunctionAttribute::VirtualKeyword(node) => {
                    self.record_virtual(&mut is_virtual, node);
                }
                input::ReceiveFunctionAttribute::OverrideSpecifier(node) => {
                    self.record_override(&mut override_specifier, node);
                }
                input::ReceiveFunctionAttribute::ModifierInvocation(node) => {
                    modifier_invocations.push(self.build_modifier_invocation(node));
                }
            }
        }

        let has_explicit_visibility = visibility.is_some();
        let visibility = visibility.unwrap_or_else(|| {
            self.report(
                owner,
                InvalidVisibility {
                    valid: vec![TerminalKind::ExternalKeyword],
                },
            );
            FunctionVisibility::External
        });

        let mutability = mutability.unwrap_or_else(|| {
            self.report(
                owner,
                InvalidMutability {
                    valid: vec![TerminalKind::PayableKeyword],
                },
            );
            FunctionMutability::Payable
        });

        Arc::new(output::FunctionAttributesStruct {
            id,
            range,
            visibility,
            has_explicit_visibility,
            mutability,
            is_virtual,
            override_specifier,
            modifier_invocations,
        })
    }

    pub(super) fn build_modifier_attributes(
        &mut self,
        attributes: &input::ModifierAttributes,
    ) -> output::FunctionAttributes {
        let id = self.next_id(output::NodeKind::FunctionAttributes);
        let range = attributes.calculate_text_range().unwrap_or_default();

        let mut is_virtual = false;
        let mut override_specifier = None;

        for attribute in &attributes.elements {
            match attribute {
                input::ModifierAttribute::VirtualKeyword(node) => {
                    self.record_virtual(&mut is_virtual, node);
                }
                input::ModifierAttribute::OverrideSpecifier(node) => {
                    self.record_override(&mut override_specifier, node);
                }
            }
        }

        Arc::new(output::FunctionAttributesStruct {
            id,
            range,
            visibility: FunctionVisibility::Internal,
            has_explicit_visibility: false,
            mutability: FunctionMutability::NonPayable,
            is_virtual,
            override_specifier,
            modifier_invocations: output::ModifierInvocations::new(),
        })
    }

    pub(super) fn build_state_variable_attributes(
        &mut self,
        attributes: &input::StateVariableAttributes,
    ) -> output::StateVariableAttributes {
        let id = self.next_id(output::NodeKind::StateVariableAttributes);
        let range = attributes.calculate_text_range().unwrap_or_default();

        let mut visibility = None;
        let mut mutability = None;
        let mut override_specifier = None;

        for attribute in &attributes.elements {
            match attribute {
                input::StateVariableAttribute::PublicKeyword(node) => {
                    self.record_visibility(&mut visibility, node, StateVariableVisibility::Public);
                }
                input::StateVariableAttribute::PrivateKeyword(node) => {
                    self.record_visibility(&mut visibility, node, StateVariableVisibility::Private);
                }
                input::StateVariableAttribute::InternalKeyword(node) => {
                    self.record_visibility(
                        &mut visibility,
                        node,
                        StateVariableVisibility::Internal,
                    );
                }
                input::StateVariableAttribute::ConstantKeyword(node) => {
                    self.record_mutability(
                        &mut mutability,
                        node,
                        StateVariableMutability::Constant,
                    );
                }
                input::StateVariableAttribute::ImmutableKeyword(node) => {
                    self.record_mutability(
                        &mut mutability,
                        node,
                        StateVariableMutability::Immutable,
                    );
                }
                input::StateVariableAttribute::TransientKeyword(node) => {
                    self.record_mutability(
                        &mut mutability,
                        node,
                        StateVariableMutability::Transient,
                    );
                }
                input::StateVariableAttribute::OverrideSpecifier(node) => {
                    self.record_override(&mut override_specifier, node);
                }
            }
        }

        Arc::new(output::StateVariableAttributesStruct {
            id,
            range,
            visibility: visibility.unwrap_or(StateVariableVisibility::Internal),
            mutability: mutability.unwrap_or(StateVariableMutability::Mutable),
            override_specifier,
        })
    }

    pub(super) fn build_function_type_attributes(
        &mut self,
        owner: &impl TextRange,
        attributes: &input::FunctionTypeAttributes,
    ) -> output::FunctionTypeAttributes {
        let id = self.next_id(output::NodeKind::FunctionTypeAttributes);
        let range = attributes.calculate_text_range().unwrap_or_default();

        let mut visibility = None;
        let mut mutability = None;

        for attribute in &attributes.elements {
            match attribute {
                input::FunctionTypeAttribute::InternalKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::Internal);
                }
                input::FunctionTypeAttribute::ExternalKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::External);
                }
                input::FunctionTypeAttribute::PrivateKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::Private);
                    self.report(
                        node,
                        InvalidVisibility {
                            valid: vec![
                                TerminalKind::InternalKeyword,
                                TerminalKind::ExternalKeyword,
                            ],
                        },
                    );
                }
                input::FunctionTypeAttribute::PublicKeyword(node) => {
                    self.record_visibility(&mut visibility, node, FunctionVisibility::Public);
                    self.report(
                        node,
                        InvalidVisibility {
                            valid: vec![
                                TerminalKind::InternalKeyword,
                                TerminalKind::ExternalKeyword,
                            ],
                        },
                    );
                }
                input::FunctionTypeAttribute::PureKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::Pure);
                }
                input::FunctionTypeAttribute::ViewKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::View);
                }
                input::FunctionTypeAttribute::PayableKeyword(node) => {
                    self.record_mutability(&mut mutability, node, FunctionMutability::Payable);
                }
            }
        }

        let visibility = visibility.unwrap_or(FunctionVisibility::Internal);
        let mutability = mutability.unwrap_or(FunctionMutability::NonPayable);

        // Only external function types can be payable.
        if mutability == FunctionMutability::Payable && visibility != FunctionVisibility::External {
            self.report(owner, PayableInternalOrPrivateFunction);
        }

        Arc::new(output::FunctionTypeAttributesStruct {
            id,
            range,
            visibility,
            mutability,
        })
    }

    //
    // Shared Utilities:
    //

    fn record_visibility<V>(&mut self, existing: &mut Option<V>, node: &impl TextRange, value: V) {
        if existing.is_some() {
            self.report(node, MultipleVisibilitySpecifiers);
        } else {
            *existing = Some(value);
        }
    }

    fn record_mutability<M>(&mut self, existing: &mut Option<M>, node: &impl TextRange, value: M) {
        if existing.is_some() {
            self.report(node, MultipleMutabilitySpecifiers);
        } else {
            *existing = Some(value);
        }
    }

    fn record_virtual(&mut self, existing: &mut bool, keyword: &input::VirtualKeyword) {
        if *existing {
            self.report(keyword, MultipleVirtualSpecifiers);
        } else {
            *existing = true;
        }
    }

    fn record_override(
        &mut self,
        existing: &mut Option<output::OverridePaths>,
        specifier: &input::OverrideSpecifier,
    ) {
        if existing.is_some() {
            self.report(specifier, MultipleOverrideSpecifiers);
        } else {
            *existing = Some(
                specifier
                    .overridden
                    .as_ref()
                    .map_or(Vec::new(), |declaration| {
                        self.build_override_paths_declaration(declaration)
                    }),
            );
        }
    }
}
