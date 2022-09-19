use std::fmt::Write;

use codegen_schema::{
    EBNFDelimitedBy, EBNFDifference, EBNFRange, EBNFRepeat, EBNFSeparatedBy, Expression,
    ExpressionRef, Grammar, EBNF,
};

use crate::production::ProductionEBNFPrivateExtensions;

pub(crate) trait ExpressionEBNFPrivateExtensions {
    fn generate_ebnf<T: Write>(&self, grammar: &Grammar, w: &mut T);
    fn generate_ebnf_subexpression<T: Write>(
        &self,
        grammar: &Grammar,
        w: &mut T,
        expr: &ExpressionRef,
    );
    fn ebnf_precedence(&self) -> u8;
}

impl ExpressionEBNFPrivateExtensions for Expression {
    fn generate_ebnf<T: Write>(&self, grammar: &Grammar, w: &mut T) {
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

            EBNF::DelimitedBy(EBNFDelimitedBy { open, expr, close }) => {
                write!(w, "'").unwrap();
                for c in open.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
                self.generate_ebnf_subexpression(grammar, w, expr);
                write!(w, "'").unwrap();
                for c in close.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                self.generate_ebnf_subexpression(grammar, w, minuend);
                write!(w, "- ").unwrap();
                self.generate_ebnf_subexpression(grammar, w, subtrahend);
            }

            EBNF::Not(expr) => {
                write!(w, "¬").unwrap();
                self.generate_ebnf_subexpression(grammar, w, expr);
            }

            EBNF::OneOrMore(expr) => {
                write!(w, "1…").unwrap();
                write!(w, "*{{ ").unwrap();
                expr.generate_ebnf(grammar, w);
                write!(w, "}} ").unwrap();
            }

            EBNF::Optional(expr) => {
                write!(w, "[ ").unwrap();
                expr.generate_ebnf(grammar, w);
                write!(w, "] ").unwrap();
            }

            EBNF::Range(EBNFRange { from, to }) => {
                write!(w, "'").unwrap();
                write_char(w, *from);
                write!(w, "'…'").unwrap();
                write_char(w, *to);
                write!(w, "' ").unwrap();
            }

            EBNF::Reference(name) => {
                let production = grammar.get_production(name);
                write!(w, "{} ", production.ebnf_display_name()).unwrap();
            }

            EBNF::Repeat(EBNFRepeat { expr, min, max, .. }) => {
                write!(w, "{}", min).unwrap();
                write!(w, "…").unwrap();
                write!(w, "{}", max).unwrap();
                write!(w, "*{{ ").unwrap();
                expr.generate_ebnf(grammar, w);
                write!(w, "}} ").unwrap();
            }

            EBNF::SeparatedBy(EBNFSeparatedBy { expr, separator }) => {
                self.generate_ebnf_subexpression(grammar, w, expr);
                write!(w, " {{ '").unwrap();
                for c in separator.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
                self.generate_ebnf_subexpression(grammar, w, expr);
                write!(w, "}} ").unwrap();
            }

            EBNF::Sequence(exprs) => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(grammar, w, expr);
                }
            }

            EBNF::Terminal(string) => {
                write!(w, "'").unwrap();
                for c in string.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
            }

            EBNF::ZeroOrMore(expr) => {
                write!(w, "{{ ").unwrap();
                expr.generate_ebnf(grammar, w);
                write!(w, "}} ").unwrap();
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
            EBNF::Choice(..) => 4,
            EBNF::DelimitedBy(..) | EBNF::SeparatedBy(..) | EBNF::Sequence(..) => 3,
            EBNF::Difference { .. } => 2,
            EBNF::OneOrMore(..)
            | EBNF::Optional(..)
            | EBNF::Range { .. }
            | EBNF::Reference(..)
            | EBNF::Repeat(..)
            | EBNF::Terminal(..)
            | EBNF::ZeroOrMore(..) => 0,
            EBNF::Not(..) => 1,
        }
    }
}
