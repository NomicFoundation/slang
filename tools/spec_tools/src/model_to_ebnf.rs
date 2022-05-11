use crate::model::*;

pub fn generate(productions: &Grammar) {
    let mut first = true;
    for p in productions {
        if first {
            first = false;
        } else {
            println!();
        }
        if let Some(expr) = &p.expr {
            print!("{} =", p.name);
            expr.generate_ebnf();
        }
        println!(" ;")
    }
}

impl Expression {
    fn precedence_ebnf(&self) -> u8 {
        match self.ebnf {
            EBNF::End
            | EBNF::Any
            | EBNF::ZeroOrMore(..)
            | EBNF::Optional(..)
            | EBNF::Terminal(..)
            | EBNF::Reference(..)
            | EBNF::Range { .. } => 0,
            EBNF::Not(..) => 1,
            EBNF::Difference { .. } => 2,
            EBNF::Sequence(..) | EBNF::OneOrMore(..) => 3,
            EBNF::Choice(..) => 4,
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
            EBNF::End => print!(" $"),

            EBNF::Any => print!(" ."),

            EBNF::ZeroOrMore(expr) => {
                print!(" {{");
                expr.generate_ebnf();
                print!(" }}");
            }

            EBNF::OneOrMore(expr) => {
                expr.generate_ebnf();
                print!(" {{");
                expr.generate_ebnf();
                print!(" }}");
            }

            EBNF::Optional(expr) => {
                print!(" [");
                expr.generate_ebnf();
                print!(" ]");
            }

            EBNF::Not(expr) => {
                print!(" ¬");
                self.generate_ebnf_subexpression(expr);
            }

            EBNF::Choice(exprs) => {
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

            EBNF::Sequence(exprs) => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(expr);
                }
            }

            EBNF::Reference(name) => {
                print!(" {}", name);
            }

            EBNF::Terminal(string) => {
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

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                self.generate_ebnf_subexpression(minuend);
                print!(" -");
                self.generate_ebnf_subexpression(subtrahend);
            }

            EBNF::Range(EBNFRange { from, to }) => {
                print!(" '");
                if from.is_ascii_graphic() || *from == '¬' || *from == '…' {
                    print!("{}", from)
                } else {
                    print!("{}", from.escape_unicode().to_string())
                }
                print!("'…'");
                if to.is_ascii_graphic() || *to == '¬' || *to == '…' {
                    print!("{}", to)
                } else {
                    print!("{}", to.escape_unicode().to_string())
                }
                print!("'");
            }
        }
    }
}
