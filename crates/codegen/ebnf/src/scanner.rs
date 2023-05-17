use codegen_schema::types::{ScannerDefinition, ScannerRef};

use crate::{nodes::EbnfNode, serialization::GenerateEbnf};

impl GenerateEbnf for ScannerRef {
    fn generate_ebnf(&self) -> EbnfNode {
        return self.definition.generate_ebnf();
    }
}

impl GenerateEbnf for ScannerDefinition {
    fn generate_ebnf(&self) -> EbnfNode {
        match &self {
            ScannerDefinition::Choice(choices) => {
                return EbnfNode::choices(
                    choices
                        .iter()
                        .map(|choice| choice.definition.generate_ebnf())
                        .collect(),
                );
            }

            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                return EbnfNode::difference(
                    minuend.definition.generate_ebnf(),
                    subtrahend.definition.generate_ebnf(),
                );
            }

            ScannerDefinition::Not(sub_expr) => {
                return EbnfNode::not(sub_expr.definition.generate_ebnf());
            }

            ScannerDefinition::OneOrMore(expr) => {
                return EbnfNode::one_or_more(expr.generate_ebnf());
            }

            ScannerDefinition::Optional(expr) => {
                return EbnfNode::optional(expr.generate_ebnf());
            }

            ScannerDefinition::Range { from, to } => {
                return EbnfNode::range(*from, *to);
            }

            ScannerDefinition::Reference(name) => {
                return EbnfNode::production_ref(name.to_owned());
            }

            ScannerDefinition::Repeat {
                min,
                max,
                expression,
            } => {
                return EbnfNode::repeat(*min, *max, expression.generate_ebnf());
            }

            ScannerDefinition::Sequence(elements) => {
                return EbnfNode::sequence(
                    elements
                        .iter()
                        .map(|element| element.definition.generate_ebnf())
                        .collect(),
                );
            }

            ScannerDefinition::Terminal(string) => {
                return EbnfNode::terminal(string.to_owned());
            }

            ScannerDefinition::TrailingContext { expression, .. } => {
                return expression.generate_ebnf();
            }

            ScannerDefinition::ZeroOrMore(expr) => {
                return EbnfNode::zero_or_more(expr.generate_ebnf());
            }
        };
    }
}
