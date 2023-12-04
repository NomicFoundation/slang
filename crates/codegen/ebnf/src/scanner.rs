use codegen_schema::types::{ScannerDefinition, ScannerRef};

use crate::nodes::EbnfNode;

impl EbnfNode {
    pub fn from_scanner(scanner: &ScannerRef) -> Self {
        match &scanner.definition {
            ScannerDefinition::Choice(scanners) => {
                return Self::choice(scanners.iter().map(Self::from_scanner).collect());
            }

            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => Self::difference(Self::from_scanner(minuend), Self::from_scanner(subtrahend)),

            ScannerDefinition::Not(scanner) => Self::not(Self::from_scanner(scanner)),

            ScannerDefinition::OneOrMore(scanner) => Self::one_or_more(Self::from_scanner(scanner)),

            ScannerDefinition::Optional(scanner) => Self::optional(Self::from_scanner(scanner)),

            ScannerDefinition::Range { from, to } => Self::range(*from, *to),

            ScannerDefinition::Reference(name) => Self::production_ref(name),

            ScannerDefinition::Sequence(scanners) => {
                return Self::sequence(scanners.iter().map(Self::from_scanner).collect());
            }

            ScannerDefinition::Terminal(terminal) => Self::terminal(terminal),

            ScannerDefinition::TrailingContext { scanner, .. } => Self::from_scanner(scanner),

            ScannerDefinition::ZeroOrMore(scanner) => {
                Self::zero_or_more(Self::from_scanner(scanner))
            }
        }
    }
}
