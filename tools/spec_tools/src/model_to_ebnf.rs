use std::fmt::Write;

use crate::model::*;

pub fn generate(productions: &Grammar) -> Result<String, std::fmt::Error> {
    let mut w = String::new();

    let mut first = true;
    for p in productions {
        if first {
            first = false;
        } else {
            writeln!(w)?;
        }
        if let Some(expr) = &p.expr {
            write!(&mut w, "{} =", p.name)?;
            expr.generate_ebnf(&mut w)?;
        }
        writeln!(&mut w, " ;")?;
    }

    Ok(w)
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

    fn generate_ebnf_subexpression<T: Write>(
        &self,
        w: &mut T,
        expr: &ExpressionRef,
    ) -> std::fmt::Result {
        if self.precedence_ebnf() < expr.precedence_ebnf() {
            write!(w, " (")?;
            expr.generate_ebnf(w)?;
            write!(w, " )")?;
        } else {
            expr.generate_ebnf(w)?;
        }

        Ok(())
    }

    fn generate_ebnf<T: Write>(&self, w: &mut T) -> std::fmt::Result {
        match &self.ebnf {
            EBNF::End => write!(w, " $")?,

            EBNF::Any => write!(w, " .")?,

            EBNF::ZeroOrMore(expr) => {
                write!(w, " {{")?;
                expr.generate_ebnf(w)?;
                write!(w, " }}")?;
            }

            EBNF::OneOrMore(expr) => {
                expr.generate_ebnf(w)?;
                write!(w, " {{")?;
                expr.generate_ebnf(w)?;
                write!(w, " }}")?;
            }

            EBNF::Optional(expr) => {
                write!(w, " [")?;
                expr.generate_ebnf(w)?;
                write!(w, " ]")?;
            }

            EBNF::Not(expr) => {
                write!(w, " ¬")?;
                self.generate_ebnf_subexpression(w, expr)?;
            }

            EBNF::Choice(exprs) => {
                let mut first = true;
                for expr in exprs {
                    if first {
                        first = false;
                    } else {
                        write!(w, " |")?;
                    }
                    self.generate_ebnf_subexpression(w, expr)?;
                }
            }

            EBNF::Sequence(exprs) => {
                for expr in exprs {
                    self.generate_ebnf_subexpression(w, expr)?;
                }
            }

            EBNF::Reference(name) => {
                write!(w, " {}", name)?;
            }

            EBNF::Terminal(string) => {
                write!(w, " '")?;
                for c in string.chars() {
                    if c == '\'' || c == '\\' {
                        write!(w, "\\{}", c)?;
                    } else if c.is_ascii_graphic() || c == '¬' || c == '…' {
                        write!(w, "{}", c)?;
                    } else {
                        write!(w, "{}", c.escape_unicode().to_string())?;
                    }
                }
                write!(w, "'")?;
            }

            EBNF::Difference(EBNFDifference {
                minuend,
                subtrahend,
            }) => {
                self.generate_ebnf_subexpression(w, minuend)?;
                write!(w, " -")?;
                self.generate_ebnf_subexpression(w, subtrahend)?;
            }

            EBNF::Range(EBNFRange { from, to }) => {
                write!(w, " '")?;
                if from.is_ascii_graphic() || *from == '¬' || *from == '…' {
                    write!(w, "{}", from)?
                } else {
                    write!(w, "{}", from.escape_unicode().to_string())?
                }
                write!(w, "'…'")?;
                if to.is_ascii_graphic() || *to == '¬' || *to == '…' {
                    write!(w, "{}", to)?
                } else {
                    write!(w, "{}", to.escape_unicode().to_string())?
                }
                write!(w, "'")?;
            }
        }

        Ok(())
    }
}
