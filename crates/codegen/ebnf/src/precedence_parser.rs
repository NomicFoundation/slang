use codegen_schema::types::precedence_parser::{OperatorModel, PrecedenceParserRef};

use super::ebnf_writer::{EBNFWritable, EBNFWriter};

impl<T: EBNFWriter> EBNFWritable<T> for PrecedenceParserRef {
    fn write_ebnf(&self, name: &str, writer: &mut T) {
        writer.write_production_definition(name);
        writer.write_operator(" = ");

        let mut is_first = true;
        for operator in &self.definition.operators {
            if is_first {
                is_first = false;
            } else {
                writer.write_operator(" | ");
            }
            writer.write_local_production_reference(name, &operator.name);
        }

        for primary_expression in &self.definition.primary_expressions {
            writer.write_operator(" | ");
            writer.write_production_reference(&primary_expression.reference);
        }

        writer.write_operator(" ;");

        for operator in &self.definition.operators {
            writer.write_line_break();
            writer.write_line_start();
            writer.write_production_definition(&operator.name);
            writer.write_operator(" = ");
            match operator.model {
                OperatorModel::BinaryRightAssociative | OperatorModel::BinaryLeftAssociative => {
                    writer.write_production_reference(name);
                    writer.write_operator(" ( ");
                    operator.definition.write_ebnf("", writer);
                    writer.write_operator(" ) ");
                    writer.write_production_reference(name);
                }

                OperatorModel::UnaryPrefix => {
                    writer.write_operator("( ");
                    operator.definition.write_ebnf("", writer);
                    writer.write_operator(" ) ");
                    writer.write_production_reference(name);
                }

                OperatorModel::UnarySuffix => {
                    writer.write_production_reference(name);
                    writer.write_operator(" ( ");
                    operator.definition.write_ebnf("", writer);
                    writer.write_operator(" )");
                }
            }
            writer.write_operator(" ;");
        }
    }
}
