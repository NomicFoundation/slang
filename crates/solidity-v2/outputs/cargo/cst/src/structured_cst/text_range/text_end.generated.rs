// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use std::rc::Rc;

use crate::structured_cst::nodes::*;

/// A trait for CST nodes that can report the end offset of their text range.
pub(super) trait TextEnd {
    /// Returns the end offset of this node's text, or `None` if the node is empty.
    fn calculate_text_end(&self) -> Option<usize>;
}

impl<T: TextEnd> TextEnd for Option<T> {
    fn calculate_text_end(&self) -> Option<usize> {
        self.as_ref().and_then(TextEnd::calculate_text_end)
    }
}

impl<T: TextEnd> TextEnd for Rc<T> {
    fn calculate_text_end(&self) -> Option<usize> {
        (**self).calculate_text_end()
    }
}

impl TextEnd for AbicoderPragmaStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.version.calculate_text_end())
            .or_else(|| self.abicoder_keyword.calculate_text_end())
    }
}

impl TextEnd for AdditiveExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| {
                self.expression_additive_expression_operator
                    .calculate_text_end()
            })
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for AddressTypeStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.payable_keyword.calculate_text_end())
            .or_else(|| self.address_keyword.calculate_text_end())
    }
}

impl TextEnd for AndExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| self.operator.calculate_text_end())
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for ArrayExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_bracket.calculate_text_end())
            .or_else(|| self.items.calculate_text_end())
            .or_else(|| self.open_bracket.calculate_text_end())
    }
}

impl TextEnd for ArrayTypeNameStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_bracket.calculate_text_end())
            .or_else(|| self.index.calculate_text_end())
            .or_else(|| self.open_bracket.calculate_text_end())
            .or_else(|| self.operand.calculate_text_end())
    }
}

impl TextEnd for AssemblyStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.flags.calculate_text_end())
            .or_else(|| self.label.calculate_text_end())
            .or_else(|| self.assembly_keyword.calculate_text_end())
    }
}

impl TextEnd for AssignmentExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| {
                self.expression_assignment_expression_operator
                    .calculate_text_end()
            })
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for BitwiseAndExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| self.operator.calculate_text_end())
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for BitwiseOrExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| self.operator.calculate_text_end())
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for BitwiseXorExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| self.operator.calculate_text_end())
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for BlockStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.statements.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
    }
}

impl TextEnd for BreakStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.break_keyword.calculate_text_end())
    }
}

impl TextEnd for CallOptionsExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.options.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
            .or_else(|| self.operand.calculate_text_end())
    }
}

impl TextEnd for CatchClauseStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.error.calculate_text_end())
            .or_else(|| self.catch_keyword.calculate_text_end())
    }
}

impl TextEnd for CatchClauseErrorStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
    }
}

impl TextEnd for ConditionalExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.false_expression.calculate_text_end())
            .or_else(|| self.colon.calculate_text_end())
            .or_else(|| self.true_expression.calculate_text_end())
            .or_else(|| self.question_mark.calculate_text_end())
            .or_else(|| self.operand.calculate_text_end())
    }
}

impl TextEnd for ConstantDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.value.calculate_text_end())
            .or_else(|| self.equal.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.constant_keyword.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for ConstructorDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.attributes.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.constructor_keyword.calculate_text_end())
    }
}

impl TextEnd for ContinueStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.continue_keyword.calculate_text_end())
    }
}

impl TextEnd for ContractDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.members.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
            .or_else(|| self.specifiers.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.contract_keyword.calculate_text_end())
            .or_else(|| self.abstract_keyword.calculate_text_end())
    }
}

impl TextEnd for DecimalNumberExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.unit.calculate_text_end())
            .or_else(|| self.literal.calculate_text_end())
    }
}

impl TextEnd for DoWhileStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.condition.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
            .or_else(|| self.while_keyword.calculate_text_end())
            .or_else(|| self.body.calculate_text_end())
            .or_else(|| self.do_keyword.calculate_text_end())
    }
}

impl TextEnd for ElseBranchStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.else_keyword.calculate_text_end())
    }
}

impl TextEnd for EmitStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.event.calculate_text_end())
            .or_else(|| self.emit_keyword.calculate_text_end())
    }
}

impl TextEnd for EnumDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.members.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.enum_keyword.calculate_text_end())
    }
}

impl TextEnd for EqualityExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| {
                self.expression_equality_expression_operator
                    .calculate_text_end()
            })
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for ErrorDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.members.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.error_keyword.calculate_text_end())
    }
}

impl TextEnd for ErrorParameterStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for ErrorParametersDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for EventDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.anonymous_keyword.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.event_keyword.calculate_text_end())
    }
}

impl TextEnd for EventParameterStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_end())
            .or_else(|| self.indexed_keyword.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for EventParametersDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for ExperimentalPragmaStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.feature.calculate_text_end())
            .or_else(|| self.experimental_keyword.calculate_text_end())
    }
}

impl TextEnd for ExponentiationExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| self.operator.calculate_text_end())
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for ExpressionStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.expression.calculate_text_end())
    }
}

impl TextEnd for FallbackFunctionDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.returns.calculate_text_end())
            .or_else(|| self.attributes.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.fallback_keyword.calculate_text_end())
    }
}

