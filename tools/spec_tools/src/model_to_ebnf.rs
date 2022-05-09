use crate::model::*;

pub fn generate(productions: &Grammar) {
    let mut first = true;
    for (name, expr) in productions {
        if first {
            first = false;
        } else {
            println!();
        }
        print!("{} =", name);
        expr.generate_ebnf();
        println!(" ;")
    }
}

impl Expression {
    fn precedence_ebnf(&self) -> u8 {
        match self {
            Expression::End { .. } => 0,
            Expression::Any { .. } => 0,
            Expression::Repeated { .. } => 0,
            Expression::Optional { .. } => 0,
            Expression::Negation { .. } => 1,
            Expression::Choice { .. } => 4,
            Expression::Sequence { .. } => 3,
            Expression::Difference { .. } => 2,
            Expression::Chars { .. } => 0,
            Expression::Identifier { .. } => 0,
            Expression::CharRange { .. } => 0,
        }
    }

    fn generate_ebnf_subexpression(&self, expr: &ExpressionRef) {
        if self.precedence_ebnf() < expr.precedence_ebnf() {
            print!(" (");
            expr.generate_ebnf();
            print!(" )")
        } else {
            expr.generate_ebnf();
        }
    }

    fn generate_ebnf(&self) {
        match self {
            Expression::End { .. } => print!(" $"),

            Expression::Any { .. } => print!(" ."),

            Expression::Repeated { expr, .. } => {
                print!(" {{");
                self.generate_ebnf_subexpression(expr);
                print!(" }}");
            }

            Expression::Optional { expr, .. } => {
                print!(" [");
                expr.generate_ebnf();
                print!(" ]");
            }

            Expression::Negation { expr, .. } => {
                print!(" ¬");
                self.generate_ebnf_subexpression(expr);
            }

            Expression::Choice { exprs, .. } => {
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        print!(" |");
                    }
                    self.generate_ebnf_subexpression(expr);
                }
            }

            Expression::Sequence { exprs, .. } => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(expr);
                }
            }

            Expression::Difference {
                minuend,
                subtrahend,
                ..
            } => {
                self.generate_ebnf_subexpression(minuend);
                print!(" -");
                self.generate_ebnf_subexpression(subtrahend);
            }

            Expression::Identifier { name, .. } => {
                print!(" {}", name);
            }

            Expression::Chars { string, .. } => {
                // TODO: encode properly
                print!(" {:?}", string);
            }

            Expression::CharRange { start, end, .. } => {
                // TODO: encode properly
                print!(" {:?} … {:?}", start, end);
            }
        }
    }
}
