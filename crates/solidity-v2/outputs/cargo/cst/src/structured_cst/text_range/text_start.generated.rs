// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use std::rc::Rc;

use crate::structured_cst::nodes::*;

/// A trait for CST nodes that can report the start offset of their text range.
pub(super) trait TextStart {
    /// Returns the start offset of this node's text, or `None` if the node is empty.
    fn calculate_text_start(&self) -> Option<usize>;
}

impl<T: TextStart> TextStart for Option<T> {
    fn calculate_text_start(&self) -> Option<usize> {
        self.as_ref().and_then(TextStart::calculate_text_start)
    }
}

impl<T: TextStart> TextStart for Rc<T> {
    fn calculate_text_start(&self) -> Option<usize> {
        (**self).calculate_text_start()
    }
}

impl TextStart for AbicoderPragmaStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.abicoder_keyword.calculate_text_start())
            .or_else(|| self.version.calculate_text_start())
    }
}

impl TextStart for AdditiveExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| {
                self.expression_additive_expression_operator
                    .calculate_text_start()
            })
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for AddressTypeStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.address_keyword.calculate_text_start())
            .or_else(|| self.payable_keyword.calculate_text_start())
    }
}

impl TextStart for AndExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| self.operator.calculate_text_start())
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for ArrayExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_bracket.calculate_text_start())
            .or_else(|| self.items.calculate_text_start())
            .or_else(|| self.close_bracket.calculate_text_start())
    }
}

impl TextStart for ArrayTypeNameStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_start())
            .or_else(|| self.open_bracket.calculate_text_start())
            .or_else(|| self.index.calculate_text_start())
            .or_else(|| self.close_bracket.calculate_text_start())
    }
}

impl TextStart for AssemblyStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.assembly_keyword.calculate_text_start())
            .or_else(|| self.label.calculate_text_start())
            .or_else(|| self.flags.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for AssignmentExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| {
                self.expression_assignment_expression_operator
                    .calculate_text_start()
            })
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for BitwiseAndExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| self.operator.calculate_text_start())
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for BitwiseOrExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| self.operator.calculate_text_start())
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for BitwiseXorExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| self.operator.calculate_text_start())
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for BlockStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.statements.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for BreakStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.break_keyword.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for CallOptionsExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_start())
            .or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.options.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for CatchClauseStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.catch_keyword.calculate_text_start())
            .or_else(|| self.error.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for CatchClauseErrorStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
    }
}

impl TextStart for ConditionalExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_start())
            .or_else(|| self.question_mark.calculate_text_start())
            .or_else(|| self.true_expression.calculate_text_start())
            .or_else(|| self.colon.calculate_text_start())
            .or_else(|| self.false_expression.calculate_text_start())
    }
}

impl TextStart for ConstantDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.constant_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.equal.calculate_text_start())
            .or_else(|| self.value.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for ConstructorDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.constructor_keyword.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.attributes.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for ContinueStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.continue_keyword.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for ContractDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.abstract_keyword.calculate_text_start())
            .or_else(|| self.contract_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.specifiers.calculate_text_start())
            .or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.members.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for DecimalNumberExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.literal.calculate_text_start())
            .or_else(|| self.unit.calculate_text_start())
    }
}

impl TextStart for DoWhileStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.do_keyword.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
            .or_else(|| self.while_keyword.calculate_text_start())
            .or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.condition.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for ElseBranchStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.else_keyword.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for EmitStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.emit_keyword.calculate_text_start())
            .or_else(|| self.event.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for EnumDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.enum_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.members.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for EqualityExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| {
                self.expression_equality_expression_operator
                    .calculate_text_start()
            })
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for ErrorDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.error_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.members.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for ErrorParameterStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
    }
}

impl TextStart for ErrorParametersDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for EventDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.event_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.anonymous_keyword.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for EventParameterStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.indexed_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
    }
}

impl TextStart for EventParametersDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for ExperimentalPragmaStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.experimental_keyword.calculate_text_start())
            .or_else(|| self.feature.calculate_text_start())
    }
}

impl TextStart for ExponentiationExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| self.operator.calculate_text_start())
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for ExpressionStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.expression.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for FallbackFunctionDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.fallback_keyword.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.attributes.calculate_text_start())
            .or_else(|| self.returns.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for ForStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.for_keyword.calculate_text_start())
            .or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.initialization.calculate_text_start())
            .or_else(|| self.condition.calculate_text_start())
            .or_else(|| self.iterator.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for FunctionCallExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
    }
}

impl TextStart for FunctionDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.function_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.attributes.calculate_text_start())
            .or_else(|| self.returns.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for FunctionTypeStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.function_keyword.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.attributes.calculate_text_start())
            .or_else(|| self.returns.calculate_text_start())
    }
}

impl TextStart for HexNumberExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.literal.calculate_text_start())
    }
}

impl TextStart for IfStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.if_keyword.calculate_text_start())
            .or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.condition.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
            .or_else(|| self.else_branch.calculate_text_start())
    }
}

