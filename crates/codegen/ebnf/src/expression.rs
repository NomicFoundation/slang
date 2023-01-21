use std::fmt::Write;

use codegen_schema::types::{
    grammar::Grammar,
    productions::{Expression, ExpressionParser, ExpressionRef},
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

        match &self.parser {
            ExpressionParser::Choice(exprs) => {
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

            ExpressionParser::DelimitedBy {
                open,
                expression,
                close,
            } => {
                write!(w, "'").unwrap();
                for c in open.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
                self.generate_ebnf_subexpression(grammar, w, expression);
                write!(w, "'").unwrap();
                for c in close.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
            }

            ExpressionParser::Difference {
                minuend,
                subtrahend,
            } => {
                self.generate_ebnf_subexpression(grammar, w, minuend);
                write!(w, "- ").unwrap();
                self.generate_ebnf_subexpression(grammar, w, subtrahend);
            }

            ExpressionParser::Not(expr) => {
                write!(w, "¬").unwrap();
                self.generate_ebnf_subexpression(grammar, w, expr);
            }

            ExpressionParser::OneOrMore(expr) => {
                write!(w, "1…").unwrap();
                write!(w, "*{{ ").unwrap();
                expr.generate_ebnf(grammar, w);
                write!(w, "}} ").unwrap();
            }

            ExpressionParser::Optional(expr) => {
                write!(w, "[ ").unwrap();
                expr.generate_ebnf(grammar, w);
                write!(w, "] ").unwrap();
            }

            ExpressionParser::Range { from, to } => {
                write!(w, "'").unwrap();
                write_char(w, *from);
                write!(w, "'…'").unwrap();
                write_char(w, *to);
                write!(w, "' ").unwrap();
            }

            ExpressionParser::Reference(name) => {
                let production = &grammar.productions[name];
                write!(w, "{} ", production.ebnf_display_name()).unwrap();
            }

            ExpressionParser::Repeat {
                expression,
                min,
                max,
                ..
            } => {
                write!(w, "{}", min).unwrap();
                write!(w, "…").unwrap();
                write!(w, "{}", max).unwrap();
                write!(w, "*{{ ").unwrap();
                expression.generate_ebnf(grammar, w);
                write!(w, "}} ").unwrap();
            }

            ExpressionParser::SeparatedBy {
                expression,
                separator,
            } => {
                self.generate_ebnf_subexpression(grammar, w, expression);
                write!(w, " {{ '").unwrap();
                for c in separator.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
                self.generate_ebnf_subexpression(grammar, w, expression);
                write!(w, "}} ").unwrap();
            }

            ExpressionParser::Sequence(exprs) => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(grammar, w, expr);
                }
            }

            ExpressionParser::Terminal(string) => {
                write!(w, "'").unwrap();
                for c in string.chars() {
                    write_char(w, c);
                }
                write!(w, "' ").unwrap();
            }

            ExpressionParser::ZeroOrMore(expr) => {
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
        match self.parser {
            ExpressionParser::Choice(..) => 4,
            ExpressionParser::DelimitedBy { .. }
            | ExpressionParser::SeparatedBy { .. }
            | ExpressionParser::Sequence(..) => 3,
            ExpressionParser::Difference { .. } => 2,
            ExpressionParser::OneOrMore(..)
            | ExpressionParser::Optional(..)
            | ExpressionParser::Range { .. }
            | ExpressionParser::Reference(..)
            | ExpressionParser::Repeat { .. }
            | ExpressionParser::Terminal(..)
            | ExpressionParser::ZeroOrMore(..) => 0,
            ExpressionParser::Not(..) => 1,
        }
    }
}
