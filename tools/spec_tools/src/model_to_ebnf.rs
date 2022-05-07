use crate::model::*;

pub fn generate(productions: &Grammar) {
    for (name, expr) in productions {
        println!("---");
        println!("PRODUCTION:");
        print!("  {}:", name);
        expr.generate_ebnf();
        println!()
    }
}

impl Expression {
    fn generate_ebnf(&self) {
        match self {
            Expression::End { .. } => print!(" EOF"),
            Expression::Any { .. } => print!(" ANY"),
            Expression::Repeated { expr, .. } => {
                print!(" {{ repeated:");
                expr.generate_ebnf();
                print!(" }}");
            }
            Expression::Optional { expr, .. } => {
                print!(" {{ optional:");
                expr.generate_ebnf();
                print!(" }}");
            }
            Expression::Negation { expr, .. } => {
                print!(" {{ not:");
                expr.generate_ebnf();
                print!(" }}");
            }

            Expression::Choice { exprs, .. } => {
                print!(" {{ alternatives: [");
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        print!(",");
                    }
                    expr.generate_ebnf();
                }
                print!(" ] }}");
            }

            Expression::Sequence { exprs, .. } => {
                print!(" {{ sequence: [");
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        print!(",");
                    }
                    expr.generate_ebnf();
                }
                print!(" ] }}");
            }

            Expression::Difference {
                minuend,
                subtrahend,
                ..
            } => {
                print!(" {{ minuend:");
                minuend.generate_ebnf();
                print!(", subtrahend:");
                subtrahend.generate_ebnf();
                print!(" }}");
            }
            Expression::Chars { string, .. } => {
                print!(" {:?}", string);
            }

            Expression::Identifier { name, .. } => {
                print!(" {}", name);
            }

            Expression::CharRange { start, end, .. } => {
                print!(" {{ range: [{:?}, {:?}] }}", start, end);
            }
        }
    }
}
