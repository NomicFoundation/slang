use crate::model::*;

pub fn generate(productions: &Grammar) {
    for (name, expr) in productions {
        print!("{} =", name);
        expr.generate_ebnf();
        println!(" ;")
    }
}

impl Expression {
    fn generate_ebnf(&self) {
        match self {
            Expression::End { .. } => print!(" $"),

            Expression::Any { .. } => print!(" ."),

            Expression::Repeated { expr, .. } => {
                print!(" {{");
                expr.generate_ebnf();
                print!(" }}");
            }

            Expression::Optional { expr, .. } => {
                print!(" [");
                expr.generate_ebnf();
                print!(" ]");
            }

            Expression::Negation { expr, .. } => {
                print!(" ¬");
                expr.generate_ebnf();
            }

            Expression::Choice { exprs, .. } => {
                print!(" ");
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        print!(" |");
                    }
                    expr.generate_ebnf();
                }
            }

            Expression::Sequence { exprs, .. } => {
                for expr in exprs {
                    expr.generate_ebnf();
                }
            }

            Expression::Difference {
                minuend,
                subtrahend,
                ..
            } => {
                minuend.generate_ebnf();
                print!(" -");
                subtrahend.generate_ebnf();
            }
            Expression::Chars { string, .. } => {
                print!(" {:?}", string);
            }

            Expression::Identifier { name, .. } => {
                print!(" {}", name);
            }

            Expression::CharRange { start, end, .. } => {
                print!(" {:?} … {:?}", start, end);
            }
        }
    }
}
