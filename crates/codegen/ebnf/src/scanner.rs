use codegen_schema::types::scanner::{ScannerDefinition, ScannerRef};

use super::ebnf_writer::{EBNFWritable, EBNFWriter};

impl<T: EBNFWriter> EBNFWritable<T> for ScannerRef {
    fn write_ebnf(&self, name: &str, writer: &mut T) {
        writer.write_production_definition(name);
        writer.write_operator(" = ");
        self.definition.write_ebnf("", writer);
        writer.write_operator(" ;");
    }
}

impl<T: EBNFWriter> EBNFWritable<T> for ScannerDefinition {
    fn write_ebnf(&self, _name: &str, writer: &mut T) {
        match self {
            ScannerDefinition::Choice(sub_exprs) => {
                let mut first = true;
                for sub_expr in sub_exprs {
                    if first {
                        first = false;
                    } else {
                        writer.write_operator(" | ");
                    }
                    write_nested(writer, self, &sub_expr.definition);
                }
            }

            ScannerDefinition::DelimitedBy {
                open,
                expression,
                close,
            } => {
                writer.write_string(open);
                writer.write_operator(" ");
                write_nested(writer, &expression.definition, &expression.definition);
                writer.write_operator(" ");
                writer.write_string(close);
            }

            ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                write_nested(writer, self, &minuend.definition);
                writer.write_operator(" - ");
                write_nested(writer, self, &subtrahend.definition);
            }

            ScannerDefinition::Not(sub_expr) => {
                writer.write_operator("¬");
                write_nested(writer, self, &sub_expr.definition);
            }

            ScannerDefinition::OneOrMore(expr) => {
                writer.write_constant("1");
                writer.write_operator("…");
                writer.write_operator("{ ");
                expr.definition.write_ebnf("", writer);
                writer.write_operator(" }");
            }

            ScannerDefinition::Optional(expr) => {
                writer.write_operator("[ ");
                expr.definition.write_ebnf("", writer);
                writer.write_operator(" ]");
            }

            ScannerDefinition::Range { from, to } => {
                writer.write_string(&from.to_string());
                writer.write_operator("…");
                writer.write_string(&to.to_string());
            }

            ScannerDefinition::Reference(name) => writer.write_production_reference(name),

            ScannerDefinition::Repeat {
                min,
                max,
                expression,
            } => {
                writer.write_constant(&min.to_string());
                writer.write_operator("…");
                writer.write_constant(&max.to_string());
                writer.write_operator("{ ");
                expression.definition.write_ebnf("", writer);
                writer.write_operator(" }");
            }

            ScannerDefinition::SeparatedBy {
                expression,
                separator,
            } => {
                write_nested(writer, &expression.definition, &expression.definition);
                writer.write_operator(" { ");
                writer.write_string(separator);
                writer.write_operator(" ");
                write_nested(writer, &expression.definition, &expression.definition);
                writer.write_operator(" }");
            }

            ScannerDefinition::Sequence(sub_exprs) => {
                let mut first = true;
                for sub_expr in sub_exprs {
                    if first {
                        first = false;
                    } else {
                        writer.write_operator(" ");
                    }
                    write_nested(writer, self, &sub_expr.definition);
                }
            }

            ScannerDefinition::Terminal(string) => {
                writer.write_string(string);
            }

            ScannerDefinition::ZeroOrMore(expr) => {
                writer.write_operator("{ ");
                expr.definition.write_ebnf("", writer);
                writer.write_operator(" }");
            }
        }
    }
}

fn write_nested<W: EBNFWriter>(
    writer: &mut W,
    parent_definition: &ScannerDefinition,
    scanner_definition: &ScannerDefinition,
) {
    if precedence(parent_definition) < precedence(scanner_definition) {
        writer.write_operator("( ");
        scanner_definition.write_ebnf("", writer);
        writer.write_operator(" )");
    } else {
        scanner_definition.write_ebnf("", writer);
    }
}

fn precedence(scanner_definition: &ScannerDefinition) -> u8 {
    match scanner_definition {
        ScannerDefinition::OneOrMore(..)
        | ScannerDefinition::Optional(..)
        | ScannerDefinition::Range { .. }
        | ScannerDefinition::Reference(..)
        | ScannerDefinition::Repeat { .. }
        | ScannerDefinition::SeparatedBy { .. }
        | ScannerDefinition::Terminal(..)
        | ScannerDefinition::ZeroOrMore(..) => 0,
        ScannerDefinition::Not(..) => 1,
        ScannerDefinition::Difference { .. } => 2,
        ScannerDefinition::DelimitedBy { .. } | ScannerDefinition::Sequence(..) => 3,
        ScannerDefinition::Choice(..) => 4,
    }
}