impl TextStart for ImportAliasStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.as_keyword.calculate_text_start())
            .or_else(|| self.identifier.calculate_text_start())
    }
}

impl TextStart for ImportDeconstructionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.symbols.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
            .or_else(|| self.from_keyword.calculate_text_start())
            .or_else(|| self.path.calculate_text_start())
    }
}

impl TextStart for ImportDeconstructionSymbolStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_start())
            .or_else(|| self.alias.calculate_text_start())
    }
}

impl TextStart for ImportDirectiveStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.import_keyword.calculate_text_start())
            .or_else(|| self.clause.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for IndexAccessEndStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.colon.calculate_text_start())
            .or_else(|| self.end.calculate_text_start())
    }
}

impl TextStart for IndexAccessExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_start())
            .or_else(|| self.open_bracket.calculate_text_start())
            .or_else(|| self.start.calculate_text_start())
            .or_else(|| self.end.calculate_text_start())
            .or_else(|| self.close_bracket.calculate_text_start())
    }
}

impl TextStart for InequalityExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| {
                self.expression_inequality_expression_operator
                    .calculate_text_start()
            })
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for InheritanceSpecifierStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.is_keyword.calculate_text_start())
            .or_else(|| self.types.calculate_text_start())
    }
}

impl TextStart for InheritanceTypeStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
    }
}

impl TextStart for InterfaceDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.interface_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.inheritance.calculate_text_start())
            .or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.members.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for LibraryDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.library_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.members.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for MappingKeyStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.key_type.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
    }
}

impl TextStart for MappingTypeStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.mapping_keyword.calculate_text_start())
            .or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.key_type.calculate_text_start())
            .or_else(|| self.equal_greater_than.calculate_text_start())
            .or_else(|| self.value_type.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for MappingValueStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
    }
}

impl TextStart for MemberAccessExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_start())
            .or_else(|| self.period.calculate_text_start())
            .or_else(|| self.member.calculate_text_start())
    }
}

impl TextStart for ModifierDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.modifier_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.attributes.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for ModifierInvocationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
    }
}

impl TextStart for MultiTypedDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.elements.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
            .or_else(|| self.value.calculate_text_start())
    }
}

impl TextStart for MultiTypedDeclarationElementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.member.calculate_text_start())
    }
}

impl TextStart for MultiplicativeExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| {
                self.expression_multiplicative_expression_operator
                    .calculate_text_start()
            })
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for NamedArgumentStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_start())
            .or_else(|| self.colon.calculate_text_start())
            .or_else(|| self.value.calculate_text_start())
    }
}

impl TextStart for NamedArgumentGroupStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for NamedArgumentsDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for NamedImportStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.asterisk.calculate_text_start())
            .or_else(|| self.alias.calculate_text_start())
            .or_else(|| self.from_keyword.calculate_text_start())
            .or_else(|| self.path.calculate_text_start())
    }
}

impl TextStart for NewExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.new_keyword.calculate_text_start())
            .or_else(|| self.type_name.calculate_text_start())
    }
}

impl TextStart for OrExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| self.operator.calculate_text_start())
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for OverridePathsDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.paths.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for OverrideSpecifierStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.override_keyword.calculate_text_start())
            .or_else(|| self.overridden.calculate_text_start())
    }
}

impl TextStart for ParameterStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.storage_location.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
    }
}

impl TextStart for ParametersDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for PathImportStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.path.calculate_text_start())
            .or_else(|| self.alias.calculate_text_start())
    }
}

impl TextStart for PositionalArgumentsDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for PostfixExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_start())
            .or_else(|| {
                self.expression_postfix_expression_operator
                    .calculate_text_start()
            })
    }
}

impl TextStart for PragmaDirectiveStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.pragma_keyword.calculate_text_start())
            .or_else(|| self.pragma.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for PrefixExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| {
            self.expression_prefix_expression_operator
                .calculate_text_start()
        })
        .or_else(|| self.operand.calculate_text_start())
    }
}

impl TextStart for ReceiveFunctionDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.receive_keyword.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.attributes.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for ReturnStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.return_keyword.calculate_text_start())
            .or_else(|| self.expression.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for ReturnsDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.returns_keyword.calculate_text_start())
            .or_else(|| self.variables.calculate_text_start())
    }
}

impl TextStart for RevertStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.revert_keyword.calculate_text_start())
            .or_else(|| self.error.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for ShiftExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.left_operand.calculate_text_start())
            .or_else(|| {
                self.expression_shift_expression_operator
                    .calculate_text_start()
            })
            .or_else(|| self.right_operand.calculate_text_start())
    }
}

impl TextStart for SingleTypedDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.declaration.calculate_text_start())
            .or_else(|| self.value.calculate_text_start())
    }
}

impl TextStart for SourceUnitStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.members.calculate_text_start())
    }
}