impl TextEnd for ForStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.iterator.calculate_text_end())
            .or_else(|| self.condition.calculate_text_end())
            .or_else(|| self.initialization.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
            .or_else(|| self.for_keyword.calculate_text_end())
    }
}

impl TextEnd for FunctionCallExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.operand.calculate_text_end())
    }
}

impl TextEnd for FunctionDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.returns.calculate_text_end())
            .or_else(|| self.attributes.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.function_keyword.calculate_text_end())
    }
}

impl TextEnd for FunctionTypeStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.returns.calculate_text_end())
            .or_else(|| self.attributes.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.function_keyword.calculate_text_end())
    }
}

impl TextEnd for HexNumberExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.literal.calculate_text_end())
    }
}

impl TextEnd for IfStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.else_branch.calculate_text_end())
            .or_else(|| self.body.calculate_text_end())
            .or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.condition.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
            .or_else(|| self.if_keyword.calculate_text_end())
    }
}

impl TextEnd for ImportAliasStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.identifier.calculate_text_end())
            .or_else(|| self.as_keyword.calculate_text_end())
    }
}

impl TextEnd for ImportDeconstructionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.path.calculate_text_end())
            .or_else(|| self.from_keyword.calculate_text_end())
            .or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.symbols.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
    }
}

impl TextEnd for ImportDeconstructionSymbolStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.alias.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
    }
}

impl TextEnd for ImportDirectiveStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.clause.calculate_text_end())
            .or_else(|| self.import_keyword.calculate_text_end())
    }
}

impl TextEnd for IndexAccessEndStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.end.calculate_text_end())
            .or_else(|| self.colon.calculate_text_end())
    }
}

impl TextEnd for IndexAccessExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_bracket.calculate_text_end())
            .or_else(|| self.end.calculate_text_end())
            .or_else(|| self.start.calculate_text_end())
            .or_else(|| self.open_bracket.calculate_text_end())
            .or_else(|| self.operand.calculate_text_end())
    }
}

impl TextEnd for InequalityExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| {
                self.expression_inequality_expression_operator
                    .calculate_text_end()
            })
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for InheritanceSpecifierStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.types.calculate_text_end())
            .or_else(|| self.is_keyword.calculate_text_end())
    }
}

impl TextEnd for InheritanceTypeStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for InterfaceDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.members.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
            .or_else(|| self.inheritance.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.interface_keyword.calculate_text_end())
    }
}

impl TextEnd for LibraryDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.members.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.library_keyword.calculate_text_end())
    }
}

impl TextEnd for MappingKeyStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_end())
            .or_else(|| self.key_type.calculate_text_end())
    }
}

impl TextEnd for MappingTypeStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.value_type.calculate_text_end())
            .or_else(|| self.equal_greater_than.calculate_text_end())
            .or_else(|| self.key_type.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
            .or_else(|| self.mapping_keyword.calculate_text_end())
    }
}

impl TextEnd for MappingValueStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for MemberAccessExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.member.calculate_text_end())
            .or_else(|| self.period.calculate_text_end())
            .or_else(|| self.operand.calculate_text_end())
    }
}

impl TextEnd for ModifierDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.attributes.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.modifier_keyword.calculate_text_end())
    }
}

impl TextEnd for ModifierInvocationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
    }
}

impl TextEnd for MultiTypedDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.calculate_text_end())
            .or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.elements.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for MultiTypedDeclarationElementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.member.calculate_text_end())
    }
}

impl TextEnd for MultiplicativeExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| {
                self.expression_multiplicative_expression_operator
                    .calculate_text_end()
            })
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for NamedArgumentStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.calculate_text_end())
            .or_else(|| self.colon.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
    }
}

impl TextEnd for NamedArgumentGroupStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
    }
}

impl TextEnd for NamedArgumentsDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for NamedImportStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.path.calculate_text_end())
            .or_else(|| self.from_keyword.calculate_text_end())
            .or_else(|| self.alias.calculate_text_end())
            .or_else(|| self.asterisk.calculate_text_end())
    }
}

impl TextEnd for NewExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.type_name.calculate_text_end())
            .or_else(|| self.new_keyword.calculate_text_end())
    }
}

impl TextEnd for OrExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| self.operator.calculate_text_end())
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for OverridePathsDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.paths.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for OverrideSpecifierStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.overridden.calculate_text_end())
            .or_else(|| self.override_keyword.calculate_text_end())
    }
}

impl TextEnd for ParameterStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_end())
            .or_else(|| self.storage_location.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for ParametersDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for PathImportStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.alias.calculate_text_end())
            .or_else(|| self.path.calculate_text_end())
    }
}

impl TextEnd for PositionalArgumentsDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for PostfixExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| {
            self.expression_postfix_expression_operator
                .calculate_text_end()
        })
        .or_else(|| self.operand.calculate_text_end())
    }
}

impl TextEnd for PragmaDirectiveStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.pragma.calculate_text_end())
            .or_else(|| self.pragma_keyword.calculate_text_end())
    }
}

impl TextEnd for PrefixExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.operand.calculate_text_end())
            .or_else(|| {
                self.expression_prefix_expression_operator
                    .calculate_text_end()
            })
    }
}

impl TextEnd for ReceiveFunctionDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.attributes.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.receive_keyword.calculate_text_end())
    }
}

impl TextEnd for ReturnStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.expression.calculate_text_end())
            .or_else(|| self.return_keyword.calculate_text_end())
    }
}

