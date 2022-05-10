use crate::model::*;

pub fn generate(productions: &Grammar) {
    let mut first = true;
    for p in productions {
        if first {
            first = false;
        } else {
            println!();
        }
        print!("{} =", p.name);
        p.expr.generate_ebnf();
        println!(" ;")
    }
}

impl Expression {
    fn precedence_ebnf(&self) -> u8 {
        match self.ebnf {
            EBNF::End { .. } => 0,
            EBNF::Any { .. } => 0,
            EBNF::Repeated { .. } => 0,
            EBNF::Optional { .. } => 0,
            EBNF::Negation { .. } => 1,
            EBNF::Choice { .. } => 4,
            EBNF::Sequence { .. } => 3,
            EBNF::Difference { .. } => 2,
            EBNF::Chars { .. } => 0,
            EBNF::Identifier { .. } => 0,
            EBNF::CharRange { .. } => 0,
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
        match &self.ebnf {
            EBNF::End { .. } => print!(" $"),

            EBNF::Any { .. } => print!(" ."),

            EBNF::Repeated { expr, .. } => {
                print!(" {{");
                expr.generate_ebnf();
                print!(" }}");
            }

            EBNF::Optional { expr, .. } => {
                print!(" [");
                expr.generate_ebnf();
                print!(" ]");
            }

            EBNF::Negation { expr, .. } => {
                print!(" ¬");
                self.generate_ebnf_subexpression(expr);
            }

            EBNF::Choice { exprs, .. } => {
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

            EBNF::Sequence { exprs, .. } => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(expr);
                }
            }

            EBNF::Difference {
                minuend,
                subtrahend,
                ..
            } => {
                self.generate_ebnf_subexpression(minuend);
                print!(" -");
                self.generate_ebnf_subexpression(subtrahend);
            }

            EBNF::Identifier { name, .. } => {
                print!(" {}", name);
            }

            EBNF::Chars { string, .. } => {
                print!(" '");
                for c in string.chars() {
                    if c == '\'' || c == '\\' {
                        print!("\\{}", c)
                    } else if c.is_ascii_graphic() || c == '¬' || c == '…' {
                        print!("{}", c)
                    } else {
                        print!("{}", c.escape_unicode().to_string())
                    }
                }
                print!("'");
            }

            EBNF::CharRange { start, end, .. } => {
                print!(" '");
                if start.is_ascii_graphic() || *start == '¬' || *start == '…' {
                    print!("{}", start)
                } else {
                    print!("{}", start.escape_unicode().to_string())
                }
                print!("'…'");
                if end.is_ascii_graphic() || *end == '¬' || *end == '…' {
                    print!("{}", end)
                } else {
                    print!("{}", end.escape_unicode().to_string())
                }
                print!("'");
            }
        }
    }
}