impl TextStart for StateVariableDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.attributes.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.value.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for StateVariableDefinitionValueStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.equal.calculate_text_start())
            .or_else(|| self.value.calculate_text_start())
    }
}

impl TextStart for StorageLayoutSpecifierStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.layout_keyword.calculate_text_start())
            .or_else(|| self.at_keyword.calculate_text_start())
            .or_else(|| self.expression.calculate_text_start())
    }
}

impl TextStart for StructDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.struct_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.members.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for StructMemberStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for TryStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.try_keyword.calculate_text_start())
            .or_else(|| self.expression.calculate_text_start())
            .or_else(|| self.returns.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
            .or_else(|| self.catch_clauses.calculate_text_start())
    }
}

impl TextStart for TupleExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.items.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for TupleValueStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.expression.calculate_text_start())
    }
}

impl TextStart for TypeExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_keyword.calculate_text_start())
            .or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for UncheckedBlockStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.unchecked_keyword.calculate_text_start())
            .or_else(|| self.block.calculate_text_start())
    }
}

impl TextStart for UserDefinedValueTypeDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.is_keyword.calculate_text_start())
            .or_else(|| self.value_type.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for UsingAliasStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.as_keyword.calculate_text_start())
            .or_else(|| self.operator.calculate_text_start())
    }
}

impl TextStart for UsingDeconstructionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.symbols.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for UsingDeconstructionSymbolStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_start())
            .or_else(|| self.alias.calculate_text_start())
    }
}

impl TextStart for UsingDirectiveStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.using_keyword.calculate_text_start())
            .or_else(|| self.clause.calculate_text_start())
            .or_else(|| self.for_keyword.calculate_text_start())
            .or_else(|| self.target.calculate_text_start())
            .or_else(|| self.global_keyword.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for VariableDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_start())
            .or_else(|| self.storage_location.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
    }
}

impl TextStart for VariableDeclarationStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.target.calculate_text_start())
            .or_else(|| self.semicolon.calculate_text_start())
    }
}

impl TextStart for VariableDeclarationValueStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.equal.calculate_text_start())
            .or_else(|| self.expression.calculate_text_start())
    }
}

impl TextStart for VersionPragmaStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.solidity_keyword.calculate_text_start())
            .or_else(|| self.sets.calculate_text_start())
    }
}

impl TextStart for VersionRangeStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.start.calculate_text_start())
            .or_else(|| self.minus.calculate_text_start())
            .or_else(|| self.end.calculate_text_start())
    }
}

impl TextStart for VersionTermStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operator.calculate_text_start())
            .or_else(|| self.literal.calculate_text_start())
    }
}

impl TextStart for WhileStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.while_keyword.calculate_text_start())
            .or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.condition.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for YulBlockStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_brace.calculate_text_start())
            .or_else(|| self.statements.calculate_text_start())
            .or_else(|| self.close_brace.calculate_text_start())
    }
}

impl TextStart for YulBreakStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.break_keyword.calculate_text_start())
    }
}

impl TextStart for YulContinueStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.continue_keyword.calculate_text_start())
    }
}

impl TextStart for YulDefaultCaseStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.default_keyword.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for YulFlagsDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.flags.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for YulForStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.for_keyword.calculate_text_start())
            .or_else(|| self.initialization.calculate_text_start())
            .or_else(|| self.condition.calculate_text_start())
            .or_else(|| self.iterator.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for YulFunctionCallExpressionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_start())
            .or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.arguments.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for YulFunctionDefinitionStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.function_keyword.calculate_text_start())
            .or_else(|| self.name.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.returns.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for YulIfStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.if_keyword.calculate_text_start())
            .or_else(|| self.condition.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for YulLeaveStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.leave_keyword.calculate_text_start())
    }
}

impl TextStart for YulParametersDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.open_paren.calculate_text_start())
            .or_else(|| self.parameters.calculate_text_start())
            .or_else(|| self.close_paren.calculate_text_start())
    }
}

impl TextStart for YulReturnsDeclarationStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.minus_greater_than.calculate_text_start())
            .or_else(|| self.variables.calculate_text_start())
    }
}

impl TextStart for YulSwitchStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.switch_keyword.calculate_text_start())
            .or_else(|| self.expression.calculate_text_start())
            .or_else(|| self.cases.calculate_text_start())
    }
}

impl TextStart for YulValueCaseStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.case_keyword.calculate_text_start())
            .or_else(|| self.value.calculate_text_start())
            .or_else(|| self.body.calculate_text_start())
    }
}

impl TextStart for YulVariableAssignmentStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.variables.calculate_text_start())
            .or_else(|| self.assignment.calculate_text_start())
            .or_else(|| self.expression.calculate_text_start())
    }
}

impl TextStart for YulVariableDeclarationStatementStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.let_keyword.calculate_text_start())
            .or_else(|| self.variables.calculate_text_start())
            .or_else(|| self.value.calculate_text_start())
    }
}