impl TextEnd for ReturnsDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.variables.calculate_text_end())
            .or_else(|| self.returns_keyword.calculate_text_end())
    }
}

impl TextEnd for RevertStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.error.calculate_text_end())
            .or_else(|| self.revert_keyword.calculate_text_end())
    }
}

impl TextEnd for ShiftExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.right_operand.calculate_text_end())
            .or_else(|| {
                self.expression_shift_expression_operator
                    .calculate_text_end()
            })
            .or_else(|| self.left_operand.calculate_text_end())
    }
}

impl TextEnd for SingleTypedDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.calculate_text_end())
            .or_else(|| self.declaration.calculate_text_end())
    }
}

impl TextEnd for SourceUnitStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.members.calculate_text_end())
    }
}

impl TextEnd for StateVariableDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.value.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.attributes.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for StateVariableDefinitionValueStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.calculate_text_end())
            .or_else(|| self.equal.calculate_text_end())
    }
}

impl TextEnd for StorageLayoutSpecifierStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.calculate_text_end())
            .or_else(|| self.at_keyword.calculate_text_end())
            .or_else(|| self.layout_keyword.calculate_text_end())
    }
}

impl TextEnd for StructDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.members.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.struct_keyword.calculate_text_end())
    }
}

impl TextEnd for StructMemberStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for TryStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.catch_clauses.calculate_text_end())
            .or_else(|| self.body.calculate_text_end())
            .or_else(|| self.returns.calculate_text_end())
            .or_else(|| self.expression.calculate_text_end())
            .or_else(|| self.try_keyword.calculate_text_end())
    }
}

impl TextEnd for TupleExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.items.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for TupleValueStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.calculate_text_end())
    }
}

impl TextEnd for TypeExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
            .or_else(|| self.type_keyword.calculate_text_end())
    }
}

impl TextEnd for UncheckedBlockStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.block.calculate_text_end())
            .or_else(|| self.unchecked_keyword.calculate_text_end())
    }
}

impl TextEnd for UserDefinedValueTypeDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.value_type.calculate_text_end())
            .or_else(|| self.is_keyword.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.type_keyword.calculate_text_end())
    }
}

impl TextEnd for UsingAliasStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.operator.calculate_text_end())
            .or_else(|| self.as_keyword.calculate_text_end())
    }
}

impl TextEnd for UsingDeconstructionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.symbols.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
    }
}

impl TextEnd for UsingDeconstructionSymbolStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.alias.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
    }
}

impl TextEnd for UsingDirectiveStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.global_keyword.calculate_text_end())
            .or_else(|| self.target.calculate_text_end())
            .or_else(|| self.for_keyword.calculate_text_end())
            .or_else(|| self.clause.calculate_text_end())
            .or_else(|| self.using_keyword.calculate_text_end())
    }
}

impl TextEnd for VariableDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.name.calculate_text_end())
            .or_else(|| self.storage_location.calculate_text_end())
            .or_else(|| self.type_name.calculate_text_end())
    }
}

impl TextEnd for VariableDeclarationStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.semicolon.calculate_text_end())
            .or_else(|| self.target.calculate_text_end())
    }
}

impl TextEnd for VariableDeclarationValueStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.calculate_text_end())
            .or_else(|| self.equal.calculate_text_end())
    }
}

impl TextEnd for VersionPragmaStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.sets.calculate_text_end())
            .or_else(|| self.solidity_keyword.calculate_text_end())
    }
}

impl TextEnd for VersionRangeStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.end.calculate_text_end())
            .or_else(|| self.minus.calculate_text_end())
            .or_else(|| self.start.calculate_text_end())
    }
}

impl TextEnd for VersionTermStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.literal.calculate_text_end())
            .or_else(|| self.operator.calculate_text_end())
    }
}

impl TextEnd for WhileStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.condition.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
            .or_else(|| self.while_keyword.calculate_text_end())
    }
}

impl TextEnd for YulBlockStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_brace.calculate_text_end())
            .or_else(|| self.statements.calculate_text_end())
            .or_else(|| self.open_brace.calculate_text_end())
    }
}

impl TextEnd for YulBreakStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.break_keyword.calculate_text_end())
    }
}

impl TextEnd for YulContinueStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.continue_keyword.calculate_text_end())
    }
}

impl TextEnd for YulDefaultCaseStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.default_keyword.calculate_text_end())
    }
}

impl TextEnd for YulFlagsDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.flags.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for YulForStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.iterator.calculate_text_end())
            .or_else(|| self.condition.calculate_text_end())
            .or_else(|| self.initialization.calculate_text_end())
            .or_else(|| self.for_keyword.calculate_text_end())
    }
}

impl TextEnd for YulFunctionCallExpressionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.arguments.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
            .or_else(|| self.operand.calculate_text_end())
    }
}

impl TextEnd for YulFunctionDefinitionStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.returns.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.name.calculate_text_end())
            .or_else(|| self.function_keyword.calculate_text_end())
    }
}

impl TextEnd for YulIfStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.condition.calculate_text_end())
            .or_else(|| self.if_keyword.calculate_text_end())
    }
}

impl TextEnd for YulLeaveStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.leave_keyword.calculate_text_end())
    }
}

