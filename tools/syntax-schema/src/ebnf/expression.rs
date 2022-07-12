use crate::schema::*;

use std::fmt::Write;

impl Expression {
    pub(crate) fn generate_ebnf<T: Write>(&self, grammar: &Grammar, w: &mut T) {
        fn write_char<T: Write>(w: &mut T, c: char) {
            if c == '\'' || c == '\\' {
                write!(w, "\\{}", c).unwrap();
            } else if c.is_ascii_graphic()
                || c == '¬'
                || c == '…'
                || c == '«'
                || c == '»'
                || c == '∞'
            {
                write!(w, "{}", c).unwrap();
            } else {
                write!(w, "{}", c.escape_unicode().to_string()).unwrap();
            }
        }

        match &self.ebnf {
            EBNF::End => write!(w, "$ ").unwrap(),

            EBNF::Repeat(EBNFRepeat {
                min,
                max,
                expr,
                separator,
            }) => match (min, max) {
                (0, None) => {
                    write!(w, "{{ ").unwrap();
                    expr.generate_ebnf(grammar, w);
                    if let Some(separator) = separator {
                        write!(w, "/ ").unwrap();
                        separator.generate_ebnf(grammar, w);
                    }
                    write!(w, "}} ").unwrap();
                }
                (0, Some(1)) => {
                    write!(w, "[ ").unwrap();
                    expr.generate_ebnf(grammar, w);
                    write!(w, "] ").unwrap();
                }
                _ => {
                    if *min != 0 {
                        write!(w, "{}", min).unwrap();
                    }
                    write!(w, "…").unwrap();
                    if let Some(max) = max {
                        write!(w, "{}", max).unwrap();
                    }
                    write!(w, "*{{ ").unwrap();
                    expr.generate_ebnf(grammar, w);
                    if let Some(separator) = separator {
                        write!(w, "/ ").unwrap();
                        separator.generate_ebnf(grammar, w);
                    }
                    write!(w, "}} ").unwrap();
                }
            },

            EBNF::Not(expr) => {
                write!(w, "¬").unwrap();
                self.generate_ebnf_subexpression(grammar, w, expr);
            }

            EBNF::Choice(exprs) => {
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        write!(w, "| ").unwrap();
                    }
                    self.generate_ebnf_subexpression(grammar, w, expr);
                }
            }

            EBNF::Sequence(exprs) => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(grammar, w, expr);
                }
            }

            EBNF::Reference(name) => {
                let production = grammar.get_production(name);
                write!(w, "{} ", production.ebnf_display_name()).unwrap();
            }

            EBNF::Terminal(string) => {
                write!(w, "'").unwrap();
                for c in string.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
            }

            EBNF::DelimitedBy(EBNFDelimitedBy { open, expr, close }) => {
                write!(w, "{} ", open).unwrap();
                self.generate_ebnf_subexpression(grammar, w, expr);
                write!(w, "{} ", close).unwrap();
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                self.generate_ebnf_subexpression(grammar, w, minuend);
                write!(w, "- ").unwrap();
                self.generate_ebnf_subexpression(grammar, w, subtrahend);
            }

            EBNF::Range(EBNFRange { from, to }) => {
                write!(w, "'").unwrap();
                write_char(w, *from);
                write!(w, "'…'").unwrap();
                write_char(w, *to);
                write!(w, "' ").unwrap();
            }
        }
    }

    fn generate_ebnf_subexpression<T: Write>(
        &self,
        grammar: &Grammar,
        w: &mut T,
        expr: &ExpressionRef,
    ) {
        if self.ebnf_precedence() < expr.ebnf_precedence() {
            write!(w, "( ").unwrap();
            expr.generate_ebnf(grammar, w);
            write!(w, ") ").unwrap();
        } else {
            expr.generate_ebnf(grammar, w);
        }
    }

    fn ebnf_precedence(&self) -> u8 {
        match self.ebnf {
            EBNF::End
            | EBNF::Repeat(..)
            | EBNF::Terminal(..)
            | EBNF::Reference(..)
            | EBNF::Range { .. } => 0,
            EBNF::Not(..) => 1,
            EBNF::Difference { .. } => 2,
            EBNF::Sequence(..) | EBNF::DelimitedBy(..) => 3,
            EBNF::Choice(..) => 4,
        }
    }
}
