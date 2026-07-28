use slang_solidity_v2_common::nodes::NodeId;

use crate::ast::visitor::{Visitor, accept_expression, accept_statement};
use crate::ast::{
    Definition, Expression, ForStatementCondition, ForStatementStruct, Identifier,
    InequalityExpressionOperator, PostfixExpressionOperator, PrefixExpressionOperator,
    YulVariableAssignmentStatement,
};

impl ForStatementStruct {
    /// Whether the loop is the simple counter solc leaves unchecked under
    /// `simpleCounterForLoopUncheckedIncrement`: the condition stops it below the counter's
    /// own maximum, so the step cannot overflow. A backend that reports arithmetic overflow
    /// needs this to agree with solc on which `for` steps revert.
    pub fn is_simple_counter_loop(&self) -> bool {
        let ForStatementCondition::ExpressionStatement(condition) = self.condition() else {
            return false;
        };
        let Expression::InequalityExpression(comparison) = condition.expression() else {
            return false;
        };
        let increment = match self.iterator() {
            Some(Expression::PrefixExpression(step))
                if matches!(step.operator(), PrefixExpressionOperator::PlusPlus(_)) =>
            {
                step.operand()
            }
            Some(Expression::PostfixExpression(step))
                if matches!(step.operator(), PostfixExpressionOperator::PlusPlus(_)) =>
            {
                step.operand()
            }
            _ => return false,
        };
        let (
            InequalityExpressionOperator::LessThan(_),
            Expression::Identifier(counter),
            Expression::Identifier(increment),
        ) = (comparison.operator(), comparison.left_operand(), increment)
        else {
            return false;
        };
        let Some(definition @ (Definition::Variable(_) | Definition::Parameter(_))) =
            counter.resolve_to_definition()
        else {
            return false;
        };
        if increment
            .resolve_to_definition()
            .map(|other| other.node_id())
            != Some(definition.node_id())
        {
            return false;
        }
        let binder = self.semantic.binder();
        if binder.node_typing(counter.node_id()).as_type_id()
            != binder
                .common_operand_typing(comparison.node_id())
                .as_type_id()
        {
            return false;
        }
        let mut writes = CounterWrites {
            counter: definition.node_id(),
            found: false,
        };
        accept_expression(&comparison.right_operand(), &mut writes);
        accept_statement(&self.body(), &mut writes);
        !writes.found
    }
}

/// Searches for the counter writes that disqualify a simple counter loop. Nothing records
/// which references are written to, so the search is structural over the positions that
/// write their target: an assignment's left operand, an increment/decrement/`delete`
/// operand, and a Yul assignment naming the counter.
struct CounterWrites {
    counter: NodeId,
    found: bool,
}

impl CounterWrites {
    fn identifier(&mut self, identifier: &Identifier) {
        self.found |= identifier
            .resolve_to_definition()
            .map(|definition| definition.node_id())
            == Some(self.counter);
    }

    fn target(&mut self, target: &Expression) {
        match target {
            Expression::Identifier(identifier) => self.identifier(identifier),
            Expression::TupleExpression(tuple) => {
                for item in tuple.items().iter().filter_map(|item| item.expression()) {
                    self.target(&item);
                }
            }
            _ => {}
        }
    }
}

impl Visitor for CounterWrites {
    fn enter_expression(&mut self, node: &Expression) -> bool {
        match node {
            Expression::AssignmentExpression(assignment) => self.target(&assignment.left_operand()),
            Expression::PostfixExpression(step) => self.target(&step.operand()),
            Expression::PrefixExpression(step)
                if matches!(
                    step.operator(),
                    PrefixExpressionOperator::PlusPlus(_)
                        | PrefixExpressionOperator::MinusMinus(_)
                        | PrefixExpressionOperator::DeleteKeyword(_)
                ) =>
            {
                self.target(&step.operand());
            }
            _ => {}
        }
        true
    }

    fn enter_yul_variable_assignment_statement(
        &mut self,
        node: &YulVariableAssignmentStatement,
    ) -> bool {
        for path in node.variables().iter() {
            for identifier in path.iter() {
                self.identifier(&identifier);
            }
        }
        true
    }
}
