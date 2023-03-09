use codegen_schema::types::precedence_parser::{OperatorModel, PrecedenceParserRef};

use super::ebnf_writer::{EBNFWritable, EBNFWriter};

impl<T: EBNFWriter> EBNFWritable<T> for PrecedenceParserRef {
    fn write_ebnf(&self, name: &str, writer: &mut T) {
        writer.write_global_definition(name);
        writer.write_operator(" = ");

        let mut is_first = true;
        for operator in &self.definition.operators {
            if is_first {
                is_first = false;
            } else {
                writer.write_operator(" | ");
            }
            writer.write_local_reference(name, &operator.name);
        }

        for primary_expression in &self.definition.primary_expressions {
            writer.write_operator(" | ");
            writer.write_local_reference(name, &primary_expression.reference);
        }

        writer.write_operator(" ;");

        for operator in &self.definition.operators {
            writer.write_line_end();
            writer.write_line_start();
            writer.write_local_definition(name, &operator.name);
            writer.write_operator(" = ");

            match operator.model {
                OperatorModel::BinaryRightAssociative | OperatorModel::BinaryLeftAssociative => {
                    writer.write_global_reference(name);
                    writer.write_operator(" ( ");
                    operator.definition.write_ebnf("", writer);
                    writer.write_operator(" ) ");
                    writer.write_global_reference(name);
                }

                OperatorModel::UnaryPrefix => {
                    writer.write_operator("( ");
                    operator.definition.write_ebnf("", writer);
                    writer.write_operator(" ) ");
                    writer.write_global_reference(name);
                }

                OperatorModel::UnaryPostfix => {
                    writer.write_global_reference(name);
                    writer.write_operator(" ( ");
                    operator.definition.write_ebnf("", writer);
                    writer.write_operator(" )");
                }
            }
            writer.write_operator(" ;");
        }
    }
}