impl TextEnd for YulParametersDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.close_paren.calculate_text_end())
            .or_else(|| self.parameters.calculate_text_end())
            .or_else(|| self.open_paren.calculate_text_end())
    }
}

impl TextEnd for YulReturnsDeclarationStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.variables.calculate_text_end())
            .or_else(|| self.minus_greater_than.calculate_text_end())
    }
}

impl TextEnd for YulSwitchStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.cases.calculate_text_end())
            .or_else(|| self.expression.calculate_text_end())
            .or_else(|| self.switch_keyword.calculate_text_end())
    }
}

impl TextEnd for YulValueCaseStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.body.calculate_text_end())
            .or_else(|| self.value.calculate_text_end())
            .or_else(|| self.case_keyword.calculate_text_end())
    }
}

impl TextEnd for YulVariableAssignmentStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.calculate_text_end())
            .or_else(|| self.assignment.calculate_text_end())
            .or_else(|| self.variables.calculate_text_end())
    }
}

impl TextEnd for YulVariableDeclarationStatementStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.value.calculate_text_end())
            .or_else(|| self.variables.calculate_text_end())
            .or_else(|| self.let_keyword.calculate_text_end())
    }
}

impl TextEnd for YulVariableDeclarationValueStruct {
    fn calculate_text_end(&self) -> Option<usize> {
        None.or_else(|| self.expression.calculate_text_end())
            .or_else(|| self.assignment.calculate_text_end())
    }
}

