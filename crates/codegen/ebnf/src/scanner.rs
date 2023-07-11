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
            ScannerDefinition::Choice(scanners) => {
                return EbnfNode::choice(
                    scanners
                        .iter()
                        .map(|scanner| scanner.generate_ebnf())
                        .collect(),
                );
            }

            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                return EbnfNode::difference(minuend.generate_ebnf(), subtrahend.generate_ebnf());
            }

            ScannerDefinition::Not(scanner) => {
                return EbnfNode::not(scanner.generate_ebnf());
            }

            ScannerDefinition::OneOrMore(scanner) => {
                return EbnfNode::one_or_more(scanner.generate_ebnf());
            }

            ScannerDefinition::Optional(scanner) => {
                return EbnfNode::optional(scanner.generate_ebnf());
            }

            ScannerDefinition::Range { from, to } => {
                return EbnfNode::range(*from, *to);
            }

            ScannerDefinition::Reference(name) => {
                return EbnfNode::production_ref(name.to_owned());
            }

            ScannerDefinition::Sequence(scanners) => {
                return EbnfNode::sequence(
                    scanners
                        .iter()
                        .map(|scanner| scanner.generate_ebnf())
                        .collect(),
                );
            }

            ScannerDefinition::Terminal(string) => {
                return EbnfNode::terminal(string.to_owned());
            }

            ScannerDefinition::TrailingContext { scanner, .. } => {
                return scanner.generate_ebnf();
            }

            ScannerDefinition::ZeroOrMore(scanner) => {
                return EbnfNode::zero_or_more(scanner.generate_ebnf());
            }
        };
    }
}