impl TextStart for YulVariableDeclarationValueStruct {
    fn calculate_text_start(&self) -> Option<usize> {
        None.or_else(|| self.assignment.calculate_text_start())
            .or_else(|| self.expression.calculate_text_start())
    }
}

impl TextStart for AbicoderVersion {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            AbicoderVersion::AbicoderV1Keyword(child) => child.calculate_text_start(),
            AbicoderVersion::AbicoderV2Keyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ArgumentsDeclaration {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(child) => {
                child.calculate_text_start()
            }
            ArgumentsDeclaration::NamedArgumentsDeclaration(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ConstructorAttribute {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ConstructorAttribute::ModifierInvocation(child) => child.calculate_text_start(),
            ConstructorAttribute::InternalKeyword(child) => child.calculate_text_start(),
            ConstructorAttribute::PayableKeyword(child) => child.calculate_text_start(),
            ConstructorAttribute::PublicKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ContractMember {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ContractMember::UsingDirective(child) => child.calculate_text_start(),
            ContractMember::FunctionDefinition(child) => child.calculate_text_start(),
            ContractMember::ConstructorDefinition(child) => child.calculate_text_start(),
            ContractMember::ReceiveFunctionDefinition(child) => child.calculate_text_start(),
            ContractMember::FallbackFunctionDefinition(child) => child.calculate_text_start(),
            ContractMember::ModifierDefinition(child) => child.calculate_text_start(),
            ContractMember::StructDefinition(child) => child.calculate_text_start(),
            ContractMember::EnumDefinition(child) => child.calculate_text_start(),
            ContractMember::EventDefinition(child) => child.calculate_text_start(),
            ContractMember::ErrorDefinition(child) => child.calculate_text_start(),
            ContractMember::UserDefinedValueTypeDefinition(child) => child.calculate_text_start(),
            ContractMember::StateVariableDefinition(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ContractSpecifier {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ContractSpecifier::InheritanceSpecifier(child) => child.calculate_text_start(),
            ContractSpecifier::StorageLayoutSpecifier(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ElementaryType {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ElementaryType::BoolKeyword(child) => child.calculate_text_start(),
            ElementaryType::StringKeyword(child) => child.calculate_text_start(),
            ElementaryType::AddressType(child) => child.calculate_text_start(),
            ElementaryType::BytesKeyword(child) => child.calculate_text_start(),
            ElementaryType::IntKeyword(child) => child.calculate_text_start(),
            ElementaryType::UintKeyword(child) => child.calculate_text_start(),
            ElementaryType::FixedKeyword(child) => child.calculate_text_start(),
            ElementaryType::UfixedKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ExperimentalFeature {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ExperimentalFeature::ABIEncoderV2Keyword(child) => child.calculate_text_start(),
            ExperimentalFeature::SMTCheckerKeyword(child) => child.calculate_text_start(),
            ExperimentalFeature::PragmaStringLiteral(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for Expression {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression::AssignmentExpression(child) => child.calculate_text_start(),
            Expression::ConditionalExpression(child) => child.calculate_text_start(),
            Expression::OrExpression(child) => child.calculate_text_start(),
            Expression::AndExpression(child) => child.calculate_text_start(),
            Expression::EqualityExpression(child) => child.calculate_text_start(),
            Expression::InequalityExpression(child) => child.calculate_text_start(),
            Expression::BitwiseOrExpression(child) => child.calculate_text_start(),
            Expression::BitwiseXorExpression(child) => child.calculate_text_start(),
            Expression::BitwiseAndExpression(child) => child.calculate_text_start(),
            Expression::ShiftExpression(child) => child.calculate_text_start(),
            Expression::AdditiveExpression(child) => child.calculate_text_start(),
            Expression::MultiplicativeExpression(child) => child.calculate_text_start(),
            Expression::ExponentiationExpression(child) => child.calculate_text_start(),
            Expression::PostfixExpression(child) => child.calculate_text_start(),
            Expression::PrefixExpression(child) => child.calculate_text_start(),
            Expression::FunctionCallExpression(child) => child.calculate_text_start(),
            Expression::CallOptionsExpression(child) => child.calculate_text_start(),
            Expression::MemberAccessExpression(child) => child.calculate_text_start(),
            Expression::IndexAccessExpression(child) => child.calculate_text_start(),
            Expression::NewExpression(child) => child.calculate_text_start(),
            Expression::TupleExpression(child) => child.calculate_text_start(),
            Expression::TypeExpression(child) => child.calculate_text_start(),
            Expression::ArrayExpression(child) => child.calculate_text_start(),
            Expression::HexNumberExpression(child) => child.calculate_text_start(),
            Expression::DecimalNumberExpression(child) => child.calculate_text_start(),
            Expression::StringExpression(child) => child.calculate_text_start(),
            Expression::ElementaryType(child) => child.calculate_text_start(),
            Expression::PayableKeyword(child) => child.calculate_text_start(),
            Expression::ThisKeyword(child) => child.calculate_text_start(),
            Expression::SuperKeyword(child) => child.calculate_text_start(),
            Expression::TrueKeyword(child) => child.calculate_text_start(),
            Expression::FalseKeyword(child) => child.calculate_text_start(),
            Expression::Identifier(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for Expression_AdditiveExpression_Operator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression_AdditiveExpression_Operator::Minus(child) => child.calculate_text_start(),
            Expression_AdditiveExpression_Operator::Plus(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for Expression_AssignmentExpression_Operator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression_AssignmentExpression_Operator::AmpersandEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::AsteriskEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::BarEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::CaretEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::Equal(child) => child.calculate_text_start(),
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(
                child,
            ) => child.calculate_text_start(),
            Expression_AssignmentExpression_Operator::LessThanLessThanEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::MinusEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::PercentEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::PlusEqual(child) => {
                child.calculate_text_start()
            }
            Expression_AssignmentExpression_Operator::SlashEqual(child) => {
                child.calculate_text_start()
            }
        }
    }
}

impl TextStart for Expression_EqualityExpression_Operator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression_EqualityExpression_Operator::BangEqual(child) => {
                child.calculate_text_start()
            }
            Expression_EqualityExpression_Operator::EqualEqual(child) => {
                child.calculate_text_start()
            }
        }
    }
}

impl TextStart for Expression_InequalityExpression_Operator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression_InequalityExpression_Operator::GreaterThan(child) => {
                child.calculate_text_start()
            }
            Expression_InequalityExpression_Operator::GreaterThanEqual(child) => {
                child.calculate_text_start()
            }
            Expression_InequalityExpression_Operator::LessThan(child) => {
                child.calculate_text_start()
            }
            Expression_InequalityExpression_Operator::LessThanEqual(child) => {
                child.calculate_text_start()
            }
        }
    }
}

impl TextStart for Expression_MultiplicativeExpression_Operator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression_MultiplicativeExpression_Operator::Asterisk(child) => {
                child.calculate_text_start()
            }
            Expression_MultiplicativeExpression_Operator::Percent(child) => {
                child.calculate_text_start()
            }
            Expression_MultiplicativeExpression_Operator::Slash(child) => {
                child.calculate_text_start()
            }
        }
    }
}

impl TextStart for Expression_PostfixExpression_Operator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression_PostfixExpression_Operator::MinusMinus(child) => {
                child.calculate_text_start()
            }
            Expression_PostfixExpression_Operator::PlusPlus(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for Expression_PrefixExpression_Operator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression_PrefixExpression_Operator::Bang(child) => child.calculate_text_start(),
            Expression_PrefixExpression_Operator::DeleteKeyword(child) => {
                child.calculate_text_start()
            }
            Expression_PrefixExpression_Operator::Minus(child) => child.calculate_text_start(),
            Expression_PrefixExpression_Operator::MinusMinus(child) => child.calculate_text_start(),
            Expression_PrefixExpression_Operator::PlusPlus(child) => child.calculate_text_start(),
            Expression_PrefixExpression_Operator::Tilde(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for Expression_ShiftExpression_Operator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Expression_ShiftExpression_Operator::GreaterThanGreaterThan(child) => {
                child.calculate_text_start()
            }
            Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(child) => {
                child.calculate_text_start()
            }
            Expression_ShiftExpression_Operator::LessThanLessThan(child) => {
                child.calculate_text_start()
            }
        }
    }
}

impl TextStart for FallbackFunctionAttribute {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            FallbackFunctionAttribute::ModifierInvocation(child) => child.calculate_text_start(),
            FallbackFunctionAttribute::OverrideSpecifier(child) => child.calculate_text_start(),
            FallbackFunctionAttribute::ExternalKeyword(child) => child.calculate_text_start(),
            FallbackFunctionAttribute::PayableKeyword(child) => child.calculate_text_start(),
            FallbackFunctionAttribute::PureKeyword(child) => child.calculate_text_start(),
            FallbackFunctionAttribute::ViewKeyword(child) => child.calculate_text_start(),
            FallbackFunctionAttribute::VirtualKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ForStatementCondition {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ForStatementCondition::ExpressionStatement(child) => child.calculate_text_start(),
            ForStatementCondition::Semicolon(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ForStatementInitialization {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(child) => {
                child.calculate_text_start()
            }
            ForStatementInitialization::ExpressionStatement(child) => child.calculate_text_start(),
            ForStatementInitialization::Semicolon(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for FunctionAttribute {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            FunctionAttribute::ModifierInvocation(child) => child.calculate_text_start(),
            FunctionAttribute::OverrideSpecifier(child) => child.calculate_text_start(),
            FunctionAttribute::ExternalKeyword(child) => child.calculate_text_start(),
            FunctionAttribute::InternalKeyword(child) => child.calculate_text_start(),
            FunctionAttribute::PayableKeyword(child) => child.calculate_text_start(),
            FunctionAttribute::PrivateKeyword(child) => child.calculate_text_start(),
            FunctionAttribute::PublicKeyword(child) => child.calculate_text_start(),
            FunctionAttribute::PureKeyword(child) => child.calculate_text_start(),
            FunctionAttribute::ViewKeyword(child) => child.calculate_text_start(),
            FunctionAttribute::VirtualKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for FunctionBody {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            FunctionBody::Block(child) => child.calculate_text_start(),
            FunctionBody::Semicolon(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for FunctionName {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            FunctionName::Identifier(child) => child.calculate_text_start(),
            FunctionName::FallbackKeyword(child) => child.calculate_text_start(),
            FunctionName::ReceiveKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for FunctionTypeAttribute {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            FunctionTypeAttribute::InternalKeyword(child) => child.calculate_text_start(),
            FunctionTypeAttribute::ExternalKeyword(child) => child.calculate_text_start(),
            FunctionTypeAttribute::PrivateKeyword(child) => child.calculate_text_start(),
            FunctionTypeAttribute::PublicKeyword(child) => child.calculate_text_start(),
            FunctionTypeAttribute::PureKeyword(child) => child.calculate_text_start(),
            FunctionTypeAttribute::ViewKeyword(child) => child.calculate_text_start(),
            FunctionTypeAttribute::PayableKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for IdentifierPathElement {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            IdentifierPathElement::Identifier(child) => child.calculate_text_start(),
            IdentifierPathElement::AddressKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ImportClause {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ImportClause::PathImport(child) => child.calculate_text_start(),
            ImportClause::NamedImport(child) => child.calculate_text_start(),
            ImportClause::ImportDeconstruction(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for MappingKeyType {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            MappingKeyType::ElementaryType(child) => child.calculate_text_start(),
            MappingKeyType::IdentifierPath(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ModifierAttribute {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ModifierAttribute::OverrideSpecifier(child) => child.calculate_text_start(),
            ModifierAttribute::VirtualKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for NumberUnit {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            NumberUnit::WeiKeyword(child) => child.calculate_text_start(),
            NumberUnit::GweiKeyword(child) => child.calculate_text_start(),
            NumberUnit::EtherKeyword(child) => child.calculate_text_start(),
            NumberUnit::SecondsKeyword(child) => child.calculate_text_start(),
            NumberUnit::MinutesKeyword(child) => child.calculate_text_start(),
            NumberUnit::HoursKeyword(child) => child.calculate_text_start(),
            NumberUnit::DaysKeyword(child) => child.calculate_text_start(),
            NumberUnit::WeeksKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for Pragma {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Pragma::VersionPragma(child) => child.calculate_text_start(),
            Pragma::AbicoderPragma(child) => child.calculate_text_start(),
            Pragma::ExperimentalPragma(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ReceiveFunctionAttribute {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            ReceiveFunctionAttribute::ModifierInvocation(child) => child.calculate_text_start(),
            ReceiveFunctionAttribute::OverrideSpecifier(child) => child.calculate_text_start(),
            ReceiveFunctionAttribute::ExternalKeyword(child) => child.calculate_text_start(),
            ReceiveFunctionAttribute::PayableKeyword(child) => child.calculate_text_start(),
            ReceiveFunctionAttribute::VirtualKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for SourceUnitMember {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            SourceUnitMember::PragmaDirective(child) => child.calculate_text_start(),
            SourceUnitMember::ImportDirective(child) => child.calculate_text_start(),
            SourceUnitMember::ContractDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::InterfaceDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::LibraryDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::StructDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::EnumDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::FunctionDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::ErrorDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::UserDefinedValueTypeDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::UsingDirective(child) => child.calculate_text_start(),
            SourceUnitMember::EventDefinition(child) => child.calculate_text_start(),
            SourceUnitMember::ConstantDefinition(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for StateVariableAttribute {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            StateVariableAttribute::OverrideSpecifier(child) => child.calculate_text_start(),
            StateVariableAttribute::ConstantKeyword(child) => child.calculate_text_start(),
            StateVariableAttribute::InternalKeyword(child) => child.calculate_text_start(),
            StateVariableAttribute::PrivateKeyword(child) => child.calculate_text_start(),
            StateVariableAttribute::PublicKeyword(child) => child.calculate_text_start(),
            StateVariableAttribute::ImmutableKeyword(child) => child.calculate_text_start(),
            StateVariableAttribute::TransientKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for Statement {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            Statement::IfStatement(child) => child.calculate_text_start(),
            Statement::ForStatement(child) => child.calculate_text_start(),
            Statement::WhileStatement(child) => child.calculate_text_start(),
            Statement::DoWhileStatement(child) => child.calculate_text_start(),
            Statement::ContinueStatement(child) => child.calculate_text_start(),
            Statement::BreakStatement(child) => child.calculate_text_start(),
            Statement::ReturnStatement(child) => child.calculate_text_start(),
            Statement::EmitStatement(child) => child.calculate_text_start(),
            Statement::TryStatement(child) => child.calculate_text_start(),
            Statement::RevertStatement(child) => child.calculate_text_start(),
            Statement::AssemblyStatement(child) => child.calculate_text_start(),
            Statement::Block(child) => child.calculate_text_start(),
            Statement::UncheckedBlock(child) => child.calculate_text_start(),
            Statement::VariableDeclarationStatement(child) => child.calculate_text_start(),
            Statement::ExpressionStatement(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for StorageLocation {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            StorageLocation::MemoryKeyword(child) => child.calculate_text_start(),
            StorageLocation::StorageKeyword(child) => child.calculate_text_start(),
            StorageLocation::CallDataKeyword(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for StringExpression {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            StringExpression::StringLiterals(child) => child.calculate_text_start(),
            StringExpression::HexStringLiterals(child) => child.calculate_text_start(),
            StringExpression::UnicodeStringLiterals(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for TypeName {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            TypeName::ArrayTypeName(child) => child.calculate_text_start(),
            TypeName::FunctionType(child) => child.calculate_text_start(),
            TypeName::MappingType(child) => child.calculate_text_start(),
            TypeName::ElementaryType(child) => child.calculate_text_start(),
            TypeName::IdentifierPath(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for UsingClause {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            UsingClause::IdentifierPath(child) => child.calculate_text_start(),
            UsingClause::UsingDeconstruction(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for UsingOperator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            UsingOperator::Ampersand(child) => child.calculate_text_start(),
            UsingOperator::Asterisk(child) => child.calculate_text_start(),
            UsingOperator::BangEqual(child) => child.calculate_text_start(),
            UsingOperator::Bar(child) => child.calculate_text_start(),
            UsingOperator::Caret(child) => child.calculate_text_start(),
            UsingOperator::EqualEqual(child) => child.calculate_text_start(),
            UsingOperator::GreaterThan(child) => child.calculate_text_start(),
            UsingOperator::GreaterThanEqual(child) => child.calculate_text_start(),
            UsingOperator::LessThan(child) => child.calculate_text_start(),
            UsingOperator::LessThanEqual(child) => child.calculate_text_start(),
            UsingOperator::Minus(child) => child.calculate_text_start(),
            UsingOperator::Percent(child) => child.calculate_text_start(),
            UsingOperator::Plus(child) => child.calculate_text_start(),
            UsingOperator::Slash(child) => child.calculate_text_start(),
            UsingOperator::Tilde(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for UsingTarget {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            UsingTarget::TypeName(child) => child.calculate_text_start(),
            UsingTarget::Asterisk(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for VariableDeclarationTarget {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(child) => {
                child.calculate_text_start()
            }
            VariableDeclarationTarget::MultiTypedDeclaration(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for VersionExpression {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            VersionExpression::VersionRange(child) => child.calculate_text_start(),
            VersionExpression::VersionTerm(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for VersionLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            VersionLiteral::SimpleVersionLiteral(child) => child.calculate_text_start(),
            VersionLiteral::PragmaStringLiteral(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for VersionOperator {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            VersionOperator::PragmaCaret(child) => child.calculate_text_start(),
            VersionOperator::PragmaTilde(child) => child.calculate_text_start(),
            VersionOperator::PragmaEqual(child) => child.calculate_text_start(),
            VersionOperator::PragmaLessThan(child) => child.calculate_text_start(),
            VersionOperator::PragmaGreaterThan(child) => child.calculate_text_start(),
            VersionOperator::PragmaLessThanEqual(child) => child.calculate_text_start(),
            VersionOperator::PragmaGreaterThanEqual(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for YulExpression {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            YulExpression::YulFunctionCallExpression(child) => child.calculate_text_start(),
            YulExpression::YulLiteral(child) => child.calculate_text_start(),
            YulExpression::YulPath(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for YulLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            YulLiteral::YulTrueKeyword(child) => child.calculate_text_start(),
            YulLiteral::YulFalseKeyword(child) => child.calculate_text_start(),
            YulLiteral::YulDecimalLiteral(child) => child.calculate_text_start(),
            YulLiteral::YulHexLiteral(child) => child.calculate_text_start(),
            YulLiteral::YulHexStringLiteral(child) => child.calculate_text_start(),
            YulLiteral::YulStringLiteral(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for YulStatement {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            YulStatement::YulBlock(child) => child.calculate_text_start(),
            YulStatement::YulFunctionDefinition(child) => child.calculate_text_start(),
            YulStatement::YulIfStatement(child) => child.calculate_text_start(),
            YulStatement::YulForStatement(child) => child.calculate_text_start(),
            YulStatement::YulSwitchStatement(child) => child.calculate_text_start(),
            YulStatement::YulLeaveStatement(child) => child.calculate_text_start(),
            YulStatement::YulBreakStatement(child) => child.calculate_text_start(),
            YulStatement::YulContinueStatement(child) => child.calculate_text_start(),
            YulStatement::YulVariableAssignmentStatement(child) => child.calculate_text_start(),
            YulStatement::YulVariableDeclarationStatement(child) => child.calculate_text_start(),
            YulStatement::YulExpression(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for YulSwitchCase {
    fn calculate_text_start(&self) -> Option<usize> {
        match self {
            YulSwitchCase::YulDefaultCase(child) => child.calculate_text_start(),
            YulSwitchCase::YulValueCase(child) => child.calculate_text_start(),
        }
    }
}

impl TextStart for ArrayValues {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for CallOptions {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for CatchClauses {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for ConstructorAttributes {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for ContractMembers {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for ContractSpecifiers {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for EnumMembers {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for ErrorParameters {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for EventParameters {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for FallbackFunctionAttributes {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for FunctionAttributes {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for FunctionTypeAttributes {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for HexStringLiterals {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for IdentifierPath {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for ImportDeconstructionSymbols {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for InheritanceTypes {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for InterfaceMembers {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for LibraryMembers {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for ModifierAttributes {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for MultiTypedDeclarationElements {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for NamedArguments {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for OverridePaths {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for Parameters {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for PositionalArguments {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for ReceiveFunctionAttributes {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for SimpleVersionLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for SourceUnitMembers {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for StateVariableAttributes {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for Statements {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for StringLiterals {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for StructMembers {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for TupleValues {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for UnicodeStringLiterals {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for UsingDeconstructionSymbols {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for VersionExpressionSet {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for VersionExpressionSets {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for YulArguments {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for YulFlags {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for YulParameters {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for YulPath {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for YulPaths {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for YulStatements {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for YulSwitchCases {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for YulVariableNames {
    fn calculate_text_start(&self) -> Option<usize> {
        self.elements
            .iter()
            .find_map(TextStart::calculate_text_start)
    }
}

impl TextStart for ABIEncoderV2Keyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AbicoderKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AbicoderV1Keyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AbicoderV2Keyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AbstractKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AddressKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AfterKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AliasKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Ampersand {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AmpersandAmpersand {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AmpersandEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AnonymousKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ApplyKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AsKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AssemblyKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Asterisk {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AsteriskAsterisk {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AsteriskEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AtKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for AutoKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Bang {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BangEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Bar {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BarBar {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BarEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BoolKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BreakKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ByteKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for BytesKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CallDataKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Caret {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CaretEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CaseKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CatchKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CloseBrace {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CloseBracket {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CloseParen {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Colon {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Comma {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ConstantKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ConstructorKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ContinueKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ContractKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for CopyOfKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DaysKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DecimalLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DefaultKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DefineKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DeleteKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for DoKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ElseKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EmitKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EnumKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Equal {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EqualEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EqualGreaterThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ErrorKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EtherKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for EventKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ExperimentalKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ExternalKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FallbackKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FalseKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FinalKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FixedKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ForKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FromKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for FunctionKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GlobalKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanGreaterThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanGreaterThanEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanGreaterThanGreaterThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GreaterThanGreaterThanGreaterThanEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for GweiKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for HexKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for HexLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for HexStringLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for HoursKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Identifier {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for IfKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ImmutableKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ImplementsKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ImportKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for InKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for IndexedKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for InlineKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for IntKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for InterfaceKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for InternalKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for IsKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LayoutKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LessThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LessThanEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LessThanLessThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LessThanLessThanEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LetKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for LibraryKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MacroKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MappingKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MatchKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MemoryKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Minus {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MinusEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MinusMinus {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MinutesKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ModifierKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for MutableKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for NewKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for NullKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OfKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OpenBrace {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OpenBracket {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OpenParen {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for OverrideKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PartialKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PayableKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Percent {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PercentEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Period {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Plus {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PlusEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PlusPlus {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaBarBar {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaCaret {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaGreaterThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaGreaterThanEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaLessThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaLessThanEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaMinus {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaPeriod {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaSemicolon {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaStringLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PragmaTilde {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PrivateKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PromiseKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PublicKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for PureKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for QuestionMark {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ReceiveKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ReferenceKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for RelocatableKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ReturnKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ReturnsKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for RevertKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SMTCheckerKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SealedKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SecondsKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Semicolon {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SizeOfKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Slash {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SlashEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SolidityKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StaticKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StorageKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StringKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StringLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for StructKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SuperKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SupportsKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for SwitchKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ThisKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ThrowKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for Tilde {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TransientKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TrueKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TryKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TypeDefKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TypeKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for TypeOfKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UfixedKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UintKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UncheckedKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UnicodeStringLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for UsingKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for VarKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for VersionSpecifier {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for ViewKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for VirtualKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for WeeksKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for WeiKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for WhileKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YearsKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulBreakKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulCaseKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulCloseBrace {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulCloseParen {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulColonEqual {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulComma {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulContinueKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulDecimalLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulDefaultKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulFalseKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulForKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulFunctionKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulHexKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulHexLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulHexStringLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulIdentifier {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulIfKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulLeaveKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulLetKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulMinusGreaterThan {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulOpenBrace {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulOpenParen {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulPeriod {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulStringLiteral {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulSuperKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulSwitchKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulThisKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}

impl TextStart for YulTrueKeyword {
    fn calculate_text_start(&self) -> Option<usize> {
        Some(self.range.start)
    }
}