impl TextEnd for AbicoderVersion {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            AbicoderVersion::AbicoderV1Keyword(child) => child.calculate_text_end(),
            AbicoderVersion::AbicoderV2Keyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ArgumentsDeclaration {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ArgumentsDeclaration::PositionalArgumentsDeclaration(child) => {
                child.calculate_text_end()
            }
            ArgumentsDeclaration::NamedArgumentsDeclaration(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ConstructorAttribute {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ConstructorAttribute::ModifierInvocation(child) => child.calculate_text_end(),
            ConstructorAttribute::InternalKeyword(child) => child.calculate_text_end(),
            ConstructorAttribute::PayableKeyword(child) => child.calculate_text_end(),
            ConstructorAttribute::PublicKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ContractMember {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ContractMember::UsingDirective(child) => child.calculate_text_end(),
            ContractMember::FunctionDefinition(child) => child.calculate_text_end(),
            ContractMember::ConstructorDefinition(child) => child.calculate_text_end(),
            ContractMember::ReceiveFunctionDefinition(child) => child.calculate_text_end(),
            ContractMember::FallbackFunctionDefinition(child) => child.calculate_text_end(),
            ContractMember::ModifierDefinition(child) => child.calculate_text_end(),
            ContractMember::StructDefinition(child) => child.calculate_text_end(),
            ContractMember::EnumDefinition(child) => child.calculate_text_end(),
            ContractMember::EventDefinition(child) => child.calculate_text_end(),
            ContractMember::ErrorDefinition(child) => child.calculate_text_end(),
            ContractMember::UserDefinedValueTypeDefinition(child) => child.calculate_text_end(),
            ContractMember::StateVariableDefinition(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ContractSpecifier {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ContractSpecifier::InheritanceSpecifier(child) => child.calculate_text_end(),
            ContractSpecifier::StorageLayoutSpecifier(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ElementaryType {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ElementaryType::BoolKeyword(child) => child.calculate_text_end(),
            ElementaryType::StringKeyword(child) => child.calculate_text_end(),
            ElementaryType::AddressType(child) => child.calculate_text_end(),
            ElementaryType::BytesKeyword(child) => child.calculate_text_end(),
            ElementaryType::IntKeyword(child) => child.calculate_text_end(),
            ElementaryType::UintKeyword(child) => child.calculate_text_end(),
            ElementaryType::FixedKeyword(child) => child.calculate_text_end(),
            ElementaryType::UfixedKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ExperimentalFeature {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ExperimentalFeature::ABIEncoderV2Keyword(child) => child.calculate_text_end(),
            ExperimentalFeature::SMTCheckerKeyword(child) => child.calculate_text_end(),
            ExperimentalFeature::PragmaStringLiteral(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for Expression {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression::AssignmentExpression(child) => child.calculate_text_end(),
            Expression::ConditionalExpression(child) => child.calculate_text_end(),
            Expression::OrExpression(child) => child.calculate_text_end(),
            Expression::AndExpression(child) => child.calculate_text_end(),
            Expression::EqualityExpression(child) => child.calculate_text_end(),
            Expression::InequalityExpression(child) => child.calculate_text_end(),
            Expression::BitwiseOrExpression(child) => child.calculate_text_end(),
            Expression::BitwiseXorExpression(child) => child.calculate_text_end(),
            Expression::BitwiseAndExpression(child) => child.calculate_text_end(),
            Expression::ShiftExpression(child) => child.calculate_text_end(),
            Expression::AdditiveExpression(child) => child.calculate_text_end(),
            Expression::MultiplicativeExpression(child) => child.calculate_text_end(),
            Expression::ExponentiationExpression(child) => child.calculate_text_end(),
            Expression::PostfixExpression(child) => child.calculate_text_end(),
            Expression::PrefixExpression(child) => child.calculate_text_end(),
            Expression::FunctionCallExpression(child) => child.calculate_text_end(),
            Expression::CallOptionsExpression(child) => child.calculate_text_end(),
            Expression::MemberAccessExpression(child) => child.calculate_text_end(),
            Expression::IndexAccessExpression(child) => child.calculate_text_end(),
            Expression::NewExpression(child) => child.calculate_text_end(),
            Expression::TupleExpression(child) => child.calculate_text_end(),
            Expression::TypeExpression(child) => child.calculate_text_end(),
            Expression::ArrayExpression(child) => child.calculate_text_end(),
            Expression::HexNumberExpression(child) => child.calculate_text_end(),
            Expression::DecimalNumberExpression(child) => child.calculate_text_end(),
            Expression::StringExpression(child) => child.calculate_text_end(),
            Expression::ElementaryType(child) => child.calculate_text_end(),
            Expression::PayableKeyword(child) => child.calculate_text_end(),
            Expression::ThisKeyword(child) => child.calculate_text_end(),
            Expression::SuperKeyword(child) => child.calculate_text_end(),
            Expression::TrueKeyword(child) => child.calculate_text_end(),
            Expression::FalseKeyword(child) => child.calculate_text_end(),
            Expression::Identifier(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for Expression_AdditiveExpression_Operator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression_AdditiveExpression_Operator::Minus(child) => child.calculate_text_end(),
            Expression_AdditiveExpression_Operator::Plus(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for Expression_AssignmentExpression_Operator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression_AssignmentExpression_Operator::AmpersandEqual(child) => {
                child.calculate_text_end()
            }
            Expression_AssignmentExpression_Operator::AsteriskEqual(child) => {
                child.calculate_text_end()
            }
            Expression_AssignmentExpression_Operator::BarEqual(child) => child.calculate_text_end(),
            Expression_AssignmentExpression_Operator::CaretEqual(child) => {
                child.calculate_text_end()
            }
            Expression_AssignmentExpression_Operator::Equal(child) => child.calculate_text_end(),
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(child) => {
                child.calculate_text_end()
            }
            Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(
                child,
            ) => child.calculate_text_end(),
            Expression_AssignmentExpression_Operator::LessThanLessThanEqual(child) => {
                child.calculate_text_end()
            }
            Expression_AssignmentExpression_Operator::MinusEqual(child) => {
                child.calculate_text_end()
            }
            Expression_AssignmentExpression_Operator::PercentEqual(child) => {
                child.calculate_text_end()
            }
            Expression_AssignmentExpression_Operator::PlusEqual(child) => {
                child.calculate_text_end()
            }
            Expression_AssignmentExpression_Operator::SlashEqual(child) => {
                child.calculate_text_end()
            }
        }
    }
}

impl TextEnd for Expression_EqualityExpression_Operator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression_EqualityExpression_Operator::BangEqual(child) => child.calculate_text_end(),
            Expression_EqualityExpression_Operator::EqualEqual(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for Expression_InequalityExpression_Operator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression_InequalityExpression_Operator::GreaterThan(child) => {
                child.calculate_text_end()
            }
            Expression_InequalityExpression_Operator::GreaterThanEqual(child) => {
                child.calculate_text_end()
            }
            Expression_InequalityExpression_Operator::LessThan(child) => child.calculate_text_end(),
            Expression_InequalityExpression_Operator::LessThanEqual(child) => {
                child.calculate_text_end()
            }
        }
    }
}

impl TextEnd for Expression_MultiplicativeExpression_Operator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression_MultiplicativeExpression_Operator::Asterisk(child) => {
                child.calculate_text_end()
            }
            Expression_MultiplicativeExpression_Operator::Percent(child) => {
                child.calculate_text_end()
            }
            Expression_MultiplicativeExpression_Operator::Slash(child) => {
                child.calculate_text_end()
            }
        }
    }
}

impl TextEnd for Expression_PostfixExpression_Operator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression_PostfixExpression_Operator::MinusMinus(child) => child.calculate_text_end(),
            Expression_PostfixExpression_Operator::PlusPlus(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for Expression_PrefixExpression_Operator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression_PrefixExpression_Operator::Bang(child) => child.calculate_text_end(),
            Expression_PrefixExpression_Operator::DeleteKeyword(child) => {
                child.calculate_text_end()
            }
            Expression_PrefixExpression_Operator::Minus(child) => child.calculate_text_end(),
            Expression_PrefixExpression_Operator::MinusMinus(child) => child.calculate_text_end(),
            Expression_PrefixExpression_Operator::PlusPlus(child) => child.calculate_text_end(),
            Expression_PrefixExpression_Operator::Tilde(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for Expression_ShiftExpression_Operator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Expression_ShiftExpression_Operator::GreaterThanGreaterThan(child) => {
                child.calculate_text_end()
            }
            Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(child) => {
                child.calculate_text_end()
            }
            Expression_ShiftExpression_Operator::LessThanLessThan(child) => {
                child.calculate_text_end()
            }
        }
    }
}

impl TextEnd for FallbackFunctionAttribute {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            FallbackFunctionAttribute::ModifierInvocation(child) => child.calculate_text_end(),
            FallbackFunctionAttribute::OverrideSpecifier(child) => child.calculate_text_end(),
            FallbackFunctionAttribute::ExternalKeyword(child) => child.calculate_text_end(),
            FallbackFunctionAttribute::PayableKeyword(child) => child.calculate_text_end(),
            FallbackFunctionAttribute::PureKeyword(child) => child.calculate_text_end(),
            FallbackFunctionAttribute::ViewKeyword(child) => child.calculate_text_end(),
            FallbackFunctionAttribute::VirtualKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ForStatementCondition {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ForStatementCondition::ExpressionStatement(child) => child.calculate_text_end(),
            ForStatementCondition::Semicolon(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ForStatementInitialization {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(child) => {
                child.calculate_text_end()
            }
            ForStatementInitialization::ExpressionStatement(child) => child.calculate_text_end(),
            ForStatementInitialization::Semicolon(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for FunctionAttribute {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            FunctionAttribute::ModifierInvocation(child) => child.calculate_text_end(),
            FunctionAttribute::OverrideSpecifier(child) => child.calculate_text_end(),
            FunctionAttribute::ExternalKeyword(child) => child.calculate_text_end(),
            FunctionAttribute::InternalKeyword(child) => child.calculate_text_end(),
            FunctionAttribute::PayableKeyword(child) => child.calculate_text_end(),
            FunctionAttribute::PrivateKeyword(child) => child.calculate_text_end(),
            FunctionAttribute::PublicKeyword(child) => child.calculate_text_end(),
            FunctionAttribute::PureKeyword(child) => child.calculate_text_end(),
            FunctionAttribute::ViewKeyword(child) => child.calculate_text_end(),
            FunctionAttribute::VirtualKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for FunctionBody {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            FunctionBody::Block(child) => child.calculate_text_end(),
            FunctionBody::Semicolon(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for FunctionName {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            FunctionName::Identifier(child) => child.calculate_text_end(),
            FunctionName::FallbackKeyword(child) => child.calculate_text_end(),
            FunctionName::ReceiveKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for FunctionTypeAttribute {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            FunctionTypeAttribute::InternalKeyword(child) => child.calculate_text_end(),
            FunctionTypeAttribute::ExternalKeyword(child) => child.calculate_text_end(),
            FunctionTypeAttribute::PrivateKeyword(child) => child.calculate_text_end(),
            FunctionTypeAttribute::PublicKeyword(child) => child.calculate_text_end(),
            FunctionTypeAttribute::PureKeyword(child) => child.calculate_text_end(),
            FunctionTypeAttribute::ViewKeyword(child) => child.calculate_text_end(),
            FunctionTypeAttribute::PayableKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for IdentifierPathElement {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            IdentifierPathElement::Identifier(child) => child.calculate_text_end(),
            IdentifierPathElement::AddressKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ImportClause {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ImportClause::PathImport(child) => child.calculate_text_end(),
            ImportClause::NamedImport(child) => child.calculate_text_end(),
            ImportClause::ImportDeconstruction(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for MappingKeyType {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            MappingKeyType::ElementaryType(child) => child.calculate_text_end(),
            MappingKeyType::IdentifierPath(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ModifierAttribute {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ModifierAttribute::OverrideSpecifier(child) => child.calculate_text_end(),
            ModifierAttribute::VirtualKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for NumberUnit {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            NumberUnit::WeiKeyword(child) => child.calculate_text_end(),
            NumberUnit::GweiKeyword(child) => child.calculate_text_end(),
            NumberUnit::EtherKeyword(child) => child.calculate_text_end(),
            NumberUnit::SecondsKeyword(child) => child.calculate_text_end(),
            NumberUnit::MinutesKeyword(child) => child.calculate_text_end(),
            NumberUnit::HoursKeyword(child) => child.calculate_text_end(),
            NumberUnit::DaysKeyword(child) => child.calculate_text_end(),
            NumberUnit::WeeksKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for Pragma {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Pragma::VersionPragma(child) => child.calculate_text_end(),
            Pragma::AbicoderPragma(child) => child.calculate_text_end(),
            Pragma::ExperimentalPragma(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ReceiveFunctionAttribute {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            ReceiveFunctionAttribute::ModifierInvocation(child) => child.calculate_text_end(),
            ReceiveFunctionAttribute::OverrideSpecifier(child) => child.calculate_text_end(),
            ReceiveFunctionAttribute::ExternalKeyword(child) => child.calculate_text_end(),
            ReceiveFunctionAttribute::PayableKeyword(child) => child.calculate_text_end(),
            ReceiveFunctionAttribute::VirtualKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for SourceUnitMember {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            SourceUnitMember::PragmaDirective(child) => child.calculate_text_end(),
            SourceUnitMember::ImportDirective(child) => child.calculate_text_end(),
            SourceUnitMember::ContractDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::InterfaceDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::LibraryDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::StructDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::EnumDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::FunctionDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::ErrorDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::UserDefinedValueTypeDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::UsingDirective(child) => child.calculate_text_end(),
            SourceUnitMember::EventDefinition(child) => child.calculate_text_end(),
            SourceUnitMember::ConstantDefinition(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for StateVariableAttribute {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            StateVariableAttribute::OverrideSpecifier(child) => child.calculate_text_end(),
            StateVariableAttribute::ConstantKeyword(child) => child.calculate_text_end(),
            StateVariableAttribute::InternalKeyword(child) => child.calculate_text_end(),
            StateVariableAttribute::PrivateKeyword(child) => child.calculate_text_end(),
            StateVariableAttribute::PublicKeyword(child) => child.calculate_text_end(),
            StateVariableAttribute::ImmutableKeyword(child) => child.calculate_text_end(),
            StateVariableAttribute::TransientKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for Statement {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            Statement::IfStatement(child) => child.calculate_text_end(),
            Statement::ForStatement(child) => child.calculate_text_end(),
            Statement::WhileStatement(child) => child.calculate_text_end(),
            Statement::DoWhileStatement(child) => child.calculate_text_end(),
            Statement::ContinueStatement(child) => child.calculate_text_end(),
            Statement::BreakStatement(child) => child.calculate_text_end(),
            Statement::ReturnStatement(child) => child.calculate_text_end(),
            Statement::EmitStatement(child) => child.calculate_text_end(),
            Statement::TryStatement(child) => child.calculate_text_end(),
            Statement::RevertStatement(child) => child.calculate_text_end(),
            Statement::AssemblyStatement(child) => child.calculate_text_end(),
            Statement::Block(child) => child.calculate_text_end(),
            Statement::UncheckedBlock(child) => child.calculate_text_end(),
            Statement::VariableDeclarationStatement(child) => child.calculate_text_end(),
            Statement::ExpressionStatement(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for StorageLocation {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            StorageLocation::MemoryKeyword(child) => child.calculate_text_end(),
            StorageLocation::StorageKeyword(child) => child.calculate_text_end(),
            StorageLocation::CallDataKeyword(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for StringExpression {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            StringExpression::StringLiterals(child) => child.calculate_text_end(),
            StringExpression::HexStringLiterals(child) => child.calculate_text_end(),
            StringExpression::UnicodeStringLiterals(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for TypeName {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            TypeName::ArrayTypeName(child) => child.calculate_text_end(),
            TypeName::FunctionType(child) => child.calculate_text_end(),
            TypeName::MappingType(child) => child.calculate_text_end(),
            TypeName::ElementaryType(child) => child.calculate_text_end(),
            TypeName::IdentifierPath(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for UsingClause {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            UsingClause::IdentifierPath(child) => child.calculate_text_end(),
            UsingClause::UsingDeconstruction(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for UsingOperator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            UsingOperator::Ampersand(child) => child.calculate_text_end(),
            UsingOperator::Asterisk(child) => child.calculate_text_end(),
            UsingOperator::BangEqual(child) => child.calculate_text_end(),
            UsingOperator::Bar(child) => child.calculate_text_end(),
            UsingOperator::Caret(child) => child.calculate_text_end(),
            UsingOperator::EqualEqual(child) => child.calculate_text_end(),
            UsingOperator::GreaterThan(child) => child.calculate_text_end(),
            UsingOperator::GreaterThanEqual(child) => child.calculate_text_end(),
            UsingOperator::LessThan(child) => child.calculate_text_end(),
            UsingOperator::LessThanEqual(child) => child.calculate_text_end(),
            UsingOperator::Minus(child) => child.calculate_text_end(),
            UsingOperator::Percent(child) => child.calculate_text_end(),
            UsingOperator::Plus(child) => child.calculate_text_end(),
            UsingOperator::Slash(child) => child.calculate_text_end(),
            UsingOperator::Tilde(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for UsingTarget {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            UsingTarget::TypeName(child) => child.calculate_text_end(),
            UsingTarget::Asterisk(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for VariableDeclarationTarget {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(child) => child.calculate_text_end(),
            VariableDeclarationTarget::MultiTypedDeclaration(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for VersionExpression {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            VersionExpression::VersionRange(child) => child.calculate_text_end(),
            VersionExpression::VersionTerm(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for VersionLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            VersionLiteral::SimpleVersionLiteral(child) => child.calculate_text_end(),
            VersionLiteral::PragmaStringLiteral(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for VersionOperator {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            VersionOperator::PragmaCaret(child) => child.calculate_text_end(),
            VersionOperator::PragmaTilde(child) => child.calculate_text_end(),
            VersionOperator::PragmaEqual(child) => child.calculate_text_end(),
            VersionOperator::PragmaLessThan(child) => child.calculate_text_end(),
            VersionOperator::PragmaGreaterThan(child) => child.calculate_text_end(),
            VersionOperator::PragmaLessThanEqual(child) => child.calculate_text_end(),
            VersionOperator::PragmaGreaterThanEqual(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for YulExpression {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            YulExpression::YulFunctionCallExpression(child) => child.calculate_text_end(),
            YulExpression::YulLiteral(child) => child.calculate_text_end(),
            YulExpression::YulPath(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for YulLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            YulLiteral::YulTrueKeyword(child) => child.calculate_text_end(),
            YulLiteral::YulFalseKeyword(child) => child.calculate_text_end(),
            YulLiteral::YulDecimalLiteral(child) => child.calculate_text_end(),
            YulLiteral::YulHexLiteral(child) => child.calculate_text_end(),
            YulLiteral::YulHexStringLiteral(child) => child.calculate_text_end(),
            YulLiteral::YulStringLiteral(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for YulStatement {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            YulStatement::YulBlock(child) => child.calculate_text_end(),
            YulStatement::YulFunctionDefinition(child) => child.calculate_text_end(),
            YulStatement::YulIfStatement(child) => child.calculate_text_end(),
            YulStatement::YulForStatement(child) => child.calculate_text_end(),
            YulStatement::YulSwitchStatement(child) => child.calculate_text_end(),
            YulStatement::YulLeaveStatement(child) => child.calculate_text_end(),
            YulStatement::YulBreakStatement(child) => child.calculate_text_end(),
            YulStatement::YulContinueStatement(child) => child.calculate_text_end(),
            YulStatement::YulVariableAssignmentStatement(child) => child.calculate_text_end(),
            YulStatement::YulVariableDeclarationStatement(child) => child.calculate_text_end(),
            YulStatement::YulExpression(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for YulSwitchCase {
    fn calculate_text_end(&self) -> Option<usize> {
        match self {
            YulSwitchCase::YulDefaultCase(child) => child.calculate_text_end(),
            YulSwitchCase::YulValueCase(child) => child.calculate_text_end(),
        }
    }
}

impl TextEnd for ArrayValues {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for CallOptions {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for CatchClauses {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for ConstructorAttributes {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for ContractMembers {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for ContractSpecifiers {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for EnumMembers {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for ErrorParameters {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for EventParameters {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for FallbackFunctionAttributes {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for FunctionAttributes {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for FunctionTypeAttributes {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for HexStringLiterals {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for IdentifierPath {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for ImportDeconstructionSymbols {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for InheritanceTypes {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for InterfaceMembers {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for LibraryMembers {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for ModifierAttributes {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for MultiTypedDeclarationElements {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for NamedArguments {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for OverridePaths {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for Parameters {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for PositionalArguments {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for ReceiveFunctionAttributes {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for SimpleVersionLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for SourceUnitMembers {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for StateVariableAttributes {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for Statements {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for StringLiterals {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for StructMembers {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for TupleValues {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for UnicodeStringLiterals {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for UsingDeconstructionSymbols {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for VersionExpressionSet {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for VersionExpressionSets {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for YulArguments {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for YulFlags {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for YulParameters {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for YulPath {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for YulPaths {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for YulStatements {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for YulSwitchCases {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for YulVariableNames {
    fn calculate_text_end(&self) -> Option<usize> {
        self.elements
            .iter()
            .rev()
            .find_map(TextEnd::calculate_text_end)
    }
}

impl TextEnd for ABIEncoderV2Keyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AbicoderKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AbicoderV1Keyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AbicoderV2Keyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AbstractKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AddressKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AfterKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AliasKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Ampersand {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AmpersandAmpersand {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AmpersandEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AnonymousKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ApplyKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AsKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AssemblyKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Asterisk {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AsteriskAsterisk {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AsteriskEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AtKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for AutoKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Bang {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BangEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Bar {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BarBar {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BarEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BoolKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BreakKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ByteKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for BytesKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CallDataKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Caret {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CaretEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CaseKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CatchKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CloseBrace {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CloseBracket {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CloseParen {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Colon {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Comma {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ConstantKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ConstructorKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ContinueKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ContractKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for CopyOfKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DaysKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DecimalLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DefaultKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DefineKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DeleteKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for DoKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ElseKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EmitKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EnumKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Equal {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EqualEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EqualGreaterThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ErrorKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EtherKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for EventKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ExperimentalKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ExternalKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FallbackKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FalseKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FinalKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FixedKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ForKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FromKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for FunctionKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GlobalKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanGreaterThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanGreaterThanEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanGreaterThanGreaterThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GreaterThanGreaterThanGreaterThanEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for GweiKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for HexKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for HexLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for HexStringLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for HoursKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Identifier {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for IfKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ImmutableKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ImplementsKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ImportKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for InKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for IndexedKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for InlineKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for IntKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for InterfaceKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for InternalKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for IsKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LayoutKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LessThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LessThanEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LessThanLessThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LessThanLessThanEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LetKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for LibraryKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MacroKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MappingKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MatchKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MemoryKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Minus {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MinusEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MinusMinus {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MinutesKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ModifierKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for MutableKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for NewKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for NullKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OfKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OpenBrace {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OpenBracket {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OpenParen {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for OverrideKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PartialKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PayableKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Percent {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PercentEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Period {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Plus {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PlusEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PlusPlus {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaBarBar {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaCaret {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaGreaterThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaGreaterThanEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaLessThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaLessThanEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaMinus {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaPeriod {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaSemicolon {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaStringLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PragmaTilde {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PrivateKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PromiseKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PublicKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for PureKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for QuestionMark {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ReceiveKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ReferenceKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for RelocatableKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ReturnKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ReturnsKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for RevertKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SMTCheckerKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SealedKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SecondsKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Semicolon {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SizeOfKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Slash {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SlashEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SolidityKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StaticKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StorageKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StringKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StringLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for StructKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SuperKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SupportsKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for SwitchKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ThisKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ThrowKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for Tilde {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TransientKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TrueKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TryKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TypeDefKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TypeKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for TypeOfKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UfixedKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UintKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UncheckedKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UnicodeStringLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for UsingKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for VarKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for VersionSpecifier {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for ViewKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for VirtualKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for WeeksKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for WeiKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for WhileKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YearsKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulBreakKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulCaseKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulCloseBrace {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulCloseParen {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulColonEqual {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulComma {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulContinueKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulDecimalLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulDefaultKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulFalseKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulForKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulFunctionKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulHexKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulHexLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulHexStringLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulIdentifier {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulIfKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulLeaveKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulLetKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulMinusGreaterThan {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulOpenBrace {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulOpenParen {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulPeriod {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulStringLiteral {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulSuperKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulSwitchKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulThisKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}

impl TextEnd for YulTrueKeyword {
    fn calculate_text_end(&self) -> Option<usize> {
        Some(self.range.end)
    }
}
