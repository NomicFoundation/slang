use codegen_schema::types::scanner::{ScannerDefinition, ScannerRef};

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
                return EbnfNode::alternatives(
                    choices
                        .iter()
                        .map(|choice| generate_nested(self, &choice.definition))
                        .collect(),
                );
            }

            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                return EbnfNode::difference(
                    generate_nested(self, &minuend.definition),
                    generate_nested(self, &subtrahend.definition),
                );
            }

            ScannerDefinition::Not(sub_expr) => {
                return EbnfNode::not(generate_nested(self, &sub_expr.definition));
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
                return EbnfNode::reference(name.to_owned());
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
                        .map(|element| generate_nested(self, &element.definition))
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

fn generate_nested(
    parent_definition: &ScannerDefinition,
    scanner_definition: &ScannerDefinition,
) -> EbnfNode {
    if precedence(parent_definition) < precedence(scanner_definition) {
        return EbnfNode::parenthesis(scanner_definition.generate_ebnf());
    } else {
        return scanner_definition.generate_ebnf();
    }
}

fn precedence(scanner_definition: &ScannerDefinition) -> u8 {
    match scanner_definition {
        ScannerDefinition::OneOrMore(..)
        | ScannerDefinition::Optional(..)
        | ScannerDefinition::Range { .. }
        | ScannerDefinition::Reference(..)
        | ScannerDefinition::Repeat { .. }
        | ScannerDefinition::Terminal(..)
        | ScannerDefinition::ZeroOrMore(..) => 0,
        ScannerDefinition::Not(..) => 1,
        ScannerDefinition::Difference { .. } => 2,
        ScannerDefinition::Sequence(..) | ScannerDefinition::TrailingContext { .. } => 3,
        ScannerDefinition::Choice(..) => 4,
    }
}
